# Migrating from older PyO3 versions

This guide can help you upgrade code through breaking changes from one PyO3 version to the next.
For a detailed list of all changes, see the [CHANGELOG](changelog.md).

## from 0.12.* to 0.13

### Minimum Rust version increased to Rust 1.45

PyO3 `0.13` makes use of new Rust language features stabilised between Rust 1.40 and Rust 1.45. If you are using a Rust compiler older than Rust 1.45, you will need to update your toolchain to be able to continue using PyO3.

### Runtime changes to support the CPython limited API

In PyO3 `0.13` support was added for compiling against the CPython limited API. This had a number of implications for _all_ PyO3 users, described here.

The largest of these is that all types created from PyO3 are what CPython calls "heap" types. The specific implications of this are:

- If you wish to subclass one of these types _from Rust_ you must mark it `#[pyclass(subclass)]`, as you would if you wished to allow subclassing it from Python code.
- Type objects are now mutable - Python code can set attributes on them.
- `__module__` on types without `#[pyclass(module="mymodule")]` no longer returns `builtins`, it now raises `AttributeError`.

## from 0.11.* to 0.12

### `PyErr` has been reworked

In PyO3 `0.12` the `PyErr` type has been re-implemented to be significantly more compatible with
the standard Rust error handling ecosystem. Specificially `PyErr` now implements
`Error + Send + Sync`, which are the standard traits used for error types.

While this has necessitated the removal of a number of APIs, the resulting `PyErr` type should now
be much more easier to work with. The following sections list the changes in detail and how to
migrate to the new APIs.

#### `PyErr::new` and `PyErr::from_type` now require `Send + Sync` for their argument

For most uses no change will be needed. If you are trying to construct `PyErr` from a value that is
not `Send + Sync`, you will need to first create the Python object and then use
`PyErr::from_instance`.

Similarly, any types which implemented `PyErrArguments` will now need to be `Send + Sync`.

#### `PyErr`'s contents are now private

It is no longer possible to access the fields `.ptype`, `.pvalue` and `.ptraceback` of a `PyErr`.
You should instead now use the new methods `PyErr::ptype()`, `PyErr::pvalue()` and `PyErr::ptraceback()`.

#### `PyErrValue` and `PyErr::from_value` have been removed

As these were part the internals of `PyErr` which have been reworked, these APIs no longer exist.

If you used this API, it is recommended to use `PyException::new_err` (see [the section on
Exception types](#exception-types-have-been-reworked)).

#### `Into<PyResult<T>>` for `PyErr` has been removed

This implementation was redundant. Just construct the `Result::Err` variant directly.

Before:
```rust,ignore
let result: PyResult<()> = PyErr::new::<TypeError, _>("error message").into();
```

After (also using the new reworked exception types; see the following section):
```rust
# use pyo3::{PyErr, PyResult, exceptions::PyTypeError};
let result: PyResult<()> = Err(PyTypeError::new_err("error message"));
```

### Exception types have been reworked

Previously exception types were zero-sized marker types purely used to construct `PyErr`. In PyO3
0.12, these types have been replaced with full definitions and are usable in the same way as `PyAny`, `PyDict` etc. This
makes it possible to interact with Python exception objects.

The new types also have names starting with the "Py" prefix. For example, before:

```rust,ignore
let err: PyErr = TypeError::py_err("error message");
```

After:

```rust
# use pyo3::{PyErr, PyResult, Python, type_object::PyTypeObject};
# use pyo3::exceptions::{PyBaseException, PyTypeError};
# Python::with_gil(|py| -> PyResult<()> {
let err: PyErr = PyTypeError::new_err("error message");

// Uses Display for PyErr, new for PyO3 0.12
assert_eq!(err.to_string(), "TypeError: error message");

// Now possible to interact with exception instances, new for PyO3 0.12
let instance: &PyBaseException = err.instance(py);
assert_eq!(instance.getattr("__class__")?, PyTypeError::type_object(py).as_ref());
# Ok(())
# }).unwrap();
```

### `FromPy` has been removed
To simplify the PyO3 conversion traits, the `FromPy` trait has been removed. Previously there were
two ways to define the to-Python conversion for a type:
`FromPy<T> for PyObject` and `IntoPy<PyObject> for T`.

Now there is only one way to define the conversion, `IntoPy`, so downstream crates may need to
adjust accordingly.

Before:
```rust,ignore
# use pyo3::prelude::*;
struct MyPyObjectWrapper(PyObject);

impl FromPy<MyPyObjectWrapper> for PyObject {
    fn from_py(other: MyPyObjectWrapper, _py: Python) -> Self {
        other.0
    }
}
```

After
```rust
# use pyo3::prelude::*;
struct MyPyObjectWrapper(PyObject);

impl IntoPy<PyObject> for MyPyObjectWrapper {
    fn into_py(self, _py: Python) -> PyObject {
        self.0
    }
}
```

Similarly, code which was using the `FromPy` trait can be trivially rewritten to use `IntoPy`.

Before:
```rust,ignore
# use pyo3::prelude::*;
# Python::with_gil(|py| {
let obj = PyObject::from_py(1.234, py);
# })
```

After:
```rust
# use pyo3::prelude::*;
# Python::with_gil(|py| {
let obj: PyObject = 1.234.into_py(py);
# })
```

### `PyObject` is now a type alias of `Py<PyAny>`
This should change very little from a usage perspective. If you implemented traits for both
`PyObject` and `Py<T>`, you may find you can just remove the `PyObject` implementation.

### `AsPyRef` has been removed
As `PyObject` has been changed to be just a type alias, the only remaining implementor of `AsPyRef`
was `Py<T>`. This removed the need for a trait, so the `AsPyRef::as_ref` method has been moved to
`Py::as_ref`.

This should require no code changes except removing `use pyo3::AsPyRef` for code which did not use
`pyo3::prelude::*`.

Before:
```rust,ignore
use pyo3::{AsPyRef, Py, types::PyList};
# pyo3::Python::with_gil(|py| {
let list_py: Py<PyList> = PyList::empty(py).into();
let list_ref: &PyList = list_py.as_ref(py);
# })
```

After:
```rust
use pyo3::{Py, types::PyList};
# pyo3::Python::with_gil(|py| {
let list_py: Py<PyList> = PyList::empty(py).into();
let list_ref: &PyList = list_py.as_ref(py);
# })
```

## from 0.10.* to 0.11

### Stable Rust
PyO3 now supports the stable Rust toolchain. The minimum required version is 1.39.0.

### `#[pyclass]` structs must now be `Send` or `unsendable`
Because `#[pyclass]` structs can be sent between threads by the Python interpreter, they must implement
`Send` or declared as `unsendable` (by `#[pyclass(unsendable)]`).
Note that `unsendable` is added in PyO3 `0.11.1` and `Send` is always required in PyO3 `0.11.0`.

This may "break" some code which previously was accepted, even though it could be unsound.
There can be two fixes:

1. If you think that your `#[pyclass]` actually must be `Send`able, then let's implement `Send`.
   A common, safer way is using thread-safe types. E.g., `Arc` instead of `Rc`, `Mutex` instead of
   `RefCell`, and `Box<dyn Send + T>` instead of `Box<dyn T>`.

   Before:
   ```rust,compile_fail
   use pyo3::prelude::*;
   use std::rc::Rc;
   use std::cell::RefCell;

   #[pyclass]
   struct NotThreadSafe {
       shared_bools: Rc<RefCell<Vec<bool>>>,
       closure: Box<dyn Fn()>
   }
   ```

   After:
   ```rust
   use pyo3::prelude::*;
   use std::sync::{Arc, Mutex};

   #[pyclass]
   struct ThreadSafe {
       shared_bools: Arc<Mutex<Vec<bool>>>,
       closure: Box<dyn Fn() + Send>
   }
   ```

   In situations where you cannot change your `#[pyclass]` to automatically implement `Send`
   (e.g., when it contains a raw pointer), you can use `unsafe impl Send`.
   In such cases, care should be taken to ensure the struct is actually thread safe.
   See [the Rustnomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html) for more.

2. If you think that your `#[pyclass]` should not be accessed by another thread, you can use
   `unsendable` flag. A class marked with `unsendable` panics when accessed by another thread,
   making it thread-safe to expose an unsendable object to the Python interpreter.

   Before:
   ```rust,compile_fail
   use pyo3::prelude::*;

   #[pyclass]
   struct Unsendable {
       pointers: Vec<*mut std::os::raw::c_char>,
   }
   ```

   After:
   ```rust
   use pyo3::prelude::*;

   #[pyclass(unsendable)]
   struct Unsendable {
       pointers: Vec<*mut std::os::raw::c_char>,
   }
   ```

### All `PyObject` and `Py<T>` methods now take `Python` as an argument
Previously, a few methods such as `Object::get_refcnt` did not take `Python` as an argument (to
ensure that the Python GIL was held by the current thread). Technically, this was not sound.
To migrate, just pass a `py` argument to any calls to these methods.

Before:
```rust,compile_fail
use pyo3::prelude::*;

let gil = Python::acquire_gil();
let py = gil.python();

py.None().get_refcnt();
```

After:
```rust
use pyo3::prelude::*;

let gil = Python::acquire_gil();
let py = gil.python();

py.None().get_refcnt(py);
```

## from 0.9.* to 0.10

### `ObjectProtocol` is removed
All methods are moved to [`PyAny`].
And since now all native types (e.g., `PyList`) implements `Deref<Target=PyAny>`,
all you need to do is remove `ObjectProtocol` from your code.
Or if you use `ObjectProtocol` by `use pyo3::prelude::*`, you have to do nothing.

Before:
```rust,compile_fail
use pyo3::ObjectProtocol;

let gil = pyo3::Python::acquire_gil();
let obj = gil.python().eval("lambda: 'Hi :)'", None, None).unwrap();
let hi: &pyo3::types::PyString = obj.call0().unwrap().downcast().unwrap();
assert_eq!(hi.len().unwrap(), 5);
```

After:
```rust
let gil = pyo3::Python::acquire_gil();
let obj = gil.python().eval("lambda: 'Hi :)'", None, None).unwrap();
let hi: &pyo3::types::PyString = obj.call0().unwrap().downcast().unwrap();
assert_eq!(hi.len().unwrap(), 5);
```

### No `#![feature(specialization)]` in user code
While PyO3 itself still requires specialization and nightly Rust,
now you don't have to use `#![feature(specialization)]` in your crate.

## from 0.8.* to 0.9

### `#[new]` interface
[`PyRawObject`](https://docs.rs/pyo3/0.8.5/pyo3/type_object/struct.PyRawObject.html)
is now removed and our syntax for constructors has changed.

Before:
```rust,compile_fail
#[pyclass]
struct MyClass {}

#[pymethods]
impl MyClass {
   #[new]
   fn new(obj: &PyRawObject) {
       obj.init(MyClass { })
   }
}
```

After:
```rust
# use pyo3::prelude::*;
#[pyclass]
struct MyClass {}

#[pymethods]
impl MyClass {
   #[new]
   fn new() -> Self {
       MyClass {}
   }
}
```

Basically you can return `Self` or `Result<Self>` directly.
For more, see [the constructor section](class.html#constructor) of this guide.

### PyCell
PyO3 0.9 introduces [`PyCell`], which is a [`RefCell`]-like object wrapper
for ensuring Rust's rules regarding aliasing of references are upheld.
For more detail, see the
[Rust Book's section on Rust's rules of references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references)

For `#[pymethods]` or `#[pyfunction]`s, your existing code should continue to work without any change.
Python exceptions will automatically be raised when your functions are used in a way which breaks Rust's
rules of references.

Here is an example.
```rust
# use pyo3::prelude::*;
#[pyclass]
struct Names {
    names: Vec<String>
}

#[pymethods]
impl Names {
    #[new]
    fn new() -> Self {
        Names { names: vec![] }
    }
    fn merge(&mut self, other: &mut Names) {
        self.names.append(&mut other.names)
    }
}
# let gil = Python::acquire_gil();
# let py = gil.python();
# let names = PyCell::new(py, Names::new()).unwrap();
# pyo3::py_run!(py, names, r"
# try:
#    names.merge(names)
#    assert False, 'Unreachable'
# except RuntimeError as e:
#    assert str(e) == 'Already borrowed'
# ");
```
`Names` has a `merge` method, which takes `&mut self` and another argument of type `&mut Self`.
Given this `#[pyclass]`, calling `names.merge(names)` in Python raises
a [`PyBorrowMutError`] exception, since it requires two mutable borrows of `names`.

However, for `#[pyproto]` and some functions, you need to manually fix the code.

#### Object creation
In 0.8 object creation was done with `PyRef::new` and `PyRefMut::new`.
In 0.9 these have both been removed.
To upgrade code, please use
[`PyCell::new`](https://docs.rs/pyo3/latest/pyo3/pycell/struct.PyCell.html#method.new) instead.
If you need [`PyRef`] or [`PyRefMut`], just call `.borrow()` or `.borrow_mut()`
on the newly-created `PyCell`.

Before:
```rust,compile_fail
# use pyo3::prelude::*;
# #[pyclass]
# struct MyClass {}
let gil = Python::acquire_gil();
let py = gil.python();
let obj_ref = PyRef::new(py, MyClass {}).unwrap();
```

After:
```rust
# use pyo3::prelude::*;
# #[pyclass]
# struct MyClass {}
let gil = Python::acquire_gil();
let py = gil.python();
let obj = PyCell::new(py, MyClass {}).unwrap();
let obj_ref = obj.borrow();
```

#### Object extraction
For `PyClass` types `T`, `&T` and `&mut T` no longer have [`FromPyObject`] implementations.
Instead you should extract `PyRef<T>` or `PyRefMut<T>`, respectively.
If `T` implements `Clone`, you can extract `T` itself.
In addition, you can also extract `&PyCell<T>`, though you rarely need it.

Before:
```ignore
let obj: &PyAny = create_obj();
let obj_ref: &MyClass = obj.extract().unwrap();
let obj_ref_mut: &mut MyClass = obj.extract().unwrap();
```

After:
```rust
# use pyo3::prelude::*;
# use pyo3::types::IntoPyDict;
# #[pyclass] #[derive(Clone)] struct MyClass {}
# #[pymethods] impl MyClass { #[new]fn new() -> Self { MyClass {} }}
# let gil = Python::acquire_gil();
# let py = gil.python();
# let typeobj = py.get_type::<MyClass>();
# let d = [("c", typeobj)].into_py_dict(py);
# let create_obj = || py.eval("c()", None, Some(d)).unwrap();
let obj: &PyAny = create_obj();
let obj_cell: &PyCell<MyClass> = obj.extract().unwrap();
let obj_cloned: MyClass = obj.extract().unwrap(); // extracted by cloning the object
{
    let obj_ref: PyRef<MyClass> = obj.extract().unwrap();
    // we need to drop obj_ref before we can extract a PyRefMut due to Rust's rules of references
}
let obj_ref_mut: PyRefMut<MyClass> = obj.extract().unwrap();
```


#### `#[pyproto]`
Most of the arguments to methods in `#[pyproto]` impls require a
[`FromPyObject`] implementation.
So if your protocol methods take `&T` or `&mut T` (where `T: PyClass`),
please use [`PyRef`] or [`PyRefMut`] instead.

Before:
```rust,compile_fail
# use pyo3::prelude::*;
# use pyo3::class::PySequenceProtocol;
#[pyclass]
struct ByteSequence {
    elements: Vec<u8>,
}
#[pyproto]
impl PySequenceProtocol for ByteSequence {
    fn __concat__(&self, other: &Self) -> PyResult<Self> {
        let mut elements = self.elements.clone();
        elements.extend_from_slice(&other.elements);
        Ok(Self { elements })
    }
}
```

After:
```rust
# use pyo3::prelude::*;
# use pyo3::class::PySequenceProtocol;
#[pyclass]
struct ByteSequence {
    elements: Vec<u8>,
}
#[pyproto]
impl PySequenceProtocol for ByteSequence {
    fn __concat__(&self, other: PyRef<'p, Self>) -> PyResult<Self> {
        let mut elements = self.elements.clone();
        elements.extend_from_slice(&other.elements);
        Ok(Self { elements })
    }
}
```

[`FromPyObject`]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.FromPyObject.html
[`PyAny`]: https://docs.rs/pyo3/latest/pyo3/types/struct.PyAny.html
[`PyCell`]: https://docs.rs/pyo3/latest/pyo3/pycell/struct.PyCell.html
[`PyBorrowMutError`]: https://docs.rs/pyo3/latest/pyo3/pycell/struct.PyBorrowMutError.html
[`PyRef`]: https://docs.rs/pyo3/latest/pyo3/pycell/struct.PyRef.html
[`PyRefMut`]: https://docs.rs/pyo3/latest/pyo3/pycell/struct.PyRef.html

[`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
