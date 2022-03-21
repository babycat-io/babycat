use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Returns ``True`` if Babycat was compiled with support for
/// reading and writing files from/to the local filesystem.
#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_filesystem() -> bool {
    crate::backend::build_info::compiled_with_filesystem()
}

/// Returns ``True`` if Babycat was compiled with multithreading support.
#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_multithreading() -> bool {
    crate::backend::build_info::compiled_with_multithreading()
}

/// Returns ``True`` if Babycat was compiled with FFmpeg support enabled.
///
/// This function will return ``True`` no matter how
/// FFmpeg was compiled or linked to Babycat.
#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_ffmpeg() -> bool {
    crate::backend::build_info::compiled_with_ffmpeg()
}

/// Returns ``True`` if Babycat was statically linked to an existing copy of FFmpeg.
#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_ffmpeg_link_static() -> bool {
    crate::backend::build_info::compiled_with_ffmpeg_link_static()
}

/// Returns ``true`` if Babycat compiled its own copy of FFmpeg.
#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_ffmpeg_build_link_static() -> bool {
    crate::backend::build_info::compiled_with_ffmpeg_build_link_static()
}

/// The copyright license for this version of Babycat.
///
/// This could change based on which features or libraries
/// were compiled into Babycat.
#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn copyright_license_spdx() -> &'static str {
    crate::backend::build_info::copyright_license_spdx()
}

/// The current Babycat version.
///
/// This function returns `"0.0.0"` for development versions
/// of Babycat.
#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn babycat_version() -> &'static str {
    crate::backend::build_info::babycat_version()
}

pub fn make_build_info_submodule(py: Python) -> PyResult<&PyModule> {
    let build_info_submodule = PyModule::new(py, "build_info")?;

    build_info_submodule.setattr(
        "__doc__",
        "
Information about compile-time features and licensing.
",
    )?;

    build_info_submodule.add_function(wrap_pyfunction!(
        compiled_with_filesystem,
        build_info_submodule
    )?)?;

    build_info_submodule.add_function(wrap_pyfunction!(
        compiled_with_multithreading,
        build_info_submodule
    )?)?;

    build_info_submodule.add_function(wrap_pyfunction!(
        compiled_with_ffmpeg,
        build_info_submodule
    )?)?;

    build_info_submodule.add_function(wrap_pyfunction!(
        compiled_with_ffmpeg_link_static,
        build_info_submodule
    )?)?;

    build_info_submodule.add_function(wrap_pyfunction!(
        compiled_with_ffmpeg_build_link_static,
        build_info_submodule
    )?)?;

    build_info_submodule.add_function(wrap_pyfunction!(
        copyright_license_spdx,
        build_info_submodule
    )?)?;

    build_info_submodule.add_function(wrap_pyfunction!(babycat_version, build_info_submodule)?)?;

    Ok(build_info_submodule)
}
