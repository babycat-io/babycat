use serde::{Deserialize, Serialize};

pub const DEFAULT_FILE_EXTENSION: &str = "";
pub const DEFAULT_MIME_TYPE: &str = "";
pub const DEFAULT_START_TIME_MILLISECONDS: u64 = 0;
pub const DEFAULT_END_TIME_MILLISECONDS: u64 = 0;
pub const DEFAULT_FRAME_RATE_HZ: u32 = 0;
pub const DEFAULT_NUM_CHANNELS: u32 = 0;
pub const DEFAULT_CONVERT_TO_MONO: bool = false;
pub const DEFAULT_ZERO_PAD_ENDING: bool = false;
pub const DEFAULT_RESAMPLE_MODE: u32 = 0;

/// Use this value to resample audio with libsamplerate.
///
/// The libsamplerate resampler is not available when Babycat
/// is compiled to the `wasm32-unknown-unknown` WebAssembly target.
pub const RESAMPLE_MODE_LIBSAMPLERATE: u32 = 1;
/// Use this value to resample audio with Babycat's Lanczos resampler.
pub const RESAMPLE_MODE_BABYCAT_LANCZOS: u32 = 2;
/// Use this value to resample audio with Babycat's sinc resampler.
pub const RESAMPLE_MODE_BABYCAT_SINC: u32 = 3;

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
    /// into Babycat.
    ///
    /// Current valid values include:
    /// * [`babycat::RESAMPLE_MODE_LIBSAMPLERATE`](crate::RESAMPLE_MODE_LIBSAMPLERATE):
    ///   This uses [libsamplerate](http://www.mega-nerd.com/SRC/) at the
    ///   `SRC_SINC_BEST_QUALITY` setting. This is the highest-quality resampler
    ///   currently offered by Babycat, although it is slightly slower than the other
    ///   resamplers. This option is only available if
    ///   it is enabled at compile-time, and is disabled for targets
    ///   missing a libc, such as the WebAssembly `wasm32-unknown-unknown` target.
    ///   This backend can be enabled or disabled at compile-time using the
    ///   Cargo feature `enable-libsamplerate`. It is enabled by default on
    ///   all platforms except WebAssembly. The libsamplerate library is written
    ///   in C and its dependence on libc makes it currently not possible
    ///   to compile libsamplerate to the `wasm32-unknown-unknown` target.
    /// * [`babycat::RESAMPLE_MODE_BABYCAT_LANCZOS`](crate::RESAMPLE_MODE_BABYCAT_LANCZOS):
    ///   A Lanczos resampler to use when compiling to targets like
    ///   `wasm32-unknown-unknown` where libsamplerate cannot be compiled to.
    ///   This is a simple impmenentation of a
    ///   [Lanczos resampler](https://en.wikipedia.org/wiki/Lanczos_resampling).
    ///   This is the fastest (and lowest-quality) resampler available in Babycat.
    /// * [`babycat::RESAMPLE_MODE_BABYCAT_SINC`](crate::RESAMPLE_MODE_BABYCAT_SINC):
    ///   This is an implementation of a sinc resampler
    ///   [as described by Stanford professor Julius O. Smith](https://ccrma.stanford.edu/~jos/resample/).
    ///   The speed and quality of this resampler is in between the above two.
    #[serde(default)]
    pub resample_mode: u32,
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
        }
    }
}
