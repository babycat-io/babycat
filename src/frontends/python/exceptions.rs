use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::backend::Error;

// Parent exception of all others.
create_exception!(babycat, BabycatError, PyException);
//
// Compile errors
create_exception!(babycat, FeatureNotCompiled, BabycatError);
//
// Input validation errors
create_exception!(babycat, WrongTimeOffset, BabycatError);
create_exception!(babycat, WrongNumChannels, BabycatError);
create_exception!(babycat, WrongNumChannelsAndMono, WrongNumChannels);
create_exception!(babycat, CannotZeroPadWithoutSpecifiedLength, BabycatError);
//
// Decoding errors
create_exception!(babycat, UnknownInputEncoding, BabycatError);
create_exception!(babycat, UnknownDecodeError, BabycatError);
create_exception!(babycat, UnknownDecodeErrorWithMessage, UnknownDecodeError);
//
// Encoding errors
create_exception!(babycat, UnknownEncodeError, BabycatError);
//
// Resampling errors
create_exception!(babycat, ResamplingError, BabycatError);
create_exception!(babycat, ResamplingErrorWithMessage, ResamplingError);
create_exception!(babycat, WrongFrameRate, BabycatError);
create_exception!(babycat, WrongFrameRateRatio, WrongFrameRate);

impl std::convert::From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            Error::FeatureNotCompiled(..) => FeatureNotCompiled::new_err(err.to_string()),

            Error::WrongTimeOffset(..) => WrongTimeOffset::new_err(err.to_string()),

            Error::WrongNumChannels(..) => WrongNumChannels::new_err(err.to_string()),

            Error::WrongNumChannelsAndMono => WrongNumChannelsAndMono::new_err(err.to_string()),

            Error::CannotZeroPadWithoutSpecifiedLength => {
                CannotZeroPadWithoutSpecifiedLength::new_err(err.to_string())
            }

            Error::UnknownInputEncoding => UnknownInputEncoding::new_err(err.to_string()),

            Error::UnknownDecodeError => UnknownDecodeError::new_err(err.to_string()),

            Error::UnknownDecodeErrorWithMessage(..) => {
                UnknownDecodeError::new_err(err.to_string())
            }

            Error::UnknownEncodeError => UnknownEncodeError::new_err(err.to_string()),

            Error::ResamplingError => ResamplingError::new_err(err.to_string()),

            Error::ResamplingErrorWithMessage(..) => ResamplingError::new_err(err.to_string()),

            Error::WrongFrameRate(..) => WrongFrameRate::new_err(err.to_string()),

            Error::WrongFrameRateRatio(..) => WrongFrameRateRatio::new_err(err.to_string()),

            Error::FilenameIsADirectory(..) => {
                pyo3::exceptions::PyIsADirectoryError::new_err(err.to_string())
            }

            Error::FileNotFound(..) => {
                pyo3::exceptions::PyFileNotFoundError::new_err(err.to_string())
            }

            Error::UnknownIOError => pyo3::exceptions::PyIOError::new_err(err.to_string()),
        }
    }
}

pub fn make_exceptions_submodule(py: Python) -> PyResult<&PyModule> {
    let exceptions_submodule = PyModule::new(py, "exceptions")?;

    exceptions_submodule.setattr(
        "__doc__",
        "This submodule contains all exceptions raised by babycat, excluding built-in exceptions like :py:class:`FileNotFoundError`.",
    )?;

    let babycat_error = py.get_type::<BabycatError>();
    babycat_error.setattr(
        "__doc__",
        "This exception is the parent class for all exceptions raised by
    babycat--once again, excluding built-in exceptions like :py:class:`FileNotFoundError`.",
    )?;
    exceptions_submodule.add("BabycatError", babycat_error)?;

    let feature_not_compiled = py.get_type::<FeatureNotCompiled>();
    feature_not_compiled.setattr(
        "__doc__",
        "Raised when you try to use a feature not compiled into your version of Babycat.",
    )?;
    exceptions_submodule.add("FeatureNotCompiled", babycat_error)?;

    let wrong_time_offset = py.get_type::<WrongTimeOffset>();
    wrong_time_offset.setattr(
        "__doc__",
        "Raised when the time offsets (``start_time_milliseconds`` and
    ``end_time_milliseconds`` are invalid.
    
    One case of the offsets being invalid is when the end time
    is before the start time.",
    )?;
    exceptions_submodule.add("WrongTimeOffset", wrong_time_offset)?;

    let wrong_num_channels = py.get_type::<WrongNumChannels>();
    wrong_num_channels.setattr(
        "__doc__",
        "Raised when the user has requested more channels than the audio has.
    
    For example, this exception will be raised when the user has
    set `num_channels` to 3 and the audio stream only has 2 channels.",
    )?;
    exceptions_submodule.add("WrongNumChannels", wrong_num_channels)?;

    let wrong_num_channels_and_mono = py.get_type::<WrongNumChannelsAndMono>();
    wrong_num_channels_and_mono.setattr(
        "__doc__",
        "Raised when the user sets both ``convert_to_mono=True`` and ``num_channels=``.",
    )?;
    exceptions_submodule.add("WrongNumChannelsAndMono", wrong_num_channels_and_mono)?;

    let cannot_zero_pad_without_specified_length =
        py.get_type::<CannotZeroPadWithoutSpecifiedLength>();
    cannot_zero_pad_without_specified_length.setattr(
        "__doc__",
        "Raised when ``zero_pad_ending`` is passed without also setting ``end_time_milliseconds``.",
    )?;
    exceptions_submodule.add(
        "CannotZeroPadWithoutSpecifiedLength",
        cannot_zero_pad_without_specified_length,
    )?;

    let unknown_input_encoding = py.get_type::<UnknownInputEncoding>();
    unknown_input_encoding.setattr(
        "__doc__",
        "Raised when we cannot decode the audio byte stream into one of our
    supported codecs.",
    )?;
    exceptions_submodule.add("UnknownInputEncoding", unknown_input_encoding)?;

    let unknown_decode_error = py.get_type::<UnknownDecodeError>();
    unknown_decode_error.setattr(
        "__doc__",
        "Raised when we failed to decode the input audio stream, but we don't know why.",
    )?;
    exceptions_submodule.add("UnknownDecodeError", unknown_decode_error)?;

    let unknown_encode_error = py.get_type::<UnknownEncodeError>();
    unknown_encode_error.setattr(
        "__doc__",
        "Raised when we failed to encode an audio stream.",
    )?;
    exceptions_submodule.add("UnknownEncodeError", unknown_decode_error)?;

    let resampling_error = py.get_type::<ResamplingError>();
    resampling_error.setattr(
        "__doc__",
        "Raised when the user wants to reencode the audio to a different sample rate
    and we are unable to.",
    )?;
    exceptions_submodule.add("ResamplingError", resampling_error)?;

    let wrong_frame_rate = py.get_type::<WrongFrameRate>();
    wrong_frame_rate.setattr(
        "__doc__",
        "Raised when the user wants to reencode the input audio stream to a
    new frame rate, but that frame rate is invalid.",
    )?;
    exceptions_submodule.add("WrongFrameRate", wrong_frame_rate)?;

    let wrong_frame_rate_ratio = py.get_type::<WrongFrameRateRatio>();
    wrong_frame_rate_ratio.setattr(
        "__doc__",
        "Raised when the ratio between the input and output frame rates is greater than 256.",
    )?;
    exceptions_submodule.add("WrongFrameRateRatio", wrong_frame_rate_ratio)?;

    Ok(exceptions_submodule)
}
