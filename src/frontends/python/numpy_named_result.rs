use numpy::PyArray2;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

/// A container for decoding operations that may have succeeded or failed.
#[pyclass(module = "babycat")]
#[derive(Clone, Debug)]
pub struct NumPyNamedResult {
    /// The "name" of a result as a :py:class:`str`, typically a filename for an audio file.
    #[pyo3(get)]
    pub name: String,
    /// A :py:class:`~babycat.array` if decoding succeeded... or ``None`` if decoding failed.
    #[pyo3(get)]
    pub array: Option<Py<PyArray2<f32>>>,
    error: Option<crate::backend::Error>,
}

#[pymethods]
impl NumPyNamedResult {
    /// ``None`` if decoding succeeded... or an exception if decoding failed.
    #[getter]
    fn get_exception(&self) -> Option<PyErr> {
        self.error.map(PyErr::from)
    }
}

impl From<crate::backend::WaveformNamedResult> for NumPyNamedResult {
    fn from(inner: crate::backend::WaveformNamedResult) -> Self {
        match inner.result {
            Ok(waveform) => Self {
                name: inner.name,
                array: Some(waveform.into()),
                error: None,
            },
            Err(err) => Self {
                name: inner.name,
                array: None,
                error: Some(err),
            },
        }
    }
}

impl std::fmt::Display for NumPyNamedResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.array {
            Some(array) => {
                write!(
                    f,
                    "<babycat.NumPyNamedResult: name={} array={}>",
                    self.name, array
                )
            }
            None => match self.error {
                Some(error) => {
                    write!(
                        f,
                        "<babycat.NumPyNamedResult name={} error={}>",
                        self.name, error
                    )
                }
                None => {
                    write!(f, "<babycat.NumPyNamedResult name={}>", self.name,)
                }
            },
        }
    }
}

#[pyproto]
impl PyObjectProtocol for NumPyNamedResult {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}
