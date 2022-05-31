use std::convert::AsRef;
use std::path::Path;
use std::sync::Once;

use ffmpeg_next::codec::context::Context as FFCodecContext;
use ffmpeg_next::codec::packet::packet::Packet as FFPacket;
use ffmpeg_next::decoder::Audio as FFDecoder;
use ffmpeg_next::format::context::Input as FFInput;
use ffmpeg_next::format::input as new_ffinput;
use ffmpeg_next::frame::Audio as FFSamplesBuffer;
use ffmpeg_next::util::error::ENOENT;
use ffmpeg_next::util::format::sample::Sample as FFSampleFormat;
use ffmpeg_next::util::format::sample::Sample::{F32, F64, I16, I32};
use ffmpeg_next::util::format::sample::Type::{Packed, Planar};
use ffmpeg_next::util::log::level::Level as FFLogLevel;
use ffmpeg_next::Error as FFError;
use ffmpeg_next::Stream as FFStream;

use crate::backend::display::est_num_frames_to_str;
use crate::backend::Error;
use crate::backend::Sample;
use crate::backend::Signal;
use crate::backend::Source;

static FFMPEG_INIT: Once = Once::new();

#[allow(clippy::missing_panics_doc)]
#[inline]
pub fn ffmpeg_init() {
    FFMPEG_INIT.call_once(|| {
        ffmpeg_next::init().unwrap();
        ffmpeg_next::util::log::set_level(FFLogLevel::Quiet);
    });
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn estimate_num_frames_inner(
    stream_duration: i64,
    stream_tb_n: i32,
    stream_tb_d: i32,
    decoder_tb_n: i32,
    decoder_tb_d: i32,
) -> usize {
    #[allow(clippy::cast_precision_loss)]
    let mut x = stream_duration as f64;
    x *= f64::from(decoder_tb_d);
    x *= f64::from(stream_tb_n);
    x /= f64::from(stream_tb_d);
    x /= f64::from(decoder_tb_n);
    x = x.ceil();
    x as usize
}

#[inline]
pub fn estimate_num_frames(stream: &FFStream, decoder: &FFDecoder) -> usize {
    let stream_duration = stream.duration();
    let stream_tb = stream.time_base();
    let decoder_tb = decoder.time_base();
    estimate_num_frames_inner(
        stream_duration,
        stream_tb.0,
        stream_tb.1,
        decoder_tb.0,
        decoder_tb.1,
    )
}

fn get_first_working_audio_stream(input: &FFInput) -> Result<(FFStream, FFDecoder), Error> {
    let mut num_found_streams = 0;
    for input_stream in input.streams() {
        num_found_streams += 1;
        match FFCodecContext::from_parameters(input_stream.parameters()) {
            Err(_) => continue,
            Ok(codec_context) => match codec_context.decoder().audio() {
                Err(_) => continue,
                Ok(mut decoder) => {
                    if decoder.set_parameters(input_stream.parameters()).is_err() {
                        continue;
                    }
                    return Ok((input_stream, decoder));
                }
            },
        }
    }
    Err(Error::NoSuitableAudioStreams(num_found_streams))
}

/// Retrieve an individual audio sample from a **PACKED** FFmpeg frame.
#[inline]
unsafe fn get_sample_packed<T: Sample>(
    sample_buffer: &FFSamplesBuffer,
    frame_idx: usize,
    channel_idx: usize,
) -> f32 {
    let sample_idx: usize = frame_idx * sample_buffer.channels() as usize + channel_idx;
    // When audio is in a "packed" format, FFmpeg stores
    // each sample interleaved in the first data plane.
    let plane_ptr: *const T = (*sample_buffer.as_ptr()).data[0] as *const T;
    let sample: T = *plane_ptr.add(sample_idx);
    sample.as_f32_sample()
}

/// Retrieve an individual audio sample from a **PLANAR** FFmpeg frame.
#[inline]
unsafe fn get_sample_planar<T: Sample>(
    sample_buffer: &FFSamplesBuffer,
    frame_idx: usize,
    channel_idx: usize,
) -> f32 {
    // When audio is stored in a "planar" format, FFmpeg
    // stores the first eight planes in the `.data` attribute.
    // If there are more than 8 planes, all of them are
    // available in the `.extended_data` attribute.
    // If there are not more than 8 planes, then
    // `.extended_data` just points to `.data`.
    let extended_data_ptr: *const *const T =
        (*sample_buffer.as_ptr()).extended_data as *const *const T;
    let plane_ptr: *const T = *extended_data_ptr.add(channel_idx);
    let sample: T = *plane_ptr.add(frame_idx);
    sample.as_f32_sample()
}

/// Retrieve an individual audio sample from an FFmpeg frame.
///
/// This function checks whether the frame's sample format is packed or planar.
#[inline]
unsafe fn get_sample(sample_buffer: &FFSamplesBuffer, frame_idx: usize, channel_idx: usize) -> f32 {
    match sample_buffer.format() {
        //
        // Packed
        I16(Packed) => get_sample_packed::<i16>(sample_buffer, frame_idx, channel_idx),
        I32(Packed) => get_sample_packed::<i32>(sample_buffer, frame_idx, channel_idx),
        F32(Packed) => get_sample_packed::<f32>(sample_buffer, frame_idx, channel_idx),
        F64(Packed) => get_sample_packed::<f64>(sample_buffer, frame_idx, channel_idx),
        //
        // Planar
        I16(Planar) => get_sample_planar::<i16>(sample_buffer, frame_idx, channel_idx),
        I32(Planar) => get_sample_planar::<i32>(sample_buffer, frame_idx, channel_idx),
        F32(Planar) => get_sample_planar::<f32>(sample_buffer, frame_idx, channel_idx),
        F64(Planar) => get_sample_planar::<f64>(sample_buffer, frame_idx, channel_idx),
        _ => panic!("FFmpegDecoder cannot decode the sample type."),
    }
}

/// Returns `true` if the given FFmpeg sample format is supported by Babycat.
#[inline]
fn sample_format_is_supported(format: FFSampleFormat) -> bool {
    matches!(
        format,
        I16(Packed | Planar) | I32(Packed | Planar) | F32(Packed | Planar) | F64(Packed | Planar)
    )
}

pub struct FFmpegDecoder {
    input: FFInput,
    decoder: FFDecoder,
    stream_index: usize,
    num_samples_remaining: usize,
    packet: Option<FFPacket>,
    samples_buffer: Option<FFSamplesBuffer>,
    buf_frame_idx: usize,
    buf_channel_idx: usize,
}

impl std::fmt::Debug for FFmpegDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "FFmpegDecoder {{ {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl FFmpegDecoder {
    fn from_ff_input(input: FFInput) -> Result<Self, Error> {
        let (stream, decoder) = get_first_working_audio_stream(&input)?;
        if !sample_format_is_supported(decoder.format()) {
            // TODO: Replace with an error about the invalid sample format.
            return Err(Error::UnknownDecodeError);
        }
        let num_channels = decoder.channels() as usize;
        let num_samples_remaining = estimate_num_frames(&stream, &decoder) * num_channels;

        let stream_index = stream.index();
        let mut new_self = Self {
            input,
            decoder,
            stream_index,
            num_samples_remaining,
            packet: None,
            samples_buffer: None,
            buf_frame_idx: 0,
            buf_channel_idx: 0,
        };
        new_self.packet = new_self.next_packet();
        Ok(new_self)
    }

    pub fn from_file<F: Clone + AsRef<Path>>(filename: F) -> Result<Self, Error> {
        ffmpeg_init();
        let filename = filename.as_ref();
        match new_ffinput(&filename) {
            Ok(input) => Self::from_ff_input(input),
            // File not found error. Audio filename was not found on the local filesystem.
            Err(FFError::Other { errno: ENOENT }) => Err(Error::FileNotFound(Box::leak(
                filename.to_string_lossy().into_owned().into_boxed_str(),
            ))),
            Err(err) => Err(Error::UnknownDecodeErrorWithMessage(leak_str!(
                err.to_string()
            ))),
        }
    }

    #[inline]
    fn next_packet(&mut self) -> Option<FFPacket> {
        let mut packet = FFPacket::empty();
        loop {
            match packet.read(&mut self.input) {
                Ok(..) => {
                    if packet.stream() == self.stream_index
                        && self.decoder.send_packet(&packet).is_ok()
                    {
                        return Some(packet);
                    }
                }
                Err(FFError::Eof) => {
                    let _ = self.decoder.send_eof();
                    return None;
                }
                Err(..) => (),
            }
        }
    }

    #[inline]
    fn next_samples_buffer(&mut self) -> Option<FFSamplesBuffer> {
        let mut fsb = FFSamplesBuffer::empty();
        if self.decoder.receive_frame(&mut fsb).is_err() {
            return None;
        }
        Some(fsb)
    }
}

impl Source for FFmpegDecoder {}

impl Signal for FFmpegDecoder {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        self.decoder.rate()
    }

    #[inline]
    fn num_channels(&self) -> u16 {
        self.decoder.channels()
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        let num_channels = self.decoder.channels() as usize;
        Some(self.num_samples_remaining / num_channels)
    }
}

impl Iterator for FFmpegDecoder {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.num_samples_remaining, None)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.samples_buffer {
                None => {
                    // If the samples buffer is None, then get the next packet.
                    self.packet = self.next_packet();
                    // If the packet is None, then we return None.
                    self.packet.as_ref()?;
                    // If the packet is Some, then we get the next samples buffer.
                    self.samples_buffer = self.next_samples_buffer();
                    continue;
                }
                Some(samples_buffer) => {
                    let buf_num_frames = samples_buffer.samples();
                    if self.buf_frame_idx < buf_num_frames {
                        // If we have not reached the end of the sample buffer,
                        // then get the next sample.
                        let sample: f32 = unsafe {
                            get_sample(samples_buffer, self.buf_frame_idx, self.buf_channel_idx)
                        };
                        self.buf_channel_idx += 1;
                        self.num_samples_remaining = self.num_samples_remaining.saturating_sub(1);
                        // If we have reached the last channel in the current frame,
                        // then move onto the next frame.
                        let num_channels = samples_buffer.channels() as usize;
                        if self.buf_channel_idx >= num_channels {
                            self.buf_frame_idx += 1;
                            self.buf_channel_idx = 0;
                        }
                        return Some(sample);
                    }
                    // If we have reached the end of the current sample buffer,
                    // then we fetch the next buffer.
                    self.samples_buffer = self.next_samples_buffer();
                    self.buf_channel_idx = 0;
                    self.buf_frame_idx = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::FFmpegDecoder;
    use crate::Signal;
    use crate::Source;

    #[test]
    fn test_first_4_samples() {
        let mut decoder = FFmpegDecoder::from_file("audio-for-tests/circus-of-freaks/track.flac")
            .expect("decoding error");
        let frame_rate_hz: u32 = 44100;
        let num_channels: u16 = 2;
        let num_frames_estimate: usize = 2491247;
        let num_samples_estimate: usize = num_frames_estimate * num_channels as usize;

        // Before any samples have been decoded.
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate));
        assert_eq!(decoder.size_hint(), (num_samples_estimate, None));

        // Sample 1.
        decoder.next();
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate - 1));
        assert_eq!(decoder.size_hint(), (num_samples_estimate - 1, None));

        // Sample 2.
        decoder.next();
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate - 1));
        assert_eq!(decoder.size_hint(), (num_samples_estimate - 2, None));

        // Sample 3.
        decoder.next();
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate - 2));
        assert_eq!(decoder.size_hint(), (num_samples_estimate - 3, None));

        // Sample 4.
        decoder.next();
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate - 2));
        assert_eq!(decoder.size_hint(), (num_samples_estimate - 4, None));

        let remaining: Vec<f32> = decoder.collect_interleaved_samples();
        assert_eq!(remaining.len(), num_samples_estimate - 4);
    }
}
