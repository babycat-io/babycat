// Copyright (c) 2017-present PyO3 Project and Contributors
//! Python type object information

use crate::conversion::IntoPyPointer;
use crate::once_cell::GILOnceCell;
use crate::pyclass::{create_type_object, py_class_attributes, PyClass};
use crate::pyclass_init::PyObjectInit;
use crate::types::{PyAny, PyType};
use crate::{ffi, AsPyPointer, PyErr, PyNativeType, PyObject, PyResult, Python};
use parking_lot::{const_mutex, Mutex};
use std::thread::{self, ThreadId};

/// `T: PyLayout<U>` represents that `T` is a concrete representaion of `U` in Python heap.
/// E.g., `PyCell` is a concrete representaion of all `pyclass`es, and `ffi::PyObject`
/// is of `PyAny`.
///
/// This trait is intended to be used internally.
pub unsafe trait PyLayout<T: PyTypeInfo> {
    const IS_NATIVE_TYPE: bool = true;
    fn get_super(&mut self) -> Option<&mut T::BaseLayout> {
        None
    }
    fn py_init(&mut self, _value: T) {}
    unsafe fn py_drop(&mut self, _py: Python) {}
}

/// `T: PySizedLayout<U>` represents `T` is not a instance of
/// [`PyVarObject`](https://docs.python.org/3.8/c-api/structures.html?highlight=pyvarobject#c.PyVarObject).
/// , in addition that `T` is a concrete representaion of `U`.
pub trait PySizedLayout<T: PyTypeInfo>: PyLayout<T> + Sized {}

/// Marker type indicates that `Self` can be a base layout of `PyClass`.
///
/// # Safety
///
/// Self should be laid out as follows:
/// ```ignore
/// #[repr(C)]
/// struct Self {
///     obj: ffi::PyObject,
///     borrow_flag: u64,
///     ...
/// }
/// ```
/// Otherwise, implementing this trait is undefined behavior.
pub unsafe trait PyBorrowFlagLayout<T: PyTypeInfo>: PyLayout<T> + Sized {}

/// Our custom type flags
#[doc(hidden)]
pub mod type_flags {
    /// Type object supports Python GC
    pub const GC: usize = 1;

    /// Type object supports Python weak references
    pub const WEAKREF: usize = 1 << 1;

    /// Type object can be used as the base type of another type
    pub const BASETYPE: usize = 1 << 2;

    /// The instances of this type have a dictionary containing instance variables
    pub const DICT: usize = 1 << 3;

    /// The class declared by #[pyclass(extends=~)]
    pub const EXTENDED: usize = 1 << 4;
}

/// Python type information.
/// All Python native types(e.g., `PyDict`) and `#[pyclass]` structs implement this trait.
///
/// This trait is marked unsafe because:
///  - specifying the incorrect layout can lead to memory errors
///  - the return value of type_object must always point to the same PyTypeObject instance
pub unsafe trait PyTypeInfo: Sized {
    /// Type of objects to store in PyObject struct
    type Type;

    /// Class name
    const NAME: &'static str;

    /// Module name, if any
    const MODULE: Option<&'static str>;

    /// Class doc string
    const DESCRIPTION: &'static str = "\0";

    /// Type flags (ie PY_TYPE_FLAG_GC, PY_TYPE_FLAG_WEAKREF)
    const FLAGS: usize = 0;

    /// Base class
    type BaseType: PyTypeInfo + PyTypeObject;

    /// Layout
    type Layout: PyLayout<Self>;

    /// Layout of Basetype.
    type BaseLayout: PySizedLayout<Self::BaseType>;

    /// Initializer for layout
    type Initializer: PyObjectInit<Self>;

    /// Utility type to make Py::as_ref work
    type AsRefTarget: crate::PyNativeType;

    /// PyTypeObject instance for this type.
    fn type_object_raw(py: Python) -> *mut ffi::PyTypeObject;

    /// Checks if `object` is an instance of this type or a subclass of this type.
    fn is_type_of(object: &PyAny) -> bool {
        unsafe { ffi::PyObject_TypeCheck(object.as_ptr(), Self::type_object_raw(object.py())) != 0 }
    }

    /// Checks if `object` is an instance of this type.
    fn is_exact_type_of(object: &PyAny) -> bool {
        unsafe { ffi::Py_TYPE(object.as_ptr()) == Self::type_object_raw(object.py()) }
    }
}

/// Python object types that have a corresponding type object.
///
/// This trait is marked unsafe because not fulfilling the contract for type_object
/// leads to UB.
///
/// See also [PyTypeInfo::type_object_raw](trait.PyTypeInfo.html#tymethod.type_object_raw).
pub unsafe trait PyTypeObject {
    /// Returns the safe abstraction over the type object.
    fn type_object(py: Python) -> &PyType;
}

unsafe impl<T> PyTypeObject for T
where
    T: PyTypeInfo,
{
    fn type_object(py: Python) -> &PyType {
        unsafe { py.from_borrowed_ptr(Self::type_object_raw(py) as _) }
    }
}

/// Lazy type object for PyClass
#[doc(hidden)]
pub struct LazyStaticType {
    // Boxed because Python expects the type object to have a stable address.
    value: GILOnceCell<*mut ffi::PyTypeObject>,
    // Threads which have begun initialization of the `tp_dict`. Used for
    // reentrant initialization detection.
    initializing_threads: Mutex<Vec<ThreadId>>,
    tp_dict_filled: GILOnceCell<PyResult<()>>,
}

impl LazyStaticType {
    pub const fn new() -> Self {
        LazyStaticType {
            value: GILOnceCell::new(),
            initializing_threads: const_mutex(Vec::new()),
            tp_dict_filled: GILOnceCell::new(),
        }
    }

    pub fn get_or_init<T: PyClass>(&self, py: Python) -> *mut ffi::PyTypeObject {
        let type_object = *self.value.get_or_init(py, || {
            create_type_object::<T>(py, T::MODULE).unwrap_or_else(|e| {
                e.print(py);
                panic!("An error occurred while initializing class {}", T::NAME)
            })
        });

        // We might want to fill the `tp_dict` with python instances of `T`
        // itself. In order to do so, we must first initialize the type object
        // with an empty `tp_dict`: now we can create instances of `T`.
        //
        // Then we fill the `tp_dict`. Multiple threads may try to fill it at
        // the same time, but only one of them will succeed.
        //
        // More importantly, if a thread is performing initialization of the
        // `tp_dict`, it can still request the type object through `get_or_init`,
        // but the `tp_dict` may appear empty of course.

        if self.tp_dict_filled.get(py).is_some() {
            // `tp_dict` is already filled: ok.
            return type_object;
        }

        {
            let thread_id = thread::current().id();
            let mut threads = self.initializing_threads.lock();
            if threads.contains(&thread_id) {
                // Reentrant call: just return the type object, even if the
                // `tp_dict` is not filled yet.
                return type_object;
            }
            threads.push(thread_id);
        }

        // Pre-compute the class attribute objects: this can temporarily
        // release the GIL since we're calling into arbitrary user code. It
        // means that another thread can continue the initialization in the
        // meantime: at worst, we'll just make a useless computation.
        let mut items = vec![];
        for attr in py_class_attributes::<T>() {
            items.push((attr.name, (attr.meth)(py)));
        }

        // Now we hold the GIL and we can assume it won't be released until we
        // return from the function.
        let result = self.tp_dict_filled.get_or_init(py, move || {
            let result = initialize_tp_dict(py, type_object as *mut ffi::PyObject, items);

            // Initialization successfully complete, can clear the thread list.
            // (No further calls to get_or_init() will try to init, on any thread.)
            *self.initializing_threads.lock() = Vec::new();
            result
        });

        if let Err(err) = result {
            err.clone_ref(py).print(py);
            panic!("An error occured while initializing `{}.__dict__`", T::NAME);
        }

        type_object
    }
}

fn initialize_tp_dict(
    py: Python,
    type_object: *mut ffi::PyObject,
    items: Vec<(&'static std::ffi::CStr, PyObject)>,
) -> PyResult<()> {
    // We hold the GIL: the dictionary update can be considered atomic from
    // the POV of other threads.
    for (key, val) in items {
        let ret = unsafe { ffi::PyObject_SetAttrString(type_object, key.as_ptr(), val.into_ptr()) };
        if ret < 0 {
            return Err(PyErr::fetch(py));
        }
    }
    Ok(())
}

// This is necessary for making static `LazyStaticType`s
unsafe impl Sync for LazyStaticType {}
