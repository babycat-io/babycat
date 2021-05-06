use pyo3::prelude::*;
use pyo3::ToPyObject;

#[macro_use]
mod common;

#[pyclass]
#[derive(Clone, Debug, PartialEq)]
struct Cloneable {
    x: i32,
}

#[test]
fn test_cloneable_pyclass() {
    let c = Cloneable { x: 10 };

    let gil = Python::acquire_gil();
    let py = gil.python();

    let py_c = Py::new(py, c.clone()).unwrap().to_object(py);

    let c2: Cloneable = py_c.extract(py).unwrap();
    assert_eq!(c, c2);
    {
        let rc: PyRef<Cloneable> = py_c.extract(py).unwrap();
        assert_eq!(&c, &*rc);
        // Drops PyRef before taking PyRefMut
    }
    let mrc: PyRefMut<Cloneable> = py_c.extract(py).unwrap();
    assert_eq!(&c, &*mrc);
}

#[pyclass(subclass)]
#[derive(Default)]
struct BaseClass {
    value: i32,
}

#[pymethods]
impl BaseClass {
    fn foo(&self) -> &'static str {
        "BaseClass"
    }
}

#[pyclass(extends=BaseClass)]
struct SubClass {}

#[pymethods]
impl SubClass {
    fn foo(&self) -> &'static str {
        "SubClass"
    }
}

#[pyclass]
struct PolymorphicContainer {
    #[pyo3(get, set)]
    inner: Py<BaseClass>,
}

#[test]
fn test_polymorphic_container_stores_base_class() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let p = PyCell::new(
        py,
        PolymorphicContainer {
            inner: Py::new(py, BaseClass::default()).unwrap(),
        },
    )
    .unwrap()
    .to_object(py);

    py_assert!(py, p, "p.inner.foo() == 'BaseClass'");
}

#[test]
fn test_polymorphic_container_stores_sub_class() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let p = PyCell::new(
        py,
        PolymorphicContainer {
            inner: Py::new(py, BaseClass::default()).unwrap(),
        },
    )
    .unwrap()
    .to_object(py);

    p.as_ref(py)
        .setattr(
            "inner",
            PyCell::new(
                py,
                PyClassInitializer::from(BaseClass::default()).add_subclass(SubClass {}),
            )
            .unwrap(),
        )
        .unwrap();

    py_assert!(py, p, "p.inner.foo() == 'SubClass'");
}

#[test]
fn test_polymorphic_container_does_not_accept_other_types() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let p = PyCell::new(
        py,
        PolymorphicContainer {
            inner: Py::new(py, BaseClass::default()).unwrap(),
        },
    )
    .unwrap()
    .to_object(py);

    let setattr = |value: PyObject| p.as_ref(py).setattr("inner", value);

    assert!(setattr(1i32.into_py(py)).is_err());
    assert!(setattr(py.None()).is_err());
    assert!(setattr((1i32, 2i32).into_py(py)).is_err());
}

#[test]
fn test_pyref_as_base() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let cell = PyCell::new(py, (SubClass {}, BaseClass { value: 120 })).unwrap();

    // First try PyRefMut
    let sub: PyRefMut<SubClass> = cell.borrow_mut();
    let mut base: PyRefMut<BaseClass> = sub.into_super();
    assert_eq!(120, base.value);
    base.value = 999;
    assert_eq!(999, base.value);
    drop(base);

    // Repeat for PyRef
    let sub: PyRef<SubClass> = cell.borrow();
    let base: PyRef<BaseClass> = sub.into_super();
    assert_eq!(999, base.value);
}

#[test]
fn test_pycell_deref() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let cell = PyCell::new(py, (SubClass {}, BaseClass { value: 120 })).unwrap();

    // Should be able to deref as PyAny
    assert_eq!(
        cell.call_method0("foo")
            .and_then(PyAny::extract::<&str>)
            .unwrap(),
        "SubClass"
    );
}
