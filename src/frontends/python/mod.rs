//use numpy::{IntoPyArray, PyArray2};
//use pyo3::gc::{PyGCProtocol, PyVisit};
use pyo3::prelude::*;
//use pyo3::PyObjectProtocol;
//use pyo3::PyTraverseError;

pub mod exceptions;
pub mod float_waveform;

#[pymodule]
pub fn babycat(py: Python, m: &PyModule) -> PyResult<()> {
    // Configure exceptions.
    let exceptions_submodule = exceptions::make_exceptions_submodule(py)?;
    m.add_submodule(&exceptions_submodule)?;
    //
    //
    m.add_class::<float_waveform::FloatWaveform>()?;
    //
    //
    m.add_class::<float_waveform::FloatWaveformNamedResult>()?;

    // End of the module
    Ok(())
}
