// Copyright (c) 2017-present PyO3 Project and Contributors
//
// based on Daniel Grunwald's https://github.com/dgrunwald/rust-cpython
use crate::{
    ffi, AsPyPointer, FromPyObject, IntoPy, PyAny, PyErr, PyNativeType, PyObject, PyResult, Python,
    ToPyObject,
};
use std::os::raw::c_double;

/// Represents a Python `float` object.
///
/// You can usually avoid directly working with this type
/// by using [`ToPyObject`](trait.ToPyObject.html)
/// and [extract](struct.PyAny.html#method.extract)
/// with `f32`/`f64`.
#[repr(transparent)]
pub struct PyFloat(PyAny);

pyobject_native_type!(
    PyFloat,
    ffi::PyFloatObject,
    ffi::PyFloat_Type,
    ffi::PyFloat_Check
);

impl PyFloat {
    /// Creates a new Python `float` object.
    pub fn new(py: Python<'_>, val: c_double) -> &PyFloat {
        unsafe { py.from_owned_ptr(ffi::PyFloat_FromDouble(val)) }
    }

    /// Gets the value of this float.
    pub fn value(&self) -> c_double {
        unsafe { ffi::PyFloat_AsDouble(self.as_ptr()) }
    }
}

impl ToPyObject for f64 {
    fn to_object(&self, py: Python) -> PyObject {
        PyFloat::new(py, *self).into()
    }
}

impl IntoPy<PyObject> for f64 {
    fn into_py(self, py: Python) -> PyObject {
        PyFloat::new(py, self).into()
    }
}

impl<'source> FromPyObject<'source> for f64 {
    // PyFloat_AsDouble returns -1.0 upon failure
    #![cfg_attr(feature = "cargo-clippy", allow(clippy::float_cmp))]
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let v = unsafe { ffi::PyFloat_AsDouble(obj.as_ptr()) };

        if v == -1.0 && PyErr::occurred(obj.py()) {
            Err(PyErr::fetch(obj.py()))
        } else {
            Ok(v)
        }
    }
}

impl ToPyObject for f32 {
    fn to_object(&self, py: Python) -> PyObject {
        PyFloat::new(py, f64::from(*self)).into()
    }
}

impl IntoPy<PyObject> for f32 {
    fn into_py(self, py: Python) -> PyObject {
        PyFloat::new(py, f64::from(self)).into()
    }
}

impl<'source> FromPyObject<'source> for f32 {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        Ok(obj.extract::<f64>()? as f32)
    }
}

#[cfg(test)]
mod test {
    #[cfg(not(Py_LIMITED_API))]
    use crate::ffi::PyFloat_AS_DOUBLE;
    #[cfg(not(Py_LIMITED_API))]
    use crate::AsPyPointer;
    use crate::{Python, ToPyObject};

    macro_rules! num_to_py_object_and_back (
        ($func_name:ident, $t1:ty, $t2:ty) => (
            #[test]
            fn $func_name() {
                use assert_approx_eq::assert_approx_eq;

                let gil = Python::acquire_gil();
                let py = gil.python();
                let val = 123 as $t1;
                let obj = val.to_object(py);
                assert_approx_eq!(obj.extract::<$t2>(py).unwrap(), val as $t2);
            }
        )
    );

    num_to_py_object_and_back!(to_from_f64, f64, f64);
    num_to_py_object_and_back!(to_from_f32, f32, f32);
    num_to_py_object_and_back!(int_to_float, i32, f64);

    #[cfg(not(Py_LIMITED_API))]
    #[test]
    fn test_as_double_macro() {
        use assert_approx_eq::assert_approx_eq;

        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = 1.23f64;
        let obj = v.to_object(py);
        assert_approx_eq!(v, unsafe { PyFloat_AS_DOUBLE(obj.as_ptr()) });
    }
}
