//use numpy::{IntoPyArray, PyArray2};
//use pyo3::gc::{PyGCProtocol, PyVisit};
use pyo3::prelude::*;
//use pyo3::PyObjectProtocol;
//use pyo3::PyTraverseError;

pub mod exceptions;

#[pyclass(module = "babycat")]
#[derive(Clone, Debug)]
pub struct FloatWaveform {
    inner: crate::backend::FloatWaveform,
}

#[pymodule]
pub fn babycat(py: Python, m: &PyModule) -> PyResult<()> {
    let exceptions_submodule = exceptions::make_exceptions_submodule(py)?;

    m.add_submodule(&exceptions_submodule)?;

    //m.add_class::<FloatWaveform>()?;

    // End of the module
    Ok(())
}
