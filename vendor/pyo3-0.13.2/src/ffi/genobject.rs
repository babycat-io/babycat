use crate::ffi::object::*;
use crate::ffi::pyport::Py_ssize_t;
use crate::ffi::PyFrameObject;
use std::os::raw::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyGenObject {
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    pub _ob_next: *mut PyObject,
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    pub _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub gi_frame: *mut PyFrameObject,
    pub gi_running: c_int,
    pub gi_code: *mut PyObject,
    pub gi_weakreflist: *mut PyObject,
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyGen_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyGen_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyGen_Type)
}

#[inline]
pub unsafe fn PyGen_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyGen_Type) as c_int
}

extern "C" {
    pub fn PyGen_New(frame: *mut PyFrameObject) -> *mut PyObject;
    // skipped PyGen_New
    // skipped PyGen_NewWithQualName
    // skipped _PyGen_SetStopIterationValue
    // skipped _PyGen_FetchStopIterationValue
    // skipped _PyGen_yf
    // skipped _PyGen_Finalize
    #[cfg(not(Py_3_9))]
    #[deprecated(note = "This function was never documented in the Python API.")]
    pub fn PyGen_NeedsFinalizing(op: *mut PyGenObject) -> c_int;
}

// skipped PyCoroObject

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyCoro_Type: PyTypeObject;
    pub static mut _PyCoroWrapper_Type: PyTypeObject;
}

#[deprecated(since = "0.14.0", note = "use PyCoro_CheckExact instead")]
#[inline]
pub unsafe fn PyCoro_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyCoro_Type)
}

#[inline]
pub unsafe fn PyCoro_CheckExact(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyCoro_Type)
}

#[deprecated(since = "0.14.0", note = "not in Python API")]
#[inline]
pub unsafe fn PyCoroWrapper_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut _PyCoroWrapper_Type)
}

// skipped _PyCoro_GetAwaitableIter
// skipped PyCoro_New

// skipped PyAsyncGenObject

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyAsyncGen_Type: PyTypeObject;
    // skipped _PyAsyncGenASend_Type
    // skipped _PyAsyncGenWrappedValue_Type
    // skipped _PyAsyncGenAThrow_Type
}

// skipped PyAsyncGen_New

#[deprecated(since = "0.14.0", note = "use PyCoro_CheckExact instead")]
#[inline]
pub unsafe fn PyAsyncGen_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyAsyncGen_Type)
}

#[inline]
pub unsafe fn PyAsyncGen_CheckExact(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyAsyncGen_Type)
}

// skipped _PyAsyncGenValueWrapperNew
