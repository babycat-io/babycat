use std::fmt;
use std::io::Read;
use std::marker::Send;
use std::marker::Sync;

use serde::{Deserialize, Serialize};

use crate::backend::common::milliseconds_to_frames;
use crate::backend::constants::*;
use crate::backend::decoder::from_encoded_bytes;
use crate::backend::decoder::from_encoded_bytes_with_hint;
use crate::backend::decoder::from_encoded_stream;
use crate::backend::decoder::from_encoded_stream_with_hint;
use crate::backend::errors::Error;
use crate::backend::resample::resample;
use crate::backend::signal::Signal;
use crate::backend::Decoder;
use crate::backend::Source;
use crate::backend::WaveformArgs;

#[cfg(feature = "enable-filesystem")]
use crate::backend::decoder::from_file;

/// Represents a fixed-length audio waveform as a `Vec<f32>`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Waveform {
    interleaved_samples: Vec<f32>,
    frame_rate_hz: u32,
    num_channels: u16,
    num_frames: usize,
}

// We manually implement the debug trait so that we don't
// print out giant vectors.
impl fmt::Debug for Waveform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Waveform {{ frame_rate_hz: {}, num_channels: {}, num_frames: {}}}",
            self.frame_rate_hz, self.num_channels, self.num_frames
        )
    }
}

impl Waveform {
    pub fn from_interleaved_samples(
        frame_rate_hz: u32,
        num_channels: u16,
        interleaved_samples: &[f32],
    ) -> Self {
        Self::new(frame_rate_hz, num_channels, interleaved_samples.to_owned())
    }

    pub fn from_decoder(args: WaveformArgs, mut decoder: Box<dyn Decoder>) -> Result<Self, Error> {
        let original_frame_rate_hz = decoder.frame_rate_hz();
        let original_num_channels = decoder.num_channels();

        // If the user has provided an end timestamp that is BEFORE
        // our start timestamp, then we raise an error.
        if args.start_time_milliseconds != DEFAULT_START_TIME_MILLISECONDS
            && args.end_time_milliseconds != DEFAULT_END_TIME_MILLISECONDS
            && args.start_time_milliseconds >= args.end_time_milliseconds
        {
            return Err(Error::WrongTimeOffset(
                args.start_time_milliseconds,
                args.end_time_milliseconds,
            ));
        }

        // If the user has not specified how long the output audio should be,
        // then we would not know how to zero-pad after it.
        if args.zero_pad_ending && args.end_time_milliseconds == DEFAULT_END_TIME_MILLISECONDS {
            return Err(Error::CannotZeroPadWithoutSpecifiedLength);
        }

        // We do not allow the user to specify that they want to extract
        // one channels AND to convert the waveform to mono.
        // Converting the waveform to mono only makes sense when
        // we are working with more than one channel.
        if args.num_channels == 1 && args.convert_to_mono {
            return Err(Error::WrongNumChannelsAndMono);
        }

        // This is the first n channels that we want to read from.
        // If the user wants to convert the output to mono, we do that after
        // reading from the first n channels.
        // If args.num_channels was unspecified, then we read from
        // all of the channels.
        if args.num_channels > original_num_channels {
            return Err(Error::WrongNumChannels(
                args.num_channels,
                original_num_channels,
            ));
        }
        let selected_num_channels: u16 = if args.num_channels == DEFAULT_NUM_CHANNELS {
            original_num_channels
        } else {
            args.num_channels
        };

        let output_num_channels: u16 = if args.convert_to_mono {
            1
        } else {
            selected_num_channels
        };

        let start_frame_idx =
            milliseconds_to_frames(original_frame_rate_hz, args.start_time_milliseconds);
        let end_frame_idx =
            milliseconds_to_frames(original_frame_rate_hz, args.end_time_milliseconds);
        let start_sample_idx = start_frame_idx * original_num_channels as usize;
        let end_sample_idx = end_frame_idx * original_num_channels as usize;

        let expected_buffer_len_from_decoder: usize =
            decoder.num_frames_estimate().unwrap_or(0) * output_num_channels as usize;
        let interleaved_samples_capacity: usize = if end_frame_idx > start_frame_idx {
            let expected_buffer_len_from_user: usize =
                (end_frame_idx - start_frame_idx) * output_num_channels as usize;
            if args.zero_pad_ending {
                std::cmp::max(
                    expected_buffer_len_from_user,
                    expected_buffer_len_from_decoder,
                )
            } else {
                std::cmp::min(
                    expected_buffer_len_from_user,
                    expected_buffer_len_from_decoder,
                )
            }
        } else {
            expected_buffer_len_from_decoder
        };
        let take_samples = if end_sample_idx > start_sample_idx {
            end_sample_idx - start_sample_idx
        } else {
            0
        };
        let mut interleaved_samples: Vec<f32> = Vec::with_capacity(interleaved_samples_capacity);

        let source = decoder.begin()?;
        let bounded_source = source
            .skip_samples(start_sample_idx)
            .take_samples(take_samples)
            .select_first_channels(selected_num_channels)
            .convert_to_mono(args.convert_to_mono);
        for sample in bounded_source {
            interleaved_samples.push(sample);
        }

        // Zero-pad the output audio vector if our start/end interval
        // is longer than the actual audio we decoded.
        if args.zero_pad_ending && end_frame_idx > start_frame_idx {
            let expected_buffer_len_from_user: usize =
                (end_frame_idx - start_frame_idx) * output_num_channels as usize;
            let actual_buffer_len = interleaved_samples.len();
            if expected_buffer_len_from_user > actual_buffer_len {
                let buffer_padding = expected_buffer_len_from_user - actual_buffer_len;
                interleaved_samples.extend(vec![0.0_f32; buffer_padding]);
            }
        }

        // If we want the audio to be at a different frame rate,
        // then resample it.
        let output_frame_rate_hz;
        if args.frame_rate_hz != DEFAULT_FRAME_RATE_HZ
            && args.frame_rate_hz != original_frame_rate_hz
        {
            output_frame_rate_hz = args.frame_rate_hz;
            interleaved_samples = resample(
                original_frame_rate_hz,
                output_frame_rate_hz,
                output_num_channels,
                &interleaved_samples,
                args.resample_mode,
            )?;
        } else {
            output_frame_rate_hz = original_frame_rate_hz;
        }

        Ok(Self::new(
            output_frame_rate_hz,
            output_num_channels,
            interleaved_samples,
        ))
    }

    /// Decodes audio stored in an in-memory byte array.
    ///
    /// # Arguments
    /// - `encoded_bytes`: A byte array containing encoded (e.g. MP3) audio.
    /// - `waveform_args`: Instructions on how to decode the audio.
    ///
    /// # Examples
    /// ```
    /// use babycat::Waveform;
    ///
    /// let encoded_bytes: Vec<u8> = std::fs::read("audio-for-tests/andreas-theme/track.flac").unwrap();
    ///
    /// let waveform_args = Default::default();
    ///
    /// let waveform = Waveform::from_encoded_bytes(
    ///     &encoded_bytes,
    ///     waveform_args,
    /// ).unwrap();
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 9586415}"
    /// );
    /// ```
    ///
    pub fn from_encoded_bytes(
        encoded_bytes: &[u8],
        waveform_args: WaveformArgs,
    ) -> Result<Self, Error> {
        let decoder = from_encoded_bytes(waveform_args.decoding_backend, encoded_bytes)?;
        Self::from_decoder(waveform_args, decoder)
    }

    /// Decodes audio in an in-memory byte array, using user-specified encoding hints.
    ///
    /// # Arguments
    /// - `encoded_bytes`: A byte array containing encoded (e.g. MP3) audio.
    /// - `waveform_args`: Instructions on how to decode the audio.
    /// - `file_extension`: A hint--in the form of a file extension--to indicate
    ///    the encoding of the audio in `encoded_bytes`.
    /// - `mime_type`: A hint--in the form of a MIME type--to indicate
    ///    the encoding of the audio in `encoded_bytes`.
    ///
    pub fn from_encoded_bytes_with_hint(
        encoded_bytes: &[u8],
        waveform_args: WaveformArgs,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Self, Error> {
        let decoder = from_encoded_bytes_with_hint(
            waveform_args.decoding_backend,
            encoded_bytes,
            file_extension,
            mime_type,
        )?;
        Self::from_decoder(waveform_args, decoder)
    }

    /// Decodes audio stored in a locaselect_first_channelsl file.
    ///
    /// # Arguments
    /// - `filename`: A filename of an encoded audio file on the local filesystem.
    /// - `waveform_args`: Instructions on how to decode the audio.
    ///
    /// # Feature flags
    /// This function is only available if the Cargo feature `enable-fileystem`
    /// flag is enabled. The `enable-filesystem` flag is enabled by default
    /// for the Babycat's Rust, Python, and C frontends, but is disabled
    /// for the WebAssembly frontend.
    ///
    /// # Examples
    /// **Decode one audio file with the default decoding arguments:**
    /// ```
    /// use babycat::{WaveformArgs, Waveform};
    ///
    /// let waveform = Waveform::from_file(
    ///    "audio-for-tests/circus-of-freaks/track.flac",
    ///     Default::default(),
    /// ).unwrap();
    ///
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 2491247}"
    /// );
    /// ```
    ///
    /// **Decode only the first 30 seconds and upsample to 48khz:**
    /// ```
    /// use babycat::{WaveformArgs, Waveform};
    ///
    /// let waveform_args = WaveformArgs {
    ///     end_time_milliseconds: 30000,
    ///     frame_rate_hz: 48000,
    ///     ..Default::default()
    /// };
    /// let waveform = Waveform::from_file(
    ///    "audio-for-tests/circus-of-freaks/track.flac",
    ///     waveform_args,
    /// ).unwrap();
    ///
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "Waveform { frame_rate_hz: 48000, num_channels: 2, num_frames: 1440000}"
    /// );
    /// ```
    #[cfg(feature = "enable-filesystem")]
    pub fn from_file(filename: &str, waveform_args: WaveformArgs) -> Result<Self, Error> {
        let decoder = from_file(waveform_args.decoding_backend, filename)?;
        Self::from_decoder(waveform_args, decoder)
    }

    /// Decodes audio from an input stream.
    ///
    /// [`Waveform`][crate::Waveform] will take ownership of the stream
    /// and read it until the end. Therefore, you cannot provide an infinte-length
    /// stream.
    ///
    /// # Arguments
    /// - `encoded_stream`: An I/O stream of encoded audio to decode.
    /// - `waveform_args`: Instructions on how to decode the audio.
    ///
    pub fn from_encoded_stream<R: 'static + Read + Send + Sync>(
        encoded_stream: R,
        waveform_args: WaveformArgs,
    ) -> Result<Self, Error> {
        let decoder = from_encoded_stream(waveform_args.decoding_backend, encoded_stream)?;
        Self::from_decoder(waveform_args, decoder)
    }

    /// Decodes audio from an input stream, using a user-specified decoding hint.
    ///
    /// # Arguments
    /// - `encoded_stream`: An I/O stream of encoded audio to decode.
    /// - `waveform_args`: Instructions on how to decode the audio.
    /// - `file_extension`: A hint--in the form of a file extension--to indicate
    ///    the encoding of the audio in `encoded_bytes`.
    /// - `mime_type`: A hint--in the form of a MIME type--to indicate
    ///   the encoding of the audio in `encoded_bytes`.
    ///
    pub fn from_encoded_stream_with_hint<R: 'static + Read + Send + Sync>(
        encoded_stream: R,
        waveform_args: WaveformArgs,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Self, Error> {
        let decoder = from_encoded_stream_with_hint(
            waveform_args.decoding_backend,
            encoded_stream,
            file_extension,
            mime_type,
        )?;
        Self::from_decoder(waveform_args, decoder)
    }

    /// Creates a silent waveform measured in frames.
    ///
    /// # Arguments
    /// - `frame_rate_hz`: The frame rate of the waveform to create.
    /// - `num_channels`: The number of channels in the waveform to create.
    /// - `num_frames`: The number of frames of audio to generate.
    ///
    /// # Examples
    /// This creates a `Waveform` containing one second of silent *stereo* audio.
    /// ```
    /// use babycat::Waveform;
    ///
    /// let waveform = Waveform::from_frames_of_silence(44100, 2, 44100);
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 44100}"
    /// );
    /// ```
    ///
    pub fn from_frames_of_silence(
        frame_rate_hz: u32,
        num_channels: u16,
        num_frames: usize,
    ) -> Self {
        Waveform {
            frame_rate_hz,
            num_channels,
            num_frames,
            interleaved_samples: vec![0.0; num_channels as usize * num_frames],
        }
    }

    /// Create a silent waveform measured in milliseconds.
    ///
    /// # Arguments
    /// - `frame_rate_hz`: The frame rate of the waveform to create.
    /// - `num_channels`: The number of channels in the waveform to create.
    /// - `duration_milliseconds`: The length of the audio waveform in milliseconds.
    ///
    /// # Examples
    /// This creates a `Waveform` containing one second of silent *stereo* audio.
    /// ```
    /// use babycat::Waveform;
    ///
    /// let waveform = Waveform::from_milliseconds_of_silence(44100, 2, 1000);
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 44100}"
    /// );
    /// ```
    ///
    pub fn from_milliseconds_of_silence(
        frame_rate_hz: u32,
        num_channels: u16,
        duration_milliseconds: usize,
    ) -> Self {
        let num_frames = milliseconds_to_frames(frame_rate_hz, duration_milliseconds);
        Self::from_frames_of_silence(frame_rate_hz, num_channels, num_frames)
    }

    /// Resamples the waveform using the default resampler.
    ///
    /// # Arguments
    /// - `frame_rate_hz`: The destination frame rate to resample to.
    ///
    /// # Examples
    /// ```
    /// use babycat::Waveform;
    ///
    /// let waveform = Waveform::from_file(
    ///     "audio-for-tests/circus-of-freaks/track.flac",
    ///     Default::default()
    /// ).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", waveform),
    ///    "Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 2491247}"
    /// );
    ///
    /// let upsampled = waveform.resample(96000).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", upsampled),
    ///    "Waveform { frame_rate_hz: 96000, num_channels: 2, num_frames: 5423123}"
    /// );
    ///
    /// let downsampled = waveform.resample(8252).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", downsampled),
    ///    "Waveform { frame_rate_hz: 8252, num_channels: 2, num_frames: 466163}"
    /// );
    /// ```
    pub fn resample(&self, frame_rate_hz: u32) -> Result<Self, Error> {
        self.resample_by_mode(frame_rate_hz, DEFAULT_RESAMPLE_MODE)
    }

    /// Resamples the audio using a specific resampler.
    ///
    /// # Arguments
    /// - `frame_rate_hz`: The destination frame rate to resample to.
    /// - `resample_mode`: The Babycat resampling backend to pick.
    ///
    /// # Examples
    /// ```
    /// use babycat::Waveform;
    ///
    /// let waveform = Waveform::from_file(
    ///     "audio-for-tests/circus-of-freaks/track.flac",
    ///     Default::default()
    /// ).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", waveform),
    ///    "Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 2491247}"
    /// );
    ///
    /// // Here we upsample our audio to 96khz with the libsamplerate resampler.
    /// let upsampled_libsamplerate = waveform.resample_by_mode(
    ///     96000,
    ///     babycat::constants::RESAMPLE_MODE_LIBSAMPLERATE
    /// ).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", upsampled_libsamplerate),
    ///    "Waveform { frame_rate_hz: 96000, num_channels: 2, num_frames: 5423123}"
    /// );
    ///
    /// // And we upsample our audio again with Babycat's Lanczos resampler.
    /// let upsampled_lanczos = waveform.resample_by_mode(
    ///     96000,
    ///     babycat::constants::RESAMPLE_MODE_BABYCAT_LANCZOS
    /// ).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", upsampled_lanczos),
    ///    "Waveform { frame_rate_hz: 96000, num_channels: 2, num_frames: 5423123}"
    /// );
    /// ```
    pub fn resample_by_mode(&self, frame_rate_hz: u32, resample_mode: u32) -> Result<Self, Error> {
        let interleaved_samples = resample(
            self.frame_rate_hz,
            frame_rate_hz,
            self.num_channels,
            &self.interleaved_samples,
            resample_mode,
        )?;
        let num_frames = interleaved_samples.len() / self.num_channels as usize;
        Ok(Self {
            interleaved_samples,
            frame_rate_hz,
            num_channels: self.num_channels,
            num_frames,
        })
    }

    /// Encodes the waveform into a WAV-encoded byte array.
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

    /// Writes the waveform to the filesystem as a WAV file.
    ///
    /// # Feature flags
    /// This function is only available if the Cargo feature `enable-fileystem`
    /// flag is enabled. The `enable-filesystem` flag is enabled by default
    /// for the Babycat's Rust, Python, and C frontends, but is disabled
    /// for the WebAssembly frontend.
    ///
    #[cfg(feature = "enable-filesystem")]
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

    /// Constructs a `Waveform` from an already-decoded vector of 32-bit float samples.
    ///
    /// # Arguments
    /// - `frame_rate_hz`:
    /// - `num_channels`:
    /// - `interleaved_samples`:
    ///
    /// # Examples
    ///
    /// This creates a `Waveform` containing one second of silent *stereo* audio.
    /// Note that the input vector contains 88,200 audio samples--which we divide into
    /// 44,100 frames containing two samples each.
    /// ```
    /// use babycat::Waveform;
    ///
    /// let frame_rate_hz = 44100;
    /// let num_channels = 2;
    /// let raw_uncompressed_audio: Vec<f32> = vec![0.0_f32; 88200];
    /// let waveform = Waveform::new(frame_rate_hz, num_channels, raw_uncompressed_audio);
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 44100}"
    /// );
    /// ```
    ///
    pub fn new(frame_rate_hz: u32, num_channels: u16, interleaved_samples: Vec<f32>) -> Self {
        let num_frames = interleaved_samples.len() / num_channels as usize;
        Waveform {
            interleaved_samples,
            frame_rate_hz,
            num_channels,
            num_frames,
        }
    }

    /// Returns the total number of decoded frames in the `Waveform`.
    pub fn num_frames(&self) -> usize {
        self.num_frames
    }

    /// Returns the waveform as a slice of channel-interleaved `f32` samples.
    pub fn to_interleaved_samples(&self) -> &[f32] {
        &self.interleaved_samples
    }
}

impl Signal for Waveform {
    /// Returns the frame rate of the `Waveform`.
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    /// Returns the number of channels in the `Waveform`.
    fn num_channels(&self) -> u16 {
        self.num_channels
    }

    fn num_frames_estimate(&self) -> Option<usize> {
        Some(self.num_frames)
    }
}

impl From<Waveform> for Vec<f32> {
    fn from(waveform: Waveform) -> Vec<f32> {
        waveform.interleaved_samples
    }
}

/// These are unit tests for functionality that is currently specific to
/// the FFmpeg backend.
#[cfg(all(test, feature = "enable-ffmpeg"))]
mod test_waveform_from_file_ffmpeg {
    const TTCT_FILENAME_OGG: &str = "./audio-for-tests/32-channel-tone/track.ogg";
    const TTCT_FILENAME_WAV: &str = "./audio-for-tests/32-channel-tone/track.wav";
    const TTCT_NUM_CHANNELS: u16 = 32;
    const TTCT_NUM_FRAMES: usize = 88200;
    const TTCT_FRAME_RATE_HZ: u32 = 44100;

    use crate::constants::DECODING_BACKEND_FFMPEG;
    use crate::Error;
    use crate::Signal;
    use crate::Waveform;
    use crate::WaveformArgs;

    #[track_caller]
    #[inline(always)]
    fn assert_waveform(
        result: Result<Waveform, Error>,
        num_channels: u16,
        num_frames: usize,
        frame_rate_hz: u32,
    ) {
        let waveform = result.unwrap();
        assert_eq!(num_channels, waveform.num_channels());
        assert_eq!(num_frames, waveform.num_frames());
        assert_eq!(frame_rate_hz, waveform.frame_rate_hz());
        assert_eq!(
            (num_frames * num_channels as usize) as usize,
            waveform.to_interleaved_samples().len()
        );
    }

    /// Try decoding a 32-channel OGG file.
    #[test]
    fn test_32_channel_tone_ogg_default_1() {
        let waveform_args = WaveformArgs {
            decoding_backend: DECODING_BACKEND_FFMPEG,
            ..Default::default()
        };
        let result = Waveform::from_file(TTCT_FILENAME_OGG, waveform_args);
        assert_waveform(
            result,
            TTCT_NUM_CHANNELS,
            TTCT_NUM_FRAMES,
            TTCT_FRAME_RATE_HZ,
        );
    }

    /// Try decoding a 32 channel WAV file.
    #[test]
    fn test_32_channel_tone_wav_default_1() {
        let waveform_args = WaveformArgs {
            decoding_backend: DECODING_BACKEND_FFMPEG,
            ..Default::default()
        };
        let result = Waveform::from_file(TTCT_FILENAME_WAV, waveform_args);
        assert_waveform(
            result,
            TTCT_NUM_CHANNELS,
            TTCT_NUM_FRAMES,
            TTCT_FRAME_RATE_HZ,
        );
    }
}
