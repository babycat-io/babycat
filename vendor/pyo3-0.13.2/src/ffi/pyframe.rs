#[cfg(not(Py_LIMITED_API))]
use crate::ffi::PyFrameObject;
use std::os::raw::c_int;

#[cfg(Py_LIMITED_API)]
opaque_struct!(PyFrameObject);

extern "C" {
    pub fn PyFrame_GetLineNumber(f: *mut PyFrameObject) -> c_int;
}
// skipped PyFrame_GetLineNumber
// skipped PyFrame_GetCode
