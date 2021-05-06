# PyO3

[![Actions Status](https://github.com/PyO3/pyo3/workflows/Test/badge.svg)](https://github.com/PyO3/pyo3/actions)
[![codecov](https://codecov.io/gh/PyO3/pyo3/branch/master/graph/badge.svg)](https://codecov.io/gh/PyO3/pyo3)
[![crates.io](http://meritbadge.herokuapp.com/pyo3)](https://crates.io/crates/pyo3)
[![minimum rustc 1.41](https://img.shields.io/badge/rustc-1.41+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![Join the dev chat](https://img.shields.io/gitter/room/nwjs/nw.js.svg)](https://gitter.im/PyO3/Lobby)

[Rust](http://www.rust-lang.org/) bindings for [Python](https://www.python.org/). This includes running and interacting with Python code from a Rust binary, as well as writing native Python modules.

* User Guide: [stable](https://pyo3.rs) | [master](https://pyo3.rs/master)

* API Documentation: [stable](https://docs.rs/pyo3/) |  [master](https://pyo3.rs/master/doc)

* Contributing Notes: [github](https://github.com/PyO3/pyo3/blob/master/Contributing.md)

A comparison with rust-cpython can be found [in the guide](https://pyo3.rs/master/rust_cpython.html).

## Usage

PyO3 supports Python 3.6 and up. The minimum required Rust version is 1.41.

Building with PyPy is also possible (via cpyext) for Python 3.6, targeted PyPy version is 7.3+.
Please refer to the [pypy section in the guide](https://pyo3.rs/master/building_and_distribution/pypy.html).

You can either write a native Python module in Rust, or use Python from a Rust binary.

However, on some OSs, you need some additional packages. E.g. if you are on *Ubuntu 18.04*, please run

```bash
sudo apt install python3-dev python-dev
```

## Using Rust from Python

PyO3 can be used to generate a native Python module.

**`Cargo.toml`**

```toml
[package]
name = "string-sum"
version = "0.1.0"
edition = "2018"

[lib]
name = "string_sum"
# "cdylib" is necessary to produce a shared library for Python to import from.
#
# Downstream Rust code (including code in `bin/`, `examples/`, and `tests/`) will not be able
# to `use string_sum;` unless the "rlib" or "lib" crate type is also included, e.g.:
# crate-type = ["cdylib", "rlib"]
crate-type = ["cdylib"]

[dependencies.pyo3]
version = "0.13.2"
features = ["extension-module"]
```

**`src/lib.rs`**

```rust
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}
```

On Windows and Linux, you can build normally with `cargo build --release`. On macOS, you need to set additional linker arguments. One option is to compile with `cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup`, the other is to create a `.cargo/config` with the following content:

```toml
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

While developing, you can symlink (or copy) and rename the shared library from the target folder: On MacOS, rename `libstring_sum.dylib` to `string_sum.so`, on Windows `libstring_sum.dll` to `string_sum.pyd`, and on Linux `libstring_sum.so` to `string_sum.so`. Then open a Python shell in the same folder and you'll be able to `import string_sum`.

To build, test and publish your crate as a Python module, you can use [maturin](https://github.com/PyO3/maturin) or [setuptools-rust](https://github.com/PyO3/setuptools-rust). You can find an example for setuptools-rust in [examples/word-count](https://github.com/PyO3/pyo3/tree/master/examples/word-count), while maturin should work on your crate without any configuration.

## Using Python from Rust

If you want your Rust application to create a Python interpreter internally and
use it to run Python code, add `pyo3` to your `Cargo.toml` like this:

```toml
[dependencies.pyo3]
version = "0.13.2"
features = ["auto-initialize"]
```

Example program displaying the value of `sys.version` and the current user name:

```rust
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> Result<(), ()> {
    Python::with_gil(|py| {
        main_(py).map_err(|e| {
          // We can't display Python exceptions via std::fmt::Display,
          // so print the error here manually.
          e.print_and_set_sys_last_vars(py);
        })
    })
}

fn main_(py: Python) -> PyResult<()> {
    let sys = py.import("sys")?;
    let version: String = sys.get("version")?.extract()?;
    let locals = [("os", py.import("os")?)].into_py_dict(py);
    let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
    let user: String = py.eval(code, None, Some(&locals))?.extract()?;
    println!("Hello {}, I'm Python {}", user, version);
    Ok(())
}
```

Our guide has [a section](https://pyo3.rs/master/python_from_rust.html) with lots of examples
about this topic.

## Tools and libraries
 * [maturin](https://github.com/PyO3/maturin) _Zero configuration build tool for Rust-made Python extensions_.
 * [setuptools-rust](https://github.com/PyO3/setuptools-rust) _Setuptools plugin for Rust support_.
 * [pyo3-built](https://github.com/PyO3/pyo3-built) _Simple macro to expose metadata obtained with the [`built`](https://crates.io/crates/built) crate as a [`PyDict`](https://docs.rs/pyo3/0.12.0/pyo3/types/struct.PyDict.html)_
 * [rust-numpy](https://github.com/PyO3/rust-numpy) _Rust binding of NumPy C-API_
 * [dict-derive](https://github.com/gperinazzo/dict-derive) _Derive FromPyObject to automatically transform Python dicts into Rust structs_
 * [pyo3-log](https://github.com/vorner/pyo3-log) _Bridge from Rust to Python logging_
 * [pythonize](https://github.com/davidhewitt/pythonize) _Serde serializer for converting Rust objects to JSON-compatible Python objects_
 * [pyo3-asyncio](https://github.com/awestlake87/pyo3-asyncio) Utilities for working with Python's Asyncio library and async functions

## Examples

 * [hyperjson](https://github.com/mre/hyperjson) _A hyper-fast Python module for reading/writing JSON data using Rust's serde-json_
 * [html-py-ever](https://github.com/PyO3/setuptools-rust/tree/master/examples/html-py-ever) _Using [html5ever](https://github.com/servo/html5ever) through [kuchiki](https://github.com/kuchiki-rs/kuchiki) to speed up html parsing and css-selecting._
 * [point-process](https://github.com/ManifoldFR/point-process-rust/tree/master/pylib) _High level API for pointprocesses as a Python library_
 * [autopy](https://github.com/autopilot-rs/autopy) _A simple, cross-platform GUI automation library for Python and Rust._
   * Contains an example of building wheels on TravisCI and appveyor using [cibuildwheel](https://github.com/joerick/cibuildwheel)
 * [orjson](https://github.com/ijl/orjson)  _Fast Python JSON library_
 * [inline-python](https://github.com/dronesforwork/inline-python) _Inline Python code directly in your Rust code_
 * [Rogue-Gym](https://github.com/kngwyu/rogue-gym) _Customizable rogue-like game for AI experiments_
   * Contains an example of building wheels on Azure Pipelines
 * [fastuuid](https://github.com/thedrow/fastuuid/) _Python bindings to Rust's UUID library_
 * [wasmer-python](https://github.com/wasmerio/wasmer-python) _Python library to run WebAssembly binaries_
 * [mocpy](https://github.com/cds-astro/mocpy) _Astronomical Python library offering data structures for describing any arbitrary coverage regions on the unit sphere_
 * [tokenizers](https://github.com/huggingface/tokenizers/tree/master/bindings/python) _Python bindings to the Hugging Face tokenizers (NLP) written in Rust_
 * [pyre](https://github.com/Project-Dream-Weaver/Pyre) _Fast Python HTTP server written in Rust_
 * [jsonschema-rs](https://github.com/Stranger6667/jsonschema-rs/tree/master/bindings/python) _Fast JSON Schema validation library_
 * [css-inline](https://github.com/Stranger6667/css-inline/tree/master/bindings/python) _CSS inlining for Python implemented in Rust_
 * [cryptography](https://github.com/pyca/cryptography/tree/master/src/rust) _Python cryptography library with some functionality in Rust_

## License

PyO3 is licensed under the [Apache-2.0 license](http://opensource.org/licenses/APACHE-2.0).
Python is licensed under the [Python License](https://docs.python.org/2/license.html).
