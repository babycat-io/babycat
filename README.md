![Babycat Logo](https://static.neocrym.com/images/babycat/v1/2x/babycat-body-icon-dark-social-media-cover--2x.png "Babycat Logo")

# Babycat is an audio decoding and manipulation library
[![Rustdoc status](https://docs.rs/babycat/badge.svg)](https://docs.rs/crate/babycat)
[![crates.io status](https://img.shields.io/crates/v/babycat.svg)](https://crates.io/crates/babycat)
[![Rust dependency status](https://deps.rs/repo/github/babycat-io/babycat/status.svg)](https://deps.rs/repo/github/babycat-io/babycat)

**Babycat is a library that makes it easy to decode and manipulate audio**.

**Babycat is written in Rust and has bindings for Python, C, and WebAssembly.**

## Examples

### Ex #1: Decoding a single audio file into memory

**Rust**
```rust
use babycat::{Signal, Waveform};

let filename = "audio-for-tests/circus-of-freaks/track.flac"
let waveform = Waveform::from_file(filename, Default::default()).unwrap();

println!(
    "Decoded {} frames with {} channels at {} hz",
    waveform.num_frames(),
    waveform.num_channels(),
    waveform.frame_rate_hz(),
);

```
**Python**
```python3
from babycat import Waveform

filename = "audio-for-tests/circus-of-freaks/track.flac"
waveform = Waveform.from_file(filename)

print(
    f"Decoded {waveform.num_frames} frames with "
    f"{waveform.num_channels} channels at "
    f"{waveform.frame_rate_hz} hz"
)
```

## Licensing

Babycat is licensed under the [MIT license][21].

However, Babycat comes with optional support for statically linking other libraries--notably FFmpeg. If you compile Babycat with FFmpeg support, the resulting binaries and derivative works are also subject to [FFmpeg's license][18]. Typically, this license is [LGPL 2.1][19] ["or later"][20], but it is also possible to compile and Link FFmpeg with other libraries that are licensed under the GPL or under a proprietary license.

## Features

### Audio demuxing and decoding

### Resampling

Babycat supports high-quality audio resampling with [libsamplerate][1], except

Babycat's Rust, Python, and C bindins support high quality audio resampling with [libsamplerate][1].

Currently, libsamplerate is not supported by Babycat's WebAssembly bindings, but Babycat's own pure-Rust sinc resampler works just fine.

### Basic audio manipulation



### Encoding

Babycat supports encoding in-memory waveforms to a WAV file or a WAV in-memory buffer. Other target encoding formats are coming.

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

### Documentation and releases
[**babycat.io**](https://babycat.io) is our main documentation website. You can find documentation and releases for each binding at:

| **Binding**     |  **Documentation**         |  **Releases**                                 |
| --------------- | -------------------------- | --------------------------------------------- |
| **Rust**        | [docs.rs/babycat][4]       | [crates.io][5]                                |
| **Python**      | [babycat.io/api/python][6] | [PyPI][7]                                     |
| **WebAssembly** | [babycat.io/api/wasm][8]   | [NPM][9]                                      |
| **C**           | [babycat.io/api/c][10]     | No releases yet. You can compile from source. |

### Tutorials
You can learn more about how to use Babycat from our long-form tutorials:

- [Audio terminology](https://babycat.io/tutorials/terminology/)
- [Installation/development requirements](https://babycat.io/tutorials/development-requirements/)
- [Using Babycat](https://babycat.io/tutorials/using-babycat/)
- [Contributing to Babycat](https://babycat.io/tutorials/contributing/)

## Acknowledgements
The first version of Babycat was an internal project at [Neocrym][2] written by [Ritik Mishra][11] Since then, the code has been extended and open-sourced by [James Mishra][12].

Babycat is built on top of *many* high-quality open source packages, including:

- [Symphonia][13] and [FFmpeg][22] for audio demuxing and decoding.
- [libsamplerate][1] for high-quality audio resampling.documentation
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
[18]: https://www.ffmpeg.org/legal.html
[19]: https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html
[20]: https://opensource.stackexchange.com/questions/6262/what-is-the-purpose-of-or-at-your-option-any-later-version-what-if-i-dont
[21]: https://github.com/babycat-io/babycat/blob/master/LICENSE
[22]: https://ffmpeg.org/
