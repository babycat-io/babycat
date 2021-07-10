use pyo3::prelude::*;

/// Creates a Python submodule to hold resampler constants.
///
/// This module just contains constants pointing to integers.
/// The integers map to different resampler backends that can be
/// used to resample a waveform.
pub fn make_resample_mode_submodule(py: Python) -> PyResult<&PyModule> {
    let resample_mode_submodule = PyModule::new(py, "resample_mode")?;

    resample_mode_submodule
        .setattr("__doc__", "Contains constants representing resample modes.")?;

    resample_mode_submodule.setattr(
        "RESAMPLE_MODE_LIBSAMPLERATE",
        crate::backend::RESAMPLE_MODE_LIBSAMPLERATE,
    )?;
    resample_mode_submodule.setattr(
        "RESAMPLE_MODE_BABYCAT_LANCZOS",
        crate::backend::RESAMPLE_MODE_BABYCAT_LANCZOS,
    )?;
    resample_mode_submodule.setattr(
        "RESAMPLE_MODE_BABYCAT_SINC",
        crate::backend::RESAMPLE_MODE_BABYCAT_SINC,
    )?;

    Ok(resample_mode_submodule)
}
