// Copyright (c) 2017-present PyO3 Project and Contributors

//! Exception types defined by Python.

use crate::type_object::PySizedLayout;
use crate::{ffi, PyResult, Python};
use std::ffi::CStr;
use std::ops;
use std::os::raw::c_char;

/// The boilerplate to convert between a Rust type and a Python exception.
#[macro_export]
macro_rules! impl_exception_boilerplate {
    ($name: ident) => {
        impl std::convert::From<&$name> for $crate::PyErr {
            fn from(err: &$name) -> $crate::PyErr {
                $crate::PyErr::from_instance(err)
            }
        }

        impl $name {
            pub fn new_err<A>(args: A) -> $crate::PyErr
            where
                A: $crate::PyErrArguments + Send + Sync + 'static,
            {
                $crate::PyErr::new::<$name, A>(args)
            }
        }

        impl std::error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                unsafe {
                    use $crate::{AsPyPointer, PyNativeType};
                    let cause: &$crate::exceptions::PyBaseException = self
                        .py()
                        .from_owned_ptr_or_opt($crate::ffi::PyException_GetCause(self.as_ptr()))?;

                    Some(cause)
                }
            }
        }
    };
}

/// Defines a Rust type for an exception defined in Python code.
///
/// # Syntax
///
/// ```import_exception!(module, MyError)```
///
/// * `module` is the name of the containing module.
/// * `MyError` is the name of the new exception type.
///
/// # Example
/// ```
/// use pyo3::import_exception;
/// use pyo3::types::IntoPyDict;
/// use pyo3::Python;
///
/// import_exception!(socket, gaierror);
///
/// fn main() {
///     let gil = Python::acquire_gil();
///     let py = gil.python();
///
///     let ctx = [("gaierror", py.get_type::<gaierror>())].into_py_dict(py);
///     py.run(
///         "import socket; assert gaierror is socket.gaierror",
///         None,
///         Some(ctx),
///     )
///     .unwrap();
/// }
///
/// ```
#[macro_export]
macro_rules! import_exception {
    ($module: expr, $name: ident) => {
        #[repr(transparent)]
        #[allow(non_camel_case_types)] // E.g. `socket.herror`
        pub struct $name($crate::PyAny);

        $crate::impl_exception_boilerplate!($name);

        $crate::pyobject_native_type_core!(
            $name,
            $crate::ffi::PyBaseExceptionObject,
            *$name::type_object_raw($crate::Python::assume_gil_acquired()),
            Some(stringify!($module))
        );

        impl $name {
            fn type_object_raw(py: $crate::Python) -> *mut $crate::ffi::PyTypeObject {
                use $crate::once_cell::GILOnceCell;
                use $crate::AsPyPointer;
                static TYPE_OBJECT: GILOnceCell<$crate::Py<$crate::types::PyType>> =
                    GILOnceCell::new();

                TYPE_OBJECT
                    .get_or_init(py, || {
                        let imp = py
                            .import(stringify!($module))
                            .expect(concat!("Can not import module: ", stringify!($module)));
                        let cls = imp.get(stringify!($name)).expect(concat!(
                            "Can not load exception class: {}.{}",
                            stringify!($module),
                            ".",
                            stringify!($name)
                        ));

                        cls.extract()
                            .expect("Imported exception should be a type object")
                    })
                    .as_ptr() as *mut _
            }
        }
    };
}

/// Defines a new exception type.
///
/// # Syntax
///
/// ```create_exception!(module, MyError, BaseException)```
///
/// * `module` is the name of the containing module.
/// * `MyError` is the name of the new exception type.
/// * `BaseException` is the superclass of `MyError`, usually `pyo3::exceptions::PyException`.
///
/// # Example
/// ```
/// use pyo3::prelude::*;
/// use pyo3::create_exception;
/// use pyo3::types::IntoPyDict;
/// use pyo3::exceptions::PyException;
///
/// create_exception!(mymodule, CustomError, PyException);
///
/// fn main() {
///     let gil = Python::acquire_gil();
///     let py = gil.python();
///     let error_type = py.get_type::<CustomError>();
///     let ctx = [("CustomError", error_type)].into_py_dict(py);
///     let type_description: String = py
///         .eval("str(CustomError)", None, Some(&ctx))
///         .unwrap()
///         .extract()
///         .unwrap();
///     assert_eq!(type_description, "<class 'mymodule.CustomError'>");
///     py.run(
///         "assert CustomError('oops').args == ('oops',)",
///         None,
///         Some(ctx),
///     )
///     .unwrap();
/// }
/// ```
#[macro_export]
macro_rules! create_exception {
    ($module: ident, $name: ident, $base: ty) => {
        #[repr(transparent)]
        #[allow(non_camel_case_types)] // E.g. `socket.herror`
        pub struct $name($crate::PyAny);

        $crate::impl_exception_boilerplate!($name);

        $crate::create_exception_type_object!($module, $name, $base);
    };
}

/// `impl $crate::type_object::PyTypeObject for $name` where `$name` is an
/// exception newly defined in Rust code.
#[macro_export]
macro_rules! create_exception_type_object {
    ($module: ident, $name: ident, $base: ty) => {
        $crate::pyobject_native_type_core!(
            $name,
            $crate::ffi::PyBaseExceptionObject,
            *$name::type_object_raw($crate::Python::assume_gil_acquired()),
            Some(stringify!($module))
        );

        impl $name {
            fn type_object_raw(py: $crate::Python) -> *mut $crate::ffi::PyTypeObject {
                use $crate::once_cell::GILOnceCell;
                use $crate::AsPyPointer;
                static TYPE_OBJECT: GILOnceCell<$crate::Py<$crate::types::PyType>> =
                    GILOnceCell::new();

                TYPE_OBJECT
                    .get_or_init(py, || unsafe {
                        $crate::Py::from_owned_ptr(
                            py,
                            $crate::PyErr::new_type(
                                py,
                                concat!(stringify!($module), ".", stringify!($name)),
                                Some(py.get_type::<$base>()),
                                None,
                            )
                            .as_ptr() as *mut $crate::ffi::PyObject,
                        )
                    })
                    .as_ptr() as *mut _
            }
        }
    };
}

macro_rules! impl_native_exception (
    ($name:ident, $legacy_name:ident, $exc_name:ident, $layout:path) => (
        pub struct $name($crate::PyAny);

        #[deprecated(note = "Exceptions now have a `Py` prefix, e.g. `PyException`, `PyTypeError`")]
        pub type $legacy_name = $crate::Py<$name>;

        $crate::impl_exception_boilerplate!($name);
        $crate::pyobject_native_type_core!($name, $layout, *(ffi::$exc_name as *mut ffi::PyTypeObject), Some("builtins"));
    );
    ($name:ident, $legacy_name:ident, $exc_name:ident) => (
        impl_native_exception!($name, $legacy_name, $exc_name, ffi::PyBaseExceptionObject);
    )
);

impl PySizedLayout<PyBaseException> for ffi::PyBaseExceptionObject {}

impl_native_exception!(PyBaseException, BaseException, PyExc_BaseException);
impl_native_exception!(PyException, Exception, PyExc_Exception);
impl_native_exception!(
    PyStopAsyncIteration,
    StopAsyncIteration,
    PyExc_StopAsyncIteration
);
impl_native_exception!(
    PyStopIteration,
    StopIteration,
    PyExc_StopIteration,
    ffi::PyStopIterationObject
);
impl_native_exception!(PyGeneratorExit, GeneratorExit, PyExc_GeneratorExit);
impl_native_exception!(PyArithmeticError, ArithmeticError, PyExc_ArithmeticError);
impl_native_exception!(PyLookupError, LookupError, PyExc_LookupError);

impl_native_exception!(PyAssertionError, AssertionError, PyExc_AssertionError);
impl_native_exception!(PyAttributeError, AttributeError, PyExc_AttributeError);
impl_native_exception!(PyBufferError, BufferError, PyExc_BufferError);
impl_native_exception!(PyEOFError, EOFError, PyExc_EOFError);
impl_native_exception!(
    PyFloatingPointError,
    FloatingPointError,
    PyExc_FloatingPointError
);
impl_native_exception!(PyOSError, OSError, PyExc_OSError, ffi::PyOSErrorObject);
impl_native_exception!(PyImportError, ImportError, PyExc_ImportError);

impl_native_exception!(
    PyModuleNotFoundError,
    ModuleNotFoundError,
    PyExc_ModuleNotFoundError
);

impl_native_exception!(PyIndexError, IndexError, PyExc_IndexError);
impl_native_exception!(PyKeyError, KeyError, PyExc_KeyError);
impl_native_exception!(
    PyKeyboardInterrupt,
    KeyboardInterrupt,
    PyExc_KeyboardInterrupt
);
impl_native_exception!(PyMemoryError, MemoryError, PyExc_MemoryError);
impl_native_exception!(PyNameError, NameError, PyExc_NameError);
impl_native_exception!(PyOverflowError, OverflowError, PyExc_OverflowError);
impl_native_exception!(PyRuntimeError, RuntimeError, PyExc_RuntimeError);
impl_native_exception!(PyRecursionError, RecursionError, PyExc_RecursionError);
impl_native_exception!(
    PyNotImplementedError,
    NotImplementedError,
    PyExc_NotImplementedError
);
impl_native_exception!(
    PySyntaxError,
    SyntaxError,
    PyExc_SyntaxError,
    ffi::PySyntaxErrorObject
);
impl_native_exception!(PyReferenceError, ReferenceError, PyExc_ReferenceError);
impl_native_exception!(PySystemError, SystemError, PyExc_SystemError);
impl_native_exception!(
    PySystemExit,
    SystemExit,
    PyExc_SystemExit,
    ffi::PySystemExitObject
);
impl_native_exception!(PyTypeError, TypeError, PyExc_TypeError);
impl_native_exception!(
    PyUnboundLocalError,
    UnboundLocalError,
    PyExc_UnboundLocalError
);
impl_native_exception!(
    PyUnicodeError,
    UnicodeError,
    PyExc_UnicodeError,
    ffi::PyUnicodeErrorObject
);
impl_native_exception!(
    PyUnicodeDecodeError,
    UnicodeDecodeError,
    PyExc_UnicodeDecodeError
);
impl_native_exception!(
    PyUnicodeEncodeError,
    UnicodeEncodeError,
    PyExc_UnicodeEncodeError
);
impl_native_exception!(
    PyUnicodeTranslateError,
    UnicodeTranslateError,
    PyExc_UnicodeTranslateError
);
impl_native_exception!(PyValueError, ValueError, PyExc_ValueError);
impl_native_exception!(
    PyZeroDivisionError,
    ZeroDivisionError,
    PyExc_ZeroDivisionError
);

impl_native_exception!(PyBlockingIOError, BlockingIOError, PyExc_BlockingIOError);
impl_native_exception!(PyBrokenPipeError, BrokenPipeError, PyExc_BrokenPipeError);
impl_native_exception!(
    PyChildProcessError,
    ChildProcessError,
    PyExc_ChildProcessError
);
impl_native_exception!(PyConnectionError, ConnectionError, PyExc_ConnectionError);
impl_native_exception!(
    PyConnectionAbortedError,
    ConnectionAbortedError,
    PyExc_ConnectionAbortedError
);
impl_native_exception!(
    PyConnectionRefusedError,
    ConnectionRefusedError,
    PyExc_ConnectionRefusedError
);
impl_native_exception!(
    PyConnectionResetError,
    ConnectionResetError,
    PyExc_ConnectionResetError
);
impl_native_exception!(PyFileExistsError, FileExistsError, PyExc_FileExistsError);
impl_native_exception!(
    PyFileNotFoundError,
    FileNotFoundError,
    PyExc_FileNotFoundError
);
impl_native_exception!(PyInterruptedError, InterruptedError, PyExc_InterruptedError);
impl_native_exception!(
    PyIsADirectoryError,
    IsADirectoryError,
    PyExc_IsADirectoryError
);
impl_native_exception!(
    PyNotADirectoryError,
    NotADirectoryError,
    PyExc_NotADirectoryError
);
impl_native_exception!(PyPermissionError, PermissionError, PyExc_PermissionError);
impl_native_exception!(
    PyProcessLookupError,
    ProcessLookupError,
    PyExc_ProcessLookupError
);
impl_native_exception!(PyTimeoutError, TimeoutError, PyExc_TimeoutError);

#[cfg(not(all(windows, PyPy)))]
impl_native_exception!(PyEnvironmentError, EnvironmentError, PyExc_EnvironmentError);
#[cfg(not(all(windows, PyPy)))]
impl_native_exception!(PyIOError, IOError, PyExc_IOError);
#[cfg(all(windows, not(PyPy)))]
impl_native_exception!(PyWindowsError, WindowsError, PyExc_WindowsError);

impl PyUnicodeDecodeError {
    pub fn new<'p>(
        py: Python<'p>,
        encoding: &CStr,
        input: &[u8],
        range: ops::Range<usize>,
        reason: &CStr,
    ) -> PyResult<&'p PyUnicodeDecodeError> {
        unsafe {
            py.from_owned_ptr_or_err(ffi::PyUnicodeDecodeError_Create(
                encoding.as_ptr(),
                input.as_ptr() as *const c_char,
                input.len() as ffi::Py_ssize_t,
                range.start as ffi::Py_ssize_t,
                range.end as ffi::Py_ssize_t,
                reason.as_ptr(),
            ))
        }
    }

    pub fn new_utf8<'p>(
        py: Python<'p>,
        input: &[u8],
        err: std::str::Utf8Error,
    ) -> PyResult<&'p PyUnicodeDecodeError> {
        let pos = err.valid_up_to();
        PyUnicodeDecodeError::new(
            py,
            CStr::from_bytes_with_nul(b"utf-8\0").unwrap(),
            input,
            pos..(pos + 1),
            CStr::from_bytes_with_nul(b"invalid utf-8\0").unwrap(),
        )
    }
}

/// Exceptions defined in `asyncio` module
pub mod asyncio {
    import_exception!(asyncio, CancelledError);
    import_exception!(asyncio, InvalidStateError);
    import_exception!(asyncio, TimeoutError);
    import_exception!(asyncio, IncompleteReadError);
    import_exception!(asyncio, LimitOverrunError);
    import_exception!(asyncio, QueueEmpty);
    import_exception!(asyncio, QueueFull);
}

/// Exceptions defined in `socket` module
pub mod socket {
    import_exception!(socket, herror);
    import_exception!(socket, gaierror);
    import_exception!(socket, timeout);
}

#[cfg(test)]
mod test {
    use super::{PyException, PyUnicodeDecodeError};
    use crate::types::{IntoPyDict, PyDict};
    use crate::{PyErr, Python};

    import_exception!(socket, gaierror);
    import_exception!(email.errors, MessageError);

    #[test]
    fn test_check_exception() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let err: PyErr = gaierror::new_err(());
        let socket = py
            .import("socket")
            .map_err(|e| e.print(py))
            .expect("could not import socket");

        let d = PyDict::new(py);
        d.set_item("socket", socket)
            .map_err(|e| e.print(py))
            .expect("could not setitem");

        d.set_item("exc", err)
            .map_err(|e| e.print(py))
            .expect("could not setitem");

        py.run("assert isinstance(exc, socket.gaierror)", None, Some(d))
            .map_err(|e| e.print(py))
            .expect("assertion failed");
    }

    #[test]
    fn test_check_exception_nested() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let err: PyErr = MessageError::new_err(());
        let email = py
            .import("email")
            .map_err(|e| e.print(py))
            .expect("could not import email");

        let d = PyDict::new(py);
        d.set_item("email", email)
            .map_err(|e| e.print(py))
            .expect("could not setitem");
        d.set_item("exc", err)
            .map_err(|e| e.print(py))
            .expect("could not setitem");

        py.run(
            "assert isinstance(exc, email.errors.MessageError)",
            None,
            Some(d),
        )
        .map_err(|e| e.print(py))
        .expect("assertion failed");
    }

    #[test]
    fn custom_exception() {
        create_exception!(mymodule, CustomError, PyException);

        let gil = Python::acquire_gil();
        let py = gil.python();
        let error_type = py.get_type::<CustomError>();
        let ctx = [("CustomError", error_type)].into_py_dict(py);
        let type_description: String = py
            .eval("str(CustomError)", None, Some(&ctx))
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(type_description, "<class 'mymodule.CustomError'>");
        py.run(
            "assert CustomError('oops').args == ('oops',)",
            None,
            Some(&ctx),
        )
        .unwrap();
    }

    #[test]
    fn native_exception_debug() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let exc = py
            .run("raise Exception('banana')", None, None)
            .expect_err("raising should have given us an error")
            .into_instance(py)
            .into_ref(py);
        assert_eq!(
            format!("{:?}", exc),
            exc.repr().unwrap().extract::<String>().unwrap()
        );
    }

    #[test]
    fn native_exception_display() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let exc = py
            .run("raise Exception('banana')", None, None)
            .expect_err("raising should have given us an error")
            .into_instance(py)
            .into_ref(py);
        assert_eq!(
            exc.to_string(),
            exc.str().unwrap().extract::<String>().unwrap()
        );
    }

    #[test]
    fn native_exception_chain() {
        use std::error::Error;

        let gil = Python::acquire_gil();
        let py = gil.python();
        let exc = py
            .run(
                "raise Exception('banana') from TypeError('peach')",
                None,
                None,
            )
            .expect_err("raising should have given us an error")
            .into_instance(py)
            .into_ref(py);

        if py.version_info() >= (3, 7) {
            assert_eq!(format!("{:?}", exc), "Exception('banana')");
        } else {
            assert_eq!(format!("{:?}", exc), "Exception('banana',)");
        }

        let source = exc.source().expect("cause should exist");

        if py.version_info() >= (3, 7) {
            assert_eq!(format!("{:?}", source), "TypeError('peach')");
        } else {
            assert_eq!(format!("{:?}", source), "TypeError('peach',)");
        }

        let source_source = source.source();
        assert!(source_source.is_none(), "source_source should be None");
    }

    #[test]
    fn unicode_decode_error() {
        let invalid_utf8 = b"fo\xd8o";
        let err = std::str::from_utf8(invalid_utf8).expect_err("should be invalid utf8");
        Python::with_gil(|py| {
            let decode_err = PyUnicodeDecodeError::new_utf8(py, invalid_utf8, err).unwrap();
            assert_eq!(
                format!("{:?}", decode_err),
                "UnicodeDecodeError('utf-8', b'fo\\xd8o', 2, 3, 'invalid utf-8')"
            );

            // Restoring should preserve the same error
            let e: PyErr = decode_err.into();
            e.restore(py);

            assert_eq!(
                PyErr::fetch(py).to_string(),
                "UnicodeDecodeError: \'utf-8\' codec can\'t decode byte 0xd8 in position 2: invalid utf-8"
            );
        });
    }
}
