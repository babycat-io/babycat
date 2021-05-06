use crate::ffi::{PyObject, Py_buffer, Py_ssize_t};
use std::os::raw::{c_char, c_int, c_void};

#[cfg(all(Py_3_8, not(PyPy)))]
use crate::ffi::{
    pyport::PY_SSIZE_T_MAX, vectorcallfunc, PyCallable_Check, PyThreadState, PyThreadState_GET,
    PyTuple_Check, PyType_HasFeature, Py_TPFLAGS_HAVE_VECTORCALL,
};
#[cfg(all(Py_3_8, not(PyPy)))]
use libc::size_t;

extern "C" {
    #[cfg(all(Py_3_8, not(PyPy)))]
    pub fn _PyStack_AsDict(values: *const *mut PyObject, kwnames: *mut PyObject) -> *mut PyObject;
}

#[cfg(all(Py_3_8, not(PyPy)))]
const _PY_FASTCALL_SMALL_STACK: size_t = 5;

extern "C" {
    #[cfg(all(Py_3_8, not(PyPy)))]
    pub fn _Py_CheckFunctionResult(
        tstate: *mut PyThreadState,
        callable: *mut PyObject,
        result: *mut PyObject,
        where_: *const c_char,
    ) -> *mut PyObject;

    #[cfg(all(Py_3_8, not(PyPy)))]
    pub fn _PyObject_MakeTpCall(
        tstate: *mut PyThreadState,
        callable: *mut PyObject,
        args: *const *mut PyObject,
        nargs: Py_ssize_t,
        keywords: *mut PyObject,
    ) -> *mut PyObject;
}

#[cfg(all(Py_3_8, not(PyPy)))]
const PY_VECTORCALL_ARGUMENTS_OFFSET: Py_ssize_t =
    1 << (8 * std::mem::size_of::<Py_ssize_t>() as Py_ssize_t - 1);

#[cfg(all(Py_3_8, not(PyPy)))]
#[inline(always)]
pub unsafe fn PyVectorcall_NARGS(n: size_t) -> Py_ssize_t {
    assert!(n <= (PY_SSIZE_T_MAX as size_t));
    (n as Py_ssize_t) & !PY_VECTORCALL_ARGUMENTS_OFFSET
}

#[cfg(all(Py_3_8, not(PyPy)))]
#[inline(always)]
pub unsafe fn PyVectorcall_Function(callable: *mut PyObject) -> Option<vectorcallfunc> {
    assert!(!callable.is_null());
    let tp = crate::ffi::Py_TYPE(callable);
    if PyType_HasFeature(tp, Py_TPFLAGS_HAVE_VECTORCALL) == 0 {
        return None;
    }
    assert!(PyCallable_Check(callable) > 0);
    let offset = (*tp).tp_vectorcall_offset;
    assert!(offset > 0);
    let ptr = (callable as *const c_char).offset(offset) as *const Option<vectorcallfunc>;
    *ptr
}

#[cfg(all(Py_3_8, not(PyPy)))]
#[inline(always)]
pub unsafe fn _PyObject_VectorcallTstate(
    tstate: *mut PyThreadState,
    callable: *mut PyObject,
    args: *const *mut PyObject,
    nargsf: size_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    assert!(kwnames.is_null() || PyTuple_Check(kwnames) > 0);
    assert!(!args.is_null() || PyVectorcall_NARGS(nargsf) == 0);

    match PyVectorcall_Function(callable) {
        None => {
            let nargs = PyVectorcall_NARGS(nargsf);
            _PyObject_MakeTpCall(tstate, callable, args, nargs, kwnames)
        }
        Some(func) => {
            let res = func(callable, args, nargsf, kwnames);
            _Py_CheckFunctionResult(tstate, callable, res, std::ptr::null_mut())
        }
    }
}

#[cfg(all(Py_3_8, not(PyPy)))]
#[inline(always)]
pub unsafe fn PyObject_Vectorcall(
    callable: *mut PyObject,
    args: *const *mut PyObject,
    nargsf: size_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    _PyObject_VectorcallTstate(PyThreadState_GET(), callable, args, nargsf, kwnames)
}

extern "C" {
    #[cfg(all(Py_3_8, not(PyPy)))]
    #[cfg_attr(not(Py_3_9), link_name = "_PyObject_VectorcallDict")]
    pub fn PyObject_VectorcallDict(
        callable: *mut PyObject,
        args: *const *mut PyObject,
        nargsf: size_t,
        kwargs: *mut PyObject,
    ) -> *mut PyObject;

    #[cfg(all(Py_3_8, not(PyPy)))]
    #[cfg_attr(not(Py_3_9), link_name = "_PyVectorcall_Call")]
    pub fn PyVectorcall_Call(
        callable: *mut PyObject,
        tuple: *mut PyObject,
        dict: *mut PyObject,
    ) -> *mut PyObject;
}

#[cfg(all(Py_3_8, not(PyPy)))]
#[inline(always)]
pub unsafe fn _PyObject_FastCallTstate(
    tstate: *mut PyThreadState,
    func: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
) -> *mut PyObject {
    _PyObject_VectorcallTstate(tstate, func, args, nargs as size_t, std::ptr::null_mut())
}

#[cfg(all(Py_3_8, not(PyPy)))]
#[inline(always)]
pub unsafe fn _PyObject_FastCall(
    func: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
) -> *mut PyObject {
    _PyObject_FastCallTstate(PyThreadState_GET(), func, args, nargs)
}

#[cfg(all(Py_3_8, not(PyPy)))]
#[inline(always)]
pub unsafe fn _PyObject_CallNoArg(func: *mut PyObject) -> *mut PyObject {
    _PyObject_VectorcallTstate(
        PyThreadState_GET(),
        func,
        std::ptr::null_mut(),
        0,
        std::ptr::null_mut(),
    )
}

#[cfg(all(Py_3_8, not(PyPy)))]
#[inline(always)]
pub unsafe fn PyObject_CallOneArg(func: *mut PyObject, arg: *mut PyObject) -> *mut PyObject {
    assert!(!arg.is_null());
    let _args = [std::ptr::null_mut(), arg];
    let args = _args.as_ptr().offset(1); // For PY_VECTORCALL_ARGUMENTS_OFFSET
    let tstate = PyThreadState_GET();
    let nargsf = 1 | PY_VECTORCALL_ARGUMENTS_OFFSET;
    _PyObject_VectorcallTstate(tstate, func, args, nargsf as size_t, std::ptr::null_mut())
}

extern "C" {
    #[cfg(all(Py_3_9, not(PyPy)))]
    pub fn PyObject_VectorcallMethod(
        name: *mut PyObject,
        args: *const *mut PyObject,
        nargsf: size_t,
        kwnames: *mut PyObject,
    ) -> *mut PyObject;
}

#[cfg(all(Py_3_9, not(PyPy)))]
#[inline(always)]
pub unsafe fn PyObject_CallMethodNoArgs(
    self_: *mut PyObject,
    name: *mut PyObject,
) -> *mut PyObject {
    PyObject_VectorcallMethod(
        name,
        &self_,
        1 | PY_VECTORCALL_ARGUMENTS_OFFSET as size_t,
        std::ptr::null_mut(),
    )
}

#[cfg(all(Py_3_9, not(PyPy)))]
#[inline(always)]
pub unsafe fn PyObject_CallMethodOneArg(
    self_: *mut PyObject,
    name: *mut PyObject,
    arg: *mut PyObject,
) -> *mut PyObject {
    let args = [self_, arg];
    assert!(!arg.is_null());
    PyObject_VectorcallMethod(
        name,
        args.as_ptr(),
        2 | PY_VECTORCALL_ARGUMENTS_OFFSET as size_t,
        std::ptr::null_mut(),
    )
}

// skipped _PyObject_VectorcallMethodId
// skipped _PyObject_CallMethodIdNoArgs
// skipped _PyObject_CallMethodIdOneArg

// skipped _PyObject_HasLen

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyObject_LengthHint")]
    pub fn PyObject_LengthHint(o: *mut PyObject, arg1: Py_ssize_t) -> Py_ssize_t;

    #[cfg(all(Py_3_9, not(PyPy)))]
    pub fn PyObject_CheckBuffer(obj: *mut PyObject) -> c_int;
}

#[cfg(not(any(Py_3_9, PyPy)))]
#[inline]
pub unsafe fn PyObject_CheckBuffer(o: *mut PyObject) -> c_int {
    let tp_as_buffer = (*crate::ffi::Py_TYPE(o)).tp_as_buffer;
    (!tp_as_buffer.is_null() && (*tp_as_buffer).bf_getbuffer.is_some()) as c_int
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyObject_GetBuffer")]
    pub fn PyObject_GetBuffer(obj: *mut PyObject, view: *mut Py_buffer, flags: c_int) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_GetPointer")]
    pub fn PyBuffer_GetPointer(view: *mut Py_buffer, indices: *mut Py_ssize_t) -> *mut c_void;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_SizeFromFormat")]
    pub fn PyBuffer_SizeFromFormat(format: *const c_char) -> Py_ssize_t;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_ToContiguous")]
    pub fn PyBuffer_ToContiguous(
        buf: *mut c_void,
        view: *mut Py_buffer,
        len: Py_ssize_t,
        order: c_char,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_FromContiguous")]
    pub fn PyBuffer_FromContiguous(
        view: *mut Py_buffer,
        buf: *mut c_void,
        len: Py_ssize_t,
        order: c_char,
    ) -> c_int;
    pub fn PyObject_CopyData(dest: *mut PyObject, src: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_IsContiguous")]
    pub fn PyBuffer_IsContiguous(view: *const Py_buffer, fort: c_char) -> c_int;
    pub fn PyBuffer_FillContiguousStrides(
        ndims: c_int,
        shape: *mut Py_ssize_t,
        strides: *mut Py_ssize_t,
        itemsize: c_int,
        fort: c_char,
    );
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_FillInfo")]
    pub fn PyBuffer_FillInfo(
        view: *mut Py_buffer,
        o: *mut PyObject,
        buf: *mut c_void,
        len: Py_ssize_t,
        readonly: c_int,
        flags: c_int,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyBuffer_Release")]
    pub fn PyBuffer_Release(view: *mut Py_buffer);
}

#[inline]
#[cfg(not(any(all(Py_3_8, Py_LIMITED_API), PyPy)))]
pub unsafe fn PyIter_Check(o: *mut PyObject) -> c_int {
    (match (*crate::ffi::Py_TYPE(o)).tp_iternext {
        Some(tp_iternext) => {
            tp_iternext as *const c_void != crate::ffi::object::_PyObject_NextNotImplemented as _
        }
        None => false,
    }) as c_int
}

// skipped PySequence_ITEM

pub const PY_ITERSEARCH_COUNT: c_int = 1;
pub const PY_ITERSEARCH_INDEX: c_int = 2;
pub const PY_ITERSEARCH_CONTAINS: c_int = 3;

extern "C" {
    #[cfg(not(PyPy))]
    pub fn _PySequence_IterSearch(
        seq: *mut PyObject,
        obj: *mut PyObject,
        operation: c_int,
    ) -> Py_ssize_t;
}

// skipped _PyObject_RealIsInstance
// skipped _PyObject_RealIsSubclass

// skipped _PySequence_BytesToCharpArray

// skipped _Py_FreeCharPArray

// skipped _Py_add_one_to_index_F
// skipped _Py_add_one_to_index_C

// skipped _Py_convert_optional_to_ssize_t

// skipped _PyNumber_Index(*mut PyObject o)
