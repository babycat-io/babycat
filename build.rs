extern crate cbindgen;

use std::env;

fn build_cbindgen_header() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    match cbindgen::generate(crate_dir) {
        Ok(codegen) => {
            codegen.write_to_file("babycat.h");
        }
        Err(err) => {
            panic!(err.to_string());
        }
    };
}

fn main() {
    build_cbindgen_header();
}
