use crate::ffi::longobject::PyLongObject;
use crate::ffi::object::*;
use std::os::raw::{c_int, c_long};

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyBool_Type")]
    pub static mut PyBool_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyBool_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyBool_Type) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "_PyPy_FalseStruct")]
    static mut _Py_FalseStruct: PyLongObject;
    #[cfg_attr(PyPy, link_name = "_PyPy_TrueStruct")]
    static mut _Py_TrueStruct: PyLongObject;
}

#[inline]
pub unsafe fn Py_False() -> *mut PyObject {
    &mut _Py_FalseStruct as *mut PyLongObject as *mut PyObject
}

#[inline]
pub unsafe fn Py_True() -> *mut PyObject {
    &mut _Py_TrueStruct as *mut PyLongObject as *mut PyObject
}

// skipped Py_RETURN_TRUE
// skipped Py_RETURN_FALSE

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyBool_FromLong")]
    pub fn PyBool_FromLong(arg1: c_long) -> *mut PyObject;
}
