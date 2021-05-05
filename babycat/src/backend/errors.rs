use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Error {
    // Raised when a function is called that requires a feature that
    // was not compiled into the library.
    FeatureNotCompiled(&'static str),
    //
    // Input validation errors
    WrongTimeOffset(u64, u64),
    WrongNumChannels(u32, u32),
    WrongNumChannelsAndMono,
    CannotZeroPadWithoutSpecifiedLength,
    //
    // Decoding errors
    UnknownInputEncoding,
    UnknownDecodeError,
    UnknownDecodeErrorWithMessage(&'static str),
    //
    // Encoding errors
    UnknownEncodeError,
    //
    // Resampling errors
    ResamplingError,
    ResamplingErrorWithMessage(&'static str),
    WrongFrameRate(u32, u32),
    WrongFrameRateRatio(u32, u32),
    //
    // IO errors
    FilenameIsADirectory(&'static str),
    FileNotFound(&'static str),
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
