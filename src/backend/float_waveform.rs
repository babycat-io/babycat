use std::fmt;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[cfg(feature = "enable-multithreading")]
use crate::backend::batch_args::BatchArgs;
#[cfg(feature = "enable-multithreading")]
use crate::backend::named_result::NamedResult;
#[cfg(feature = "enable-multithreading")]
use rayon::prelude::*;

use symphonia::core::audio::AudioBufferRef;
use symphonia::core::audio::Signal;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::backend::common::milliseconds_to_frames;
use crate::backend::decode_args::*;
use crate::backend::errors::Error;
use crate::backend::resample::resample;
use crate::backend::waveform::Waveform;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatWaveform {
    interleaved_samples: Vec<f32>,
    frame_rate_hz: u32,
    num_channels: u32,
    num_frames: u64,
}

impl From<crate::backend::int_waveform::IntWaveform> for FloatWaveform {
    fn from(item: crate::backend::int_waveform::IntWaveform) -> Self {
        let buffer: Vec<f32> = item
            .interleaved_samples()
            .iter()
            .map(|val| (*val as f32) / 0x8000 as f32)
            .collect();

        FloatWaveform {
            interleaved_samples: buffer,
            frame_rate_hz: item.frame_rate_hz(),
            num_channels: item.num_channels(),
            num_frames: item.num_frames(),
        }
    }
}

// We manually implement the debug trait so that we don't
// print out giant vectors.
impl fmt::Debug for FloatWaveform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FloatWaveform {{ frame_rate_hz: {}, num_channels: {}, num_frames: {}}}",
            self.frame_rate_hz(),
            self.num_channels(),
            self.num_frames()
        )
    }
}

impl FloatWaveform {
    pub fn new(frame_rate_hz: u32, num_channels: u32, interleaved_samples: Vec<f32>) -> Self {
        let num_frames = interleaved_samples.len() as u64 / num_channels as u64;
        FloatWaveform {
            interleaved_samples,
            frame_rate_hz,
            num_channels,
            num_frames,
        }
    }
    pub fn from_frames_of_silence(frame_rate_hz: u32, num_channels: u32, num_frames: u64) -> Self {
        FloatWaveform {
            frame_rate_hz,
            num_channels,
            num_frames,
            interleaved_samples: vec![0.0; (num_channels as u64 * num_frames) as usize],
        }
    }

    pub fn from_milliseconds_of_silence(
        frame_rate_hz: u32,
        num_channels: u32,
        duration_milliseconds: u64,
    ) -> Self {
        let num_frames = milliseconds_to_frames(frame_rate_hz, duration_milliseconds);
        Self::from_frames_of_silence(frame_rate_hz, num_channels, num_frames)
    }

    pub fn from_encoded_stream_with_hint<R: 'static + Read>(
        encoded_stream: R,
        decode_args: DecodeArgs,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Self, Error> {
        // If the user has provided an end timestamp that is BEFORE
        // our start timestamp, then we raise an error.
        if decode_args.start_time_milliseconds != DEFAULT_START_TIME_MILLISECONDS
            && decode_args.end_time_milliseconds != DEFAULT_END_TIME_MILLISECONDS
            && decode_args.start_time_milliseconds >= decode_args.end_time_milliseconds
        {
            return Err(Error::WrongTimeOffset(
                decode_args.start_time_milliseconds,
                decode_args.end_time_milliseconds,
            ));
        }

        // If the user has not specified how long the output audio should be,
        // then we would not know how to zero-pad after it.
        if decode_args.zero_pad_ending
            && decode_args.end_time_milliseconds == DEFAULT_END_TIME_MILLISECONDS
        {
            return Err(Error::CannotZeroPadWithoutSpecifiedLength);
        }

        // We do not allow the user to specify that they want to extract
        // one channels AND to convert the waveform to mono.
        // Converting the waveform to mono only makes sense when
        // we are working with more than one channel.
        if decode_args.num_channels == 1 && decode_args.convert_to_mono {
            return Err(Error::WrongNumChannelsAndMono);
        }

        // Set up defaults for the decoder.
        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = DecoderOptions { verify: false };

        // Provide file extension and mime type hints to speed up
        // guessing which audio format the input is.
        // An incorrect hint will not prevent a successful decoding.
        let mut hint = Hint::new();
        if file_extension != DEFAULT_FILE_EXTENSION {
            hint.with_extension(&file_extension);
        }
        if mime_type != DEFAULT_MIME_TYPE {
            hint.mime_type(&mime_type);
        }

        // Initialize the decoder.
        let media_source: Box<dyn MediaSource> = Box::new(ReadOnlySource::new(encoded_stream));
        let media_source_stream = MediaSourceStream::new(media_source, Default::default());
        let probed = match symphonia::default::get_probe().format(
            &hint,
            media_source_stream,
            &format_opts,
            &metadata_opts,
        ) {
            Ok(value) => value,
            // If we could not identify the input as one of our supported
            // encodings, then throw an error.
            Err(symphonia::core::errors::Error::Unsupported { .. }) => {
                return Err(Error::UnknownInputEncoding);
            }
            // Raise unknown errors.
            Err(err) => {
                return Err(Error::UnknownDecodeErrorWithMessage(leak_str!(
                    err.to_string()
                )))
            }
        };
        let mut reader = probed.format;
        let stream = reader.default_stream().unwrap();
        let codec_params = &stream.codec_params;
        let mut decoder = match symphonia::default::get_codecs().make(codec_params, &decoder_opts) {
            Ok(value) => value,
            // If we could not identify the input as one of our supported
            // encodings, then throw an error.
            Err(symphonia::core::errors::Error::Unsupported { .. }) => {
                return Err(Error::UnknownInputEncoding);
            }
            // Raise unknown errors.
            Err(_) => {
                return Err(Error::UnknownDecodeError);
            }
        };
        // Examine the actual shape of this audio file.
        let original_frame_rate_hz = codec_params.sample_rate.unwrap();
        let original_num_channels = codec_params.channels.unwrap().count() as u32;

        // If the user provided a negative frame rate, throw an error.
        // We waited this long to throw an error because we also want to
        // tell them what the REAL frame rate is for this audio stream.
        if decode_args.frame_rate_hz != DEFAULT_FRAME_RATE_HZ && decode_args.frame_rate_hz < 1 {
            return Err(Error::WrongFrameRate(
                original_frame_rate_hz,
                decode_args.frame_rate_hz,
            ));
        }

        // This is the first n channels that we want to read from.
        // If the user wants to convert the output to mono, we do that after
        // reading from the first n channels.
        // If decode_args.num_channels was unspecified, then we read from
        // all of the channels.
        let selected_num_channels = {
            if decode_args.num_channels == DEFAULT_NUM_CHANNELS {
                original_num_channels
            } else if decode_args.num_channels < 1 {
                decoder.close();
                return Err(Error::WrongNumChannels(
                    decode_args.num_channels,
                    original_num_channels,
                ));
            } else if original_num_channels >= decode_args.num_channels {
                decode_args.num_channels
            } else {
                decoder.close();
                return Err(Error::WrongNumChannels(
                    decode_args.num_channels,
                    original_num_channels,
                ));
            }
        };

        // Compute the exact start and end sample indexes for us to begin
        // and end decoding.
        let start_time_samples: u64 =
            decode_args.start_time_milliseconds * original_frame_rate_hz as u64 / 1000;
        let end_time_samples: u64 =
            decode_args.end_time_milliseconds * original_frame_rate_hz as u64 / 1000;

        // Decode all packets, ignoring decode errors.
        let mut buffer: Vec<f32> = Vec::new();
        let mut current_sample: u64 = 0;
        'packet_loop: while let Ok(packet) = reader.next_packet() {
            match decoder.decode(&packet) {
                // Decode errors are not fatal.
                // We will just try to decode the next packet.
                Err(symphonia::core::errors::Error::DecodeError(_)) => {
                    continue;
                }

                Err(_) => {
                    decoder.close();
                    return Err(Error::UnknownDecodeError);
                }

                Ok(decoded_buffer_ref) => {
                    // Decode the packet into an AudioBufferRef of float32 values.
                    let decoded_buffer = match decoded_buffer_ref {
                        AudioBufferRef::F32(buf) => buf,
                        AudioBufferRef::S32(int_buf) => {
                            std::borrow::Cow::Owned(int_buf.make_equivalent())
                        }
                    };

                    // Collect references to buffers for each audio channel.
                    let channel_buffers: Vec<Vec<f32>> = (0..selected_num_channels)
                        .map(|channel_idx| decoded_buffer.chan(channel_idx as usize).to_vec())
                        .collect();

                    // Iterate over all of the samples in the current packet.
                    for current_sample_in_packet in 0..decoded_buffer.frames() {
                        current_sample += 1;

                        // If the current sample is before our start offset,
                        // then ignore it.
                        if current_sample <= start_time_samples {
                            continue;
                        }

                        // If we have a defined end offset and we are past it,
                        // then stop the decoding loop entirely.
                        if decode_args.end_time_milliseconds != DEFAULT_END_TIME_MILLISECONDS
                            && current_sample > end_time_samples
                        {
                            break 'packet_loop;
                        }
                        // If we are going to convert this audio waveform to mono,
                        // then we append the average value of the selected input channels.
                        if decode_args.convert_to_mono {
                            let mut current_sample_sum: f32 = 0.0_f32;
                            for channel in channel_buffers.iter() {
                                current_sample_sum += channel[current_sample_in_packet];
                            }
                            current_sample_sum /= channel_buffers.len() as f32;
                            buffer.push(current_sample_sum);
                        } else {
                            // Iterate over every channel buffer in the sample and
                            // append its value to our return type.
                            for channel in channel_buffers.iter() {
                                buffer.push(channel[current_sample_in_packet]);
                            }
                        }
                    }
                }
            }
        }
        decoder.close();

        let num_channels = if decode_args.convert_to_mono {
            1
        } else {
            selected_num_channels
        };

        // Zero-pad the output audio vector if our start/end interval
        // is longer than the actual audio we decoded.
        if decode_args.zero_pad_ending
            && decode_args.end_time_milliseconds != DEFAULT_END_TIME_MILLISECONDS
        {
            let expected_buffer_len = (end_time_samples - start_time_samples) * num_channels as u64;
            let buffer_padding = expected_buffer_len - buffer.len() as u64;
            if buffer_padding > 0 {
                buffer.extend(vec![0.0_f32; buffer_padding as usize]);
            }
        }

        #[allow(unused_mut)]
        let mut final_frame_rate_hz = original_frame_rate_hz;
        // If we want the audio to be at a different frame rate,
        // then resample it.
        if decode_args.frame_rate_hz != DEFAULT_FRAME_RATE_HZ
            && decode_args.frame_rate_hz != original_frame_rate_hz
        {
            final_frame_rate_hz = decode_args.frame_rate_hz;
            buffer = match resample(
                original_frame_rate_hz,
                final_frame_rate_hz,
                num_channels,
                &buffer,
                decode_args.resample_mode,
            ) {
                Ok(resampled) => resampled,
                Err(err) => return Err(err),
            }
        }

        let num_frames = (buffer.len() / num_channels as usize) as u64;
        Ok(FloatWaveform {
            frame_rate_hz: final_frame_rate_hz,
            num_channels,
            num_frames,
            interleaved_samples: buffer,
        })
    }

    pub fn from_encoded_stream<R: 'static + Read>(
        encoded_stream: R,
        decode_args: DecodeArgs,
    ) -> Result<Self, Error> {
        Self::from_encoded_stream_with_hint(
            encoded_stream,
            decode_args,
            DEFAULT_FILE_EXTENSION,
            DEFAULT_MIME_TYPE,
        )
    }

    pub fn from_file(filename: &str, decode_args: DecodeArgs) -> Result<Self, Error> {
        let pathname = std::path::Path::new(filename);
        let file = match std::fs::File::open(pathname) {
            Ok(f) => f,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(Error::FileNotFound(Box::leak(
                        filename.to_owned().into_boxed_str(),
                    )));
                }
                _ => {
                    return Err(Error::UnknownIOError);
                }
            },
        };
        if let Ok(metadata) = file.metadata() {
            if metadata.is_dir() {
                return Err(Error::FilenameIsADirectory(Box::leak(
                    filename.to_owned().into_boxed_str(),
                )));
            }
        }
        let file_extension = match pathname.extension() {
            Some(os_str) => match os_str.to_str() {
                Some(str) => str,
                None => DEFAULT_FILE_EXTENSION,
            },
            None => DEFAULT_FILE_EXTENSION,
        };

        Self::from_encoded_stream_with_hint(file, decode_args, file_extension, DEFAULT_MIME_TYPE)
    }

    #[cfg(feature = "enable-multithreading")]
    pub fn from_many_files(
        filenames: &[&str],
        decode_args: DecodeArgs,
        batch_args: BatchArgs,
    ) -> Vec<NamedResult<Self, Error>> {
        let thread_pool: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
            .num_threads(batch_args.num_workers)
            .build()
            .unwrap();

        thread_pool.install(|| {
            filenames
                .par_iter()
                .map(|filename| NamedResult {
                    name: (*filename).to_string(),
                    result: Self::from_file(&filename, decode_args),
                })
                .collect::<Vec<NamedResult<Self, Error>>>()
        })
    }

    pub fn interleaved_samples(&self) -> &[f32] {
        &self.interleaved_samples
    }

    pub fn to_wav_buffer(&self) -> Result<Vec<u8>, Error> {
        let writer_spec = hound::WavSpec {
            channels: self.num_channels as u16,
            sample_rate: self.frame_rate_hz as u32,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        let mut writer = match hound::WavWriter::new(&mut cursor, writer_spec) {
            Ok(w) => w,
            Err(_) => return Err(Error::UnknownEncodeError),
        };
        for sample in &self.interleaved_samples {
            let sample_result = writer.write_sample(*sample);
            if sample_result.is_err() {
                return Err(Error::UnknownEncodeError);
            }
        }
        let finalize_result = writer.finalize();
        if finalize_result.is_err() {
            return Err(Error::UnknownEncodeError);
        }
        Ok(cursor.into_inner())
    }

    pub fn to_wav_file(&self, filename: &str) -> Result<(), Error> {
        let writer_spec = hound::WavSpec {
            channels: self.num_channels as u16,
            sample_rate: self.frame_rate_hz as u32,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = match hound::WavWriter::create(filename, writer_spec) {
            Ok(w) => w,
            Err(_) => return Err(Error::UnknownEncodeError),
        };
        for sample in &self.interleaved_samples {
            let sample_result = writer.write_sample(*sample);
            if sample_result.is_err() {
                return Err(Error::UnknownEncodeError);
            }
        }
        let finalize_result = writer.finalize();
        if finalize_result.is_err() {
            return Err(Error::UnknownEncodeError);
        }
        Ok(())
    }
}

impl crate::backend::waveform::Waveform for FloatWaveform {
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    fn num_channels(&self) -> u32 {
        self.num_channels
    }

    fn num_frames(&self) -> u64 {
        self.num_frames
    }
}

#[cfg(test)]
mod test_float_waveform_from_bytes {
    use crate::backend::float_waveform::FloatWaveform;
    use crate::backend::waveform::Waveform;
    use std::io::Cursor;

    const COF_FILENAME: &str = "./audio-for-tests/circus-of-freaks/track.mp3";
    const COF_NUM_CHANNELS: u32 = 2;
    const COF_NUM_FRAMES: u64 = 2492928;
    const COF_FRAME_RATE_HZ: u32 = 44100;

    #[test]
    fn test_circus_of_freaks_default_1() {
        let bytes = std::fs::read(COF_FILENAME).unwrap();
        let cursor = Cursor::new(bytes);
        let waveform = FloatWaveform::from_encoded_stream(cursor, Default::default()).unwrap();
        assert_eq!(waveform.num_channels(), COF_NUM_CHANNELS);
        assert_eq!(waveform.num_frames(), COF_NUM_FRAMES);
        assert_eq!(waveform.frame_rate_hz(), COF_FRAME_RATE_HZ)
    }
}

#[cfg(test)]
mod test_float_waveform_from_file {
    use crate::backend::errors::Error;
    use crate::DecodeArgs;
    use crate::FloatWaveform;
    use crate::Waveform;

    const COF_FILENAME: &str = "./audio-for-tests/circus-of-freaks/track.mp3";
    const COF_NUM_CHANNELS: u32 = 2;
    const COF_NUM_FRAMES: u64 = 2492928;
    const COF_FRAME_RATE_HZ: u32 = 44100;

    const LCT_FILENAME: &str = "./audio-for-tests/left-channel-tone/track.mp3";
    const LCT_NUM_CHANNELS: u32 = 2;
    const LCT_NUM_FRAMES: u64 = 1325952;
    const LCT_FRAME_RATE_HZ: u32 = 44100;

    fn decode_cof_mp3(decode_args: DecodeArgs) -> Result<FloatWaveform, Error> {
        FloatWaveform::from_file(COF_FILENAME, decode_args)
    }

    fn decode_lct_mp3(decode_args: DecodeArgs) -> Result<FloatWaveform, Error> {
        FloatWaveform::from_file(LCT_FILENAME, decode_args)
    }

    fn assert_error(result: Result<FloatWaveform, Error>, error_type: &str) {
        assert_eq!(error_type, result.unwrap_err().error_type());
    }

    fn assert_waveform(
        result: Result<FloatWaveform, Error>,
        num_channels: u32,
        num_frames: u64,
        frame_rate_hz: u32,
    ) {
        let waveform = result.unwrap();
        assert_eq!(num_channels, waveform.num_channels());
        assert_eq!(num_frames, waveform.num_frames());
        assert_eq!(frame_rate_hz, waveform.frame_rate_hz());
        assert_eq!(
            (num_frames * num_channels as u64) as usize,
            waveform.interleaved_samples().len()
        );
    }

    #[test]
    fn test_circus_of_freaks_default_1() {
        let decode_args = DecodeArgs {
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_wrong_time_offset_1() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 1000,
            end_time_milliseconds: 999,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_error(result, "WrongTimeOffset(1000,999)");
    }

    #[test]
    fn test_circus_of_freaks_wrong_time_offset_2() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 1000,
            end_time_milliseconds: 1000,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_error(result, "WrongTimeOffset(1000,1000)");
    }

    #[test]
    fn test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_1() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 5,
            end_time_milliseconds: 0,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_error(result, "CannotZeroPadWithoutSpecifiedLength");
    }

    #[test]
    fn test_circus_of_freaks_get_channels_1() {
        let decode_args = DecodeArgs {
            num_channels: 1,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_get_channels_2() {
        let decode_args = DecodeArgs {
            num_channels: 2,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, 2, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_get_channels_too_many_1() {
        let decode_args = DecodeArgs {
            num_channels: 3,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_error(result, "WrongNumChannels(3,2)");
    }

    #[test]
    fn test_circus_of_freaks_convert_to_mono_1() {
        let decode_args = DecodeArgs {
            num_channels: 2,
            convert_to_mono: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }
    #[test]
    fn test_circus_of_freaks_convert_to_mono_2() {
        let decode_args = DecodeArgs {
            convert_to_mono: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_left_channel_tone_convert_to_mono_1() {
        // In this test, we do mono and stereo decoding of an audio file
        // that only has audio in one of its two channels.
        // First, let's do the mono decoding.
        let mono_decode_args = DecodeArgs {
            convert_to_mono: true,
            ..Default::default()
        };
        let mono_result = decode_lct_mp3(mono_decode_args);
        let mono_waveform = mono_result.unwrap();
        assert_eq!(1, mono_waveform.num_channels());
        assert_eq!(LCT_NUM_FRAMES, mono_waveform.num_frames());
        assert_eq!(LCT_FRAME_RATE_HZ, mono_waveform.frame_rate_hz());
        let mono_sum_waveform: f32 = mono_waveform.interleaved_samples().iter().sum();
        // Now, let's do the stereo decoding.
        let stereo_decode_args = DecodeArgs {
            ..Default::default()
        };
        let stereo_result = decode_lct_mp3(stereo_decode_args);
        let stereo_waveform = stereo_result.unwrap();
        assert_eq!(LCT_NUM_CHANNELS, stereo_waveform.num_channels());
        assert_eq!(LCT_NUM_FRAMES, stereo_waveform.num_frames());
        assert_eq!(LCT_FRAME_RATE_HZ, stereo_waveform.frame_rate_hz());
        let stereo_sum_waveform: f32 = stereo_waveform.interleaved_samples().iter().sum();
        // Check that the mono waveform is quieter because we made it
        // by averaging in the other silent channel.
        assert!(float_cmp::approx_eq!(
            f32,
            mono_sum_waveform * 2.0_f32,
            stereo_sum_waveform,
            ulps = 3
        ));
    }

    #[test]
    fn test_circus_of_freaks_convert_to_mono_invalid_1() {
        let decode_args = DecodeArgs {
            num_channels: 1,
            convert_to_mono: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_error(result, "WrongNumChannelsAndMono");
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_1() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 1,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_2() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 10,
            end_time_milliseconds: 11,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_3() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 30000,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_4() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 15000,
            end_time_milliseconds: 45000,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_5() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 30000,
            end_time_milliseconds: 60000,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 1169928, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_1() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 1,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_2() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 10,
            end_time_milliseconds: 11,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_3() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 30000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_4() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 15000,
            end_time_milliseconds: 45000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_5() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 30000,
            end_time_milliseconds: 60000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_6() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_7() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 90000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 3969000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_8() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 30000,
            end_time_milliseconds: 90000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_end_milliseconds_zero_pad_ending_1() {
        let decode_args = DecodeArgs {
            end_time_milliseconds: 90000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 3969000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_invalid_resample_1() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 1,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_error(result, "WrongFrameRateRatio(44100,1)");
    }

    #[test]
    fn test_circus_of_freaks_invalid_resample_2() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 20,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_error(result, "WrongFrameRateRatio(44100,20)");
    }

    #[test]
    fn test_circus_of_freaks_invalid_resample_3() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 172,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_error(result, "WrongFrameRateRatio(44100,172)");
    }

    #[test]
    fn test_circus_of_freaks_resample_no_change_1() {
        let decode_args = DecodeArgs {
            frame_rate_hz: COF_FRAME_RATE_HZ,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_resample_1() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 22050,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 1246464, 22050);
    }

    #[test]
    fn test_circus_of_freaks_resample_2() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 11025,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 623232, 11025);
    }

    #[test]
    fn test_circus_of_freaks_resample_3() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 88200,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 4985856, 88200);
    }

    #[test]
    fn test_circus_of_freaks_resample_4() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 4410,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 249293, 4410);
    }

    #[test]
    fn test_circus_of_freaks_resample_5() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 44099,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 2492872, 44099);
    }

    #[test]
    fn test_circus_of_freaks_resample_6() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 48000,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 2713392, 48000);
    }

    #[test]
    fn test_circus_of_freaks_resample_7() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 60000,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 3391739, 60000);
    }

    #[test]
    fn test_circus_of_freaks_resample_8() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 88200,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 4985856, 88200);
    }

    #[test]
    fn test_circus_of_freaks_resample_9() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 96000,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 5426783, 96000);
    }

    #[test]
    fn test_circus_of_freaks_resample_10() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 200,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 11306, 200);
    }

    #[test]
    fn test_circus_of_freaks_resample_11() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 2000,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 113058, 2000);
    }

    #[test]
    fn test_circus_of_freaks_resample_12() {
        let decode_args = DecodeArgs {
            frame_rate_hz: 173,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 9780, 173);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_1() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            frame_rate_hz: 48000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 2880000, 48000);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_2() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            frame_rate_hz: 44099,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 2645940, 44099);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_3() {
        let decode_args = DecodeArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            frame_rate_hz: 22050,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(decode_args);
        assert_waveform(result, COF_NUM_CHANNELS, 1323000, 22050);
    }
}

#[cfg(all(test, feature = "enable-multithreading"))]
mod test_float_waveform_from_many_filenames {
    use crate::{BatchArgs, DecodeArgs, FloatWaveform, Waveform};

    const AT_FILENAME: &str = "./audio-for-tests/andreas-theme/track.mp3";
    const AT_NUM_CHANNELS: u32 = 2;
    const AT_NUM_FRAMES: u64 = 9586944;
    const AT_FRAME_RATE_HZ: u32 = 44100;

    const BT_FILENAME: &str = "./audio-for-tests/blippy-trance/track.mp3";
    const BT_NUM_CHANNELS: u32 = 2;
    const BT_NUM_FRAMES: u64 = 5294592;
    const BT_FRAME_RATE_HZ: u32 = 44100;

    const COF_FILENAME: &str = "./audio-for-tests/circus-of-freaks/track.mp3";
    const COF_NUM_CHANNELS: u32 = 2;
    const COF_NUM_FRAMES: u64 = 2492928;
    const COF_FRAME_RATE_HZ: u32 = 44100;

    const LCT_FILENAME: &str = "./audio-for-tests/left-channel-tone/track.mp3";
    const LCT_NUM_CHANNELS: u32 = 2;
    const LCT_NUM_FRAMES: u64 = 1325952;
    const LCT_FRAME_RATE_HZ: u32 = 44100;

    const MONO_DTMF_FILENAME: &str = "./audio-for-tests/mono-dtmf-tones/track.mp3";
    const MONO_DTMF_NUM_CHANNELS: u32 = 1;
    const MONO_DTMF_NUM_FRAMES: u64 = 443520;
    const MONO_DTMF_FRAME_RATE_HZ: u32 = 44100;

    const OHFY_FILENAME: &str = "./audio-for-tests/on-hold-for-you/track.mp3";
    const OHFY_NUM_CHANNELS: u32 = 2;
    const OHFY_NUM_FRAMES: u64 = 9620352;
    const OHFY_FRAME_RATE_HZ: u32 = 44100;

    const TMS_FILENAME: &str = "./audio-for-tests/tone-missing-sounds/track.mp3";
    const TMS_NUM_CHANNELS: u32 = 1;
    const TMS_NUM_FRAMES: u64 = 1325952;
    const TMS_FRAME_RATE_HZ: u32 = 44100;

    const VR_FILENAME: &str = "./audio-for-tests/voxel-revolution/track.mp3";
    const VR_NUM_CHANNELS: u32 = 2;
    const VR_NUM_FRAMES: u64 = 5728896;
    const VR_FRAME_RATE_HZ: u32 = 44100;

    const ALL_FILENAMES: &[&str] = &[
        AT_FILENAME,
        BT_FILENAME,
        COF_FILENAME,
        LCT_FILENAME,
        MONO_DTMF_FILENAME,
        OHFY_FILENAME,
        TMS_FILENAME,
        VR_FILENAME,
    ];

    const ALL_NUM_CHANNELS: &[u32] = &[
        AT_NUM_CHANNELS,
        BT_NUM_CHANNELS,
        COF_NUM_CHANNELS,
        LCT_NUM_CHANNELS,
        MONO_DTMF_NUM_CHANNELS,
        OHFY_NUM_CHANNELS,
        TMS_NUM_CHANNELS,
        VR_NUM_CHANNELS,
    ];

    const ALL_NUM_FRAMES: &[u64] = &[
        AT_NUM_FRAMES,
        BT_NUM_FRAMES,
        COF_NUM_FRAMES,
        LCT_NUM_FRAMES,
        MONO_DTMF_NUM_FRAMES,
        OHFY_NUM_FRAMES,
        TMS_NUM_FRAMES,
        VR_NUM_FRAMES,
    ];

    const ALL_FRAME_RATE_HZ: &[u32] = &[
        AT_FRAME_RATE_HZ,
        BT_FRAME_RATE_HZ,
        COF_FRAME_RATE_HZ,
        LCT_FRAME_RATE_HZ,
        MONO_DTMF_FRAME_RATE_HZ,
        OHFY_FRAME_RATE_HZ,
        TMS_FRAME_RATE_HZ,
        VR_FRAME_RATE_HZ,
    ];

    #[test]
    fn test_all_same_file_1() {
        let filenames = &[COF_FILENAME, COF_FILENAME, COF_FILENAME];
        let decode_args = Default::default();
        let batch_args = Default::default();
        let batch = FloatWaveform::from_many_files(filenames, decode_args, batch_args);
        for named_result in batch {
            let waveform = named_result.result.unwrap();
            assert_eq!(COF_NUM_CHANNELS, waveform.num_channels());
            assert_eq!(COF_NUM_FRAMES, waveform.num_frames());
            assert_eq!(COF_FRAME_RATE_HZ, waveform.frame_rate_hz());
            assert_eq!(
                (COF_NUM_FRAMES * COF_NUM_CHANNELS as u64) as usize,
                waveform.interleaved_samples().len()
            );
        }
    }

    #[test]
    fn test_all_same_file_2() {
        let filenames = &[COF_FILENAME, COF_FILENAME, COF_FILENAME];
        let decode_args = DecodeArgs {
            end_time_milliseconds: 15000,
            ..Default::default()
        };
        let batch_args = Default::default();
        let batch = FloatWaveform::from_many_files(filenames, decode_args, batch_args);
        let num_frames = 661500;
        for named_result in batch {
            let waveform = named_result.result.unwrap();
            assert_eq!(COF_NUM_CHANNELS, waveform.num_channels());
            assert_eq!(num_frames, waveform.num_frames());
            assert_eq!(COF_FRAME_RATE_HZ, waveform.frame_rate_hz());
            assert_eq!(
                (num_frames * COF_NUM_CHANNELS as u64) as usize,
                waveform.interleaved_samples().len()
            );
        }
    }

    #[test]
    fn test_all_same_file_single_threaded_1() {
        let filenames = &[COF_FILENAME, COF_FILENAME, COF_FILENAME];
        let decode_args = Default::default();
        let batch_args = BatchArgs {
            num_workers: 1,
            ..Default::default()
        };
        let batch = FloatWaveform::from_many_files(filenames, decode_args, batch_args);
        for named_result in batch {
            let waveform = named_result.result.unwrap();
            assert_eq!(COF_NUM_CHANNELS, waveform.num_channels());
            assert_eq!(COF_NUM_FRAMES, waveform.num_frames());
            assert_eq!(COF_FRAME_RATE_HZ, waveform.frame_rate_hz());
            assert_eq!(
                (COF_NUM_FRAMES * COF_NUM_CHANNELS as u64) as usize,
                waveform.interleaved_samples().len()
            );
        }
    }

    #[test]
    fn test_different_filenames_1() {
        let decode_args = Default::default();
        let batch_args = Default::default();
        let batch = FloatWaveform::from_many_files(ALL_FILENAMES, decode_args, batch_args);
        for (i, named_result) in batch.into_iter().enumerate() {
            let waveform = named_result.result.unwrap();
            assert_eq!(ALL_NUM_CHANNELS[i], waveform.num_channels());
            assert_eq!(ALL_NUM_FRAMES[i], waveform.num_frames());
            assert_eq!(ALL_FRAME_RATE_HZ[i], waveform.frame_rate_hz());
            assert_eq!(
                (ALL_NUM_FRAMES[i] * ALL_NUM_CHANNELS[i] as u64) as usize,
                waveform.interleaved_samples().len()
            );
        }
    }

    #[test]
    fn test_file_not_found_error_1() {
        let filenames = &[COF_FILENAME, "asdfasdf"];
        let decode_args = Default::default();
        let batch_args = Default::default();
        let batch = FloatWaveform::from_many_files(filenames, decode_args, batch_args);
        assert_eq!(batch.len(), 2);
        let first_result = batch[0].result.as_ref().unwrap();
        assert_eq!(COF_NUM_CHANNELS, first_result.num_channels());
        assert_eq!(COF_NUM_FRAMES, first_result.num_frames());
        assert_eq!(COF_FRAME_RATE_HZ, first_result.frame_rate_hz());
        let second_result = batch[1].result.as_ref().unwrap_err();
        assert_eq!(second_result.error_type(), "FileNotFound(asdfasdf)");
    }
}
