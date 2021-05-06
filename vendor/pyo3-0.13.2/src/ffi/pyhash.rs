use crate::ffi::pyport::{Py_hash_t, Py_ssize_t};
use std::os::raw::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyHash_FuncDef {
    pub hash: Option<extern "C" fn(arg1: *const c_void, arg2: Py_ssize_t) -> Py_hash_t>,
    pub name: *const c_char,
    pub hash_bits: c_int,
    pub seed_bits: c_int,
}

impl Default for PyHash_FuncDef {
    #[inline]
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

extern "C" {
    pub fn PyHash_GetFuncDef() -> *mut PyHash_FuncDef;
    #[cfg(not(PyPy))]
    pub fn _Py_HashBytes(src: *const c_void, len: Py_ssize_t) -> Py_hash_t;
}
