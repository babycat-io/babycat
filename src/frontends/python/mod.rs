use pyo3::prelude::*;

pub mod exceptions;
pub mod float_waveform;
pub mod resample_mode;

/// Module docstring first line
///
/// Module docstring second line
#[pymodule]
pub fn babycat(py: Python, m: &PyModule) -> PyResult<()> {
    // Configure the exceptions submodule.
    let exceptions_submodule = exceptions::make_exceptions_submodule(py)?;
    m.add_submodule(exceptions_submodule)?;

    let resample_submodule = resample_mode::make_resample_mode_submodule(py)?;
    m.add_submodule(resample_submodule)?;

    // Configure the FloatWaveform class, which does most of the heavy lifting.
    m.add_class::<float_waveform::FloatWaveform>()?;

    // Configure the FloatWaveformNamedResult class, which we
    // use to wrap error messages when decoding.
    m.add_class::<float_waveform::FloatWaveformNamedResult>()?;

    // End of the module
    Ok(())
}
