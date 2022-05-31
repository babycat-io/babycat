/// The default file extension hint when decoding. Hints are not necessary.
pub const DEFAULT_FILE_EXTENSION: &str = "";
/// The default MIME type hint when decoding. Hints are not necessary.
pub const DEFAULT_MIME_TYPE: &str = "";
/// The default start time cutoff when decoding audio. We start at the beginning.
pub const DEFAULT_START_TIME_MILLISECONDS: usize = 0;
/// The default end time cutoff when decoding audio. We continue decoding until the end of the file.
pub const DEFAULT_END_TIME_MILLISECONDS: usize = 0;
/// The default frame rate to resample to. By default, we do not change the frame rate's audio.
pub const DEFAULT_FRAME_RATE_HZ: u32 = 0;
/// The default number of channels to decode. By default, we decode all of the available channels.
pub const DEFAULT_NUM_CHANNELS: u16 = 0;
/// By default, we do not flatten all audio channels into a mono channel.
pub const DEFAULT_CONVERT_TO_MONO: bool = false;
/// By default, we do not zero-pad the ending of an audio file.
pub const DEFAULT_ZERO_PAD_ENDING: bool = false;
/// By default, we do not repeat-pad the ending of an audio file.
pub const DEFAULT_REPEAT_PAD_ENDING: bool = false;
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

/// Sets the decoding backend as [`SymphoniaDecoder`](crate::decoder::SymphoniaDecoder).
pub const DECODING_BACKEND_SYMPHONIA: u32 = 1;

/// Sets the decoding backend as [`FFmpegDecoder`](crate::decoder::FFmpegDecoder).
#[allow(dead_code)]
pub const DECODING_BACKEND_FFMPEG: u32 = 2;

/// The default number of threads to use for multithreaded operations.
/// By default, we will initialize as many threads as *logical*
/// CPU cores on your machine.
pub const DEFAULT_NUM_WORKERS: u32 = 0;
