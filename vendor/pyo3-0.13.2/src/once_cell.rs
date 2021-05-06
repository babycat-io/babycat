use crate::Python;
use std::cell::UnsafeCell;

/// A write-once cell similar to [`once_cell::OnceCell`](https://docs.rs/once_cell/1.4.0/once_cell/).
///
/// Unlike `once_cell::sync` which blocks threads to achieve thread safety, this implementation
/// uses the Python GIL to mediate concurrent access. This helps in cases where `once_sync` or
/// `lazy_static`'s synchronization strategy can lead to deadlocks when interacting with the Python
/// GIL. For an example, see [the FAQ section](https://pyo3.rs/master/faq.html) of the guide.
///
/// # Example
///
/// The following example shows how to use `GILOnceCell` to share a reference to a Python list
/// between threads:
///
/// ```
/// use pyo3::prelude::*;
/// use pyo3::types::PyList;
/// use pyo3::once_cell::GILOnceCell;
///
/// static LIST_CELL: GILOnceCell<Py<PyList>> = GILOnceCell::new();
///
/// pub fn get_shared_list(py: Python) -> &PyList {
///     LIST_CELL
///         .get_or_init(py, || PyList::empty(py).into())
///         .as_ref(py)
/// }
/// # let gil = Python::acquire_gil();
/// # let py = gil.python();
/// # assert_eq!(get_shared_list(py).len(), 0 );
/// ```
pub struct GILOnceCell<T>(UnsafeCell<Option<T>>);

// T: Send is needed for Sync because the thread which drops the GILOnceCell can be different
// to the thread which fills it.
unsafe impl<T: Send + Sync> Sync for GILOnceCell<T> {}
unsafe impl<T: Send> Send for GILOnceCell<T> {}

impl<T> GILOnceCell<T> {
    /// Create a `GILOnceCell` which does not yet contain a value.
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    /// Get a reference to the contained value, or `None` if the cell has not yet been written.
    pub fn get(&self, _py: Python) -> Option<&T> {
        // Safe because if the cell has not yet been written, None is returned.
        unsafe { &*self.0.get() }.as_ref()
    }

    /// Get a reference to the contained value, initializing it if needed using the provided
    /// closure.
    ///
    /// Note that:
    ///  1) reentrant initialization can cause a stack overflow.
    ///  2) if f() temporarily releases the GIL (e.g. by calling `Python::import`) then it is
    ///     possible (and well-defined) that a second thread may also call get_or_init and begin
    ///     calling `f()`. Even when this happens `GILOnceCell` guarantees that only **one** write
    ///     to the cell ever occurs - other threads will simply discard the value they compute and
    ///     return the result of the first complete computation.
    ///  3) if f() does not release the GIL and does not panic, it is guaranteed to be called
    ///     exactly once, even if multiple threads attempt to call `get_or_init`
    ///  4) if f() can panic but still does not release the GIL, it may be called multiple times,
    ///     but it is guaranteed that f() will never be called concurrently
    pub fn get_or_init<F>(&self, py: Python, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        let inner = unsafe { &*self.0.get() }.as_ref();
        if let Some(value) = inner {
            return value;
        }

        // Note that f() could temporarily release the GIL, so it's possible that another thread
        // writes to this GILOnceCell before f() finishes. That's fine; we'll just have to discard
        // the value computed here and accept a bit of wasted computation.
        let value = f();
        let _ = self.set(py, value);

        self.get(py).unwrap()
    }

    /// Get the contents of the cell mutably. This is only possible if the reference to the cell is
    /// unique.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        // Safe because we have &mut self
        unsafe { &mut *self.0.get() }.as_mut()
    }

    /// Set the value in the cell.
    ///
    /// If the cell has already been written, `Err(value)` will be returned containing the new
    /// value which was not written.
    pub fn set(&self, _py: Python, value: T) -> Result<(), T> {
        // Safe because GIL is held, so no other thread can be writing to this cell concurrently.
        let inner = unsafe { &mut *self.0.get() };
        if inner.is_some() {
            return Err(value);
        }

        *inner = Some(value);
        Ok(())
    }
}
