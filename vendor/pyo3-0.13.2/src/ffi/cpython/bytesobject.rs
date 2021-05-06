use crate::ffi::object::PyObject;
use crate::ffi::pyport::Py_ssize_t;
use std::os::raw::c_int;

extern "C" {
    pub fn _PyBytes_Resize(bytes: *mut *mut PyObject, newsize: Py_ssize_t) -> c_int;
}
