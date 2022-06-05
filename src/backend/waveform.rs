use std::io::Read;
use std::marker::Send;
use std::marker::Sync;

use either::Either::{Left, Right};
use serde::{Deserialize, Serialize};

use crate::backend::constants::{
    DEFAULT_END_TIME_MILLISECONDS, DEFAULT_FRAME_RATE_HZ, DEFAULT_NUM_CHANNELS,
    DEFAULT_RESAMPLE_MODE, DEFAULT_START_TIME_MILLISECONDS,
};
use crate::backend::decoder;
use crate::backend::display::est_num_frames_to_str;
use crate::backend::errors::Error;
use crate::backend::resample::resample;
use crate::backend::source::WaveformSource;
use crate::backend::units::milliseconds_to_frames;
use crate::backend::Signal;
use crate::backend::Source;
use crate::backend::WaveformArgs;

/// Represents a fixed-length audio waveform as a `Vec<f32>`.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Waveform {
    interleaved_samples: Vec<f32>,
    frame_rate_hz: u32,
    num_channels: u16,
    num_frames: usize,
}

impl std::fmt::Debug for Waveform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Waveform {{ {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
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

    pub fn from_source(args: WaveformArgs, source: Box<dyn Source + '_>) -> Result<Self, Error> {
        let original_frame_rate_hz = source.frame_rate_hz();
        let original_num_channels = source.num_channels();

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

        // The user cannot set zero_pad_ending and repeat_pad_ending at the same time.
        if args.zero_pad_ending && args.repeat_pad_ending {
            return Err(Error::CannotSetZeroPadEndingAndRepeatPadEnding);
        }

        // If the user has not specified how long the output audio should be,
        // then we would not know how to zero-pad after it.
        if args.zero_pad_ending && args.end_time_milliseconds == DEFAULT_END_TIME_MILLISECONDS {
            return Err(Error::CannotZeroPadWithoutSpecifiedLength);
        }

        // If the user has not specified how long the output audio should be,
        // then we would not know how to repeat-pad after it.
        if args.repeat_pad_ending && args.end_time_milliseconds == DEFAULT_END_TIME_MILLISECONDS {
            return Err(Error::CannotRepeatPadWithoutSpecifiedLength);
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
            milliseconds_to_frames(args.start_time_milliseconds, original_frame_rate_hz);
        let end_frame_idx =
            milliseconds_to_frames(args.end_time_milliseconds, original_frame_rate_hz);

        let take_frames = end_frame_idx.saturating_sub(start_frame_idx);

        // Skip frames.
        let source = if start_frame_idx != 0 {
            Left(source.skip_frames(start_frame_idx))
        } else {
            Right(source)
        };

        // Take frames.
        let source = if take_frames != 0 {
            Left(source.take_frames(take_frames))
        } else {
            Right(source)
        };

        // Select the first n channels.
        let source = if selected_num_channels != original_num_channels {
            Left(source.select_first_channels(selected_num_channels))
        } else {
            Right(source)
        };

        // Convert to mono.
        let source = if args.convert_to_mono {
            Left(source.convert_to_mono())
        } else {
            Right(source)
        };

        let mut interleaved_samples: Vec<f32> = source.collect();

        // Pad the waveform if necessary.
        if (args.zero_pad_ending || args.repeat_pad_ending) && end_frame_idx > start_frame_idx {
            let expected_buffer_len_from_user: usize =
                (end_frame_idx - start_frame_idx) * output_num_channels as usize;
            let actual_buffer_len = interleaved_samples.len();
            if expected_buffer_len_from_user > actual_buffer_len {
                // Pad with zeros.
                if args.zero_pad_ending {
                    interleaved_samples.resize(expected_buffer_len_from_user, 0.0_f32);
                }
                // Pad with elements from the beginning, looping multiple times if necessary.
                else if args.repeat_pad_ending {
                    let buffer_padding_len = expected_buffer_len_from_user - actual_buffer_len;
                    for idx in 0..buffer_padding_len {
                        let rounded_idx = idx % actual_buffer_len;
                        interleaved_samples.push(interleaved_samples[rounded_idx]);
                    }
                }
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
    /// use babycat::assertions::assert_debug;
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
    /// assert_debug(
    ///     &waveform,
    ///     "Waveform { 9586415 frames,  2 channels,  44100 hz,  3m 37s 379ms }",
    /// );
    /// ```
    ///
    pub fn from_encoded_bytes(
        encoded_bytes: &[u8],
        waveform_args: WaveformArgs,
    ) -> Result<Self, Error> {
        let d =
            decoder::from_encoded_bytes_by_backend(waveform_args.decoding_backend, encoded_bytes)?;
        Self::from_source(waveform_args, d)
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
        let d = decoder::from_encoded_bytes_with_hint_by_backend(
            waveform_args.decoding_backend,
            encoded_bytes,
            file_extension,
            mime_type,
        )?;
        Self::from_source(waveform_args, d)
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
    /// use babycat::{assertions::assert_debug, WaveformArgs, Waveform};
    ///
    /// let waveform = Waveform::from_file(
    ///    "audio-for-tests/circus-of-freaks/track.flac",
    ///     Default::default(),
    /// ).unwrap();
    ///
    /// assert_debug(
    ///     &waveform,
    ///     "Waveform { 2491247 frames,  2 channels,  44100 hz,  56s 490ms }",
    /// );
    ///
    /// ```
    ///
    /// **Decode only the first 30 seconds and upsample to 48khz:**
    /// ```
    /// use babycat::{assertions::assert_debug, WaveformArgs, Waveform};
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
    /// assert_debug(
    ///     &waveform,
    ///     "Waveform { 1440000 frames,  2 channels,  48000 hz,  30s }"    
    /// );
    /// ```
    #[cfg(feature = "enable-filesystem")]
    pub fn from_file(filename: &str, waveform_args: WaveformArgs) -> Result<Self, Error> {
        let d = decoder::from_file_by_backend(waveform_args.decoding_backend, filename)?;
        Self::from_source(waveform_args, d)
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
        let d = decoder::from_encoded_stream_by_backend(
            waveform_args.decoding_backend,
            encoded_stream,
        )?;
        Self::from_source(waveform_args, d)
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
        let d = decoder::from_encoded_stream_with_hint_by_backend(
            waveform_args.decoding_backend,
            encoded_stream,
            file_extension,
            mime_type,
        )?;
        Self::from_source(waveform_args, d)
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
    /// use babycat::assertions::assert_debug;
    ///
    /// let waveform = Waveform::from_frames_of_silence(44100, 2, 44100);
    /// assert_debug(
    ///     &waveform,
    ///     "Waveform { 44100 frames,  2 channels,  44100 hz,  1s }"
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
    /// use babycat::assertions::assert_debug;
    ///
    /// let waveform = Waveform::from_milliseconds_of_silence(44100, 2, 1000);
    /// assert_debug(
    ///     &waveform,
    ///     "Waveform { 44100 frames,  2 channels,  44100 hz,  1s }",
    /// );
    /// ```
    ///
    pub fn from_milliseconds_of_silence(
        frame_rate_hz: u32,
        num_channels: u16,
        duration_milliseconds: usize,
    ) -> Self {
        let num_frames = milliseconds_to_frames(duration_milliseconds, frame_rate_hz);
        Self::from_frames_of_silence(frame_rate_hz, num_channels, num_frames)
    }

    /// Resamples the waveform using the default resampler.
    ///
    /// # Arguments
    /// - `frame_rate_hz`: The destination frame rate to resample to.
    ///
    /// # Examples
    /// ```
    /// use babycat::{assertions::assert_debug, Waveform};
    ///
    /// let waveform = Waveform::from_file(
    ///     "audio-for-tests/circus-of-freaks/track.flac",
    ///     Default::default()
    /// ).unwrap();
    /// assert_debug(
    ///    &waveform,
    ///    "Waveform { 2491247 frames,  2 channels,  44100 hz,  56s 490ms }"
    /// );
    ///
    /// let upsampled = waveform.resample(96000).unwrap();
    /// assert_debug(
    ///     &upsampled,
    ///     "Waveform { 5423123 frames,  2 channels,  96000 hz,  56s 490ms }"
    /// );
    ///
    /// let downsampled = waveform.resample(8252).unwrap();
    /// assert_debug(
    ///     &downsampled,
    ///     "Waveform { 466163 frames,  2 channels,  8252 hz,  56s 490ms }",
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
    /// use babycat::{assertions::assert_debug, Waveform};
    ///
    /// let waveform = Waveform::from_file(
    ///     "audio-for-tests/circus-of-freaks/track.flac",
    ///     Default::default()
    /// ).unwrap();
    /// assert_debug(
    ///     &waveform,
    ///     "Waveform { 2491247 frames,  2 channels,  44100 hz,  56s 490ms }",
    /// );
    ///
    /// // Here we upsample our audio to 96khz with the libsamplerate resampler.
    /// let upsampled_libsamplerate = waveform.resample_by_mode(
    ///     96000,
    ///     babycat::constants::RESAMPLE_MODE_LIBSAMPLERATE
    /// ).unwrap();
    /// assert_debug(
    ///     &upsampled_libsamplerate,
    ///     "Waveform { 5423123 frames,  2 channels,  96000 hz,  56s 490ms }",
    /// );
    ///
    /// // And we upsample our audio again with Babycat's Lanczos resampler.
    /// let upsampled_lanczos = waveform.resample_by_mode(
    ///     96000,
    ///     babycat::constants::RESAMPLE_MODE_BABYCAT_LANCZOS
    /// ).unwrap();
    /// assert_debug(
    ///     &upsampled_lanczos,
    ///     "Waveform { 5423123 frames,  2 channels,  96000 hz,  56s 490ms }",
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
    /// use babycat::assertions::assert_debug;
    ///
    /// let frame_rate_hz = 44100;
    /// let num_channels = 2;
    /// let raw_uncompressed_audio: Vec<f32> = vec![0.0_f32; 88200];
    /// let waveform = Waveform::new(frame_rate_hz, num_channels, raw_uncompressed_audio);
    /// assert_debug(
    ///     &waveform,
    ///     "Waveform { 44100 frames,  2 channels,  44100 hz,  1s }",
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

    /// Return a given audio sample belonging to a specific frame and channel.
    ///
    /// This method performs bounds checks before returning an audio sample.
    /// If you want a method that does not perform bounds checks,
    /// use [`get_unchecked_sample`](crate::Waveform::get_unchecked_sample).
    ///
    /// # Examples
    /// ```
    /// use babycat::Waveform;
    ///
    /// let interleaved_samples: Vec<f32> = vec![
    ///    -1.0, -0.9, -0.8, //
    ///    -0.7, -0.6, -0.5, //
    ///    -0.4, -0.3, -0.2, //
    ///    -0.1, 0.0, 0.1, //
    ///    0.2, 0.3, 0.4,
    /// ];
    ///
    /// let frame_rate_hz = 44100;
    /// let num_channels = 3;
    ///
    /// let waveform = Waveform::from_interleaved_samples(
    ///     frame_rate_hz, num_channels, &interleaved_samples,
    /// );
    ///
    /// assert_eq!(waveform.get_sample(0, 0).unwrap(), -1.0);
    /// assert_eq!(waveform.get_sample(0, 1).unwrap(), -0.9);
    /// assert_eq!(waveform.get_sample(0, 2).unwrap(), -0.8);
    ///
    /// assert_eq!(waveform.get_sample(1, 0).unwrap(), -0.7);
    /// assert_eq!(waveform.get_sample(1, 1).unwrap(), -0.6);
    /// assert_eq!(waveform.get_sample(1, 2).unwrap(), -0.5);
    /// ```
    #[inline]
    pub fn get_sample(&self, frame_idx: usize, channel_idx: u16) -> Option<f32> {
        if frame_idx >= self.num_frames || channel_idx >= self.num_channels {
            return None;
        }
        unsafe { Some(self.get_unchecked_sample(frame_idx, channel_idx)) }
    }

    /// Return a given audio sample belonging to a specific frame and channel,
    /// *without* performing any bounds checks.
    ///
    /// If you want a method that performs bounds checks,
    /// use [`get_sample`](crate::Waveform::get_sample).
    ///
    /// # Examples
    /// ```
    /// use babycat::Waveform;
    ///
    /// let interleaved_samples: Vec<f32> = vec![
    ///    -1.0, -0.9, -0.8, //
    ///    -0.7, -0.6, -0.5, //
    ///    -0.4, -0.3, -0.2, //
    ///    -0.1, 0.0, 0.1, //
    ///    0.2, 0.3, 0.4,
    /// ];
    ///
    /// let frame_rate_hz = 44100;
    /// let num_channels = 3;
    ///
    /// let waveform = Waveform::from_interleaved_samples(
    ///     frame_rate_hz, num_channels, &interleaved_samples,
    /// );
    ///
    /// unsafe {
    ///     assert_eq!(waveform.get_unchecked_sample(0, 0), -1.0);
    ///     assert_eq!(waveform.get_unchecked_sample(0, 1), -0.9);
    ///     assert_eq!(waveform.get_unchecked_sample(0, 2), -0.8);
    ///
    ///     assert_eq!(waveform.get_unchecked_sample(1, 0), -0.7);
    ///     assert_eq!(waveform.get_unchecked_sample(1, 1), -0.6);
    ///     assert_eq!(waveform.get_unchecked_sample(1, 2), -0.5);
    /// }
    /// ```
    ///
    /// # Safety
    /// Because this method does not peform any bounds checks, it is unsafe.
    #[inline]
    pub unsafe fn get_unchecked_sample(&self, frame_idx: usize, channel_idx: u16) -> f32 {
        *(self
            .interleaved_samples
            .get_unchecked(frame_idx * self.num_channels as usize + channel_idx as usize))
    }

    #[inline]
    pub fn get_interleaved_sample(&self, sample_idx: usize) -> Option<f32> {
        self.interleaved_samples.get(sample_idx).copied()
    }

    /// # Safety
    /// Because this method does not peform any bounds checks, it is unsafe.
    #[inline]
    pub unsafe fn get_unchecked_interleaved_sample(&self, sample_idx: usize) -> f32 {
        *(self.interleaved_samples.get_unchecked(sample_idx))
    }

    pub fn num_samples(&self) -> usize {
        self.interleaved_samples.len()
    }

    /// Returns the total number of decoded frames in the `Waveform`.
    pub fn num_frames(&self) -> usize {
        self.num_frames
    }

    /// Returns the waveform as a slice of channel-interleaved `f32` samples.
    pub fn to_interleaved_samples(&self) -> &[f32] {
        &self.interleaved_samples
    }

    pub fn into_source(self) -> WaveformSource {
        WaveformSource::new(self)
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
    #[inline]
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
