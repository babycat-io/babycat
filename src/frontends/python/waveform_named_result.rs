use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

/// A container for decoding operations that may have succeeded or failed.
#[pyclass(module = "babycat")]
#[derive(Clone, Debug)]
pub struct WaveformNamedResult {
    /// The "name" of a result as a :py:class:`str`, typically a filename for an audio file.
    #[pyo3(get)]
    pub name: String,
    /// A :py:class:`~babycat.Waveform` if decoding succeeded... or ``None`` if decoding failed.
    #[pyo3(get)]
    pub waveform: Option<crate::frontends::python::waveform::Waveform>,
    error: Option<crate::backend::Error>,
}

#[pymethods]
impl WaveformNamedResult {
    /// ``None`` if decoding succeeded... or an exception if decoding failed.
    #[getter]
    fn get_exception(&self) -> Option<PyErr> {
        self.error.map(PyErr::from)
    }
}

impl From<crate::backend::WaveformNamedResult> for WaveformNamedResult {
    fn from(inner: crate::backend::WaveformNamedResult) -> Self {
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

impl std::fmt::Display for WaveformNamedResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.waveform {
            Some(waveform) => {
                write!(
                    f,
                    "<babycat.WaveformNamedResult: name={} waveform={}>",
                    self.name, waveform
                )
            }
            None => match self.error {
                Some(error) => {
                    write!(
                        f,
                        "<babycat.WaveformNamedResult name={} error={}>",
                        self.name,
                        error.to_string()
                    )
                }
                None => {
                    write!(f, "<babycat.WaveformNamedResult name={}>", self.name,)
                }
            },
        }
    }
}

#[pyproto]
impl PyObjectProtocol for WaveformNamedResult {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}
