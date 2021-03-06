use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;

use crate::backend::Signal;

pub type PyArraySamples = Py<PyArray2<f32>>;

/// An in-memory audio waveform.
#[pyclass(module = "babycat")]
#[derive(Clone, Debug)]
pub struct Waveform {
    inner: crate::backend::Waveform,
}

impl From<crate::backend::Waveform> for Waveform {
    fn from(inner: crate::backend::Waveform) -> Waveform {
        Waveform { inner }
    }
}

///
/// # Panics
/// This function panics if we cannot create a NumPy array of shape
/// `.(num_frames, num_channels)`.
pub fn interleaved_samples_to_pyarray(
    py: Python<'_>,
    num_channels: u16,
    num_frames: usize,
    interleaved_samples: Vec<f32>,
) -> PyArraySamples {
    interleaved_samples
        .into_pyarray(py)
        .reshape([num_frames, num_channels as usize])
        .unwrap()
        .into()
}

impl IntoPy<PyArraySamples> for crate::backend::Waveform {
    fn into_py(self, py: Python<'_>) -> PyArraySamples {
        let num_channels = self.num_channels();
        let num_frames = self.num_frames();
        let interleaved_samples: Vec<f32> = self.into();
        interleaved_samples_to_pyarray(py, num_channels, num_frames, interleaved_samples)
    }
}

impl std::fmt::Display for Waveform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<babycat.Waveform: {} frames, {} channels, {} hz>",
            self.inner.num_frames(),
            self.inner.num_channels(),
            self.inner.frame_rate_hz(),
        )
    }
}

#[pymethods]
impl Waveform {
    /// Creates a silent waveform of ``num_frames`` frames.
    ///
    /// Example:
    ///     **Creating 1000 frames of silence (in stereo).**
    ///
    ///     >>> from babycat import Waveform
    ///     >>> Waveform.from_frames_of_silence(
    ///     ...    frame_rate_hz=44100,
    ///     ...    num_channels=2,
    ///     ...    num_frames=1000,
    ///     ... )
    ///     <babycat.Waveform: 1000 frames, 2 channels, 44100 hz>
    ///
    /// Args:
    ///     frame_rate_hz(int): The frame rate to set for this silent audio
    ///         waveform.
    ///
    ///     num_channels(int): The number of channels to set.
    ///
    ///     num_frames(int): The number of frames to set.
    ///
    /// Returns:
    ///     Waveform: A waveform representing silence.
    ///
    #[staticmethod]
    #[args("*", frame_rate_hz, num_channels, num_frames)]
    #[pyo3(text_signature = "(
        frame_rate_hz,
        num_channels,
        num_frames,
    )")]
    pub fn from_frames_of_silence(
        py: Python<'_>,
        frame_rate_hz: u32,
        num_channels: u16,
        num_frames: usize,
    ) -> Self {
        py.allow_threads(move || {
            crate::backend::Waveform::from_frames_of_silence(
                frame_rate_hz,
                num_channels,
                num_frames,
            )
        })
        .into()
    }

    /// Creates a silent waveform measured in milliseconds.
    ///
    /// Example:
    ///     **Create 30 seconds of silence (in stereo).**
    ///
    ///     >>> from babycat import Waveform
    ///     >>> Waveform.from_milliseconds_of_silence(
    ///     ...    frame_rate_hz=44100,
    ///     ...    num_channels=2,
    ///     ...    duration_milliseconds=30_000,
    ///     ... )
    ///     <babycat.Waveform: 1323000 frames, 2 channels, 44100 hz>
    ///
    /// Args:
    ///     frame_rate_hz(int): The frame rate to set for this silent audio
    ///         waveform.
    ///
    ///     num_channels(int): The number of channels to set.
    ///
    ///     num_frames(int): The number of frames to set.
    ///
    /// Returns:
    ///     Waveform: A waveform representing silence.
    ///
    #[staticmethod]
    #[args("*", frame_rate_hz, num_channels, duration_milliseconds)]
    #[pyo3(text_signature = "(
        frame_rate_hz,
        num_channels,
        duration_milliseconds,
    )")]
    pub fn from_milliseconds_of_silence(
        py: Python<'_>,
        frame_rate_hz: u32,
        num_channels: u16,
        duration_milliseconds: usize,
    ) -> Self {
        py.allow_threads(move || {
            crate::backend::Waveform::from_milliseconds_of_silence(
                frame_rate_hz,
                num_channels,
                duration_milliseconds,
            )
        })
        .into()
    }

    /// Creates a :py:class:`Waveform` from interleaved audio samples.
    ///
    /// Example:
    ///     >>> from babycat import Waveform
    ///     >>> interleaved_samples = [-1.0, 0.0, 1.0, -1.0, 0.0, 1.0]
    ///     >>> waveform = Waveform.from_interleaved_samples(
    ///     ...     frame_rate_hz=44_100,
    ///     ...     num_channels=3,
    ///     ...     interleaved_samples=interleaved_samples,
    ///     ... )
    ///     >>> waveform
    ///     <babycat.Waveform: 2 frames, 3 channels, 44100 hz>
    ///
    /// Args:
    ///     frame_rate_hz(int): The frame rate that applies to the waveform
    ///         described by ``interleaved_samples``.
    ///
    ///     num_channels(int): The number of channels in the waveform
    ///         described by ``interleaved_samples``.
    ///
    ///     interleaved_samples(list): A one-dimensional Python list of
    ///         interleaved :py:class:`float` audio samples.
    ///
    /// Returns:
    ///     Waveform: A waveform representing ``interleaved_samples``.
    ///
    #[staticmethod]
    #[args("*", frame_rate_hz, num_channels, interleaved_samples)]
    #[pyo3(text_signature = "(
        frame_rate_hz,
        num_channels,
        interleaved_samples,
    )")]
    #[allow(clippy::too_many_arguments)]
    pub fn from_interleaved_samples(
        py: Python<'_>,
        frame_rate_hz: u32,
        num_channels: u16,
        interleaved_samples: Vec<f32>,
    ) -> Self {
        py.allow_threads(move || {
            crate::backend::Waveform::new(frame_rate_hz, num_channels, interleaved_samples)
        })
        .into()
    }

    /// Creates a :py:class:`Waveform` from a two-dimensional NumPy ``float32`` array.
    ///
    /// This static method takes a two-dimensional NumPy array of the
    /// shape ``(frames, channels)``.
    ///
    /// Example:
    ///     >>> import numpy as np
    ///     >>> from babycat import Waveform
    ///     >>> frame = np.array([-1.0, 0.0, 1.0], dtype="float32")
    ///     >>> arr = np.stack([frame, frame])
    ///     >>> waveform = Waveform.from_numpy(
    ///     ...     frame_rate_hz=44_100,
    ///     ...     arr=arr,
    ///     ... )
    ///     waveform
    ///     <babycat.Waveform: 2 frames, 3 channel, 44100 hz>
    ///
    /// Args:
    ///     frame_rate_hz(int): The frame rate that applies to the waveform
    ///         described by ``arr``.
    ///
    ///     arr: A two-dimensional NumPy array with the channels dimension
    ///         on axis 1.
    ///
    /// Returns:
    ///     Waveform: A waveform with a copy of the waveform in ``arr``.
    ///
    /// Raises:
    ///     TypeError: Raised when ``arr`` is the wrong shape or dtype.
    ///
    #[staticmethod]
    #[args("*", frame_rate_hz, arr)]
    #[pyo3(text_signature = "(
        frame_rate_hz,
        arr,
    )")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_numpy(
        py: Python<'_>,
        frame_rate_hz: u32,
        arr: PyReadonlyArray2<f32>,
    ) -> PyResult<Self> {
        #[allow(clippy::cast_possible_truncation)]
        let num_channels: u16 = arr.shape()[1] as u16;
        let interleaved_samples: Vec<f32> = arr.to_vec().unwrap();
        let waveform = py.allow_threads(move || {
            crate::backend::Waveform::new(frame_rate_hz, num_channels, interleaved_samples)
        });
        Ok(waveform.into())
    }

    /// Decodes audio stored as ``bytes``.
    ///
    /// Example:
    ///     **Decode from bytes while auto-detecting the format as MP3.**
    ///
    ///     >>> from babycat import Waveform
    ///     >>> with open("audio-for-tests/andreas-theme/track.flac", "rb") as fh:
    ///     ...     the_bytes = fh.read()
    ///     >>> waveform = Waveform.from_encoded_bytes(the_bytes)
    ///     >>> waveform
    ///     <babycat.Waveform: 9586415 frames, 2 channels, 44100 hz>
    ///
    /// Example:
    ///     **Decode from bytes with a file extension hint.**
    ///
    ///     >>> waveform2 = Waveform.from_encoded_bytes(
    ///     ...     the_bytes,
    ///     ...     file_extension="mp3",
    ///     ... )
    ///
    /// Args:
    ///     encoded_bytes(bytes): A :py:class:`bytes` object
    ///         containing an *encoded* audio file, such as MP3 file.
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
    ///         Note that setting ``zero_pad_ending = True`` is
    ///         mutually exclusive with setting ``repeat_pad_ending = True``.
    ///
    ///     repeat_pad_ending(bool, optional): If you set this to ``True``,
    ///         Babycat will repeat the audio waveform to ensure that
    ///         the output waveform's duration is exactly
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         By default, ``repeat_pad_ending = False``, in which case
    ///         the output waveform will be shorter than
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         Note that setting ``repeat_pad_ending = True`` is
    ///         mutually exclusive with setting ``zero_pad_ending = True``.
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
    ///     file_extension(str, optional): An *optional hint* of the input audio file's
    ///         encoding. An example of a valid value is ``"mp3"``. Babycat
    ///         will automatically detect the correct encoding of ``input_audio``,
    ///         even if ``file_extension`` is an incorrect guess.
    ///
    ///     mime_type(str, optional): An *optional hint* of the input audio file's
    ///         encoding. An example of a valid value is ``"audio/mpeg"``. Babycat
    ///         will automatically detect the correct encoding of ``input_audio``,
    ///         even if ``mime_type`` is an incorrect guess.
    ///
    /// Returns:
    ///     Waveform: Returns a waveform decoded from ``encoded_bytes``.
    ///
    /// Raises:
    ///     babycat.exceptions.FeatureNotCompiled: Raised when you are trying
    ///         to use a feature at runtime that as not included in Babycat
    ///         at compile-time.
    ///
    ///     babycat.exceptions.WrongTimeOffset: Raised when
    ///         ``start_time_milliseconds``and/or ``end_time_milliseconds``
    ///         is invalid.
    ///
    ///     babycat.exceptions.WrongNumChannels: Raised when you specified
    ///         a value for ``num_channels`` that is greater than the
    ///         number of channels the audio has.
    ///
    ///     babycat.exceptions.WrongNumChannelsAndMono: Raised when the
    ///         user sets both ``convert_to_mono = True`` and
    ///         ``num_channels = 1``.
    ///
    ///     babycat.exceptions.CannotZeroPadWithoutSpecifiedLength: Raised
    ///         when ``zero_pad_ending`` is set without setting
    ///         ``end_time_milliseconds``.
    ///
    ///     babycat.exceptions.UnknownInputEncoding: Raised when we
    ///         failed to detect valid audio in the input data.
    ///
    ///     babycat.exceptions.UnknownDecodeError: Raised when we
    ///         failed to decode the input audio stream, but
    ///         we don't know why.
    ///
    ///     babycat.exceptions.ResamplingError: Raised when we
    ///         failed to encode an audio stream into an output format.
    ///
    ///     babycat.exceptions.WrongFrameRate: Raised when the
    ///         user set ``frame_rate_hz`` to a value that we
    ///         cannot resample to.
    ///
    ///     babycat.exceptions.WrongFrameRateRatio: Raised
    ///         when ``frame_rate_hz`` would upsample or
    ///         downsample by a factor ``>= 256``. Try resampling in
    ///         smaller increments.
    ///
    #[staticmethod]
    #[args(
        encoded_bytes,
        "*",
        start_time_milliseconds = 0,
        end_time_milliseconds = 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = false,
        zero_pad_ending = false,
        repeat_pad_ending = false,
        resample_mode = 0,
        decoding_backend = 0,
        file_extension = "\"\"",
        mime_type = "\"\""
    )]
    #[pyo3(text_signature = "(
        encoded_bytes,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        repeat_pad_ending = False,
        resample_mode = 0,
        decoding_backend = 0,
        file_extension = \"\",
        mime_type = \"\",
    )")]
    #[allow(clippy::too_many_arguments)]
    pub fn from_encoded_bytes(
        py: Python<'_>,
        encoded_bytes: &[u8],
        start_time_milliseconds: usize,
        end_time_milliseconds: usize,
        frame_rate_hz: u32,
        num_channels: u16,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        repeat_pad_ending: bool,
        resample_mode: u32,
        decoding_backend: u32,
        file_extension: &str,
        mime_type: &str,
    ) -> PyResult<Self> {
        let wr = py.allow_threads(move || {
            let waveform_args = crate::backend::WaveformArgs {
                start_time_milliseconds,
                end_time_milliseconds,
                frame_rate_hz,
                num_channels,
                convert_to_mono,
                zero_pad_ending,
                repeat_pad_ending,
                resample_mode,
                decoding_backend,
            };
            crate::backend::Waveform::from_encoded_bytes_with_hint(
                encoded_bytes,
                waveform_args,
                file_extension,
                mime_type,
            )
        });
        let waveform = wr?;
        Ok(waveform.into())
    }

    /// Decodes audio stored as ``bytes``, directly returning a NumPy array.
    ///
    /// This method is just like :py:meth:`from_encoded_bytes`, but it
    /// returns a NumPy array of shape ``(frames, channels)`` instead of
    /// a :py:class:`Waveform` object.
    ///
    /// See the documentation for :py:meth:`from_encoded_bytes`
    /// for a complete list of raised exceptions.
    ///
    /// Args:
    ///     encoded_bytes(bytes): A :py:class:`bytes` object
    ///         containing an *encoded* audio file, such as MP3 file.
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
    ///         Note that setting ``zero_pad_ending = True`` is
    ///         mutually exclusive with setting ``repeat_pad_ending = True``.
    ///
    ///     repeat_pad_ending(bool, optional): If you set this to ``True``,
    ///         Babycat will repeat the audio waveform to ensure that
    ///         the output waveform's duration is exactly
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         By default, ``repeat_pad_ending = False``, in which case
    ///         the output waveform will be shorter than
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         Note that setting ``repeat_pad_ending = True`` is
    ///         mutually exclusive with setting ``zero_pad_ending = True``.
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
    /// Returns:
    ///     numpy.ndarray: A NumPy array of shape ``(frames, channels)``
    ///     of the decoded audio waveform.
    ///
    #[staticmethod]
    #[args(
        encoded_bytes,
        "*",
        start_time_milliseconds = 0,
        end_time_milliseconds = 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = false,
        zero_pad_ending = false,
        repeat_pad_ending = false,
        resample_mode = 0,
        decoding_backend = 0,
        file_extension = "\"\"",
        mime_type = "\"\""
    )]
    #[pyo3(text_signature = "(
        encoded_bytes,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        repeat_pad_ending = False,
        resample_mode = 0,
        decoding_backend = 0,
        file_extension = \"\",
        mime_type = \"\",
    )")]
    #[allow(clippy::too_many_arguments)]
    pub fn from_encoded_bytes_into_numpy(
        py: Python<'_>,
        encoded_bytes: &[u8],
        start_time_milliseconds: usize,
        end_time_milliseconds: usize,
        frame_rate_hz: u32,
        num_channels: u16,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        repeat_pad_ending: bool,
        resample_mode: u32,
        decoding_backend: u32,
        file_extension: &str,
        mime_type: &str,
    ) -> PyResult<PyArraySamples> {
        let wr = py.allow_threads(move || {
            let waveform_args = crate::backend::WaveformArgs {
                start_time_milliseconds,
                end_time_milliseconds,
                frame_rate_hz,
                num_channels,
                convert_to_mono,
                zero_pad_ending,
                repeat_pad_ending,
                resample_mode,
                decoding_backend,
            };
            crate::backend::Waveform::from_encoded_bytes_with_hint(
                encoded_bytes,
                waveform_args,
                file_extension,
                mime_type,
            )
        });
        let waveform = wr?;
        Ok(waveform.into_py(py))
    }

    /// Decodes audio stored in a local file.
    ///
    /// Example:
    ///     **Decode an entire audio file with default arguments.**
    ///
    ///     >>> from babycat import Waveform
    ///     >>> waveform = Waveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.flac",
    ///     ... )
    ///     >>> waveform
    ///     <babycat.Waveform: 9586415 frames, 2 channels, 44100 hz>
    ///     >>> waveform.num_frames
    ///     9586415
    ///     >>> waveform.num_channels
    ///     2
    ///     >>> waveform.frame_rate_hz
    ///     44100
    ///     >>> waveform.to_numpy().shape
    ///     (9586415, 2)
    ///
    /// Example:
    ///     **Decode the first 30 seconds of the audio file.**
    ///
    ///     >>> waveform = Waveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.flac",
    ///     ...     end_time_milliseconds=30_000,
    ///     ... )
    ///     >>> waveform
    ///     <babycat.Waveform: 1323000 frames, 2 channels, 44100 hz>
    ///
    /// Example:
    ///     **Decode the entire audio file and resampling up to 48,000hz.**
    ///
    ///     >>> waveform = Waveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.flac",
    ///     ...     frame_rate_hz=48000,
    ///     ... )
    ///     >>> waveform
    ///     <babycat.Waveform: 10434194 frames, 2 channels, 48000 hz>
    ///
    /// Example:
    ///     **Decode the first 30 seconds and resample up to 48,000hz.**
    ///
    ///     >>> waveform = Waveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.flac",
    ///     ...     end_time_milliseconds=30_000,
    ///     ...     frame_rate_hz=48000,
    ///     ... )
    ///     >>> waveform
    ///     <babycat.Waveform: 1440000 frames, 2 channels, 48000 hz>
    ///
    /// Args:
    ///     filename(str): The path to an audio file on the local
    ///         filesystem.
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
    ///         Note that setting ``zero_pad_ending = True`` is
    ///         mutually exclusive with setting ``repeat_pad_ending = True``.
    ///
    ///     repeat_pad_ending(bool, optional): If you set this to ``True``,
    ///         Babycat will repeat the audio waveform to ensure that
    ///         the output waveform's duration is exactly
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         By default, ``repeat_pad_ending = False``, in which case
    ///         the output waveform will be shorter than
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         Note that setting ``repeat_pad_ending = True`` is
    ///         mutually exclusive with setting ``zero_pad_ending = True``.
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
    /// Returns:
    ///     Waveform: A waveform decoded from ``filename``.
    ///
    /// Raises:
    ///     FileNotFoundError: Raised when we cannot find
    ///         ``filename`` on the local filesystem.
    ///
    ///     IsADirectoryError: Raised when ``filename``
    ///         resolves to a directory on the local
    ///         instead of a file.
    ///
    ///     babycat.exceptions.FeatureNotCompiled: Raised when you are trying
    ///         to use a feature at runtime that as not included in Babycat
    ///         at compile-time.
    ///
    ///     babycat.exceptions.WrongTimeOffset: Raised when
    ///         ``start_time_milliseconds``and/or ``end_time_milliseconds``
    ///         is invalid.
    ///
    ///     babycat.exceptions.WrongNumChannels: Raised when you specified
    ///         a value for ``num_channels`` that is greater than the
    ///         number of channels the audio has.
    ///
    ///     babycat.exceptions.WrongNumChannelsAndMono: Raised when the
    ///         user sets both ``convert_to_mono = True`` and
    ///         ``num_channels = 1``.
    ///
    ///     babycat.exceptions.CannotZeroPadWithoutSpecifiedLength: Raised
    ///         when ``zero_pad_ending`` is set without setting
    ///         ``end_time_milliseconds``.
    ///
    ///     babycat.exceptions.UnknownInputEncoding: Raised when we
    ///         failed to detect valid audio in the input data.
    ///
    ///     babycat.exceptions.UnknownDecodeError: Raised when we
    ///         failed to decode the input audio stream, but
    ///         we don't know why.
    ///
    ///     babycat.exceptions.ResamplingError: Raised when we
    ///         failed to encode an audio stream into an output format.
    ///
    ///     babycat.exceptions.WrongFrameRate: Raised when the
    ///         user set ``frame_rate_hz`` to a value that we
    ///         cannot resample to.
    ///
    ///     babycat.exceptions.WrongFrameRateRatio: Raised
    ///         when ``frame_rate_hz`` would upsample or
    ///         downsample by a factor ``>= 256``. Try resampling in
    ///         smaller increments.
    ///
    #[cfg(feature = "enable-filesystem")]
    #[staticmethod]
    #[args(
        filename,
        "*",
        start_time_milliseconds = 0,
        end_time_milliseconds = 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = false,
        zero_pad_ending = false,
        repeat_pad_ending = false,
        resample_mode = 0,
        decoding_backend = 0
    )]
    #[pyo3(text_signature = "(
        filename,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        repeat_pad_ending = False,
        resample_mode = 0,
        decoding_backend = 0,
    )")]
    #[allow(clippy::too_many_arguments)]
    pub fn from_file(
        py: Python<'_>,
        filename: &str,
        start_time_milliseconds: usize,
        end_time_milliseconds: usize,
        frame_rate_hz: u32,
        num_channels: u16,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        repeat_pad_ending: bool,
        resample_mode: u32,
        decoding_backend: u32,
    ) -> PyResult<Self> {
        let wr = py.allow_threads(move || {
            let waveform_args = crate::backend::WaveformArgs {
                start_time_milliseconds,
                end_time_milliseconds,
                frame_rate_hz,
                num_channels,
                convert_to_mono,
                zero_pad_ending,
                repeat_pad_ending,
                resample_mode,
                decoding_backend,
            };
            crate::backend::Waveform::from_file(filename, waveform_args)
        });
        let waveform = wr?;
        Ok(waveform.into())
    }

    /// Decodes audio stored in a local file, directly returning a NumPy array.
    ///
    /// This method is just like :py:meth:`from_file`, but it
    /// returns a NumPy array of shape ``(frames, channels)`` instead of
    /// a :py:class:`Waveform` object.
    ///
    ///
    /// See the documentation for :py:meth:`from_file`
    /// for a complete list of raised exceptions.
    ///
    /// Args:
    ///     filename(str): The path to an audio file on the local
    ///         filesystem.
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
    ///         Note that setting ``zero_pad_ending = True`` is
    ///         mutually exclusive with setting ``repeat_pad_ending = True``.
    ///
    ///     repeat_pad_ending(bool, optional): If you set this to ``True``,
    ///         Babycat will repeat the audio waveform to ensure that
    ///         the output waveform's duration is exactly
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         By default, ``repeat_pad_ending = False``, in which case
    ///         the output waveform will be shorter than
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         Note that setting ``repeat_pad_ending = True`` is
    ///         mutually exclusive with setting ``zero_pad_ending = True``.
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
    /// Returns:
    ///     numpy.ndarray: A NumPy array of shape ``(frames, channels)``
    ///     of the decoded audio waveform.
    ///
    #[cfg(feature = "enable-filesystem")]
    #[staticmethod]
    #[args(
        filename,
        "*",
        start_time_milliseconds = 0,
        end_time_milliseconds = 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = false,
        zero_pad_ending = false,
        repeat_pad_ending = false,
        resample_mode = 0,
        decoding_backend = 0
    )]
    #[pyo3(text_signature = "(
        filename,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        repeat_pad_ending = False,
        resample_mode = 0,
        decoding_backend = 0,
    )")]
    #[allow(clippy::too_many_arguments)]
    pub fn from_file_into_numpy(
        py: Python<'_>,
        filename: &str,
        start_time_milliseconds: usize,
        end_time_milliseconds: usize,
        frame_rate_hz: u32,
        num_channels: u16,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        repeat_pad_ending: bool,
        resample_mode: u32,
        decoding_backend: u32,
    ) -> PyResult<PyArraySamples> {
        let wr = py.allow_threads(move || {
            let waveform_args = crate::backend::WaveformArgs {
                start_time_milliseconds,
                end_time_milliseconds,
                frame_rate_hz,
                num_channels,
                convert_to_mono,
                zero_pad_ending,
                repeat_pad_ending,
                resample_mode,
                decoding_backend,
            };
            crate::backend::Waveform::from_file(filename, waveform_args)
        });
        let waveform = wr?;
        Ok(waveform.into_py(py))
    }

    /// Returns the decoded waveform's frame rate in hertz.
    ///
    /// If you did not set ``frame_rate_hz`` as an argument during decoding,
    /// then this value will be the frame rate that Babycat detected from
    /// the audio.
    ///
    /// If you *did* set ``frame_rate_hz`` during decoding, then this value
    /// will be the value you set.
    ///
    /// Returns:
    ///     int: The frame rate.
    ///
    #[getter]
    pub fn get_frame_rate_hz(&self) -> u32 {
        self.inner.frame_rate_hz()
    }

    /// Returns the number of channels in the decoded waveform.
    ///
    /// If you did not set ``num_channels`` as an argument during decoding,
    /// then this value will be the total number of channels found in the audio.
    ///
    /// If you *did* set ``num_channels`` during decoding, then this value
    /// will be the value you set.
    ///
    /// Returns:
    ///     int: The number of channels
    ///
    #[getter]
    pub fn get_num_channels(&self) -> u16 {
        self.inner.num_channels()
    }

    /// Returns the number of frames in the decoded waveform.
    ///
    /// This will be the total number of frames founded in the encoded
    /// audio--unless you trimmed the waveform during decoding by setting
    /// ``start_time_milliseconds``, ``end_time_milliseconds``, or both.
    ///
    /// Returns:
    ///     int: The number of frames
    ///
    #[getter]
    pub fn get_num_frames(&self) -> usize {
        self.inner.num_frames()
    }

    /// Resamples the waveform using the default resampler.
    ///
    /// By default, Babycat resamples audio using
    /// `libsamplerate <http://www.mega-nerd.com/SRC/>`_ at its
    /// highest-quality setting.
    ///
    /// Example:
    ///     **Resample from 44,100 hz to 88,200 hz.**
    ///
    ///     >>> from babycat import Waveform
    ///     >>>
    ///     >>> waveform = Waveform.from_frames_of_silence(
    ///     ...     frame_rate_hz=44100,
    ///     ...     num_channels=2,
    ///     ...     num_frames=1000,
    ///     ... )
    ///     >>> waveform
    ///     <babycat.Waveform: 1000 frames, 2 channels, 44100 hz>
    ///     >>> resampled = waveform.resample(11025)
    ///     <babycat.Waveform: 250 frames, 2 channels, 11025 hz>
    ///
    /// Args:
    ///     frame_rate_hz(int): The target frame rate to resample the waveform to.
    ///
    /// Returns:
    ///     Waveform: A new waveform resampled at the given
    ///     frame rate.
    ///
    /// Raises:
    ///     babycat.exceptions.FeatureNotCompiled: Raised when you are trying
    ///         to use a feature at runtime that as not included in Babycat
    ///         at compile-time.
    ///
    ///     babycat.exceptions.ResamplingError: Raised when we
    ///         failed to encode an audio stream into an output format.
    ///
    #[args(frame_rate_hz)]
    #[pyo3(text_signature = "(
        frame_rate_hz,
    )")]
    pub fn resample(&self, py: Python<'_>, frame_rate_hz: u32) -> PyResult<Self> {
        let wr = py.allow_threads(move || self.inner.resample(frame_rate_hz));
        let waveform = wr?;
        Ok(waveform.into())
    }

    /// Resamples the waveform with the resampler of your choice.
    ///
    /// Babycat comes with different backends for resampling audio
    /// waveforms from one frame rate to another frame rate.
    ///
    /// By default, Babycat resamples audio using
    /// `libsamplerate <http://www.mega-nerd.com/SRC/>`_ at its
    /// highest-quality setting. Babycat also comes with two other
    /// resampling backends that are often faster--but produce
    /// slightly lower quality output.
    ///
    /// Example:
    ///     **Resample from 44,100 hz to 88,200 hz.**
    ///
    ///     >>> from babycat import Waveform
    ///     >>> from babycat.resample_mode import *
    ///     >>>
    ///     >>> waveform = Waveform.from_frames_of_silence(
    ///     ...     frame_rate_hz=44100,
    ///     ...     num_channels=2,
    ///     ...     num_frames=1000,
    ///     ... )
    ///     >>> waveform
    ///     <babycat.Waveform: 1000 frames, 2 channels, 44100 hz>
    ///     >>> resampled = waveform.resample_by_mode(
    ///     ...     frame_rate_hz=11025,
    ///     ...     resample_mode=RESAMPLE_MODE_BABYCAT_SINC,
    ///     ... )
    ///     <babycat.Waveform: 250 frames, 2 channels, 11025 hz>
    ///
    /// Args:
    ///     frame_rate_hz(int): The target frame rate to resample to.
    ///
    ///     resample_mode(int): The resampler to use. This has to be
    ///         one of the constants in :py:mod:`babycat.resample_mode`.
    ///
    /// Returns:
    ///     Waveform: A new waveform resampled at the given
    ///     frame rate.
    ///
    /// Raises:
    ///     babycat.exceptions.FeatureNotCompiled: Raised when you are trying
    ///         to use a feature at runtime that as not included in Babycat
    ///         at compile-time.
    ///
    ///     babycat.exceptions.ResamplingError: Raised when we
    ///         failed to encode an audio stream into an output format.
    ///
    #[args("*", frame_rate_hz, resample_mode)]
    #[pyo3(text_signature = "(
        frame_rate_hz,
        resample_mode,
    )")]
    pub fn resample_by_mode(
        &self,
        py: Python<'_>,
        frame_rate_hz: u32,
        resample_mode: u32,
    ) -> PyResult<Self> {
        let wr =
            py.allow_threads(move || self.inner.resample_by_mode(frame_rate_hz, resample_mode));
        let waveform = wr?;
        Ok(waveform.into())
    }

    /// Return a given audio sample belonging to a specific frame and channel.
    ///
    /// This method performs bounds checks. If you want an unsafe
    /// method that does not perform bounds checks, use
    /// :py:meth:`get_unchecked_sample`.
    ///
    /// Args:
    ///     frame_idx: The index of the given frame to query.
    ///
    ///     channel_idx: the index of the given channel to query.
    ///
    /// Returns:
    ///     Returns ``None`` if ``frame_idx`` or  ``channel_idx``
    ///     is out-of-bounds. Otherwise, it returns an Audio sample as
    ///     a native Python 64-bit :py:class:`float` value.
    ///
    #[args(frame_idx, channel_idx)]
    #[pyo3(text_signature = "(
        self,
        frame_idx,
        channel_idx,
    )")]
    pub fn get_sample(&self, frame_idx: usize, channel_idx: u16) -> Option<f32> {
        self.inner.get_sample(frame_idx, channel_idx)
    }

    /// Return a given audio sample belonging to a specific frame and channel,
    /// *without* performing any bounds checks.
    ///
    /// If you want bounds checking, use the :py:meth:`get_sample` method.
    ///
    /// Args:
    ///     frame_idx: The index of the given frame to query.
    ///
    ///     channel_idx: the index of the given channel to query.
    ///
    /// Returns:
    ///     Returns ``None`` if ``frame_idx`` or ``channel_idx``
    ///     is out-of-bounds. Otherwise, it returns an Audio sample as
    ///     a native Python 64-bit :py:class:`float` value.
    ///
    #[args(frame_idx, channel_idx)]
    #[pyo3(text_signature = "(
        self,
        frame_idx,
        channel_idx,
    )")]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked_sample(&self, frame_idx: usize, channel_idx: u16) -> f32 {
        self.inner.get_unchecked_sample(frame_idx, channel_idx)
    }

    /// Returns the audio waveform as a Python list of interleaved samples.
    #[args()]
    #[pyo3(text_signature = "()")]
    pub fn to_interleaved_samples(&self) -> Vec<f32> {
        self.inner.to_interleaved_samples().to_owned()
    }

    /// Returns the waveform as a 2D :py:class:`numpy.ndarray` array with shape ``(frames, channels)``
    ///
    /// Babycat internally stores decoded audio as a Rust ``Vec<f32>``.
    /// This method converts the ``Vec<f32>`` into a NumPy array.
    /// Babycat does not internally cache the NumPy array, so avoid
    /// calling this method multiple times on the same
    /// :py:class:`~babycat.Waveform`.
    ///
    /// Babycat is also designed to release the Python Global Interpreter
    /// Lock (GIL) when *decoding* audio into a ``Vec<f32>``, but Babycat
    /// re-acquires the GIL when converting the  the ``Vec<f32>`` into a NumPy array.
    ///
    /// Returns:
    ///     numpy.ndarray: A NumPy array with frames as the first axis
    ///     and channels as the second axis.
    ///
    #[args()]
    #[pyo3(text_signature = "()")]
    pub fn to_numpy(&self, py: Python) -> PyArraySamples {
        let num_channels = self.inner.num_channels();
        let num_frames = self.inner.num_frames();
        let interleaved_samples: Vec<f32> = self.inner.to_interleaved_samples().to_owned();
        interleaved_samples_to_pyarray(py, num_channels, num_frames, interleaved_samples)
    }

    /// Encodes the waveform into a :py:class:`bytearray` in the WAV format.
    ///
    /// Example:
    ///     **Decode an MP3 file and re-encode it as WAV.**
    ///
    ///     >>> from babycat import Waveform
    ///     >>> waveform = Waveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.flac",
    ///     ... )
    ///     >>> waveform
    ///     <babycat.Waveform: 9586415 frames, 2 channels, 44100 hz>
    ///     >>> arr = waveform.to_wav_buffer()
    ///     >>> type(arr)
    ///     >>> len(arr)
    ///
    /// Returns:
    ///     bytearray: The encoded WAV file.
    ///
    /// Raises:
    ///     babycat.exceptions.UnknownEncodeError: When something went wrong with the
    ///         encoding.
    ///
    #[args()]
    #[pyo3(text_signature = "()")]
    pub fn to_wav_buffer(&self, py: Python) -> PyResult<Py<PyAny>> {
        match self.inner.to_wav_buffer() {
            Ok(vec_u8) => Ok((*PyByteArray::new(py, &vec_u8)).to_object(py)),
            Err(err) => Err(PyErr::from(err)),
        }
    }

    /// Writes the waveform to the filesystem as a WAV file.
    ///
    /// Example:
    ///     **Decode an MP3 file and re-encode it as WAV.**
    ///
    ///     >>> from babycat import Waveform
    ///     >>> waveform = Waveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.flac",
    ///     ... )
    ///     >>> waveform
    ///     <babycat.Waveform: 9586415 frames, 2 channels, 44100 hz>
    ///     >>> waveform.to_wav_file("track.wav")
    ///
    /// Args:
    ///     filename(str): The filename to write the WAV file to.
    ///
    /// Raises:
    ///     babycat.exceptions.UnknownEncodeError: When something went wrong with the
    ///         encoding.
    ///
    #[cfg(feature = "enable-filesystem")]
    #[args(filename)]
    #[pyo3(text_signature = "(filename)")]
    pub fn to_wav_file(&self, filename: &str) -> PyResult<()> {
        self.inner.to_wav_file(filename).map_err(PyErr::from)
    }

    /// Generates an HTML audio widget in IPython and Jupyter notebooks.
    pub fn _repr_html_(&self) -> PyResult<String> {
        let wav = self.inner.to_wav_buffer()?;
        let wav_buffer_base64 = base64::encode(&wav);
        Ok(format!(
            "
<audio controls>
    <source src='data:audio/wav;base64,{}' type='audio/wav' />
    Your browser does not support the audio element.
</audio>",
            wav_buffer_base64
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}
