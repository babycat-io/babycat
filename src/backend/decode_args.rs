use serde::{Deserialize, Serialize};

/// The default file extension hint when decoding. Hints are not necessary.
pub const DEFAULT_FILE_EXTENSION: &str = "";
/// The default MIME type hint when decoding. Hints are not necessary.
pub const DEFAULT_MIME_TYPE: &str = "";
/// The default start time cutoff when decoding audio. We start at the beginning.
pub const DEFAULT_START_TIME_MILLISECONDS: u64 = 0;
/// The default end time cutoff when decoding audio. We continue decoding until the end of the file.
pub const DEFAULT_END_TIME_MILLISECONDS: u64 = 0;
/// The default frame rate to resample to. By default, we do not change the frame rate's audio.
pub const DEFAULT_FRAME_RATE_HZ: u32 = 0;
/// The default number of channels to decode. By default, we decode all of the available channels.
pub const DEFAULT_NUM_CHANNELS: u32 = 0;
/// By default, we do not flatten all audio channels into a mono channel.
pub const DEFAULT_CONVERT_TO_MONO: bool = false;
/// By default, we do not zero-pad the ending of an audio file.
pub const DEFAULT_ZERO_PAD_ENDING: bool = false;
/// Sets the default resampler.
pub const DEFAULT_RESAMPLE_MODE: u32 = 0;
/// Sets the default audio decoding backend.
pub const DEFAULT_DECODING_BACKEND: u32 = 0;

/// Use this value to resample audio with libsamplerate.
///
/// The libsamplerate resampler is not available when Babycat
/// is compiled to the `wasm32-unknown-unknown` WebAssembly target.
pub const RESAMPLE_MODE_LIBSAMPLERATE: u32 = 1;
/// Use this value to resample audio with Babycat's Lanczos resampler.
pub const RESAMPLE_MODE_BABYCAT_LANCZOS: u32 = 2;
/// Use this value to resample audio with Babycat's sinc resampler.
pub const RESAMPLE_MODE_BABYCAT_SINC: u32 = 3;

/// Sets the decoding backend as [`SymphoniaDecoder`](crate::decode::SymphoniaDecoder).
pub const DECODING_BACKEND_SYMPHONIA: u32 = 1;

/// Specifies what transformations to apply to the audio during the decoding
/// process.
///
/// The default value for this struct will tell Babycat to decode audio
/// as-is and not change anything.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecodeArgs {
    /// We discard any audio before this millisecond
    /// offset. By default, this does nothing and the
    /// audio is decoded from the beginning.
    /// Negative offsets are invalid.
    #[serde(default)]
    pub start_time_milliseconds: u64,
    /// We discard any audio after this millisecond offset. By default,
    /// this does nothing and the audio is decoded all the way
    /// to the end. If `start_time_milliseconds` is specified,
    /// then `end_time_milliseconds` must be greater. The resulting
    #[serde(default)]
    pub end_time_milliseconds: u64,
    /// A destination frame rate to resample
    /// the audio to. Do not specify this parameter if you wish
    /// Babycat to preserve the audio's original frame rate.
    /// This does nothing if `frame_rate_hz` is equal to the
    /// audio's original frame rate.
    #[serde(default)]
    pub frame_rate_hz: u32,
    /// Set this to a positive integer `n`
    /// to select the *first* `n` channels stored in the
    /// audio file. By default, Babycat will return all of the channels
    /// in the original audio. This will raise an exception
    /// if you specify a `num_channels` greater than the actual
    /// number of channels in the audio.
    #[serde(default)]
    pub num_channels: u32,
    /// Set to `true` to average all channels
    /// into a single monophonic (mono) channel. If
    /// `num_channels = n` is also specified, then only the
    /// first `n` channels will be averaged. Note that
    /// `convert_to_mono` cannot be set to `true` while
    /// also setting `num_channels = 1`.
    #[serde(default)]
    pub convert_to_mono: bool,
    /// If you set this to `true`,
    /// Babycat will zero-pad the ending of the decoded waveform
    /// to ensure that the output waveform's duration is exactly
    /// `end_time_milliseconds - start_time_milliseconds`.
    /// By default, `zero_pad_ending = false`, in which case
    /// the output waveform will be shorter than
    /// `end_time_milliseconds - start_time_milliseconds`
    /// if the input audio is shorter than `end_time_milliseconds`.
    #[serde(default)]
    pub zero_pad_ending: bool,
    /// Sets which resampling method is used if you have set `frame_rate_hz`.
    /// This usually defaults to the highest-accuracy resampler compiled
    /// into Babycat. The available choices are:
    /// * [`babycat::RESAMPLE_MODE_LIBSAMPLERATE`](crate::RESAMPLE_MODE_LIBSAMPLERATE)
    /// * [`babycat::RESAMPLE_MODE_BABYCAT_LANCZOS`](crate::RESAMPLE_MODE_BABYCAT_LANCZOS)
    /// * [`babycat::RESAMPLE_MODE_BABYCAT_SINC`](crate::RESAMPLE_MODE_BABYCAT_SINC)
    ///
    #[serde(default)]
    pub resample_mode: u32,
    #[serde(default)]
    /// Sets which audio decoding backend to use.
    /// Currently the only available decoding backend is
    /// [`babycat::DECODING_BACKEND_SYMPHONIA`](crate::DECODING_BACKEND_SYMPHONIA), which
    /// corresponds to the [`SymphoniaDecoder`](crate::decode::SymphoniaDecoder), which
    /// is a wrapper for the [`symphonia`] library.
    pub decoding_backend: u32,
}

impl Default for DecodeArgs {
    fn default() -> Self {
        DecodeArgs {
            start_time_milliseconds: DEFAULT_START_TIME_MILLISECONDS,
            end_time_milliseconds: DEFAULT_END_TIME_MILLISECONDS,
            frame_rate_hz: DEFAULT_FRAME_RATE_HZ,
            num_channels: DEFAULT_NUM_CHANNELS,
            convert_to_mono: DEFAULT_CONVERT_TO_MONO,
            zero_pad_ending: DEFAULT_ZERO_PAD_ENDING,
            resample_mode: DEFAULT_RESAMPLE_MODE,
            decoding_backend: DEFAULT_DECODING_BACKEND,
        }
    }
}
