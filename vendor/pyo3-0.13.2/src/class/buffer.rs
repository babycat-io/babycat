// Copyright (c) 2017-present PyO3 Project and Contributors

//! Represent Python Buffer protocol implementation
//!
//! For more information check [buffer protocol](https://docs.python.org/3/c-api/buffer.html)
//! c-api
use crate::callback::IntoPyCallbackOutput;
use crate::{ffi, PyCell, PyClass, PyRefMut};
use std::os::raw::c_int;

/// Buffer protocol interface
///
/// For more information check [buffer protocol](https://docs.python.org/3/c-api/buffer.html)
/// c-api.
#[allow(unused_variables)]
pub trait PyBufferProtocol<'p>: PyClass {
    // No default implementations so that implementors of this trait provide both methods.

    fn bf_getbuffer(slf: PyRefMut<Self>, view: *mut ffi::Py_buffer, flags: c_int) -> Self::Result
    where
        Self: PyBufferGetBufferProtocol<'p>;

    fn bf_releasebuffer(slf: PyRefMut<Self>, view: *mut ffi::Py_buffer) -> Self::Result
    where
        Self: PyBufferReleaseBufferProtocol<'p>;
}

pub trait PyBufferGetBufferProtocol<'p>: PyBufferProtocol<'p> {
    type Result: IntoPyCallbackOutput<()>;
}

pub trait PyBufferReleaseBufferProtocol<'p>: PyBufferProtocol<'p> {
    type Result: IntoPyCallbackOutput<()>;
}

#[doc(hidden)]
pub unsafe extern "C" fn getbuffer<T>(
    slf: *mut ffi::PyObject,
    arg1: *mut ffi::Py_buffer,
    arg2: c_int,
) -> c_int
where
    T: for<'p> PyBufferGetBufferProtocol<'p>,
{
    crate::callback_body!(py, {
        let slf = py.from_borrowed_ptr::<PyCell<T>>(slf);
        T::bf_getbuffer(slf.try_borrow_mut()?, arg1, arg2).convert(py)
    })
}

#[doc(hidden)]
pub unsafe extern "C" fn releasebuffer<T>(slf: *mut ffi::PyObject, arg1: *mut ffi::Py_buffer)
where
    T: for<'p> PyBufferReleaseBufferProtocol<'p>,
{
    crate::callback_body!(py, {
        let slf = py.from_borrowed_ptr::<crate::PyCell<T>>(slf);
        T::bf_releasebuffer(slf.try_borrow_mut()?, arg1).convert(py)
    })
}
