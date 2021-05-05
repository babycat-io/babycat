use std::ffi::CStr;
use std::os::raw::c_char;
use crate::backend::Error;
use crate::backend::DecodeArgs;
use crate::backend::FloatWaveform;
use crate::backend::Waveform;

pub const BABYCAT_NO_ERROR: u32 = 0;
pub const BABYCAT_ERROR_FEATURE_NOT_COMPILED: u32 = 100;
//
//
pub const BABYCAT_ERROR_WRONG_TIME_OFFSET: u32 = 200;
pub const BABYCAT_ERROR_WRONG_NUM_CHANNELS: u32 = 201;
pub const BABYCAT_ERROR_WRONG_NUM_CHANNELS_AND_MONO: u32 = 202;
pub const BABYCAT_ERROR_CANNOT_ZERO_PAD: u32 = 203;
//
//
pub const BABYCAT_ERROR_UNKNOWN_INPUT_ENCODING: u32 = 300;
pub const BABYCAT_ERROR_UNKNOWN_DECODE_ERROR: u32 = 301;
//
//
pub const BABYCAT_ERROR_UNKNOWN_ENCODE_ERROR: u32 = 400;
pub const BABYCAT_ERROR_RESAMPLING_ERROR: u32 = 500;
pub const BABYCAT_ERROR_WRONG_FRAME_RATE: u32 = 501;
pub const BABYCAT_ERROR_WRONG_FRAME_RATE_RATIO: u32 = 502;
//
//
pub const BABYCAT_ERROR_FILENAME_IS_A_DIRECTORY: u32 = 600;
pub const BABYCAT_ERROR_FILE_NOT_FOUND: u32 = 601;
pub const BABYCAT_ERROR_UNKNOWN_IO_ERROR: u32 = 602;

fn error_to_num(err: Error) -> u32 {
    match err {
        Error::FeatureNotCompiled(..) => BABYCAT_ERROR_FEATURE_NOT_COMPILED,

        Error::WrongTimeOffset(..) => BABYCAT_ERROR_WRONG_TIME_OFFSET,

        Error::WrongNumChannels(..) => BABYCAT_ERROR_WRONG_NUM_CHANNELS,

        Error::WrongNumChannelsAndMono => BABYCAT_ERROR_WRONG_NUM_CHANNELS_AND_MONO,

        Error::CannotZeroPadWithoutSpecifiedLength => BABYCAT_ERROR_CANNOT_ZERO_PAD,

        Error::UnknownInputEncoding => BABYCAT_ERROR_UNKNOWN_INPUT_ENCODING,

        Error::UnknownDecodeError => BABYCAT_ERROR_UNKNOWN_DECODE_ERROR,

        Error::UnknownDecodeErrorWithMessage(..) => BABYCAT_ERROR_UNKNOWN_DECODE_ERROR,

        Error::UnknownEncodeError => BABYCAT_ERROR_UNKNOWN_ENCODE_ERROR,

        Error::ResamplingError => BABYCAT_ERROR_RESAMPLING_ERROR,

        Error::ResamplingErrorWithMessage(..) => BABYCAT_ERROR_RESAMPLING_ERROR,

        Error::WrongFrameRate(..) => BABYCAT_ERROR_WRONG_FRAME_RATE,

        Error::WrongFrameRateRatio(..) => BABYCAT_ERROR_WRONG_FRAME_RATE_RATIO,

        Error::FilenameIsADirectory(..) => BABYCAT_ERROR_FILENAME_IS_A_DIRECTORY,

        Error::FileNotFound(..) => BABYCAT_ERROR_FILE_NOT_FOUND,

        Error::UnknownIOError => BABYCAT_ERROR_UNKNOWN_IO_ERROR,
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct FloatWaveformResult {
    error_num: u32,
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
                    result: Box::into_raw(boxed)
                }
            }
            Err(error) => {
                FloatWaveformResult {
                    error_num: error_to_num(error),
                    result: std::ptr::null_mut::<FloatWaveform>()
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct NamedFloatWaveformResult {
    error_num: u32,
    name: *const c_char,
    result: *mut FloatWaveform
}


#[no_mangle]
pub extern "C" fn babycat_float_waveform_from_frames_of_silence(
    frame_rate_hz: u32,
    num_channels: u32,
    num_frames: u64,
) -> *mut FloatWaveform {
    FloatWaveform::from_frames_of_silence(
        frame_rate_hz,
        num_channels,
        num_frames
    ).into()
}

#[no_mangle]
pub extern "C" fn babycat_float_waveform_from_milliseconds_of_silence(
    frame_rate_hz: u32,
    num_channels: u32,
    duration_milliseconds: u64,
) -> *mut FloatWaveform {
    FloatWaveform::from_milliseconds_of_silence(
        frame_rate_hz,
        num_channels,
        duration_milliseconds
    ).into()
}

#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_from_file(
    filename: *const c_char,
    decode_args: DecodeArgs,
) -> FloatWaveformResult {
    let filename_rust = CStr::from_ptr(filename).to_str().unwrap();
    FloatWaveform::from_file(
        filename_rust,
        decode_args
    ).into()
}

#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_get_frame_rate_hz(waveform: *mut FloatWaveform) -> u32 {
    Box::from_raw(waveform).frame_rate_hz()
}

#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_get_num_channels(
    waveform: *mut FloatWaveform
) -> u32 {
    Box::from_raw(waveform).num_channels()
}

#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_get_num_frames(
    waveform: *mut FloatWaveform
) -> u64 {
    Box::from_raw(waveform).num_frames()
}

#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_get_num_samples(
    waveform: *mut FloatWaveform
) -> u64 {
    let unboxed_waveform = Box::from_raw(waveform);
    unboxed_waveform.num_frames() * unboxed_waveform.num_channels() as u64
}

#[no_mangle]
pub unsafe extern "C" fn babycat_float_waveform_get_interleaved_samples(waveform: *mut FloatWaveform) -> *mut f32 {
    Box::from_raw(waveform).interleaved_samples().to_owned().as_mut_ptr()
}