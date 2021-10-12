use pyo3::prelude::*;

pub mod decoding_backend;
pub mod exceptions;
pub mod resample_mode;
pub mod waveform;

/// Module docstring first line
///
/// Module docstring second line
#[pymodule]
pub fn babycat(py: Python, m: &PyModule) -> PyResult<()> {
    // Configure the exceptions submodule.
    let exceptions_submodule = exceptions::make_exceptions_submodule(py)?;
    m.add_submodule(exceptions_submodule)?;

    // Configure the resample submodule.
    let resample_submodule = resample_mode::make_resample_mode_submodule(py)?;
    m.add_submodule(resample_submodule)?;

    // Configure the decoding backend submodule.
    let decoding_backend_submodule = decoding_backend::make_decoding_backend_submodule(py)?;
    m.add_submodule(decoding_backend_submodule)?;

    // Configure the Waveform class, which does most of the heavy lifting.
    m.add_class::<waveform::Waveform>()?;

    // Configure the WaveformNamedResult class, which we
    // use to wrap error messages when decoding.
    m.add_class::<waveform::WaveformNamedResult>()?;

    // End of the module
    Ok(())
}
