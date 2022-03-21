use std::ffi::CString;
use std::os::raw::c_char;

use crate::backend::build_info::*;

/// Returns `true` if Babycat was compiled with support for
/// reading and writing files to/from the local filesystem.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_filesystem() -> bool {
    compiled_with_filesystem()
}

/// Returns `true` if Babycat was compiled with multithreading support.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_multithreading() -> bool {
    compiled_with_multithreading()
}

/// Returns `true` if Babycat was compiled with FFmpeg support enabled.
///
/// This function will return `true` no matter how FFmpeg was compiled
/// or linked to Babycat.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_ffmpeg() -> bool {
    compiled_with_ffmpeg()
}

/// Returns `true` if Babycat was statically linked to an existing copy of FFmpeg.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_ffmpeg_link_static() -> bool {
    compiled_with_ffmpeg_link_static()
}

/// Returns `true` if Babycat compiled its own copy of FFmpeg.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_ffmpeg_build_link_static() -> bool {
    compiled_with_ffmpeg_build_link_static()
}

/// The copyright license for this version of Babycat.
///
/// Babycat's license can vary based on which features or libraries were
/// compiled into Babycat.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_copyright_license_spdx() -> *const c_char {
    let c_string = CString::new(copyright_license_spdx()).unwrap();
    c_string.as_ptr()
}

/// The current Babycat version.
///
/// This function returns `"0.0.0"` for development versions
/// of Babycat.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_version() -> *const c_char {
    let c_string = CString::new(babycat_version()).unwrap();
    c_string.as_ptr()
}
