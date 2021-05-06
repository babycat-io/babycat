use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod common;

#[pyfunction]
fn take_str(_s: &str) {}

#[test]
fn test_unicode_encode_error() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let take_str = wrap_pyfunction!(take_str)(py).unwrap();
    py_expect_exception!(
        py,
        take_str,
        "take_str('\\ud800')",
        PyUnicodeEncodeError,
        "'utf-8' codec can't encode character '\\ud800' in position 0: surrogates not allowed"
    );
}
