use crate::ffi::object::*;
use crate::ffi::pyport::Py_ssize_t;
use std::os::raw::c_int;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyList_Type")]
    pub static mut PyList_Type: PyTypeObject;
    pub static mut PyListIter_Type: PyTypeObject;
    pub static mut PyListRevIter_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyList_Check(op: *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_LIST_SUBCLASS)
}

#[inline]
pub unsafe fn PyList_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyList_Type) as c_int
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyList_New")]
    pub fn PyList_New(size: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyList_Size")]
    pub fn PyList_Size(arg1: *mut PyObject) -> Py_ssize_t;
    #[cfg_attr(PyPy, link_name = "PyPyList_GetItem")]
    pub fn PyList_GetItem(arg1: *mut PyObject, arg2: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyList_SetItem")]
    pub fn PyList_SetItem(arg1: *mut PyObject, arg2: Py_ssize_t, arg3: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_Insert")]
    pub fn PyList_Insert(arg1: *mut PyObject, arg2: Py_ssize_t, arg3: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_Append")]
    pub fn PyList_Append(arg1: *mut PyObject, arg2: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_GetSlice")]
    pub fn PyList_GetSlice(
        arg1: *mut PyObject,
        arg2: Py_ssize_t,
        arg3: Py_ssize_t,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyList_SetSlice")]
    pub fn PyList_SetSlice(
        arg1: *mut PyObject,
        arg2: Py_ssize_t,
        arg3: Py_ssize_t,
        arg4: *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_Sort")]
    pub fn PyList_Sort(arg1: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_Reverse")]
    pub fn PyList_Reverse(arg1: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyList_AsTuple")]
    pub fn PyList_AsTuple(arg1: *mut PyObject) -> *mut PyObject;
}
