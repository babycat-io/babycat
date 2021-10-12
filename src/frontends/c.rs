use crate::backend::DecodeArgs;
use crate::backend::Error;
use crate::backend::FloatWaveform;
use std::ffi::CStr;
use std::os::raw::c_char;

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
//
/// Returned when we were not able to detect the encoded input as decodable audio.
pub const ERROR_UNKNOWN_INPUT_ENCODING: u32 = 300;
/// Returned when we were not able to decode the given (encoded) audio.
pub const ERROR_UNKNOWN_DECODE_ERROR: u32 = 301;
/// Returned when we were not able to recognize the given decoding backend.
pub const ERROR_UNKNOWN_DECODING_BACKEND: u32 = 302;
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

fn error_to_num(err: Error) -> u32 {
    match err {
        Error::FeatureNotCompiled(..) => ERROR_FEATURE_NOT_COMPILED,

        Error::WrongTimeOffset(..) => ERROR_WRONG_TIME_OFFSET,

        Error::WrongNumChannels(..) => ERROR_WRONG_NUM_CHANNELS,

        Error::WrongNumChannelsAndMono => ERROR_WRONG_NUM_CHANNELS_AND_MONO,

        Error::CannotZeroPadWithoutSpecifiedLength => ERROR_CANNOT_ZERO_PAD,

        Error::UnknownDecodingBackend(..) => ERROR_UNKNOWN_DECODING_BACKEND,

        Error::UnknownInputEncoding => ERROR_UNKNOWN_INPUT_ENCODING,

        Error::UnknownDecodeError => ERROR_UNKNOWN_DECODE_ERROR,

        Error::UnknownDecodeErrorWithMessage(..) => ERROR_UNKNOWN_DECODE_ERROR,

        Error::UnknownEncodeError => ERROR_UNKNOWN_ENCODE_ERROR,

        Error::ResamplingError => ERROR_RESAMPLING_ERROR,

        Error::ResamplingErrorWithMessage(..) => ERROR_RESAMPLING_ERROR,

        Error::WrongFrameRate(..) => ERROR_WRONG_FRAME_RATE,

        Error::WrongFrameRateRatio(..) => ERROR_WRONG_FRAME_RATE_RATIO,

        Error::FilenameIsADirectory(..) => ERROR_FILENAME_IS_A_DIRECTORY,

        Error::FileNotFound(..) => ERROR_FILE_NOT_FOUND,

        Error::UnknownIOError => ERROR_UNKNOWN_IO_ERROR,
    }
}

/// A struct that contains an error value and a pointer to a `babycat_FloatWaveform`.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct FloatWaveformResult {
    /// The error number.
    ///
    /// This value is either going to be `babycat_NO_ERROR`
    /// or one of the constants with a `babycat_ERROR` prefix.
    error_num: u32,
    /// A pointer to a FloatWaveform.
    result: *mut FloatWaveform,
}

impl From<FloatWaveform> for *mut FloatWaveform {
    fn from(item: FloatWaveform) -> Self {
        Box::into_raw(Box::new(item))
    }
}

impl From<Result<FloatWaveform, Error>> for FloatWaveformResult {
    fn from(item: Result<FloatWaveform, Error>) -> Self {
        match item {
            Ok(result) => {
                let boxed = Box::new(result);
                FloatWaveformResult {
                    error_num: 0,
                    result: Box::into_raw(boxed),
                }
            }
            Err(error) => FloatWaveformResult {
                error_num: error_to_num(error),
                result: std::ptr::null_mut::<FloatWaveform>(),
            },
        }
    }
}

/// Returns a `babycat_DecodeArgs` struct with all default values.
#[no_mangle]
pub extern "C" fn babycat_decode_args_init_default() -> DecodeArgs {
    DecodeArgs {
        ..Default::default()
    }
}

/// Frees a `babycat_FloatWaveform` struct.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_free(waveform: *mut FloatWaveform) {
    Box::from_raw(waveform);
}

/// Creates a silent waveform measured in frames.
///
/// @param frame_rate_hz The frame rate of the waveform to create.
/// @param num_channels The number of channels in the waveform to create.
/// @param num_frames The number of frames of audio to generate.
///
#[no_mangle]
pub extern "C" fn babycat_float_waveform_from_frames_of_silence(
    frame_rate_hz: u32,
    num_channels: u32,
    num_frames: u64,
) -> *mut FloatWaveform {
    FloatWaveform::from_frames_of_silence(frame_rate_hz, num_channels, num_frames).into()
}

/// Create a silent waveform measured in milliseconds.
///
/// @param frame_rate_hz The frame rate of the waveform to create.
/// @param num_channels The number of channels in the waveform to create.
/// @param duration_milliseconds The length of the audio waveform in milliseconds.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn babycat_float_waveform_from_milliseconds_of_silence(
    frame_rate_hz: u32,
    num_channels: u32,
    duration_milliseconds: u64,
) -> *mut FloatWaveform {
    FloatWaveform::from_milliseconds_of_silence(frame_rate_hz, num_channels, duration_milliseconds)
        .into()
}

/// Decodes audio in an in-memory byte array, using user-specified encoding hints.
///
/// @param encoded_bytes A byte array containing encoded (e.g. MP3) audio.
/// @param encoded_bytes_len The length of the `encoded_bytes` byte array.
/// @param decode_args Instructions on how to decode the audio.
/// @param file_extension A hint, in the form of a file extension, to indicate
///        the encoding of the audio in `encoded_bytes`.
/// @param mime_type A hint, in the form of a MIME type, to indicate
///        the encoding of the audio in `encoded_bytes`.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_from_encoded_bytes_with_hint(
    encoded_bytes: *mut u8,
    encoded_bytes_len: usize,
    decode_args: DecodeArgs,
    file_extension: *const c_char,
    mime_type: *const c_char,
) -> FloatWaveformResult {
    let encoded_bytes_vec =
        Vec::<u8>::from_raw_parts(encoded_bytes, encoded_bytes_len, encoded_bytes_len);
    let file_extension_str = CStr::from_ptr(file_extension).to_str().unwrap();
    let mime_type_str = CStr::from_ptr(mime_type).to_str().unwrap();
    FloatWaveform::from_encoded_bytes_with_hint(
        &encoded_bytes_vec,
        decode_args,
        file_extension_str,
        mime_type_str,
    )
    .into()
}

/// Decodes audio stored in an in-memory byte array.
///
/// @param encoded_bytes A byte array containing encoded (e.g. MP3) audio.
/// @param encoded_bytes_len The length of the `encoded_bytes` byte array.
/// @param decode_args Instructions on how to decode the audio.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_from_encoded_bytes(
    encoded_bytes: *mut u8,
    encoded_bytes_len: usize,
    decode_args: DecodeArgs,
) -> FloatWaveformResult {
    let encoded_bytes_vec =
        Vec::<u8>::from_raw_parts(encoded_bytes, encoded_bytes_len, encoded_bytes_len);
    FloatWaveform::from_encoded_bytes(&encoded_bytes_vec, decode_args).into()
}

/// Decodes audio stored in a local file.
///
/// @param filename A filename of an encoded audio file on the local filesystem.
/// @param decode_args Instructions on how to decode the audio.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_from_file(
    filename: *const c_char,
    decode_args: DecodeArgs,
) -> FloatWaveformResult {
    let filename_rust = CStr::from_ptr(filename).to_str().unwrap();
    FloatWaveform::from_file(filename_rust, decode_args).into()
}

/// Returns the frame rate of an existing `babycat_FloatWaveform`.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_get_frame_rate_hz(
    waveform: *mut FloatWaveform,
) -> u32 {
    (*(waveform)).frame_rate_hz()
}

/// Returns the number of channels of an existing `babycat_FloatWaveform`.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_get_num_channels(
    waveform: *mut FloatWaveform,
) -> u32 {
    (*(waveform)).num_channels()
}

/// Returns the number of frames in an existing `babycat_FloatWaveform`.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_get_num_frames(
    waveform: *mut FloatWaveform,
) -> u64 {
    (*(waveform)).num_frames()
}

/// Returns the number of samples in an existing `babycat_FloatWaveform`.
///
/// @param waveform A pointer to the `babycat_FloatWaveform`.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_get_num_samples(
    waveform: *mut FloatWaveform,
) -> u64 {
    let w = &*(waveform);
    w.num_frames() * w.num_channels() as u64
}

/// Returns a pointer to an in-memory array of interleaved audio samples.
///
/// @param waveform A pointer to the `babycat_FloatWaveform`.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_to_interleaved_samples(
    waveform: *mut FloatWaveform,
) -> *const f32 {
    waveform.as_ref().unwrap().to_interleaved_samples().as_ptr()
}

/// Resample a `babycat_FloatWaveform` with the default resampler.
///
/// @param waveform A pointer to the `babycat_FloatWaveform` to resample.
/// @param frame_rate_hz The destination frame rate to resample to.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_resample(
    waveform: *mut FloatWaveform,
    frame_rate_hz: u32,
) -> FloatWaveformResult {
    (*(waveform)).resample(frame_rate_hz).into()
}

/// Resamples a `babycat_FloatWaveform` using a specific resampler.
///
/// @param waveform A pointer to the `babycat_FloatWaveform` to resample.
/// @param frame_rate_hz The destination frame rate to resample to.
/// @param resample_mode The Babycat resampling backend to pick.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_resample_by_mode(
    waveform: *mut FloatWaveform,
    frame_rate_hz: u32,
    resample_mode: u32,
) -> FloatWaveformResult {
    (*(waveform))
        .resample_by_mode(frame_rate_hz, resample_mode)
        .into()
}
