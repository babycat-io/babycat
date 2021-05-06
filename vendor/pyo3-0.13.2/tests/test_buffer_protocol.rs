#![cfg(not(Py_LIMITED_API))]

use pyo3::buffer::PyBuffer;
use pyo3::class::PyBufferProtocol;
use pyo3::exceptions::PyBufferError;
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::AsPyPointer;
use std::ffi::CStr;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[pyclass]
struct TestBufferClass {
    vec: Vec<u8>,
    drop_called: Arc<AtomicBool>,
}

#[pyproto]
impl PyBufferProtocol for TestBufferClass {
    fn bf_getbuffer(slf: PyRefMut<Self>, view: *mut ffi::Py_buffer, flags: c_int) -> PyResult<()> {
        if view.is_null() {
            return Err(PyBufferError::new_err("View is null"));
        }

        if (flags & ffi::PyBUF_WRITABLE) == ffi::PyBUF_WRITABLE {
            return Err(PyBufferError::new_err("Object is not writable"));
        }

        unsafe {
            (*view).obj = slf.as_ptr();
            ffi::Py_INCREF((*view).obj);
        }

        let bytes = &slf.vec;

        unsafe {
            (*view).buf = bytes.as_ptr() as *mut c_void;
            (*view).len = bytes.len() as isize;
            (*view).readonly = 1;
            (*view).itemsize = 1;

            (*view).format = ptr::null_mut();
            if (flags & ffi::PyBUF_FORMAT) == ffi::PyBUF_FORMAT {
                let msg = CStr::from_bytes_with_nul(b"B\0").unwrap();
                (*view).format = msg.as_ptr() as *mut _;
            }

            (*view).ndim = 1;
            (*view).shape = ptr::null_mut();
            if (flags & ffi::PyBUF_ND) == ffi::PyBUF_ND {
                (*view).shape = (&((*view).len)) as *const _ as *mut _;
            }

            (*view).strides = ptr::null_mut();
            if (flags & ffi::PyBUF_STRIDES) == ffi::PyBUF_STRIDES {
                (*view).strides = &((*view).itemsize) as *const _ as *mut _;
            }

            (*view).suboffsets = ptr::null_mut();
            (*view).internal = ptr::null_mut();
        }

        Ok(())
    }

    fn bf_releasebuffer(_slf: PyRefMut<Self>, _view: *mut ffi::Py_buffer) {}
}

impl Drop for TestBufferClass {
    fn drop(&mut self) {
        print!("dropped");
        self.drop_called.store(true, Ordering::Relaxed);
    }
}

#[test]
fn test_buffer() {
    let drop_called = Arc::new(AtomicBool::new(false));

    {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let instance = Py::new(
            py,
            TestBufferClass {
                vec: vec![b' ', b'2', b'3'],
                drop_called: drop_called.clone(),
            },
        )
        .unwrap();
        let env = [("ob", instance)].into_py_dict(py);
        py.run("assert bytes(ob) == b' 23'", None, Some(env))
            .unwrap();
    }

    assert!(drop_called.load(Ordering::Relaxed));
}

#[test]
fn test_buffer_referenced() {
    let drop_called = Arc::new(AtomicBool::new(false));

    let buf = {
        let input = vec![b' ', b'2', b'3'];
        let gil = Python::acquire_gil();
        let py = gil.python();
        let instance: PyObject = TestBufferClass {
            vec: input.clone(),
            drop_called: drop_called.clone(),
        }
        .into_py(py);

        let buf = PyBuffer::<u8>::get(instance.as_ref(py)).unwrap();
        assert_eq!(buf.to_vec(py).unwrap(), input);
        drop(instance);
        buf
    };

    assert!(!drop_called.load(Ordering::Relaxed));

    {
        let _py = Python::acquire_gil().python();
        drop(buf);
    }

    assert!(drop_called.load(Ordering::Relaxed));
}
