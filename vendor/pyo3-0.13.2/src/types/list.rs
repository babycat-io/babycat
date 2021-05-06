// Copyright (c) 2017-present PyO3 Project and Contributors
//
// based on Daniel Grunwald's https://github.com/dgrunwald/rust-cpython

use crate::err::{self, PyResult};
use crate::ffi::{self, Py_ssize_t};
use crate::{
    AsPyPointer, IntoPy, IntoPyPointer, PyAny, PyNativeType, PyObject, Python, ToBorrowedObject,
    ToPyObject,
};

/// Represents a Python `list`.
#[repr(transparent)]
pub struct PyList(PyAny);

pyobject_native_var_type!(PyList, ffi::PyList_Type, ffi::PyList_Check);

impl PyList {
    /// Constructs a new list with the given elements.
    pub fn new<T, U>(py: Python<'_>, elements: impl IntoIterator<Item = T, IntoIter = U>) -> &PyList
    where
        T: ToPyObject,
        U: ExactSizeIterator<Item = T>,
    {
        let elements_iter = elements.into_iter();
        let len = elements_iter.len();
        unsafe {
            let ptr = ffi::PyList_New(len as Py_ssize_t);
            for (i, e) in elements_iter.enumerate() {
                let obj = e.to_object(py).into_ptr();
                ffi::PyList_SetItem(ptr, i as Py_ssize_t, obj);
            }
            py.from_owned_ptr::<PyList>(ptr)
        }
    }

    /// Constructs a new empty list.
    pub fn empty(py: Python) -> &PyList {
        unsafe { py.from_owned_ptr::<PyList>(ffi::PyList_New(0)) }
    }

    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        // non-negative Py_ssize_t should always fit into Rust usize
        unsafe { ffi::PyList_Size(self.as_ptr()) as usize }
    }

    /// Checks if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the item at the specified index.
    ///
    /// Panics if the index is out of range.
    pub fn get_item(&self, index: isize) -> &PyAny {
        assert!((index.abs() as usize) < self.len());
        unsafe {
            let ptr = ffi::PyList_GetItem(self.as_ptr(), index as Py_ssize_t);

            // PyList_GetItem return borrowed ptr; must make owned for safety (see #890).
            ffi::Py_INCREF(ptr);
            self.py().from_owned_ptr(ptr)
        }
    }

    /// Gets the item at the specified index.
    ///
    /// Panics if the index is out of range.
    pub fn get_parked_item(&self, index: isize) -> PyObject {
        unsafe {
            PyObject::from_borrowed_ptr(
                self.py(),
                ffi::PyList_GetItem(self.as_ptr(), index as Py_ssize_t),
            )
        }
    }

    /// Sets the item at the specified index.
    ///
    /// Panics if the index is out of range.
    pub fn set_item<I>(&self, index: isize, item: I) -> PyResult<()>
    where
        I: ToPyObject,
    {
        unsafe {
            err::error_on_minusone(
                self.py(),
                ffi::PyList_SetItem(self.as_ptr(), index, item.to_object(self.py()).into_ptr()),
            )
        }
    }

    /// Appends an item to the list.
    pub fn append<I>(&self, item: I) -> PyResult<()>
    where
        I: ToBorrowedObject,
    {
        item.with_borrowed_ptr(self.py(), |item| unsafe {
            err::error_on_minusone(self.py(), ffi::PyList_Append(self.as_ptr(), item))
        })
    }

    /// Inserts an item at the specified index.
    ///
    /// Panics if the index is out of range.
    pub fn insert<I>(&self, index: isize, item: I) -> PyResult<()>
    where
        I: ToBorrowedObject,
    {
        item.with_borrowed_ptr(self.py(), |item| unsafe {
            err::error_on_minusone(self.py(), ffi::PyList_Insert(self.as_ptr(), index, item))
        })
    }

    /// Returns an iterator over this list's items.
    pub fn iter(&self) -> PyListIterator {
        PyListIterator {
            list: self,
            index: 0,
        }
    }

    /// Sorts the list in-place. Equivalent to the Python expression `l.sort()`.
    pub fn sort(&self) -> PyResult<()> {
        unsafe { err::error_on_minusone(self.py(), ffi::PyList_Sort(self.as_ptr())) }
    }

    /// Reverses the list in-place. Equivalent to the Python expression `l.reverse()`.
    pub fn reverse(&self) -> PyResult<()> {
        unsafe { err::error_on_minusone(self.py(), ffi::PyList_Reverse(self.as_ptr())) }
    }
}

/// Used by `PyList::iter()`.
pub struct PyListIterator<'a> {
    list: &'a PyList,
    index: isize,
}

impl<'a> Iterator for PyListIterator<'a> {
    type Item = &'a PyAny;

    #[inline]
    fn next(&mut self) -> Option<&'a PyAny> {
        if self.index < self.list.len() as isize {
            let item = self.list.get_item(self.index);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a> std::iter::IntoIterator for &'a PyList {
    type Item = &'a PyAny;
    type IntoIter = PyListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> ToPyObject for [T]
where
    T: ToPyObject,
{
    fn to_object(&self, py: Python<'_>) -> PyObject {
        unsafe {
            let ptr = ffi::PyList_New(self.len() as Py_ssize_t);
            for (i, e) in self.iter().enumerate() {
                let obj = e.to_object(py).into_ptr();
                ffi::PyList_SetItem(ptr, i as Py_ssize_t, obj);
            }
            PyObject::from_owned_ptr(py, ptr)
        }
    }
}

macro_rules! array_impls {
    ($($N:expr),+) => {
        $(
            impl<T> IntoPy<PyObject> for [T; $N]
            where
                T: ToPyObject
            {
                fn into_py(self, py: Python) -> PyObject {
                    self.as_ref().to_object(py)
                }
            }
        )+
    }
}

array_impls!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32
);

impl<T> ToPyObject for Vec<T>
where
    T: ToPyObject,
{
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.as_slice().to_object(py)
    }
}

impl<T> IntoPy<PyObject> for Vec<T>
where
    T: IntoPy<PyObject>,
{
    fn into_py(self, py: Python) -> PyObject {
        unsafe {
            let ptr = ffi::PyList_New(self.len() as Py_ssize_t);
            for (i, e) in self.into_iter().enumerate() {
                let obj = e.into_py(py).into_ptr();
                ffi::PyList_SetItem(ptr, i as Py_ssize_t, obj);
            }
            PyObject::from_owned_ptr(py, ptr)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::types::PyList;
    use crate::Python;
    use crate::{IntoPy, PyObject, PyTryFrom, ToPyObject};

    #[test]
    fn test_new() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![2, 3, 5, 7];
        let list = PyList::new(py, &v);
        assert_eq!(2, list.get_item(0).extract::<i32>().unwrap());
        assert_eq!(3, list.get_item(1).extract::<i32>().unwrap());
        assert_eq!(5, list.get_item(2).extract::<i32>().unwrap());
        assert_eq!(7, list.get_item(3).extract::<i32>().unwrap());
    }

    #[test]
    fn test_len() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![1, 2, 3, 4];
        let ob = v.to_object(py);
        let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
        assert_eq!(4, list.len());
    }

    #[test]
    fn test_get_item() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![2, 3, 5, 7];
        let ob = v.to_object(py);
        let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
        assert_eq!(2, list.get_item(0).extract::<i32>().unwrap());
        assert_eq!(3, list.get_item(1).extract::<i32>().unwrap());
        assert_eq!(5, list.get_item(2).extract::<i32>().unwrap());
        assert_eq!(7, list.get_item(3).extract::<i32>().unwrap());
    }

    #[test]
    fn test_get_parked_item() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![2, 3, 5, 7];
        let ob = v.to_object(py);
        let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
        assert_eq!(2, list.get_parked_item(0).extract::<i32>(py).unwrap());
        assert_eq!(3, list.get_parked_item(1).extract::<i32>(py).unwrap());
        assert_eq!(5, list.get_parked_item(2).extract::<i32>(py).unwrap());
        assert_eq!(7, list.get_parked_item(3).extract::<i32>(py).unwrap());
    }

    #[test]
    fn test_set_item() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![2, 3, 5, 7];
        let ob = v.to_object(py);
        let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
        let val = 42i32.to_object(py);
        assert_eq!(2, list.get_item(0).extract::<i32>().unwrap());
        list.set_item(0, val).unwrap();
        assert_eq!(42, list.get_item(0).extract::<i32>().unwrap());
    }

    #[test]
    fn test_set_item_refcnt() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let cnt;
        {
            let _pool = unsafe { crate::GILPool::new() };
            let v = vec![2];
            let ob = v.to_object(py);
            let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
            let none = py.None();
            cnt = none.get_refcnt(py);
            list.set_item(0, none).unwrap();
        }

        assert_eq!(cnt, py.None().get_refcnt(py));
    }

    #[test]
    fn test_insert() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![2, 3, 5, 7];
        let ob = v.to_object(py);
        let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
        let val = 42i32.to_object(py);
        assert_eq!(4, list.len());
        assert_eq!(2, list.get_item(0).extract::<i32>().unwrap());
        list.insert(0, val).unwrap();
        assert_eq!(5, list.len());
        assert_eq!(42, list.get_item(0).extract::<i32>().unwrap());
        assert_eq!(2, list.get_item(1).extract::<i32>().unwrap());
    }

    #[test]
    fn test_insert_refcnt() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let cnt;
        {
            let _pool = unsafe { crate::GILPool::new() };
            let list = PyList::empty(py);
            let none = py.None();
            cnt = none.get_refcnt(py);
            list.insert(0, none).unwrap();
        }

        assert_eq!(cnt, py.None().get_refcnt(py));
    }

    #[test]
    fn test_append() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![2];
        let ob = v.to_object(py);
        let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
        list.append(3).unwrap();
        assert_eq!(2, list.get_item(0).extract::<i32>().unwrap());
        assert_eq!(3, list.get_item(1).extract::<i32>().unwrap());
    }

    #[test]
    fn test_append_refcnt() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let cnt;
        {
            let _pool = unsafe { crate::GILPool::new() };
            let list = PyList::empty(py);
            let none = py.None();
            cnt = none.get_refcnt(py);
            list.append(none).unwrap();
        }
        assert_eq!(cnt, py.None().get_refcnt(py));
    }

    #[test]
    fn test_iter() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![2, 3, 5, 7];
        let ob = v.to_object(py);
        let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
        let mut idx = 0;
        for el in list.iter() {
            assert_eq!(v[idx], el.extract::<i32>().unwrap());
            idx += 1;
        }
        assert_eq!(idx, v.len());
    }

    #[test]
    fn test_into_iter() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![1, 2, 3, 4];
        let ob = v.to_object(py);
        let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
        for (i, item) in list.iter().enumerate() {
            assert_eq!((i + 1) as i32, item.extract::<i32>().unwrap());
        }
    }

    #[test]
    fn test_extract() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![2, 3, 5, 7];
        let ob = v.to_object(py);
        let list = <PyList as PyTryFrom>::try_from(ob.as_ref(py)).unwrap();
        let v2 = list.as_ref().extract::<Vec<i32>>().unwrap();
        assert_eq!(v, v2);
    }

    #[test]
    fn test_sort() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![7, 3, 2, 5];
        let list = PyList::new(py, &v);
        assert_eq!(7, list.get_item(0).extract::<i32>().unwrap());
        assert_eq!(3, list.get_item(1).extract::<i32>().unwrap());
        assert_eq!(2, list.get_item(2).extract::<i32>().unwrap());
        assert_eq!(5, list.get_item(3).extract::<i32>().unwrap());
        list.sort().unwrap();
        assert_eq!(2, list.get_item(0).extract::<i32>().unwrap());
        assert_eq!(3, list.get_item(1).extract::<i32>().unwrap());
        assert_eq!(5, list.get_item(2).extract::<i32>().unwrap());
        assert_eq!(7, list.get_item(3).extract::<i32>().unwrap());
    }

    #[test]
    fn test_reverse() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec![2, 3, 5, 7];
        let list = PyList::new(py, &v);
        assert_eq!(2, list.get_item(0).extract::<i32>().unwrap());
        assert_eq!(3, list.get_item(1).extract::<i32>().unwrap());
        assert_eq!(5, list.get_item(2).extract::<i32>().unwrap());
        assert_eq!(7, list.get_item(3).extract::<i32>().unwrap());
        list.reverse().unwrap();
        assert_eq!(7, list.get_item(0).extract::<i32>().unwrap());
        assert_eq!(5, list.get_item(1).extract::<i32>().unwrap());
        assert_eq!(3, list.get_item(2).extract::<i32>().unwrap());
        assert_eq!(2, list.get_item(3).extract::<i32>().unwrap());
    }

    #[test]
    fn test_array_into_py() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let array: PyObject = [1, 2].into_py(py);
        let list = <PyList as PyTryFrom>::try_from(array.as_ref(py)).unwrap();
        assert_eq!(1, list.get_item(0).extract::<i32>().unwrap());
        assert_eq!(2, list.get_item(1).extract::<i32>().unwrap());
    }
}
