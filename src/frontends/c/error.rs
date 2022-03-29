use crate::backend::Error;

/// Returned when a given operation has succeeded without any error.
pub const NO_ERROR: u32 = 0;
/// Returned when you are trying to use a feature at runtime that was not included at compile-time.
///
/// For example, you may receive this error if you are trying to resample
/// audio using a method that was not compiled for your target or binding.
pub const ERROR_FEATURE_NOT_COMPILED: u32 = 100;
//
/// Returned when `start_time_milliseconds` or `end_time_milliseconds` is invalid.
pub const ERROR_WRONG_TIME_OFFSET: u32 = 200;
/// Returned when you wanted to decode more channels than the audio actually had.
pub const ERROR_WRONG_NUM_CHANNELS: u32 = 201;
/// Returned if you specified `convert_to_mono` as `true` and `num_channels` as 1.
pub const ERROR_WRONG_NUM_CHANNELS_AND_MONO: u32 = 202;
/// Returned if you set `zero_pad_ending` as `true` without also specifying `end_time_milliseconds`.
pub const ERROR_CANNOT_ZERO_PAD: u32 = 203;
/// Returned if you set `repeat_pad_ending` as `true` without also specifying `end_time_milliseconds`.
pub const ERROR_CANNOT_REPEAT_PAD: u32 = 204;
/// Returned if you try setting both `zero_pad_ending` and  `repeat_pad_ending` as `true`.
pub const ERROR_CANNOT_ZERO_PAD_AND_REPEAT_PAD: u32 = 205;
//
/// Returned when we were not able to detect the encoded input as decodable audio.
pub const ERROR_UNKNOWN_INPUT_ENCODING: u32 = 300;
/// Returned when we were not able to decode the given (encoded) audio.
pub const ERROR_UNKNOWN_DECODE_ERROR: u32 = 301;
/// Returned when we were not able to recognize the given decoding backend.
pub const ERROR_UNKNOWN_DECODING_BACKEND: u32 = 302;
///
pub const ERROR_NO_SUITABLE_AUDIO_STREAMS: u32 = 303;
//
/// Returned whe we encountered an unknown error when encoding a waveform into particular format.
pub const ERROR_UNKNOWN_ENCODE_ERROR: u32 = 400;
/// Returned when we were not able to resample the audio.
pub const ERROR_RESAMPLING_ERROR: u32 = 500;
/// Returned when we cannot resample from the input frame rate to the output frame rate.
pub const ERROR_WRONG_FRAME_RATE: u32 = 501;
/// Returned if you are trying to upsample or downsample audio by a factor greater than 256.
pub const ERROR_WRONG_FRAME_RATE_RATIO: u32 = 502;
//
/// Returned if you asked Babycat to read a file, but you gave it a path to a directory.
pub const ERROR_FILENAME_IS_A_DIRECTORY: u32 = 600;
/// Returned if you asked Babycat to read a file that does not exist.
pub const ERROR_FILE_NOT_FOUND: u32 = 601;
/// Returned when something else went wrong while doing I/O.
pub const ERROR_UNKNOWN_IO_ERROR: u32 = 602;

pub fn error_to_num(err: Error) -> u32 {
    match err {
        Error::FeatureNotCompiled(..) => ERROR_FEATURE_NOT_COMPILED,

        Error::WrongTimeOffset(..) => ERROR_WRONG_TIME_OFFSET,

        Error::WrongNumChannels(..) => ERROR_WRONG_NUM_CHANNELS,

        Error::WrongNumChannelsAndMono => ERROR_WRONG_NUM_CHANNELS_AND_MONO,

        Error::CannotZeroPadWithoutSpecifiedLength => ERROR_CANNOT_ZERO_PAD,

        Error::CannotRepeatPadWithoutSpecifiedLength => ERROR_CANNOT_REPEAT_PAD,

        Error::CannotSetZeroPadEndingAndRepeatPadEnding => ERROR_CANNOT_ZERO_PAD_AND_REPEAT_PAD,

        Error::UnknownDecodingBackend(..) => ERROR_UNKNOWN_DECODING_BACKEND,

        Error::NoSuitableAudioStreams(..) => ERROR_NO_SUITABLE_AUDIO_STREAMS,

        Error::UnknownInputEncoding => ERROR_UNKNOWN_INPUT_ENCODING,

        Error::UnknownDecodeError | Error::UnknownDecodeErrorWithMessage(..) => {
            ERROR_UNKNOWN_DECODE_ERROR
        }

        Error::UnknownEncodeError => ERROR_UNKNOWN_ENCODE_ERROR,

        Error::ResamplingError | Error::ResamplingErrorWithMessage(..) => ERROR_RESAMPLING_ERROR,

        Error::WrongFrameRate(..) => ERROR_WRONG_FRAME_RATE,

        Error::WrongFrameRateRatio(..) => ERROR_WRONG_FRAME_RATE_RATIO,

        Error::FilenameIsADirectory(..) => ERROR_FILENAME_IS_A_DIRECTORY,

        Error::FileNotFound(..) => ERROR_FILE_NOT_FOUND,

        Error::UnknownIOError => ERROR_UNKNOWN_IO_ERROR,
    }
}
