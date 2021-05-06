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
        "Contains all built-in exceptions raised by Babycat.

        However, Babycat does raise a few exceptions that are not included
        in this module, such as like :py:class:`FileNotFoundError`.",
    )?;

    let babycat_error = py.get_type::<BabycatError>();
    babycat_error.setattr("__module__", "babycat.exceptions")?;
    babycat_error.setattr("__doc__", "Parent class for all Babycat exceptions.")?;
    exceptions_submodule.add("BabycatError", babycat_error)?;

    let feature_not_compiled = py.get_type::<FeatureNotCompiled>();
    feature_not_compiled.setattr("__module__", "babycat.exceptions")?;
    feature_not_compiled.setattr(
        "__doc__",
        "Raised when you are trying to use a feature that wasn't included at compile-time.",
    )?;
    exceptions_submodule.add("FeatureNotCompiled", feature_not_compiled)?;

    let wrong_time_offset = py.get_type::<WrongTimeOffset>();
    wrong_time_offset.setattr("__module__", "babycat.exceptions")?;
    wrong_time_offset.setattr(
        "__doc__",
        "Raised when ``start_time_milliseconds`` and/or ``end_time_milliseconds`` is invalid.
    
        For example, this exception is raised if you specify an
        ``end_time_milliseconds`` that would actually be *before*
        the ``start_time_milliseconds``.
        ",
    )?;

    exceptions_submodule.add("WrongTimeOffset", wrong_time_offset)?;

    let wrong_num_channels = py.get_type::<WrongNumChannels>();
    wrong_num_channels.setattr("__module__", "babycat.exceptions")?;
    wrong_num_channels.setattr(
        "__doc__",
        "Raised when you want more channels than the audio has.
    
        For example, if you passed ``num_channels = 3`` as a
        decoding argument, but the audio file only has two channels,
        then we'll raise this exception.",
    )?;
    exceptions_submodule.add("WrongNumChannels", wrong_num_channels)?;

    let wrong_num_channels_and_mono = py.get_type::<WrongNumChannelsAndMono>();
    wrong_num_channels_and_mono.setattr("__module__", "babycat.exceptions")?;
    wrong_num_channels_and_mono.setattr(
        "__doc__",
        "Raised when the user sets both ``convert_to_mono = True`` and ``num_channels = 1``.

        The ``num_channels`` flag is used to select the *first* channel in a
        (potentially) multi-channel audio file. the ``convert_to_mono`` flag
        takes all selected channels and flattens them into a mono channel.
        It is redundant to set ``num_channels = 1`` and also ``convert_to_mono = True``.",
    )?;
    exceptions_submodule.add("WrongNumChannelsAndMono", wrong_num_channels_and_mono)?;

    let cannot_zero_pad_without_specified_length =
        py.get_type::<CannotZeroPadWithoutSpecifiedLength>();
    cannot_zero_pad_without_specified_length.setattr("__module__", "babycat.exceptions")?;
    cannot_zero_pad_without_specified_length.setattr(
        "__doc__",
        "Raised when ``zero_pad_ending`` is set without setting ``end_time_milliseconds``.
        
        The ``zero_pad_ending`` argument is used to pad the ending of an
        audio waveform with zeros if the audio file runs out of audio
        from offsets ``start_time_milliseconds`` to ``end_time_milliseconds``.

        If you have not set an ``end_time_milliseconds``, then Babycat
        will return all of the audio from ``start_time_milliseconds``
        to the end of the audio file. In this context,
        ``zero_pad_ending = True`` is not meaningful.
        ",
    )?;
    exceptions_submodule.add(
        "CannotZeroPadWithoutSpecifiedLength",
        cannot_zero_pad_without_specified_length,
    )?;

    let unknown_input_encoding = py.get_type::<UnknownInputEncoding>();
    unknown_input_encoding.setattr("__module__", "babycat.exceptions")?;
    unknown_input_encoding.setattr(
        "__doc__",
        "Raised when we failed to detect valid audio in the input data.",
    )?;
    exceptions_submodule.add("UnknownInputEncoding", unknown_input_encoding)?;

    let unknown_decode_error = py.get_type::<UnknownDecodeError>();
    unknown_decode_error.setattr("__module__", "babycat.exceptions")?;
    unknown_decode_error.setattr(
        "__doc__",
        "Raised when we failed to decode the input audio stream, but we don't know why.",
    )?;
    exceptions_submodule.add("UnknownDecodeError", unknown_decode_error)?;

    let unknown_encode_error = py.get_type::<UnknownEncodeError>();
    unknown_encode_error.setattr("__module__", "babycat.exceptions")?;
    unknown_encode_error.setattr(
        "__doc__",
        "Raised when we failed to encode an audio stream into an output format.",
    )?;
    exceptions_submodule.add("UnknownEncodeError", unknown_encode_error)?;

    let resampling_error = py.get_type::<ResamplingError>();
    resampling_error.setattr("__module__", "babycat.exceptions")?;
    resampling_error.setattr("__doc__", "Raised when we failed to resample the waveform.")?;
    exceptions_submodule.add("ResamplingError", resampling_error)?;

    let wrong_frame_rate = py.get_type::<WrongFrameRate>();
    wrong_frame_rate.setattr("__module__", "babycat.exceptions")?;
    wrong_frame_rate.setattr(
        "__doc__",
        "Raised when the user set ``frame_rate_hz`` to a value that we cannot resample to.",
    )?;
    exceptions_submodule.add("WrongFrameRate", wrong_frame_rate)?;

    let wrong_frame_rate_ratio = py.get_type::<WrongFrameRateRatio>();
    wrong_frame_rate_ratio.setattr("__module__", "babycat.exceptions")?;
    wrong_frame_rate_ratio.setattr(
        "__doc__",
        "Raised when ``frame_rate_hz`` would upsample or downsample by a factor ``>= 256``.

        The ratio between ``frame_rate_hz`` and the audio's original frame rate
        has to be less than 256--in both cases where ``frame_rate_hz`` 
        is less than the audio's frame rate (downsampling) or greater
        than the audio's frame rate (upsampling).
        ",
    )?;
    exceptions_submodule.add("WrongFrameRateRatio", wrong_frame_rate_ratio)?;

    Ok(exceptions_submodule)
}
