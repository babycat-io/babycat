use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Error {
    /// Raised when you are trying to use a feature at runtime that was not included at compile-time.
    ///
    /// For example, you may receive this error if you are trying to resample
    /// audio using a method that was not compiled for your target or binding.
    FeatureNotCompiled(&'static str),
    //
    // Input validation errors
    /// Raised when [`WaveformArgs.start_time_milliseconds`][crate::WaveformArgs#structfield.start_time_milliseconds] or [`WaveformArgs.end_time_milliseconds`][crate::WaveformArgs#structfield.end_time_milliseconds] is invalid.
    ///
    /// For example, this error is raised if you set the end timestamp to be before
    /// the start timestamp.
    WrongTimeOffset(u64, u64),
    /// Raised when you wanted to decode more channels than the audio actually had.
    WrongNumChannels(u32, u32),
    /// Raised if you specified [`WaveformArgs.convert_to_mono`][crate::WaveformArgs#structfield.convert_to_mono] as `true` and [`WaveformArgs.num_channels`][crate::WaveformArgs#structfield.num_channels] as 1.
    ///
    /// Setting both parameters is redundant and contradictory. You should either use
    /// [`WaveformArgs.convert_to_mono`][crate::WaveformArgs#structfield.convert_to_mono]
    /// to flatten all channels or [`WaveformArgs.num_channels`][crate::WaveformArgs#structfield.num_channels] = 1 to select the first channel and discard the rest.
    /// You can set [`WaveformArgs.num_channels`][crate::WaveformArgs#structfield.num_channels]
    /// `n > 1` and use  [`WaveformArgs.convert_to_mono`][crate::WaveformArgs#structfield.convert_to_mono] to only flatten those `n` channels.
    /// If you need to select channels in some other way, then do not provide either
    /// [`WaveformArgs.convert_to_mono`][crate::WaveformArgs#structfield.convert_to_mono]
    /// or [`WaveformArgs.num_channels`][crate::WaveformArgs#structfield.num_channels].
    /// All channels will be decoded and you can decide what to do with them.
    WrongNumChannelsAndMono,
    /// Raised if you set [`WaveformArgs.zero_pad_ending`][crate::WaveformArgs#structfield.zero_pad_ending] as `true` without also specifying [`WaveformArgs.end_time_milliseconds`][crate::WaveformArgs#structfield.end_time_milliseconds].
    CannotZeroPadWithoutSpecifiedLength,
    //
    // Decoding errors
    /// Raised when we do not recognize the decoding backend.
    UnknownDecodingBackend(u32),
    /// Raised when we were not able to detect the encoded input as decodable audio.
    UnknownInputEncoding,
    /// Raised when we were not able to decode the given (encoded) audio.
    UnknownDecodeError,
    /// Raised when we were not able to decode the given (encoded) audio.
    UnknownDecodeErrorWithMessage(&'static str),
    //
    // Encoding errors
    /// Raised when we encountered an unknown error when encoding a waveform into a particular format.
    UnknownEncodeError,
    //
    // Resampling errors
    /// Raised when we were not able to resample the audio.
    ResamplingError,
    // Also raised when we were not able to resample the audio.
    ResamplingErrorWithMessage(&'static str),
    /// Raised when we cannot resample from the input frame rate to the output frame rate.
    WrongFrameRate(u32, u32),
    /// Raised if you are trying upsample or downsample audio by a factor greater than 256.
    WrongFrameRateRatio(u32, u32),
    //
    // IO errors
    /// Raised if you asked Babycat to read a file but gave it a path to a directory.
    FilenameIsADirectory(&'static str),
    /// Raised if you asked Babycat to read a file that does not exist.
    FileNotFound(&'static str),
    /// Raised when something else went wrong while doing I/O.
    UnknownIOError,
}

impl std::error::Error for Error {}

impl Error {
    #[allow(dead_code)]
    pub fn error_type(&self) -> String {
        match *self {
            Error::FeatureNotCompiled(name) => format!("FeatureNotCompiledError({})", name),

            Error::WrongTimeOffset(t1, t2) => format!("WrongTimeOffset({},{})", t1, t2),

            Error::WrongNumChannels(c1, c2) => format!("WrongNumChannels({},{})", c1, c2),
            Error::WrongNumChannelsAndMono => "WrongNumChannelsAndMono".to_string(),

            Error::CannotZeroPadWithoutSpecifiedLength => {
                "CannotZeroPadWithoutSpecifiedLength".to_string()
            }

            Error::UnknownDecodingBackend(b) => format!("UnknownDecodingBackend({})", b),

            Error::UnknownInputEncoding => "UnknownInputEncoding".to_string(),

            Error::UnknownDecodeError => "UnknownDecodeError".to_string(),

            Error::UnknownDecodeErrorWithMessage(msg) => {
                format!("UnknownDecodeErrorWithMessage: {}", msg)
            }

            Error::UnknownEncodeError => "UnknownEncodeError".to_string(),

            Error::ResamplingError => "ResamplingError".to_string(),

            Error::ResamplingErrorWithMessage(msg) => {
                format!("ResamplingErrorWithMessage({})", msg)
            }

            Error::WrongFrameRate(f1, f2) => format!("WrongFrameRate({},{})", f1, f2),

            Error::WrongFrameRateRatio(f1, f2) => format!("WrongFrameRateRatio({},{})", f1, f2),

            Error::FilenameIsADirectory(dir) => format!("FilenameIsADirectory({})", dir),

            Error::FileNotFound(filename) => format!("FileNotFound({})", filename),

            Error::UnknownIOError => "UnknownIOError".to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::FeatureNotCompiled(name) => write!(f, "You are requesting a feature that was not included in the library at compile time. Feature name: {}", name),

            Error::WrongTimeOffset(t1, t2) => write!(f, "You provided invalid millisecond time offsets for trimming audio. You specified a `start_time_milliseconds` of {} ms and an `end_time_milliseconds` of {} ms. The start time needs to be a smaller number than the end time.", t1, t2),

            Error::WrongNumChannels(c1, c2) => write!(f, "Cannot change the number of audio channels. You asked to return the first {} channels, but the audio stream only has {} channels.", c1, c2),

            Error::WrongNumChannelsAndMono => write!(f, "You cannot set `convert_to_mono` as `true` and also set `num_channels` = 1. Pick one or the other. You either want `convert_to_mono` to average all channels as one channel... or you want to select the first channel with `num_channels`."),

            Error::CannotZeroPadWithoutSpecifiedLength => write!(f, "You cannot set `zero_pad_ending` without also specifying *where* the ending should be. Either set `zero_pad_ending` = `false` or specify `end_time_milliseconds` to a value above 0."),

            Error::UnknownDecodingBackend(b) => write!(f, "Could not recognize the audio decoding backend `{}`.", b),

            Error::UnknownInputEncoding => write!(f, "Unknown input encoding for audio."),

            Error::UnknownDecodeError => write!(f, "Unknown decoding error."),

            Error::UnknownDecodeErrorWithMessage(msg) => write!(f, "Unknown decoding error: {}", msg),

            Error::UnknownEncodeError => write!(f, "Unknown encoding error."),

            Error::ResamplingError => write!(f, "Unknown error when resampling to a different frame rate."),

            Error::ResamplingErrorWithMessage(msg) => write!(f, "Unknown error when resampling to a different frame rate: {}", msg),

            Error::WrongFrameRate(f1, f2) => write!(f, "Cannot resample the audio to the given frame rate. You asked to resample the audio to a frame rate of {} hz when the audio's original frame rate is {} hz.", f2,f1),
            Error::WrongFrameRateRatio(f1, f2) => write!(f, "We currently only support resampling when the ratio between input and output frame rates is 256 or less (whether upsampling or downsampling). We were given input and output frame rates of {} and {}", f1, f2),

            Error::FilenameIsADirectory(dir) => write!(
                f,
                "The filename {} is a directory. Pass an individual audo file instead.",
                dir
            ),

            Error::FileNotFound(filename) => {
                write!(f, "Cannot find the given filename {}.", filename)
            }

            Error::UnknownIOError => write!(f, "Unknown I/O error."),
        }
    }
}
