use std::fmt::Display;

use wasm_bindgen::prelude::*;

pub fn throw_js_error<E: Display>(err: E) -> JsValue {
    let err_string: String = err.to_string();
    js_sys::Error::new(&err_string).into()
}
