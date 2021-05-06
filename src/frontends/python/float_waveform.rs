use crate::backend::Waveform;
use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::PyObjectProtocol;

#[pyclass(module = "babycat")]
#[derive(Clone, Debug)]
pub struct FloatWaveformNamedResult {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub waveform: Option<FloatWaveform>,
    error: Option<crate::backend::Error>,
}

#[pymethods]
impl FloatWaveformNamedResult {
    #[getter]
    fn get_exception(&self) -> Option<PyErr> {
        match self.error {
            Some(error) => Some(PyErr::from(error)),
            None => None,
        }
    }
}

impl From<crate::backend::NamedResult<crate::backend::FloatWaveform, crate::backend::Error>>
    for FloatWaveformNamedResult
{
    fn from(
        inner: crate::backend::NamedResult<crate::backend::FloatWaveform, crate::backend::Error>,
    ) -> Self {
        match inner.result {
            Ok(waveform) => Self {
                name: inner.name,
                waveform: Some(waveform.into()),
                error: None,
            },
            Err(err) => Self {
                name: inner.name,
                waveform: None,
                error: Some(err),
            },
        }
    }
}

impl std::fmt::Display for FloatWaveformNamedResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.waveform {
            Some(waveform) => {
                write!(
                    f,
                    "<babycat.FloatWaveformNamedResult name: {} waveform: {}>",
                    self.name, waveform
                )
            }
            None => match self.error {
                Some(error) => {
                    write!(
                        f,
                        "<babycat.FloatWaveformNamedResult name: {} error: {}>",
                        self.name,
                        error.to_string()
                    )
                }
                None => {
                    write!(
                        f,
                        "<babycat.FloatWaveformNamedResult name: {} and no value>",
                        self.name,
                    )
                }
            },
        }
    }
}

#[pyproto]
impl PyObjectProtocol for FloatWaveformNamedResult {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

#[pyclass(module = "babycat")]
#[derive(Clone, Debug)]
pub struct FloatWaveform {
    inner: crate::backend::FloatWaveform,
}

impl From<crate::backend::FloatWaveform> for FloatWaveform {
    fn from(inner: crate::backend::FloatWaveform) -> FloatWaveform {
        FloatWaveform { inner }
    }
}

impl std::fmt::Display for FloatWaveform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<babycat.FloatWaveform frame_rate_hz: {} num_channels: {} num_frames: {}>",
            self.inner.frame_rate_hz(),
            self.inner.num_channels(),
            self.inner.num_frames(),
        )
    }
}

#[pyproto]
impl PyObjectProtocol for FloatWaveform {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

fn float_waveform_to_pyresult(
    result: Result<crate::backend::FloatWaveform, crate::backend::Error>,
) -> PyResult<FloatWaveform> {
    match result {
        Ok(waveform) => Ok(waveform.into()),
        Err(error) => Err(error.into()),
    }
}

#[pymethods]
impl FloatWaveform {
    #[staticmethod]
    #[args("*", frame_rate_hz, num_channels, num_frames)]
    #[text_signature = "(
        frame_rate_hz,
        num_channels,
        num_frames,
    )"]
    pub fn from_frames_of_silence(frame_rate_hz: u32, num_channels: u32, num_frames: u64) -> Self {
        crate::backend::FloatWaveform::from_frames_of_silence(
            frame_rate_hz,
            num_channels,
            num_frames,
        )
        .into()
    }

    /// First line of a docstring
    ///
    /// Second line of a docstring
    #[staticmethod]
    #[args("*", frame_rate_hz, num_channels, duration_milliseconds)]
    #[text_signature = "(
        frame_rate_hz,
        num_channels,
        duration_milliseconds,
    )"]
    pub fn from_milliseconds_of_silence(
        frame_rate_hz: u32,
        num_channels: u32,
        duration_milliseconds: u64,
    ) -> Self {
        crate::backend::FloatWaveform::from_milliseconds_of_silence(
            frame_rate_hz,
            num_channels,
            duration_milliseconds,
        )
        .into()
    }

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
        resample_mode = 0,
        file_extension = "\"\"",
        mime_type = "\"\""
    )]
    #[text_signature = "(
        encoded_bytes,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        resample_mode = 0,
        file_extension = \"\",
        mime_type = \"\",
    )"]
    #[allow(clippy::too_many_arguments)]
    pub fn from_encoded_bytes(
        encoded_bytes: Vec<u8>,
        start_time_milliseconds: u64,
        end_time_milliseconds: u64,
        frame_rate_hz: u32,
        num_channels: u32,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        resample_mode: u32,
        file_extension: &str,
        mime_type: &str,
    ) -> PyResult<Self> {
        let decode_args = crate::backend::DecodeArgs {
            start_time_milliseconds,
            end_time_milliseconds,
            frame_rate_hz,
            num_channels,
            convert_to_mono,
            zero_pad_ending,
            resample_mode,
        };
        float_waveform_to_pyresult(crate::backend::FloatWaveform::from_encoded_bytes_with_hint(
            &encoded_bytes,
            decode_args,
            file_extension,
            mime_type,
        ))
    }

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
        resample_mode = 0
    )]
    #[text_signature = "(
        filename,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        resample_mode = 0,
    )"]
    #[allow(clippy::too_many_arguments)]
    pub fn from_file(
        filename: &str,
        start_time_milliseconds: u64,
        end_time_milliseconds: u64,
        frame_rate_hz: u32,
        num_channels: u32,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        resample_mode: u32,
    ) -> PyResult<Self> {
        let decode_args = crate::backend::DecodeArgs {
            start_time_milliseconds,
            end_time_milliseconds,
            frame_rate_hz,
            num_channels,
            convert_to_mono,
            zero_pad_ending,
            resample_mode,
        };
        float_waveform_to_pyresult(crate::backend::FloatWaveform::from_file(
            filename,
            decode_args,
        ))
    }

    #[cfg(all(feature = "enable-multithreading", feature = "enable-filesystem"))]
    #[staticmethod]
    #[args(
        filenames,
        "*",
        start_time_milliseconds = 0,
        end_time_milliseconds = 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = false,
        zero_pad_ending = false,
        resample_mode = 0,
        num_workers = 0
    )]
    #[text_signature = "(
        filenames,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        resample_mode = 0,
        num_workers = 0,
    )"]
    #[allow(clippy::too_many_arguments)]
    pub fn from_many_files(
        filenames: Vec<String>,
        start_time_milliseconds: u64,
        end_time_milliseconds: u64,
        frame_rate_hz: u32,
        num_channels: u32,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        resample_mode: u32,
        num_workers: usize,
    ) -> Vec<FloatWaveformNamedResult> {
        let decode_args = crate::backend::DecodeArgs {
            start_time_milliseconds,
            end_time_milliseconds,
            frame_rate_hz,
            num_channels,
            convert_to_mono,
            zero_pad_ending,
            resample_mode,
        };
        let batch_args = crate::backend::BatchArgs { num_workers };
        let filenames_ref: Vec<&str> = filenames.iter().map(|f| f.as_str()).collect();
        crate::backend::FloatWaveform::from_many_files(&filenames_ref, decode_args, batch_args)
            .into_iter()
            .map(FloatWaveformNamedResult::from)
            .collect()
    }

    #[getter]
    pub fn get_frame_rate_hz(&self) -> u32 {
        self.inner.frame_rate_hz()
    }

    #[getter]
    pub fn get_num_channels(&self) -> u32 {
        self.inner.num_channels()
    }

    #[getter]
    pub fn get_num_frames(&self) -> u64 {
        self.inner.num_frames()
    }

    #[args()]
    #[text_signature = "()"]
    pub fn numpy(&self, py: Python) -> Py<PyArray2<f32>> {
        self.inner
            .interleaved_samples()
            .to_owned()
            .into_pyarray(py)
            .reshape([
                self.inner.num_frames() as usize,
                self.inner.num_channels() as usize,
            ])
            .unwrap()
            .into()
    }

    #[args()]
    #[text_signature = "()"]
    pub fn to_wav_buffer(&self, py: Python) -> PyResult<Py<PyAny>> {
        match self.inner.to_wav_buffer() {
            Ok(vec_u8) => Ok((*PyByteArray::new(py, &vec_u8)).to_object(py)),
            Err(err) => Err(PyErr::from(err)),
        }
    }

    #[cfg(feature = "enable-filesystem")]
    #[args(filename)]
    #[text_signature = "(filename)"]
    pub fn to_wav_file(&self, filename: &str) -> PyResult<()> {
        self.inner.to_wav_file(filename).map_err(PyErr::from)
    }
}
