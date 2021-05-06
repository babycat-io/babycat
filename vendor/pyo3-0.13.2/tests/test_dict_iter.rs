use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[test]
fn iter_dict_nosegv() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    const LEN: usize = 10_000_000;
    let dict = (0..LEN as u64).map(|i| (i, i * 2)).into_py_dict(py);
    let mut sum = 0;
    for (k, _v) in dict.iter() {
        let i: u64 = k.extract().unwrap();
        sum += i;
    }
    assert_eq!(sum, 49_999_995_000_000);
}
