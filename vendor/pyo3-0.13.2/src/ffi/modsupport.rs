use crate::ffi::methodobject::PyMethodDef;
use crate::ffi::moduleobject::PyModuleDef;
use crate::ffi::object::PyObject;
use crate::ffi::pyport::Py_ssize_t;
use std::os::raw::{c_char, c_int, c_long};

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyArg_Parse")]
    pub fn PyArg_Parse(arg1: *mut PyObject, arg2: *const c_char, ...) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyArg_ParseTuple")]
    pub fn PyArg_ParseTuple(arg1: *mut PyObject, arg2: *const c_char, ...) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyArg_ParseTupleAndKeywords")]
    pub fn PyArg_ParseTupleAndKeywords(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *const c_char,
        arg4: *mut *mut c_char,
        ...
    ) -> c_int;
    pub fn PyArg_ValidateKeywordArguments(arg1: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyArg_UnpackTuple")]
    pub fn PyArg_UnpackTuple(
        arg1: *mut PyObject,
        arg2: *const c_char,
        arg3: Py_ssize_t,
        arg4: Py_ssize_t,
        ...
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPy_BuildValue")]
    pub fn Py_BuildValue(arg1: *const c_char, ...) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "_PyPy_BuildValue_SizeT")]
    //pub fn _Py_BuildValue_SizeT(arg1: *const c_char, ...)
    // -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPy_VaBuildValue")]
    //pub fn Py_VaBuildValue(arg1: *const c_char, arg2: va_list)
    // -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyModule_AddObject")]
    pub fn PyModule_AddObject(
        arg1: *mut PyObject,
        arg2: *const c_char,
        arg3: *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyModule_AddIntConstant")]
    pub fn PyModule_AddIntConstant(arg1: *mut PyObject, arg2: *const c_char, arg3: c_long)
        -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyModule_AddStringConstant")]
    pub fn PyModule_AddStringConstant(
        arg1: *mut PyObject,
        arg2: *const c_char,
        arg3: *const c_char,
    ) -> c_int;
    pub fn PyModule_SetDocString(arg1: *mut PyObject, arg2: *const c_char) -> c_int;
    pub fn PyModule_AddFunctions(arg1: *mut PyObject, arg2: *mut PyMethodDef) -> c_int;
    pub fn PyModule_ExecDef(module: *mut PyObject, def: *mut PyModuleDef) -> c_int;
}

pub const Py_CLEANUP_SUPPORTED: i32 = 0x2_0000;

pub const PYTHON_API_VERSION: i32 = 1013;
pub const PYTHON_ABI_VERSION: i32 = 3;

extern "C" {
    #[cfg(not(py_sys_config = "Py_TRACE_REFS"))]
    #[cfg_attr(PyPy, link_name = "PyPyModule_Create2")]
    pub fn PyModule_Create2(module: *mut PyModuleDef, apiver: c_int) -> *mut PyObject;

    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    fn PyModule_Create2TraceRefs(module: *mut PyModuleDef, apiver: c_int) -> *mut PyObject;

    #[cfg(not(py_sys_config = "Py_TRACE_REFS"))]
    pub fn PyModule_FromDefAndSpec2(
        def: *mut PyModuleDef,
        spec: *mut PyObject,
        module_api_version: c_int,
    ) -> *mut PyObject;

    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    fn PyModule_FromDefAndSpec2TraceRefs(
        def: *mut PyModuleDef,
        spec: *mut PyObject,
        module_api_version: c_int,
    ) -> *mut PyObject;
}

#[cfg(py_sys_config = "Py_TRACE_REFS")]
#[inline]
pub unsafe fn PyModule_Create2(module: *mut PyModuleDef, apiver: c_int) -> *mut PyObject {
    PyModule_Create2TraceRefs(module, apiver)
}

#[cfg(py_sys_config = "Py_TRACE_REFS")]
#[inline]
pub unsafe fn PyModule_FromDefAndSpec2(
    def: *mut PyModuleDef,
    spec: *mut PyObject,
    module_api_version: c_int,
) -> *mut PyObject {
    PyModule_FromDefAndSpec2TraceRefs(def, spec, module_api_version)
}

#[inline]
pub unsafe fn PyModule_Create(module: *mut PyModuleDef) -> *mut PyObject {
    PyModule_Create2(
        module,
        if cfg!(Py_LIMITED_API) {
            PYTHON_ABI_VERSION
        } else {
            PYTHON_API_VERSION
        },
    )
}

#[inline]
pub unsafe fn PyModule_FromDefAndSpec(def: *mut PyModuleDef, spec: *mut PyObject) -> *mut PyObject {
    PyModule_FromDefAndSpec2(
        def,
        spec,
        if cfg!(Py_LIMITED_API) {
            PYTHON_ABI_VERSION
        } else {
            PYTHON_API_VERSION
        },
    )
}
