use pyo3::prelude::*;

/// Creates the `babycat.decoding_backend` submodule, which is used to
/// store constants pointing to resampler backends.
pub fn make_decoding_backend_submodule(py: Python) -> PyResult<&PyModule> {
    let decoding_backend_submodule = PyModule::new(py, "decoding_backend")?;

    decoding_backend_submodule.setattr(
        "__doc__",
        "
A Python submodule to hold constants representing different decoding backends.

Babycat comes with different backends for demuxing/decoding audio
files into waveforms.
",
    )?;
    decoding_backend_submodule.setattr(
        "DEFAULT_DECODING_BACKEND",
        crate::backend::DEFAULT_DECODING_BACKEND,
    )?;
    decoding_backend_submodule.setattr(
        "DECODING_BACKEND_SYMPHONIA",
        crate::backend::DECODING_BACKEND_SYMPHONIA,
    )?;

    decoding_backend_submodule.setattr(
        "DECODING_BACKEND_FFMPEG",
        crate::backend::DECODING_BACKEND_FFMPEG,
    )?;

    Ok(decoding_backend_submodule)
}
