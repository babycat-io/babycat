# libsamplerate-sys

Rust bindings for [libsamplerate](http://www.mega-nerd.com/SRC/api.html).

Will build libsamplerate from source. Don't forget to initialize git submodules (`git submodule update --init`) or clone with `--recursive`.

To conform with the `-sys` naming scheme, this project does not provide any higher level API.

The bindings have been auto-generated with [bindgen](https://crates.io/crates/bindgen):

    bindgen wrapper.h --no-layout-tests -o src/bindings.rs
