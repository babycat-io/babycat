use pyo3::prelude::*;

pub mod exceptions;
pub mod float_waveform;

/// Module docstring first line
///
/// Module docstring second line
#[pymodule]
pub fn babycat(py: Python, m: &PyModule) -> PyResult<()> {
    // Configure exceptions.
    let exceptions_submodule = exceptions::make_exceptions_submodule(py)?;
    m.add_submodule(&exceptions_submodule)?;

    m.add_class::<float_waveform::FloatWaveform>()?;

    m.add_class::<float_waveform::FloatWaveformNamedResult>()?;

    // End of the module
    Ok(())
}
