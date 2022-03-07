use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_filesystem() -> bool {
    crate::backend::build_info::compiled_with_filesystem()
}

#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_multithreading() -> bool {
    crate::backend::build_info::compiled_with_multithreading()
}

#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_ffmpeg() -> bool {
    crate::backend::build_info::compiled_with_ffmpeg()
}

#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_ffmpeg_link_static() -> bool {
    crate::backend::build_info::compiled_with_ffmpeg_link_static()
}

#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn compiled_with_ffmpeg_build_link_static() -> bool {
    crate::backend::build_info::compiled_with_ffmpeg_build_link_static()
}

#[pyfunction()]
#[pyo3(text_signature = "()")]
#[allow(clippy::too_many_arguments)]
pub fn copyright_license_spdx() -> &'static str {
    crate::backend::build_info::copyright_license_spdx()
}

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
Various build-time constants.
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
