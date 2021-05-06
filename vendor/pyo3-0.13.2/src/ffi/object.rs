use crate::ffi::pyport::{Py_hash_t, Py_ssize_t};
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

pub type FreeFunc = extern "C" fn(*mut c_void) -> c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[cfg(not(PyPy))]
pub struct PyObject {
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    _ob_next: *mut PyObject,
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg(PyPy)]
pub struct PyObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_pypy_link: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
}

#[cfg(py_sys_config = "Py_TRACE_REFS")]
#[cfg(not(PyPy))]
pub const PyObject_HEAD_INIT: PyObject = PyObject {
    _ob_next: std::ptr::null_mut(),
    _ob_prev: std::ptr::null_mut(),
    ob_refcnt: 1,
    ob_type: std::ptr::null_mut(),
};

#[cfg(not(py_sys_config = "Py_TRACE_REFS"))]
#[cfg(not(PyPy))]
pub const PyObject_HEAD_INIT: PyObject = PyObject {
    ob_refcnt: 1,
    ob_type: std::ptr::null_mut(),
};

#[cfg(py_sys_config = "Py_TRACE_REFS")]
#[cfg(PyPy)]
pub const PyObject_HEAD_INIT: PyObject = PyObject {
    _ob_next: std::ptr::null_mut(),
    _ob_prev: std::ptr::null_mut(),
    ob_refcnt: 1,
    ob_pypy_link: 0,
    ob_type: std::ptr::null_mut(),
};

#[cfg(not(py_sys_config = "Py_TRACE_REFS"))]
#[cfg(PyPy)]
pub const PyObject_HEAD_INIT: PyObject = PyObject {
    ob_refcnt: 1,
    ob_pypy_link: 0,
    ob_type: std::ptr::null_mut(),
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyVarObject {
    pub ob_base: PyObject,
    pub ob_size: Py_ssize_t,
}

#[inline]
pub unsafe fn Py_REFCNT(ob: *mut PyObject) -> Py_ssize_t {
    if ob.is_null() {
        panic!();
    }
    (*ob).ob_refcnt
}

#[inline]
pub unsafe fn Py_TYPE(ob: *mut PyObject) -> *mut PyTypeObject {
    (*ob).ob_type
}

#[inline]
pub unsafe fn Py_SIZE(ob: *mut PyObject) -> Py_ssize_t {
    (*(ob as *mut PyVarObject)).ob_size
}

pub type unaryfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject;

pub type binaryfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;

pub type ternaryfunc = unsafe extern "C" fn(
    arg1: *mut PyObject,
    arg2: *mut PyObject,
    arg3: *mut PyObject,
) -> *mut PyObject;

pub type inquiry = unsafe extern "C" fn(arg1: *mut PyObject) -> c_int;

pub type lenfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> Py_ssize_t;

pub type ssizeargfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t) -> *mut PyObject;

pub type ssizessizeargfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t, arg3: Py_ssize_t) -> *mut PyObject;

pub type ssizeobjargproc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t, arg3: *mut PyObject) -> c_int;

pub type ssizessizeobjargproc = unsafe extern "C" fn(
    arg1: *mut PyObject,
    arg2: Py_ssize_t,
    arg3: Py_ssize_t,
    arg4: *mut PyObject,
) -> c_int;

pub type objobjargproc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> c_int;

#[cfg(not(Py_LIMITED_API))]
mod bufferinfo {
    use crate::ffi::pyport::Py_ssize_t;
    use std::os::raw::{c_char, c_int, c_void};
    use std::ptr;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Py_buffer {
        pub buf: *mut c_void,
        pub obj: *mut crate::ffi::PyObject,
        pub len: Py_ssize_t,
        pub itemsize: Py_ssize_t,
        pub readonly: c_int,
        pub ndim: c_int,
        pub format: *mut c_char,
        pub shape: *mut Py_ssize_t,
        pub strides: *mut Py_ssize_t,
        pub suboffsets: *mut Py_ssize_t,
        pub internal: *mut c_void,
    }

    impl Py_buffer {
        pub const fn new() -> Self {
            Py_buffer {
                buf: ptr::null_mut(),
                obj: ptr::null_mut(),
                len: 0,
                itemsize: 0,
                readonly: 0,
                ndim: 0,
                format: ptr::null_mut(),
                shape: ptr::null_mut(),
                strides: ptr::null_mut(),
                suboffsets: ptr::null_mut(),
                internal: ptr::null_mut(),
            }
        }
    }

    pub type getbufferproc = unsafe extern "C" fn(
        arg1: *mut crate::ffi::PyObject,
        arg2: *mut Py_buffer,
        arg3: c_int,
    ) -> c_int;
    pub type releasebufferproc =
        unsafe extern "C" fn(arg1: *mut crate::ffi::PyObject, arg2: *mut Py_buffer);

    /// Maximum number of dimensions
    pub const PyBUF_MAX_NDIM: c_int = 64;

    /* Flags for getting buffers */
    pub const PyBUF_SIMPLE: c_int = 0;
    pub const PyBUF_WRITABLE: c_int = 0x0001;
    /*  we used to include an E, backwards compatible alias  */
    pub const PyBUF_WRITEABLE: c_int = PyBUF_WRITABLE;
    pub const PyBUF_FORMAT: c_int = 0x0004;
    pub const PyBUF_ND: c_int = 0x0008;
    pub const PyBUF_STRIDES: c_int = 0x0010 | PyBUF_ND;
    pub const PyBUF_C_CONTIGUOUS: c_int = 0x0020 | PyBUF_STRIDES;
    pub const PyBUF_F_CONTIGUOUS: c_int = 0x0040 | PyBUF_STRIDES;
    pub const PyBUF_ANY_CONTIGUOUS: c_int = 0x0080 | PyBUF_STRIDES;
    pub const PyBUF_INDIRECT: c_int = 0x0100 | PyBUF_STRIDES;

    pub const PyBUF_CONTIG: c_int = PyBUF_ND | PyBUF_WRITABLE;
    pub const PyBUF_CONTIG_RO: c_int = PyBUF_ND;

    pub const PyBUF_STRIDED: c_int = PyBUF_STRIDES | PyBUF_WRITABLE;
    pub const PyBUF_STRIDED_RO: c_int = PyBUF_STRIDES;

    pub const PyBUF_RECORDS: c_int = PyBUF_STRIDES | PyBUF_WRITABLE | PyBUF_FORMAT;
    pub const PyBUF_RECORDS_RO: c_int = PyBUF_STRIDES | PyBUF_FORMAT;

    pub const PyBUF_FULL: c_int = PyBUF_INDIRECT | PyBUF_WRITABLE | PyBUF_FORMAT;
    pub const PyBUF_FULL_RO: c_int = PyBUF_INDIRECT | PyBUF_FORMAT;

    pub const PyBUF_READ: c_int = 0x100;
    pub const PyBUF_WRITE: c_int = 0x200;
}
#[cfg(not(Py_LIMITED_API))]
pub use self::bufferinfo::*;

pub type objobjproc = unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> c_int;
pub type visitproc = unsafe extern "C" fn(object: *mut PyObject, arg: *mut c_void) -> c_int;
pub type traverseproc =
    unsafe extern "C" fn(slf: *mut PyObject, visit: visitproc, arg: *mut c_void) -> c_int;

pub type freefunc = unsafe extern "C" fn(arg1: *mut c_void);
pub type destructor = unsafe extern "C" fn(arg1: *mut PyObject);
#[cfg(not(Py_LIMITED_API))]
pub type printfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut ::libc::FILE, arg3: c_int) -> c_int;
pub type getattrfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut c_char) -> *mut PyObject;
pub type getattrofunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
pub type setattrfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut c_char, arg3: *mut PyObject) -> c_int;
pub type setattrofunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> c_int;
pub type reprfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject;
pub type hashfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> Py_hash_t;
pub type richcmpfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: c_int) -> *mut PyObject;
pub type getiterfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject;
pub type iternextfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject;
pub type descrgetfunc = unsafe extern "C" fn(
    arg1: *mut PyObject,
    arg2: *mut PyObject,
    arg3: *mut PyObject,
) -> *mut PyObject;
pub type descrsetfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> c_int;
pub type initproc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> c_int;
pub type newfunc = unsafe extern "C" fn(
    arg1: *mut PyTypeObject,
    arg2: *mut PyObject,
    arg3: *mut PyObject,
) -> *mut PyObject;
pub type allocfunc =
    unsafe extern "C" fn(arg1: *mut PyTypeObject, arg2: Py_ssize_t) -> *mut PyObject;
#[cfg(Py_3_8)]
pub type vectorcallfunc = unsafe extern "C" fn(
    callable: *mut PyObject,
    args: *const *mut PyObject,
    nargsf: libc::size_t,
    kwnames: *mut PyObject,
) -> *mut PyObject;

#[cfg(Py_LIMITED_API)]
mod typeobject {
    opaque_struct!(PyTypeObject);
}

#[cfg(not(Py_LIMITED_API))]
mod typeobject {
    use crate::ffi::pyport::Py_ssize_t;
    use crate::ffi::{self, object};
    use std::mem;
    use std::os::raw::{c_char, c_uint, c_ulong, c_void};
    use std::ptr;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PyNumberMethods {
        pub nb_add: Option<object::binaryfunc>,
        pub nb_subtract: Option<object::binaryfunc>,
        pub nb_multiply: Option<object::binaryfunc>,
        pub nb_remainder: Option<object::binaryfunc>,
        pub nb_divmod: Option<object::binaryfunc>,
        pub nb_power: Option<object::ternaryfunc>,
        pub nb_negative: Option<object::unaryfunc>,
        pub nb_positive: Option<object::unaryfunc>,
        pub nb_absolute: Option<object::unaryfunc>,
        pub nb_bool: Option<object::inquiry>,
        pub nb_invert: Option<object::unaryfunc>,
        pub nb_lshift: Option<object::binaryfunc>,
        pub nb_rshift: Option<object::binaryfunc>,
        pub nb_and: Option<object::binaryfunc>,
        pub nb_xor: Option<object::binaryfunc>,
        pub nb_or: Option<object::binaryfunc>,
        pub nb_int: Option<object::unaryfunc>,
        pub nb_reserved: *mut c_void,
        pub nb_float: Option<object::unaryfunc>,
        pub nb_inplace_add: Option<object::binaryfunc>,
        pub nb_inplace_subtract: Option<object::binaryfunc>,
        pub nb_inplace_multiply: Option<object::binaryfunc>,
        pub nb_inplace_remainder: Option<object::binaryfunc>,
        pub nb_inplace_power: Option<object::ternaryfunc>,
        pub nb_inplace_lshift: Option<object::binaryfunc>,
        pub nb_inplace_rshift: Option<object::binaryfunc>,
        pub nb_inplace_and: Option<object::binaryfunc>,
        pub nb_inplace_xor: Option<object::binaryfunc>,
        pub nb_inplace_or: Option<object::binaryfunc>,
        pub nb_floor_divide: Option<object::binaryfunc>,
        pub nb_true_divide: Option<object::binaryfunc>,
        pub nb_inplace_floor_divide: Option<object::binaryfunc>,
        pub nb_inplace_true_divide: Option<object::binaryfunc>,
        pub nb_index: Option<object::unaryfunc>,
        pub nb_matrix_multiply: Option<object::binaryfunc>,
        pub nb_inplace_matrix_multiply: Option<object::binaryfunc>,
    }

    macro_rules! as_expr {
        ($e:expr) => {
            $e
        };
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct PySequenceMethods {
        pub sq_length: Option<object::lenfunc>,
        pub sq_concat: Option<object::binaryfunc>,
        pub sq_repeat: Option<object::ssizeargfunc>,
        pub sq_item: Option<object::ssizeargfunc>,
        pub was_sq_slice: *mut c_void,
        pub sq_ass_item: Option<object::ssizeobjargproc>,
        pub was_sq_ass_slice: *mut c_void,
        pub sq_contains: Option<object::objobjproc>,
        pub sq_inplace_concat: Option<object::binaryfunc>,
        pub sq_inplace_repeat: Option<object::ssizeargfunc>,
    }

    #[repr(C)]
    #[derive(Clone, Default)]
    pub struct PyMappingMethods {
        pub mp_length: Option<object::lenfunc>,
        pub mp_subscript: Option<object::binaryfunc>,
        pub mp_ass_subscript: Option<object::objobjargproc>,
    }

    #[repr(C)]
    #[derive(Clone, Default)]
    pub struct PyAsyncMethods {
        pub am_await: Option<object::unaryfunc>,
        pub am_aiter: Option<object::unaryfunc>,
        pub am_anext: Option<object::unaryfunc>,
    }

    #[repr(C)]
    #[derive(Clone, Default)]
    pub struct PyBufferProcs {
        pub bf_getbuffer: Option<object::getbufferproc>,
        pub bf_releasebuffer: Option<object::releasebufferproc>,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct PyTypeObject {
        #[cfg(PyPy)]
        pub ob_refcnt: Py_ssize_t,
        #[cfg(PyPy)]
        pub ob_pypy_link: Py_ssize_t,
        #[cfg(PyPy)]
        pub ob_type: *mut PyTypeObject,
        #[cfg(PyPy)]
        pub ob_size: Py_ssize_t,
        #[cfg(not(PyPy))]
        pub ob_base: object::PyVarObject,
        pub tp_name: *const c_char,
        pub tp_basicsize: Py_ssize_t,
        pub tp_itemsize: Py_ssize_t,
        pub tp_dealloc: Option<object::destructor>,
        #[cfg(not(Py_3_8))]
        pub tp_print: Option<object::printfunc>,
        #[cfg(Py_3_8)]
        pub tp_vectorcall_offset: Py_ssize_t,
        pub tp_getattr: Option<object::getattrfunc>,
        pub tp_setattr: Option<object::setattrfunc>,
        pub tp_as_async: *mut PyAsyncMethods,
        pub tp_repr: Option<object::reprfunc>,
        pub tp_as_number: *mut PyNumberMethods,
        pub tp_as_sequence: *mut PySequenceMethods,
        pub tp_as_mapping: *mut PyMappingMethods,
        pub tp_hash: Option<object::hashfunc>,
        pub tp_call: Option<object::ternaryfunc>,
        pub tp_str: Option<object::reprfunc>,
        pub tp_getattro: Option<object::getattrofunc>,
        pub tp_setattro: Option<object::setattrofunc>,
        pub tp_as_buffer: *mut PyBufferProcs,
        pub tp_flags: c_ulong,
        pub tp_doc: *const c_char,
        pub tp_traverse: Option<object::traverseproc>,
        pub tp_clear: Option<object::inquiry>,
        pub tp_richcompare: Option<object::richcmpfunc>,
        pub tp_weaklistoffset: Py_ssize_t,
        pub tp_iter: Option<object::getiterfunc>,
        pub tp_iternext: Option<object::iternextfunc>,
        pub tp_methods: *mut ffi::methodobject::PyMethodDef,
        pub tp_members: *mut ffi::structmember::PyMemberDef,
        pub tp_getset: *mut ffi::descrobject::PyGetSetDef,
        pub tp_base: *mut PyTypeObject,
        pub tp_dict: *mut object::PyObject,
        pub tp_descr_get: Option<object::descrgetfunc>,
        pub tp_descr_set: Option<object::descrsetfunc>,
        pub tp_dictoffset: Py_ssize_t,
        pub tp_init: Option<object::initproc>,
        pub tp_alloc: Option<object::allocfunc>,
        pub tp_new: Option<object::newfunc>,
        pub tp_free: Option<object::freefunc>,
        pub tp_is_gc: Option<object::inquiry>,
        pub tp_bases: *mut object::PyObject,
        pub tp_mro: *mut object::PyObject,
        pub tp_cache: *mut object::PyObject,
        pub tp_subclasses: *mut object::PyObject,
        pub tp_weaklist: *mut object::PyObject,
        pub tp_del: Option<object::destructor>,
        pub tp_version_tag: c_uint,
        pub tp_finalize: Option<object::destructor>,
        #[cfg(Py_3_8)]
        pub tp_vectorcall: Option<object::vectorcallfunc>,
        #[cfg(PyPy)]
        pub tp_pypy_flags: std::os::raw::c_long,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_allocs: Py_ssize_t,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_frees: Py_ssize_t,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_maxalloc: Py_ssize_t,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_prev: *mut PyTypeObject,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_next: *mut PyTypeObject,
    }

    macro_rules! _type_object_init {
        ({$($head:tt)*}, $($tail:tt)*) => {
            as_expr! {
                PyTypeObject {
                    $($head)*
                    tp_name: ptr::null(),
                    tp_basicsize: 0,
                    tp_itemsize: 0,
                    tp_dealloc: None,
                    #[cfg(not(Py_3_8))]
                    tp_print: None,
                    #[cfg(Py_3_8)]
                    tp_vectorcall_offset: 0,
                    tp_getattr: None,
                    tp_setattr: None,
                    tp_as_async: ptr::null_mut(),
                    tp_repr: None,
                    tp_as_number: ptr::null_mut(),
                    tp_as_sequence: ptr::null_mut(),
                    tp_as_mapping: ptr::null_mut(),
                    tp_hash: None,
                    tp_call: None,
                    tp_str: None,
                    tp_getattro: None,
                    tp_setattro: None,
                    tp_as_buffer: ptr::null_mut(),
                    tp_flags: object::Py_TPFLAGS_DEFAULT,
                    tp_doc: ptr::null(),
                    tp_traverse: None,
                    tp_clear: None,
                    tp_richcompare: None,
                    tp_weaklistoffset: 0,
                    tp_iter: None,
                    tp_iternext: None,
                    tp_methods: ptr::null_mut(),
                    tp_members: ptr::null_mut(),
                    tp_getset: ptr::null_mut(),
                    tp_base: ptr::null_mut(),
                    tp_dict: ptr::null_mut(),
                    tp_descr_get: None,
                    tp_descr_set: None,
                    tp_dictoffset: 0,
                    tp_init: None,
                    tp_alloc: None,
                    tp_new: None,
                    tp_free: None,
                    tp_is_gc: None,
                    tp_bases: ptr::null_mut(),
                    tp_mro: ptr::null_mut(),
                    tp_cache: ptr::null_mut(),
                    tp_subclasses: ptr::null_mut(),
                    tp_weaklist: ptr::null_mut(),
                    tp_del: None,
                    tp_version_tag: 0,
                    tp_finalize: None,
                    #[cfg(Py_3_8)]
                    tp_vectorcall: None,
                    $($tail)*
                }
            }
        }
    }

    #[cfg(PyPy)]
    macro_rules! type_object_init {
        ($($tail:tt)*) => {
            _type_object_init!({
                    ob_refcnt: 1,
                    ob_pypy_link: 0,
                    ob_type: ptr::null_mut(),
                    ob_size: 0,
                },
                tp_pypy_flags: 0,
                $($tail)*
            )
        }
    }

    #[cfg(not(PyPy))]
    macro_rules! type_object_init {
        ($($tail:tt)*) => {
            _type_object_init!({
                ob_base: object::PyVarObject {
                    ob_base: object::PyObject_HEAD_INIT,
                    ob_size: 0
                },},
                $($tail)*
            )
        }
    }

    #[cfg(py_sys_config = "COUNT_ALLOCS")]
    pub const PyTypeObject_INIT: PyTypeObject = type_object_init! {
        tp_allocs: 0,
        tp_frees: 0,
        tp_maxalloc: 0,
        tp_prev: ptr::null_mut(),
        tp_next: ptr::null_mut(),
    };

    #[cfg(not(py_sys_config = "COUNT_ALLOCS"))]
    pub const PyTypeObject_INIT: PyTypeObject = type_object_init!();

    #[repr(C)]
    #[derive(Clone)]
    pub struct PyHeapTypeObject {
        pub ht_type: PyTypeObject,
        pub as_async: PyAsyncMethods,
        pub as_number: PyNumberMethods,
        pub as_mapping: PyMappingMethods,
        pub as_sequence: PySequenceMethods,
        pub as_buffer: PyBufferProcs,
        pub ht_name: *mut object::PyObject,
        pub ht_slots: *mut object::PyObject,
        pub ht_qualname: *mut object::PyObject,
        pub ht_cached_keys: *mut c_void,
    }

    impl Default for PyHeapTypeObject {
        #[inline]
        fn default() -> Self {
            unsafe { mem::zeroed() }
        }
    }

    #[inline]
    pub unsafe fn PyHeapType_GET_MEMBERS(
        etype: *mut PyHeapTypeObject,
    ) -> *mut ffi::structmember::PyMemberDef {
        let py_type = object::Py_TYPE(etype as *mut object::PyObject);
        let ptr = etype.offset((*py_type).tp_basicsize);
        ptr as *mut ffi::structmember::PyMemberDef
    }
}

// The exported types depend on whether Py_LIMITED_API is set
pub use self::typeobject::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyType_Slot {
    pub slot: c_int,
    pub pfunc: *mut c_void,
}

impl Default for PyType_Slot {
    fn default() -> PyType_Slot {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyType_Spec {
    pub name: *const c_char,
    pub basicsize: c_int,
    pub itemsize: c_int,
    pub flags: c_uint,
    pub slots: *mut PyType_Slot,
}

impl Default for PyType_Spec {
    fn default() -> PyType_Spec {
        unsafe { mem::zeroed() }
    }
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyType_FromSpec")]
    pub fn PyType_FromSpec(arg1: *mut PyType_Spec) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "PyPyType_FromSpecWithBases")]
    pub fn PyType_FromSpecWithBases(arg1: *mut PyType_Spec, arg2: *mut PyObject) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "PyPyType_GetSlot")]
    pub fn PyType_GetSlot(arg1: *mut PyTypeObject, arg2: c_int) -> *mut c_void;
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyType_IsSubtype")]
    pub fn PyType_IsSubtype(a: *mut PyTypeObject, b: *mut PyTypeObject) -> c_int;
}

#[inline]
pub unsafe fn PyObject_TypeCheck(ob: *mut PyObject, tp: *mut PyTypeObject) -> c_int {
    (Py_TYPE(ob) == tp || PyType_IsSubtype(Py_TYPE(ob), tp) != 0) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    /// built-in 'type'
    #[cfg_attr(PyPy, link_name = "PyPyType_Type")]
    pub static mut PyType_Type: PyTypeObject;
    /// built-in 'object'
    #[cfg_attr(PyPy, link_name = "PyPyBaseObject_Type")]
    pub static mut PyBaseObject_Type: PyTypeObject;
    /// built-in 'super'
    pub static mut PySuper_Type: PyTypeObject;
}

extern "C" {
    pub fn PyType_GetFlags(arg1: *mut PyTypeObject) -> c_ulong;
}

#[inline]
pub unsafe fn PyType_Check(op: *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_TYPE_SUBCLASS)
}

#[inline]
pub unsafe fn PyType_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyType_Type) as c_int
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyType_Ready")]
    pub fn PyType_Ready(t: *mut PyTypeObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyType_GenericAlloc")]
    pub fn PyType_GenericAlloc(t: *mut PyTypeObject, nitems: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyType_GenericNew")]
    pub fn PyType_GenericNew(
        t: *mut PyTypeObject,
        args: *mut PyObject,
        kwds: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyType_ClearCache() -> c_uint;
    #[cfg_attr(PyPy, link_name = "PyPyType_Modified")]
    pub fn PyType_Modified(t: *mut PyTypeObject);

    #[cfg(not(Py_LIMITED_API))]
    #[cfg_attr(PyPy, link_name = "PyPyObject_Print")]
    pub fn PyObject_Print(o: *mut PyObject, fp: *mut ::libc::FILE, flags: c_int) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Repr")]
    pub fn PyObject_Repr(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Str")]
    pub fn PyObject_Str(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_ASCII")]
    pub fn PyObject_ASCII(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Bytes")]
    pub fn PyObject_Bytes(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_RichCompare")]
    pub fn PyObject_RichCompare(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: c_int,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_RichCompareBool")]
    pub fn PyObject_RichCompareBool(arg1: *mut PyObject, arg2: *mut PyObject, arg3: c_int)
        -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_GetAttrString")]
    pub fn PyObject_GetAttrString(arg1: *mut PyObject, arg2: *const c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_SetAttrString")]
    pub fn PyObject_SetAttrString(
        arg1: *mut PyObject,
        arg2: *const c_char,
        arg3: *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_HasAttrString")]
    pub fn PyObject_HasAttrString(arg1: *mut PyObject, arg2: *const c_char) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_GetAttr")]
    pub fn PyObject_GetAttr(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_SetAttr")]
    pub fn PyObject_SetAttr(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject)
        -> c_int;
    pub fn PyObject_HasAttr(arg1: *mut PyObject, arg2: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_SelfIter")]
    pub fn PyObject_SelfIter(arg1: *mut PyObject) -> *mut PyObject;

    #[cfg(not(Py_LIMITED_API))]
    #[cfg(not(PyPy))]
    pub fn _PyObject_NextNotImplemented(arg1: *mut PyObject) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "PyPyObject_GenericGetAttr")]
    pub fn PyObject_GenericGetAttr(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_GenericSetAttr")]
    pub fn PyObject_GenericSetAttr(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> c_int;
    #[cfg(not(all(Py_LIMITED_API, not(Py_3_10))))]
    pub fn PyObject_GenericGetDict(arg1: *mut PyObject, arg2: *mut c_void) -> *mut PyObject;
    pub fn PyObject_GenericSetDict(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut c_void,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Hash")]
    pub fn PyObject_Hash(arg1: *mut PyObject) -> Py_hash_t;
    #[cfg_attr(PyPy, link_name = "PyPyObject_HashNotImplemented")]
    pub fn PyObject_HashNotImplemented(arg1: *mut PyObject) -> Py_hash_t;
    #[cfg_attr(PyPy, link_name = "PyPyObject_IsTrue")]
    pub fn PyObject_IsTrue(arg1: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Not")]
    pub fn PyObject_Not(arg1: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyCallable_Check")]
    pub fn PyCallable_Check(arg1: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_ClearWeakRefs")]
    pub fn PyObject_ClearWeakRefs(arg1: *mut PyObject);
    #[cfg(not(Py_LIMITED_API))]
    pub fn PyObject_CallFinalizer(arg1: *mut PyObject);
    #[cfg(not(Py_LIMITED_API))]
    #[cfg_attr(PyPy, link_name = "PyPyObject_CallFinalizerFromDealloc")]
    pub fn PyObject_CallFinalizerFromDealloc(arg1: *mut PyObject) -> c_int;

    #[cfg_attr(PyPy, link_name = "PyPyObject_Dir")]
    pub fn PyObject_Dir(arg1: *mut PyObject) -> *mut PyObject;
    pub fn Py_ReprEnter(arg1: *mut PyObject) -> c_int;
    pub fn Py_ReprLeave(arg1: *mut PyObject);
}

// Flag bits for printing:
pub const Py_PRINT_RAW: c_int = 1; // No string quotes etc.

/// Set if the type object is dynamically allocated
pub const Py_TPFLAGS_HEAPTYPE: c_ulong = 1 << 9;

/// Set if the type allows subclassing
pub const Py_TPFLAGS_BASETYPE: c_ulong = 1 << 10;

/// Set if the type implements the vectorcall protocol (PEP 590)
#[cfg(all(Py_3_8, not(Py_LIMITED_API)))]
pub const Py_TPFLAGS_HAVE_VECTORCALL: c_ulong = 1 << 11;

/// Set if the type is 'ready' -- fully initialized
pub const Py_TPFLAGS_READY: c_ulong = 1 << 12;

/// Set while the type is being 'readied', to prevent recursive ready calls
pub const Py_TPFLAGS_READYING: c_ulong = 1 << 13;

/// Objects support garbage collection (see objimp.h)
pub const Py_TPFLAGS_HAVE_GC: c_ulong = 1 << 14;

const Py_TPFLAGS_HAVE_STACKLESS_EXTENSION: c_ulong = 0;

/// Objects support type attribute cache
pub const Py_TPFLAGS_HAVE_VERSION_TAG: c_ulong = 1 << 18;
pub const Py_TPFLAGS_VALID_VERSION_TAG: c_ulong = 1 << 19;

/* Type is abstract and cannot be instantiated */
pub const Py_TPFLAGS_IS_ABSTRACT: c_ulong = 1 << 20;

/* These flags are used to determine if a type is a subclass. */
pub const Py_TPFLAGS_LONG_SUBCLASS: c_ulong = 1 << 24;
pub const Py_TPFLAGS_LIST_SUBCLASS: c_ulong = 1 << 25;
pub const Py_TPFLAGS_TUPLE_SUBCLASS: c_ulong = 1 << 26;
pub const Py_TPFLAGS_BYTES_SUBCLASS: c_ulong = 1 << 27;
pub const Py_TPFLAGS_UNICODE_SUBCLASS: c_ulong = 1 << 28;
pub const Py_TPFLAGS_DICT_SUBCLASS: c_ulong = 1 << 29;
pub const Py_TPFLAGS_BASE_EXC_SUBCLASS: c_ulong = 1 << 30;
pub const Py_TPFLAGS_TYPE_SUBCLASS: c_ulong = 1 << 31;

pub const Py_TPFLAGS_DEFAULT: c_ulong =
    Py_TPFLAGS_HAVE_STACKLESS_EXTENSION | Py_TPFLAGS_HAVE_VERSION_TAG;

pub const Py_TPFLAGS_HAVE_FINALIZE: c_ulong = 1;

#[inline]
#[cfg(Py_LIMITED_API)]
pub unsafe fn PyType_HasFeature(t: *mut PyTypeObject, f: c_ulong) -> c_int {
    ((PyType_GetFlags(t) & f) != 0) as c_int
}

#[inline]
#[cfg(not(Py_LIMITED_API))]
pub unsafe fn PyType_HasFeature(t: *mut PyTypeObject, f: c_ulong) -> c_int {
    (((*t).tp_flags & f) != 0) as c_int
}

#[inline]
pub unsafe fn PyType_FastSubclass(t: *mut PyTypeObject, f: c_ulong) -> c_int {
    PyType_HasFeature(t, f)
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "_PyPy_Dealloc")]
    pub fn _Py_Dealloc(arg1: *mut PyObject);
}

// Reference counting macros.
#[inline]
pub unsafe fn Py_INCREF(op: *mut PyObject) {
    if cfg!(py_sys_config = "Py_REF_DEBUG") {
        Py_IncRef(op)
    } else {
        (*op).ob_refcnt += 1
    }
}

#[inline]
pub unsafe fn Py_DECREF(op: *mut PyObject) {
    if cfg!(py_sys_config = "Py_REF_DEBUG") {
        Py_DecRef(op)
    } else {
        (*op).ob_refcnt -= 1;
        if (*op).ob_refcnt == 0 {
            _Py_Dealloc(op)
        }
    }
}

#[inline]
pub unsafe fn Py_CLEAR(op: &mut *mut PyObject) {
    let tmp = *op;
    if !tmp.is_null() {
        *op = ptr::null_mut();
        Py_DECREF(tmp);
    }
}

#[inline]
pub unsafe fn Py_XINCREF(op: *mut PyObject) {
    if !op.is_null() {
        Py_INCREF(op)
    }
}

#[inline]
pub unsafe fn Py_XDECREF(op: *mut PyObject) {
    if !op.is_null() {
        Py_DECREF(op)
    }
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPy_IncRef")]
    pub fn Py_IncRef(o: *mut PyObject);
    #[cfg_attr(PyPy, link_name = "PyPy_DecRef")]
    pub fn Py_DecRef(o: *mut PyObject);
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "_PyPy_NoneStruct")]
    static mut _Py_NoneStruct: PyObject;
    #[cfg_attr(PyPy, link_name = "_PyPy_NotImplementedStruct")]
    static mut _Py_NotImplementedStruct: PyObject;
}

#[inline]
pub unsafe fn Py_None() -> *mut PyObject {
    &mut _Py_NoneStruct
}

#[inline]
pub unsafe fn Py_NotImplemented() -> *mut PyObject {
    &mut _Py_NotImplementedStruct
}

/* Rich comparison opcodes */
pub const Py_LT: c_int = 0;
pub const Py_LE: c_int = 1;
pub const Py_EQ: c_int = 2;
pub const Py_NE: c_int = 3;
pub const Py_GT: c_int = 4;
pub const Py_GE: c_int = 5;

#[inline]
pub fn PyObject_Check(_arg1: *mut PyObject) -> c_int {
    1
}

#[inline]
pub fn PySuper_Check(_arg1: *mut PyObject) -> c_int {
    0
}

#[cfg(not(PyPy))]
extern "C" {
    pub fn _PyObject_GetDictPtr(obj: *mut PyObject) -> *mut *mut PyObject;
}
