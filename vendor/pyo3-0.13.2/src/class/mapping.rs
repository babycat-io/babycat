// Copyright (c) 2017-present PyO3 Project and Contributors

//! Python Mapping Interface
//! Trait and support implementation for implementing mapping support

use crate::callback::IntoPyCallbackOutput;
use crate::{exceptions, ffi, FromPyObject, PyClass, PyObject};

/// Mapping interface
#[allow(unused_variables)]
pub trait PyMappingProtocol<'p>: PyClass {
    fn __len__(&'p self) -> Self::Result
    where
        Self: PyMappingLenProtocol<'p>,
    {
        unimplemented!()
    }

    fn __getitem__(&'p self, key: Self::Key) -> Self::Result
    where
        Self: PyMappingGetItemProtocol<'p>,
    {
        unimplemented!()
    }

    fn __setitem__(&'p mut self, key: Self::Key, value: Self::Value) -> Self::Result
    where
        Self: PyMappingSetItemProtocol<'p>,
    {
        unimplemented!()
    }

    fn __delitem__(&'p mut self, key: Self::Key) -> Self::Result
    where
        Self: PyMappingDelItemProtocol<'p>,
    {
        unimplemented!()
    }

    fn __reversed__(&'p self) -> Self::Result
    where
        Self: PyMappingReversedProtocol<'p>,
    {
        unimplemented!()
    }
}

// The following are a bunch of marker traits used to detect
// the existance of a slotted method.

pub trait PyMappingLenProtocol<'p>: PyMappingProtocol<'p> {
    type Result: IntoPyCallbackOutput<usize>;
}

pub trait PyMappingGetItemProtocol<'p>: PyMappingProtocol<'p> {
    type Key: FromPyObject<'p>;
    type Result: IntoPyCallbackOutput<PyObject>;
}

pub trait PyMappingSetItemProtocol<'p>: PyMappingProtocol<'p> {
    type Key: FromPyObject<'p>;
    type Value: FromPyObject<'p>;
    type Result: IntoPyCallbackOutput<()>;
}

pub trait PyMappingDelItemProtocol<'p>: PyMappingProtocol<'p> {
    type Key: FromPyObject<'p>;
    type Result: IntoPyCallbackOutput<()>;
}

pub trait PyMappingReversedProtocol<'p>: PyMappingProtocol<'p> {
    type Result: IntoPyCallbackOutput<PyObject>;
}

py_len_func!(len, PyMappingLenProtocol, Self::__len__);
py_binary_func!(getitem, PyMappingGetItemProtocol, Self::__getitem__);
py_func_set!(setitem, PyMappingSetItemProtocol, Self::__setitem__);
py_func_del!(delitem, PyMappingDelItemProtocol, Self::__delitem__);
py_func_set_del!(
    setdelitem,
    PyMappingSetItemProtocol,
    PyMappingDelItemProtocol,
    Self,
    __setitem__,
    __delitem__
);
