use std::fmt;
use std::io::Read;
use std::marker::Send;

use serde::{Deserialize, Serialize};

#[cfg(feature = "enable-multithreading")]
use crate::backend::batch_args::BatchArgs;
#[cfg(feature = "enable-multithreading")]
use crate::backend::named_result::NamedResult;
#[cfg(feature = "enable-multithreading")]
use rayon::prelude::*;

use crate::backend::common::milliseconds_to_frames;
use crate::backend::decode::Decoder;
use crate::backend::decode::SymphoniaDecoder;
use crate::backend::decode_args::*;
use crate::backend::errors::Error;
use crate::backend::resample::resample;

/// Represents a fixed-length audio waveform as a `Vec<f32>`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatWaveform {
    interleaved_samples: Vec<f32>,
    frame_rate_hz: u32,
    num_channels: u32,
    num_frames: u64,
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
    ///
    pub fn from_interleaved_samples(
        frame_rate_hz: u32,
        num_channels: u32,
        interleaved_samples: &[f32],
    ) -> Self {
        Self::new(frame_rate_hz, num_channels, interleaved_samples.to_owned())
    }
    /// Decodes audio stored in an in-memory byte array.
    ///
    /// # Arguments
    /// - `encoded_bytes`: A byte array containing encoded (e.g. MP3) audio.
    /// - `decode_args`: Instructions on how to decode the audio.
    ///
    /// # Examples
    /// ```
    /// use babycat::FloatWaveform;
    ///
    /// let encoded_bytes: Vec<u8> = std::fs::read("audio-for-tests/andreas-theme/track.mp3").unwrap();
    ///
    /// let decode_args = Default::default();
    ///
    /// let waveform = FloatWaveform::from_encoded_bytes(
    ///     &encoded_bytes,
    ///     decode_args,
    /// ).unwrap();
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 9586944}"
    /// );
    /// ```
    ///
    pub fn from_encoded_bytes(
        encoded_bytes: &[u8],
        decode_args: DecodeArgs,
    ) -> Result<Self, Error> {
        Self::from_encoded_bytes_with_hint(
            encoded_bytes,
            decode_args,
            DEFAULT_FILE_EXTENSION,
            DEFAULT_MIME_TYPE,
        )
    }

    /// Decodes audio in an in-memory byte array, using user-specified encoding hints.
    ///
    /// # Arguments
    /// - `encoded_bytes`: A byte array containing encoded (e.g. MP3) audio.
    /// - `decode_args`: Instructions on how to decode the audio.
    /// - `file_extension`: A hint--in the form of a file extension--to indicate
    ///    the encoding of the audio in `encoded_bytes`.
    /// - `mime_type`: A hint--in the form of a MIME type--to indicate
    ///    the encoding of the audio in `encoded_bytes`.
    ///
    pub fn from_encoded_bytes_with_hint(
        encoded_bytes: &[u8],
        decode_args: DecodeArgs,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Self, Error> {
        let owned = encoded_bytes.to_owned();
        let encoded_stream = std::io::Cursor::new(owned);
        Self::from_encoded_stream_with_hint(encoded_stream, decode_args, file_extension, mime_type)
    }

    /// Decodes audio stored in a local file.
    ///
    /// # Arguments
    /// - `filename`: A filename of an encoded audio file on the local filesystem.
    /// - `decode_args`: Instructions on how to decode the audio.
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
    /// use babycat::{DecodeArgs, FloatWaveform};
    ///
    /// let waveform = FloatWaveform::from_file(
    ///    "audio-for-tests/circus-of-freaks/track.mp3",
    ///     Default::default(),
    /// ).unwrap();
    ///
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 2491776}"
    /// );
    /// ```
    ///
    /// **Decode only the first 30 seconds and upsample to 48khz:**
    /// ```
    /// use babycat::{DecodeArgs, FloatWaveform};
    ///
    /// let decode_args = DecodeArgs {
    ///     end_time_milliseconds: 30000,
    ///     frame_rate_hz: 48000,
    ///     ..Default::default()
    /// };
    /// let waveform = FloatWaveform::from_file(
    ///    "audio-for-tests/circus-of-freaks/track.mp3",
    ///     decode_args,
    /// ).unwrap();
    ///
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "FloatWaveform { frame_rate_hz: 48000, num_channels: 2, num_frames: 1440000}"
    /// );
    /// ```
    #[cfg(feature = "enable-filesystem")]
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

    /// Decodes a list of audio files in parallel.
    ///
    /// # Arguments
    /// - `filenames`: A filename of an encoded audio file on the local filesystem.
    /// - `decode_args`: Instructions on how to decode the audio.
    /// - `batch_args`: Instructions on how to divide the work across multiple threads.
    ///
    /// # Feature flags
    /// This function is only available if both of the `enable-filesystem`
    /// and `enable-multithreading` features are enabled. These features
    /// are enabled by default in Babycat's Rust, Python, and C frontends.
    /// These features are disabled in Babycat's WebAssembly frontend.
    ///
    /// # Examples
    /// **(Attempt to) decode three files:**
    ///
    /// In this example, we process three filenames and demonstrate how to handle errors.
    /// The first two files are successfully processed, and we catch a
    /// [`Error::FileNotFound`][crate::Error::FileNotFound] error when processing the third file.
    /// ```
    /// use babycat::{Error, FloatWaveform, NamedResult};
    ///
    /// let filenames = &[
    ///     "audio-for-tests/andreas-theme/track.mp3",
    ///     "audio-for-tests/blippy-trance/track.mp3",
    ///     "does-not-exist",
    /// ];
    /// let decode_args = Default::default();
    /// let batch_args = Default::default();
    /// let batch = babycat::FloatWaveform::from_many_files(
    ///     filenames,
    ///     decode_args,
    ///     batch_args
    /// );
    ///
    /// fn display_result(nr: &NamedResult<FloatWaveform, Error>) -> String {
    ///     match &nr.result {
    ///         Ok(waveform) => format!("\nSuccess: {}:\n{:?}", nr.name, waveform),
    ///         Err(err) => format!("\nFailure: {}:\n{}", nr.name, err),
    ///     }
    /// }
    /// assert_eq!(
    ///     display_result(&batch[0]),
    ///      "
    /// Success: audio-for-tests/andreas-theme/track.mp3:
    /// FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 9586944}",
    /// );
    /// assert_eq!(
    ///     display_result(&batch[1]),
    ///      "
    /// Success: audio-for-tests/blippy-trance/track.mp3:
    /// FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 5293440}",
    /// );
    /// assert_eq!(
    ///     display_result(&batch[2]),
    ///      "
    /// Failure: does-not-exist:
    /// Cannot find the given filename does-not-exist.",
    /// );
    /// ```
    #[cfg(all(feature = "enable-multithreading", feature = "enable-filesystem"))]
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
                    result: Self::from_file(filename, decode_args),
                })
                .collect::<Vec<NamedResult<Self, Error>>>()
        })
    }

    /// Decodes audio from an input stream.
    ///
    /// [`FloatWaveform`][crate::FloatWaveform] will take ownership of the stream
    /// and read it until the end. Therefore, you cannot provide an infinte-length
    /// stream.
    ///
    /// # Arguments
    /// - `encoded_stream`: An I/O stream of encoded audio to decode.
    /// - `decode_args`: Instructions on how to decode the audio.
    ///
    pub fn from_encoded_stream<R: 'static + Read + Send>(
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

    /// Decodes audio from an input stream, using a user-specified decoding hint.
    ///
    /// # Arguments
    /// - `encoded_stream`: An I/O stream of encoded audio to decode.
    /// - `decode_args`: Instructions on how to decode the audio.
    /// - `file_extension`: A hint--in the form of a file extension--to indicate
    ///    the encoding of the audio in `encoded_bytes`.
    /// - `mime_type`: A hint--in the form of a MIME type--to indicate
    ///   the encoding of the audio in `encoded_bytes`.
    ///
    pub fn from_encoded_stream_with_hint<R: 'static + Read + Send>(
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

        // Initialize our audio decoding backend.
        let mut decoder = match decode_args.decoding_backend {
            DEFAULT_DECODING_BACKEND | DECODING_BACKEND_SYMPHONIA => {
                SymphoniaDecoder::new(encoded_stream, file_extension, mime_type)?
            }
            _ => return Err(Error::UnknownDecodingBackend(decode_args.decoding_backend)),
        };

        // Examine the actual shape of this audio file.
        let original_frame_rate_hz = decoder.frame_rate_hz();
        let original_num_channels = decoder.num_channels();

        // If the user provided a negative frame rate, throw an error.
        // We waited this long to throw an error because we also want to
        // tell them what the REAL frame rate is for this audio track.
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
                return Err(Error::WrongNumChannels(
                    decode_args.num_channels,
                    original_num_channels,
                ));
            } else if original_num_channels >= decode_args.num_channels {
                decode_args.num_channels
            } else {
                return Err(Error::WrongNumChannels(
                    decode_args.num_channels,
                    original_num_channels,
                ));
            }
        };

        // Compute the exact start and end sample indexes for us to begin
        // and end decoding.
        let start_time_frames: u64 =
            decode_args.start_time_milliseconds * original_frame_rate_hz as u64 / 1000;
        let end_time_frames: u64 =
            decode_args.end_time_milliseconds * original_frame_rate_hz as u64 / 1000;

        let start_time_samples = start_time_frames * original_num_channels as u64;
        let end_time_samples = end_time_frames * original_num_channels as u64;

        // Decode all packets, ignoring decode errors.
        let mut buffer: Vec<f32> = Vec::new();
        let mut current_sample_idx: u64 = 0;

        'frame_loop: loop {
            // If the current sample is before our start offset,
            // then ignore it.
            if current_sample_idx < start_time_samples {
                let _ = decoder.next();
                current_sample_idx += 1;
                continue;
            }
            // If we have a defined end offset and we are past it,
            // then stop the decoding loop entirely.
            if decode_args.end_time_milliseconds != DEFAULT_END_TIME_MILLISECONDS
                && current_sample_idx >= end_time_samples
            {
                break;
            }

            if decode_args.convert_to_mono {
                let mut current_sample_sum = 0.0_f32;
                for channel_idx in 0..original_num_channels {
                    match decoder.next() {
                        None => break 'frame_loop,
                        Some(next_sample_result) => {
                            current_sample_idx += 1;
                            let next_sample = next_sample_result?;
                            if channel_idx < selected_num_channels {
                                current_sample_sum += next_sample;
                            }
                        }
                    }
                }
                current_sample_sum /= selected_num_channels as f32;
                buffer.push(current_sample_sum);
            } else {
                for channel_idx in 0..original_num_channels {
                    match decoder.next() {
                        None => break 'frame_loop,
                        Some(next_sample_result) => {
                            current_sample_idx += 1;
                            if channel_idx < selected_num_channels {
                                let next_sample = next_sample_result?;
                                buffer.push(next_sample);
                            }
                        }
                    }
                }
            }
        }
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
            let expected_buffer_len = (end_time_frames - start_time_frames) * num_channels as u64;
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

        let num_frames = buffer.len() as u64 / num_channels as u64;
        Ok(FloatWaveform {
            frame_rate_hz: final_frame_rate_hz,
            num_channels,
            num_frames,
            interleaved_samples: buffer,
        })
    }

    /// Creates a silent waveform measured in frames.
    ///
    /// # Arguments
    /// - `frame_rate_hz`: The frame rate of the waveform to create.
    /// - `num_channels`: The number of channels in the waveform to create.
    /// - `num_frames`: The number of frames of audio to generate.
    ///
    /// # Examples
    /// This creates a `FloatWaveform` containing one second of silent *stereo* audio.
    /// ```
    /// use babycat::FloatWaveform;
    ///
    /// let waveform = FloatWaveform::from_frames_of_silence(44100, 2, 44100);
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 44100}"
    /// );
    /// ```
    ///
    pub fn from_frames_of_silence(frame_rate_hz: u32, num_channels: u32, num_frames: u64) -> Self {
        FloatWaveform {
            frame_rate_hz,
            num_channels,
            num_frames,
            interleaved_samples: vec![0.0; (num_channels as u64 * num_frames) as usize],
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
    /// This creates a `FloatWaveform` containing one second of silent *stereo* audio.
    /// ```
    /// use babycat::FloatWaveform;
    ///
    /// let waveform = FloatWaveform::from_milliseconds_of_silence(44100, 2, 1000);
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 44100}"
    /// );
    /// ```
    ///
    pub fn from_milliseconds_of_silence(
        frame_rate_hz: u32,
        num_channels: u32,
        duration_milliseconds: u64,
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
    /// use babycat::FloatWaveform;
    ///
    /// let waveform = FloatWaveform::from_file(
    ///     "audio-for-tests/circus-of-freaks/track.mp3",
    ///     Default::default()
    /// ).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", waveform),
    ///    "FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 2491776}"
    /// );
    ///
    /// let upsampled = waveform.resample(96000).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", upsampled),
    ///    "FloatWaveform { frame_rate_hz: 96000, num_channels: 2, num_frames: 5424275}"
    /// );
    ///
    /// let downsampled = waveform.resample(8252).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", downsampled),
    ///    "FloatWaveform { frame_rate_hz: 8252, num_channels: 2, num_frames: 466262}"
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
    /// use babycat::FloatWaveform;
    ///
    /// let waveform = FloatWaveform::from_file(
    ///     "audio-for-tests/circus-of-freaks/track.mp3",
    ///     Default::default()
    /// ).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", waveform),
    ///    "FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 2491776}"
    /// );
    ///
    /// // Here we upsample our audio to 96khz with the libsamplerate resampler.
    /// let upsampled_libsamplerate = waveform.resample_by_mode(
    ///     96000,
    ///     babycat::RESAMPLE_MODE_LIBSAMPLERATE
    /// ).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", upsampled_libsamplerate),
    ///    "FloatWaveform { frame_rate_hz: 96000, num_channels: 2, num_frames: 5424275}"
    /// );
    ///
    /// // And we upsample our audio again with Babycat's Lanczos resampler.
    /// let upsampled_lanczos = waveform.resample_by_mode(
    ///     96000,
    ///     babycat::RESAMPLE_MODE_BABYCAT_LANCZOS
    /// ).unwrap();
    /// assert_eq!(
    ///    format!("{:?}", upsampled_lanczos),
    ///    "FloatWaveform { frame_rate_hz: 96000, num_channels: 2, num_frames: 5424275}"
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
        let num_frames = interleaved_samples.len() as u64 / self.num_channels as u64;
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

    /// Constructs a `FloatWaveform` from an already-decoded vector of 32-bit float samples.
    ///
    /// # Arguments
    /// - `frame_rate_hz`:
    /// - `num_channels`:
    /// - `interleaved_samples`:
    ///
    /// # Examples
    ///
    /// This creates a `FloatWaveform` containing one second of silent *stereo* audio.
    /// Note that the input vector contains 88,200 audio samples--which we divide into
    /// 44,100 frames containing two samples each.
    /// ```
    /// use babycat::FloatWaveform;
    ///
    /// let frame_rate_hz = 44100;
    /// let num_channels = 2;
    /// let raw_uncompressed_audio: Vec<f32> = vec![0.0_f32; 88200];
    /// let waveform = FloatWaveform::new(frame_rate_hz, num_channels, raw_uncompressed_audio);
    /// assert_eq!(
    ///     format!("{:?}", waveform),
    ///     "FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 44100}"
    /// );
    /// ```
    ///
    pub fn new(frame_rate_hz: u32, num_channels: u32, interleaved_samples: Vec<f32>) -> Self {
        let num_frames = interleaved_samples.len() as u64 / num_channels as u64;
        FloatWaveform {
            interleaved_samples,
            frame_rate_hz,
            num_channels,
            num_frames,
        }
    }

    /// Returns the frame rate of the `FloatWaveform`.
    pub fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    /// Returns the number of channels in the `FloatWaveform`.
    pub fn num_channels(&self) -> u32 {
        self.num_channels
    }

    /// Returns the total number of decoded frames in the `FloatWaveform`.
    pub fn num_frames(&self) -> u64 {
        self.num_frames
    }

    /// Returns the waveform as a slice of channel-interleaved `f32` samples.
    pub fn to_interleaved_samples(&self) -> &[f32] {
        &self.interleaved_samples
    }
}
