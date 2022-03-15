use std::ffi::CString;
use std::os::raw::c_char;

use crate::backend::build_info::*;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_filesystem() -> bool {
    compiled_with_filesystem()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_multithreading() -> bool {
    compiled_with_multithreading()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_ffmpeg() -> bool {
    compiled_with_ffmpeg()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_ffmpeg_link_static() -> bool {
    compiled_with_ffmpeg_link_static()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_compiled_with_ffmpeg_build_link_static() -> bool {
    compiled_with_ffmpeg_build_link_static()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_copyright_license_spdx() -> *const c_char {
    let c_string = CString::new(copyright_license_spdx()).unwrap();
    c_string.as_ptr()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_build_info_version() -> *const c_char {
    let c_string = CString::new(babycat_version()).unwrap();
    c_string.as_ptr()
}
