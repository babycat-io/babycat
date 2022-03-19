use numpy::PyArray2;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;

use crate::backend::Waveform;

/// Uses multithreading in Rust to decode many audio files in parallel.
///
///
/// Example:
///     **(Attempt to) decode three files.**
///
///     In this example, we succesfully decode two MP3 files with
///     the default decoding arguments. Then, we demonstrate
///     how to catch an error when decoding the third file.    
///
///     >>> import babycat
///     >>> filenames = [
///     ...     "audio-for-tests/andreas-theme/track.flac",
///     ...     "audio-for-tests/blippy-trance/track.wav",
///     ...     "does-not-exist",
///     ... ]
///     >>>
///     >>> batch = babycat.batch.waveforms_from_files(filenames)
///
///     The first two files are decoded as expected, with the
///     ``exception`` field being ``None`` and the ``waveform``
///     field containing a :py:class:`Waveform`.
///
///     >>> batch[0].name
///     'audio-for-tests/andreas-theme/track.flac'
///     >>> print(batch[0].exception)
///     None
///     >>> batch[0].waveform
///     <babycat.Waveform: 9586415 frames, 2 channels, 44100 hz>
///
///     >>> batch[1].name
///     'audio-for-tests/blippy-trance/track.wav'
///     >>> print(batch[1].exception)
///     None
///     >>> batch[1].waveform
///     <babycat.Waveform: 5292911 frames, 2 channels, 44100 hz>
///
///     For the third file, the ``waveform`` field is ``None`` and the
///     ``exception`` field contains a reference to a
///     :py:class:`FileNotFoundError`. The ``name`` field helps us
///     identify which file is missing.
///
///     >>> batch[2].name
///     'does-not-exist'
///     >>> type(batch[2].exception)
///     <class 'FileNotFoundError'>
///     >>> print(batch[2].waveform)
///     None
///     >>>
///
///     .. admonition:: Remember to raise exceptions when needed
///         :class: danger
///
///         :py:meth:`~waveforms_from_files` will return
///         exceptions but **will not raise them** for you. It is your
///         responsibility to check every ``exception`` field for
///         a not-``None`` exception that you can raise or
///         intentionally ignore.
///
/// Args:
///     filenames(list[str]): A :py:class:`list` of filenames--each as
///         :py:class:`str`--to decode in parallel.
///
///     start_time_milliseconds(int, optional): We discard
///         any audio before this millisecond offset. By default, this
///         does nothing and the audio is decoded from the beginning.
///         Negative offsets are invalid.   
///
///     end_time_milliseconds(int, optional): We discard
///         any audio after this millisecond offset. By default,
///         this does nothing and the audio is decoded all the way
///         to the end. If ``start_time_milliseconds`` is specified,
///         then ``end_time_milliseconds`` must be greater. The resulting
///
///     frame_rate_hz(int, optional): A destination frame rate to resample
///         the audio to. Do not specify this parameter if you wish
///         Babycat to preserve the audio's original frame rate.
///         This does nothing if ``frame_rate_hz`` is equal to the
///         audio's original frame rate.
///
///     num_channels(int, optional): Set this to a positive integer ``n``
///         to select the *first* ``n`` channels stored in the
///         audio file. By default, Babycat will return all of the channels
///         in the original audio. This will raise an exception
///         if you specify a ``num_channels`` greater than the actual
///         number of channels in the audio.
///
///     convert_to_mono(bool, optional): Set to ``True`` to average all channels
///         into a single monophonic (mono) channel. If
///         ``num_channels = n`` is also specified, then only the
///         first ``n`` channels will be averaged. Note that
///         ``convert_to_mono`` cannot be set to ``True`` while
///         also setting ``num_channels = 1``.
///
///     zero_pad_ending(bool, optional): If you set this to ``True``,
///         Babycat will zero-pad the ending of the decoded waveform
///         to ensure that the output waveform's duration is exactly
///         ``end_time_milliseconds - start_time_milliseconds``.
///         By default, ``zero_pad_ending = False``, in which case
///         the output waveform will be shorter than
///         ``end_time_milliseconds - start_time_milliseconds``
///         if the input audio is shorter than ``end_time_milliseconds``.
///
///     resample_mode(int, optional): If you set ``frame_rate_hz``
///         to resample the audio when decoding, you can also set
///         ``resample_mode`` to pick which resampling backend to use.
///         The :py:mod:`babycat.resample_mode` submodule contains
///         the various available resampling algorithms compiled into Babycat.
///         By default, Babycat resamples audio using
///         `libsamplerate <http://www.mega-nerd.com/SRC/>`_ at its
///         highest-quality setting.
///
///     decoding_backend(int, optional): Sets the audio decoding
///         backend to use. Defaults to the Symphonia backend.
///
///     num_workers(int, optional): The number of threads--*Rust threads*, not Python
///         threads--to use for parallel decoding of the audio files in
///         ``filenames``. By default, Babycat creates the same
///         number of threads as the number of logical CPU cores on
///         your machine.
///
/// Returns:
///     list[WaveformNamedResult]: A list of objects that contain
///     a :py:class:`~babycat.Waveform` for every successful encoding
///     and a Python exception for every failed encoding. Look at
///     the ``"Raises"`` section of :py:meth:`Waveform.decode_from_filename`
///     for a list of possible exceptions that can be returned by this method.
///
#[cfg(all(feature = "enable-multithreading", feature = "enable-filesystem"))]
#[pyfunction(
    filenames,
    "*",
    start_time_milliseconds = 0,
    end_time_milliseconds = 0,
    frame_rate_hz = 0,
    num_channels = 0,
    convert_to_mono = false,
    zero_pad_ending = false,
    resample_mode = 0,
    decoding_backend = 0,
    num_workers = 0
)]
#[pyo3(text_signature = "(
    filenames,
    start_time_milliseconds = 0,
    end_time_milliseconds= 0,
    frame_rate_hz = 0,
    num_channels = 0,
    convert_to_mono = False,
    zero_pad_ending = False,
    resample_mode = 0,
    decoding_backend = 0,
    num_workers = 0,
)")]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::needless_pass_by_value)]
pub fn waveforms_from_files(
    filenames: Vec<String>,
    start_time_milliseconds: usize,
    end_time_milliseconds: usize,
    frame_rate_hz: u32,
    num_channels: u16,
    convert_to_mono: bool,
    zero_pad_ending: bool,
    resample_mode: u32,
    decoding_backend: u32,
    num_workers: usize,
) -> Vec<crate::frontends::python::waveform_named_result::WaveformNamedResult> {
    let waveform_args = crate::backend::WaveformArgs {
        start_time_milliseconds,
        end_time_milliseconds,
        frame_rate_hz,
        num_channels,
        convert_to_mono,
        zero_pad_ending,
        resample_mode,
        decoding_backend,
    };
    let batch_args = crate::backend::BatchArgs { num_workers };
    let filenames_ref: Vec<&str> = filenames.iter().map(String::as_str).collect();
    crate::backend::batch::waveforms_from_files(&filenames_ref, waveform_args, batch_args)
        .into_iter()
        .map(crate::frontends::python::waveform_named_result::WaveformNamedResult::from)
        .collect::<Vec<crate::frontends::python::waveform_named_result::WaveformNamedResult>>()
}

#[cfg(all(feature = "enable-multithreading", feature = "enable-filesystem"))]
#[pyfunction(
    filenames,
    "*",
    start_time_milliseconds = 0,
    end_time_milliseconds = 0,
    frame_rate_hz = 0,
    num_channels = 0,
    convert_to_mono = false,
    zero_pad_ending = false,
    resample_mode = 0,
    decoding_backend = 0,
    num_workers = 0
)]
#[pyo3(text_signature = "(
    filenames,
    start_time_milliseconds = 0,
    end_time_milliseconds= 0,
    frame_rate_hz = 0,
    num_channels = 0,
    convert_to_mono = False,
    zero_pad_ending = False,
    resample_mode = 0,
    decoding_backend = 0,
    num_workers = 0,
)")]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::needless_pass_by_value)]
pub fn waveforms_from_files_to_numpy_arrays(
    filenames: Vec<String>,
    start_time_milliseconds: usize,
    end_time_milliseconds: usize,
    frame_rate_hz: u32,
    num_channels: u16,
    convert_to_mono: bool,
    zero_pad_ending: bool,
    resample_mode: u32,
    decoding_backend: u32,
    num_workers: usize,
) -> Vec<crate::frontends::python::numpy_named_result::NumPyNamedResult> {
    let waveform_args = crate::backend::WaveformArgs {
        start_time_milliseconds,
        end_time_milliseconds,
        frame_rate_hz,
        num_channels,
        convert_to_mono,
        zero_pad_ending,
        resample_mode,
        decoding_backend,
    };
    let batch_args = crate::backend::BatchArgs { num_workers };
    let filenames_ref: Vec<&str> = filenames.iter().map(String::as_str).collect();
    crate::backend::batch::waveforms_from_files(&filenames_ref, waveform_args, batch_args)
        .into_iter()
        .map(crate::frontends::python::numpy_named_result::NumPyNamedResult::from)
        .collect::<Vec<crate::frontends::python::numpy_named_result::NumPyNamedResult>>()
}

#[cfg(all(feature = "enable-multithreading", feature = "enable-filesystem"))]
#[pyfunction(
    filenames,
    "*",
    start_time_milliseconds = 0,
    end_time_milliseconds = 0,
    frame_rate_hz = 0,
    num_channels = 0,
    convert_to_mono = false,
    zero_pad_ending = false,
    resample_mode = 0,
    decoding_backend = 0,
    num_workers = 0
)]
#[pyo3(text_signature = "(
    filenames,
    start_time_milliseconds = 0,
    end_time_milliseconds= 0,
    frame_rate_hz = 0,
    num_channels = 0,
    convert_to_mono = False,
    zero_pad_ending = False,
    resample_mode = 0,
    decoding_backend = 0,
    num_workers = 0,
)")]
#[allow(clippy::too_many_arguments)]
pub fn waveforms_from_files_to_numpy_arrays_unwrapped(
    filenames: Vec<String>,
    start_time_milliseconds: usize,
    end_time_milliseconds: usize,
    frame_rate_hz: u32,
    num_channels: u16,
    convert_to_mono: bool,
    zero_pad_ending: bool,
    resample_mode: u32,
    decoding_backend: u32,
    num_workers: usize,
) -> Vec<Py<PyArray2<f32>>> {
    let waveform_args = crate::backend::WaveformArgs {
        start_time_milliseconds,
        end_time_milliseconds,
        frame_rate_hz,
        num_channels,
        convert_to_mono,
        zero_pad_ending,
        resample_mode,
        decoding_backend,
    };
    let thread_pool: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_workers)
        .build()
        .unwrap();

    let waveforms: Vec<Waveform> = thread_pool.install(|| {
        filenames
            .par_iter()
            .map(|filename| Waveform::from_file(filename, waveform_args).unwrap())
            .collect()
    });
    let waveform_arrays: Vec<Py<PyArray2<f32>>> = waveforms.into_iter().map(|w| w.into()).collect();
    waveform_arrays
}

pub fn make_batch_submodule(py: Python) -> PyResult<&PyModule> {
    let batch_submodule = PyModule::new(py, "batch")?;

    batch_submodule.setattr(
        "__doc__",
        "
Functions that use multithreading to manipulate multiple audio files in parallel.
",
    )?;

    batch_submodule.add_function(wrap_pyfunction!(waveforms_from_files, batch_submodule)?)?;

    batch_submodule.add_function(wrap_pyfunction!(
        waveforms_from_files_to_numpy_arrays,
        batch_submodule
    )?)?;

    batch_submodule.add_function(wrap_pyfunction!(
        waveforms_from_files_to_numpy_arrays_unwrapped,
        batch_submodule
    )?)?;

    Ok(batch_submodule)
}
