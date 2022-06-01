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
    sample_format: FFSampleFormat,
    stream_index: usize,
    frame_rate_hz: u32,
    num_channels: usize,
    num_samples_remaining: usize,
    packet: Option<FFPacket>,
    samples_buffer: Option<FFSamplesBuffer>,
    buf_num_frames: usize,
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
        let sample_format = decoder.format();
        if !sample_format_is_supported(sample_format) {
            // TODO: Replace with an error about the invalid sample format.
            return Err(Error::UnknownDecodeError);
        }
        let frame_rate_hz = decoder.rate();
        let num_channels = decoder.channels() as usize;
        let num_samples_remaining = estimate_num_frames(&stream, &decoder) * num_channels;

        let stream_index = stream.index();
        Ok(Self {
            input,
            decoder,
            sample_format,
            stream_index,
            frame_rate_hz,
            num_channels,
            num_samples_remaining,
            packet: None,
            samples_buffer: None,
            buf_num_frames: 0,
            buf_frame_idx: 0,
            buf_channel_idx: 0,
        })
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
        self.buf_channel_idx = 0;
        self.buf_frame_idx = 0;
        let mut fsb = FFSamplesBuffer::empty();
        if self.decoder.receive_frame(&mut fsb).is_err() {
            self.buf_num_frames = 0;
            return None;
        }
        self.buf_num_frames = fsb.samples();
        Some(fsb)
    }

    /// Retrieve an individual audio sample from a **PACKED** FFmpeg frame.
    #[inline]
    unsafe fn get_sample_packed<T: Sample>(&self) -> f32 {
        let samples_buffer: &FFSamplesBuffer = self.samples_buffer.as_ref().unwrap_unchecked();
        let sample_idx: usize = self.buf_frame_idx * self.num_channels + self.buf_channel_idx;
        // When audio is in a "packed" format, FFmpeg stores
        // each sample interleaved in the first data plane.
        let plane_ptr: *const T = (*samples_buffer.as_ptr()).data[0] as *const T;
        let sample: T = *plane_ptr.add(sample_idx);
        sample.as_f32_sample()
    }

    /// Retrieve an individual audio sample from a **PLANAR** FFmpeg frame.
    #[inline]
    unsafe fn get_sample_planar<T: Sample>(&self) -> f32 {
        let samples_buffer: &FFSamplesBuffer = self.samples_buffer.as_ref().unwrap_unchecked();
        // When audio is stored in a "planar" format, FFmpeg
        // stores the first eight planes in the `.data` attribute.
        // If there are more than 8 planes, all of them are
        // available in the `.extended_data` attribute.
        // If there are not more than 8 planes, then
        // `.extended_data` just points to `.data`.
        let extended_data_ptr: *const *const T =
            (*samples_buffer.as_ptr()).extended_data as *const *const T;
        let plane_ptr: *const T = *extended_data_ptr.add(self.buf_channel_idx);
        let sample: T = *plane_ptr.add(self.buf_frame_idx);
        sample.as_f32_sample()
    }

    #[inline]
    unsafe fn get_sample(&self) -> f32 {
        match self.sample_format {
            //
            // Packed
            I16(Packed) => self.get_sample_packed::<i16>(),
            I32(Packed) => self.get_sample_packed::<i32>(),
            F32(Packed) => self.get_sample_packed::<f32>(),
            F64(Packed) => self.get_sample_packed::<f64>(),
            //
            // Planar
            I16(Planar) => self.get_sample_planar::<i16>(),
            I32(Planar) => self.get_sample_planar::<i32>(),
            F32(Planar) => self.get_sample_planar::<f32>(),
            F64(Planar) => self.get_sample_planar::<f64>(),
            _ => panic!("FFmpegDecoder cannot decode the sample type."),
        }
    }

    #[inline]
    unsafe fn next_sample(&mut self) -> f32 {
        let sample: f32 = self.get_sample();
        // If we have reached the last channel in the current frame,
        // then move onto the next frame.
        self.buf_channel_idx += 1;
        self.num_samples_remaining = self.num_samples_remaining.saturating_sub(1);
        if self.buf_channel_idx >= self.num_channels {
            self.buf_frame_idx += 1;
            self.buf_channel_idx = 0;
        }
        sample
    }
}

impl Source for FFmpegDecoder {}

impl Signal for FFmpegDecoder {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn num_channels(&self) -> u16 {
        self.num_channels as u16
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        Some(self.num_samples_remaining / self.num_channels)
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
            // If we have not reached the end of the sample buffer,
            // then get the next sample.
            if self.buf_frame_idx < self.buf_num_frames {
                let sample: f32 = unsafe { self.next_sample() };
                return Some(sample);
            }

            // We exhausted our previous samples buffer. We need a new one.
            self.samples_buffer = self.next_samples_buffer();
            if self.samples_buffer.is_some() {
                continue;
            }

            // We exhausted our packet. We need a new one.
            self.packet = self.next_packet();
            if self.packet.is_some() {
                continue;
            }

            // There are no more packets. We are done.
            return None;
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
