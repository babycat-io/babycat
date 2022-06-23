use pyo3::prelude::*;

use crate::frontends::python::waveform::PyArraySamples;

/// A container for decoding operations that may have succeeded or failed.
///
/// :py:class:`NumPyNamedResult` contains either a :py:class:`numpy.ndarray`
/// from a successful audio decoding or a Python exception from a failed
/// audio decoding.
///
#[pyclass(module = "babycat")]
#[derive(Clone, Debug)]
pub struct NumPyNamedResult {
    /// The "name" of a result as a :py:class:`str`, typically a filename for an audio file.
    #[pyo3(get)]
    pub name: String,
    /// A :py:class:`numpy.ndarray` if decoding succeeded... or ``None`` if decoding failed.
    #[pyo3(get)]
    pub array: Option<PyArraySamples>,
    error: Option<crate::backend::Error>,
}

#[pymethods]
impl NumPyNamedResult {
    /// ``None`` if decoding succeeded... or an exception if decoding failed.
    #[getter]
    fn get_exception(&self) -> Option<PyErr> {
        self.error.map(PyErr::from)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

impl IntoPy<NumPyNamedResult> for crate::backend::WaveformNamedResult {
    fn into_py(self, py: Python<'_>) -> NumPyNamedResult {
        match self.result {
            Ok(waveform) => NumPyNamedResult {
                name: self.name,
                array: Some(waveform.into_py(py)),
                error: None,
            },
            Err(err) => NumPyNamedResult {
                name: self.name,
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
