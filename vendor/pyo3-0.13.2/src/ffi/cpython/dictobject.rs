use crate::ffi::object::*;
use crate::ffi::pyport::Py_ssize_t;
use std::os::raw::c_int;

opaque_struct!(PyDictKeysObject);

#[repr(C)]
#[derive(Debug)]
// Not moved because dict.rs uses PyDictObject extensively.
pub struct PyDictObject {
    pub ob_base: PyObject,
    pub ma_used: Py_ssize_t,
    pub ma_version_tag: u64,
    pub ma_keys: *mut PyDictKeysObject,
    pub ma_values: *mut *mut PyObject,
}

extern "C" {
    // skipped _PyDict_GetItem_KnownHash
    // skipped _PyDict_GetItemIdWithError
    // skipped _PyDict_GetItemStringWithError
    // skipped PyDict_SetDefault
    pub fn _PyDict_SetItem_KnownHash(
        mp: *mut PyObject,
        key: *mut PyObject,
        item: *mut PyObject,
        hash: crate::ffi::Py_hash_t,
    ) -> c_int;
    // skipped _PyDict_DelItem_KnownHash
    // skipped _PyDict_DelItemIf
    // skipped _PyDict_NewKeysForClass
    pub fn _PyDict_Next(
        mp: *mut PyObject,
        pos: *mut Py_ssize_t,
        key: *mut *mut PyObject,
        value: *mut *mut PyObject,
        hash: *mut crate::ffi::Py_hash_t,
    ) -> c_int;
    // skipped PyDict_GET_SIZE
    // skipped _PyDict_Contains_KnownHash
    // skipped _PyDict_ContainsId
    pub fn _PyDict_NewPresized(minused: Py_ssize_t) -> *mut PyObject;
    // skipped _PyDict_MaybeUntrack
    // skipped _PyDict_HasOnlyStringKeys
    // skipped _PyDict_KeysSize
    // skipped _PyDict_SizeOf
    // skipped _PyDict_Pop
    // skipped _PyDict_Pop_KnownHash
    // skipped _PyDict_FromKeys
    // skipped _PyDict_HasSplitTable
    // skipped _PyDict_MergeEx
    // skipped _PyDict_SetItemId
    // skipped _PyDict_DelItemId
    // skipped _PyDict_DebugMallocStats
    // skipped _PyObjectDict_SetItem
    // skipped _PyDict_LoadGlobal
    // skipped _PyDict_GetItemHint
    // skipped _PyDictViewObject
    // skipped _PyDictView_New
    // skipped _PyDictView_Intersect
    #[cfg(not(Py_3_10))]
    pub fn _PyDict_Contains(mp: *mut PyObject, key: *mut PyObject, hash: Py_ssize_t) -> c_int;
}
