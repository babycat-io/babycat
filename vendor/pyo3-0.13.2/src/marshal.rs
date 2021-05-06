#![cfg(not(Py_LIMITED_API))]
//! Support for the Python `marshal` format. Not supported in limited API
//! builds.

use crate::ffi;
use crate::types::{PyAny, PyBytes};
use crate::{AsPyPointer, FromPyPointer, PyResult, Python};
use std::os::raw::{c_char, c_int};

/// The current version of the marshal binary format.
pub const VERSION: i32 = 4;

/// Serialize an object to bytes using the Python built-in marshal module.
///
/// The built-in marshalling only supports a limited range of objects.
/// The exact types supported depend on the version argument.
/// The [`VERSION`] constant holds the highest version currently supported.
///
/// See the [Python documentation](https://docs.python.org/3/library/marshal.html) for more details.
///
/// # Example:
/// ```
/// # use pyo3::{marshal, types::PyDict};
/// # let gil = pyo3::Python::acquire_gil();
/// # let py = gil.python();
/// #
/// let dict = PyDict::new(py);
/// dict.set_item("aap", "noot").unwrap();
/// dict.set_item("mies", "wim").unwrap();
/// dict.set_item("zus", "jet").unwrap();
///
/// let bytes = marshal::dumps(py, dict, marshal::VERSION);
/// ```
pub fn dumps<'a>(py: Python<'a>, object: &impl AsPyPointer, version: i32) -> PyResult<&'a PyBytes> {
    unsafe {
        let bytes = ffi::PyMarshal_WriteObjectToString(object.as_ptr(), version as c_int);
        FromPyPointer::from_owned_ptr_or_err(py, bytes)
    }
}

/// Deserialize an object from bytes using the Python built-in marshal module.
pub fn loads<'a, B>(py: Python<'a>, data: &B) -> PyResult<&'a PyAny>
where
    B: AsRef<[u8]> + ?Sized,
{
    let data = data.as_ref();
    unsafe {
        let c_str = data.as_ptr() as *const c_char;
        let object = ffi::PyMarshal_ReadObjectFromString(c_str, data.len() as isize);
        FromPyPointer::from_owned_ptr_or_err(py, object)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::PyDict;

    #[test]
    fn marhshal_roundtrip() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let dict = PyDict::new(py);
        dict.set_item("aap", "noot").unwrap();
        dict.set_item("mies", "wim").unwrap();
        dict.set_item("zus", "jet").unwrap();

        let bytes = dumps(py, dict, VERSION)
            .expect("marshalling failed")
            .as_bytes();
        let deserialzed = loads(py, bytes).expect("unmarshalling failed");

        assert!(equal(py, dict, deserialzed));
    }

    fn equal(_py: Python, a: &impl AsPyPointer, b: &impl AsPyPointer) -> bool {
        unsafe { ffi::PyObject_RichCompareBool(a.as_ptr(), b.as_ptr(), ffi::Py_EQ) != 0 }
    }
}
