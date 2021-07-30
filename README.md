![Babycat Logo](https://static.neocrym.com/images/babycat/v1/2x/babycat-body-icon-dark-social-media-cover--2x.png "Babycat Logo")

# Babycat is an audio decoding and manipulation library

## Features

### Decoding, resampling, and encoding
Babycat's core feature set includes:

- decoding MP3, FLAC, and WAV.
- resampling audio to different frame rates.
- encoding waveforms to WAV.

### Bindings for Rust, Python, WebAssembly, and C
Babycat can be used from the following target languages:

- **Rust**. The majority of Babycat is written in Rust, with the exception of a few C dependencies like [libsamplerate][1].
- **Python**. Babycat's Python bindings allow you to decode, resample, and encode audio without being slowed down by Python's Global Interpreter Lock (GIL). Babycat also integrates with Jupyter, allowing you to play and listen to audio streams decoded by Babycat inside of a Jupyter notebook.
- **WebAssembly**. Babycat generates JavaScript/WebAssembly bindings that can run either in a web browser or in Node.js.
- **C**. Babycat exposes a C API, which is useful for both creating audio analysis projects in C or creating Babycat bindings for languages not mentioned above.

### Effective multithreading and parallelism
Babycat is designed to parallelize the decoding of many audio files across multiple CPU cores. Babycat's Python bindings allow for parallel audio decoding without being slowed down by Python's Global Interpreter Lock.

### Open source under the MIT license
The audio ecosystem is full of expensive proprietary software packages, or (L)GPL-licensed code that restricts how you can use it. In contrast, Babycat is licensed under the MIT license, allowing you to use Babycat any way you want for free.

## Babycat is battle-tested in industry
Babycat was built at and is actively maintained by [Neocrym][2], a record label that uses artificial intelligence to find and promote the world's greatest musicians. Neocrym uses Babycat to decode millions of songs as part of audio feature engineering pipelines for machine learning models.

## Learn more

### Source code and issues
You can find Babycat's source code at [github.com/babycat-io/babycat][3].

### API documentation and releases
[**babycat.io**](https://babycat.io) is our main documentation website. You can find documentation and releases for each binding at:

| **Binding**     |  **Documentation**         |  **Releases**                                 |
| --------------- | -------------------------- | --------------------------------------------- |
| **Rust**        | [docs.rs/babycat][4]       | [crates.io/crates/babycat][5]                 |
| **Python**      | [babycat.io/api/python][6] | [pypi.org/project/babycat][7]                 |
| **WebAssembly** | [babycat.io/api/wasm][8]   | [npmjs.com/package/babycat][9]                |
| **C**           | [babycat.io/api/c][10]     | No releases yet. You can compile from source. |

### Tutorials
You can learn more about how to use Babycat from our long-form tutorials:

- [Audio terminology](https://babycat.io/tutorials/terminology/)
- [Installation/development requirements](https://babycat.io/tutorials/development-requirements/)
- [Using Babycat](https://babycat.io/tutorials/using-babycat/)
- [Contributing to Babycat](https://babycat.io/tutorials/contributing/)

## Acknowledgements
The first version of Babycat was an internal project at Neocrym written by [Ritik Mishra][11] Since then, the code has been extended and open-sourced by [James Mishra][12].

Babycat is built on top of *many* high-quality open source packages, including:

- [Symphonia][13] for audio decoding.
- [libsamplerate][1] for high-quality audio resampling.
- [Hound][14] for WAV encoding.
- [PyO3][15] for generating Python bindings.
- [cbindgen][16] for generating C bindings.
- [wasm-bindgen][17] for generating WebAssembly bindings.

Babycat's goal is to provide a simple and consistent API on top of the existing audio ecosystem, without sacrificing performance, portability, or permissive licensing.

[1]: http://www.mega-nerd.com/SRC/index.html
[2]: https://www.neocrym.com
[3]: https://github.com/babycat-io/babycat
[4]: https://docs.rs/babycat
[5]: https://crates.io/crates/babycat
[6]: https://babycat.io/api/python/
[7]: https://pypi.org/project/babycat/
[8]: https://babycat.io/api/wasm
[9]: https://www.npmjs.com/package/babycat
[10]: https://babycat.io/api/c/
[11]: https://www.linkedin.com/in/ritikmishra
[12]: https://www.linkedin.com/in/jamesmishra
[13]: https://github.com/pdeljanov/Symphonia
[14]: https://github.com/ruuda/hound
[15]: https://github.com/PyO3/pyo3
[16]: https://github.com/eqrion/cbindgen
[17]: https://github.com/rustwasm/wasm-bindgen
