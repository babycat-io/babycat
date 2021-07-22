![Babycat Logo](https://static.neocrym.com/images/babycat/v1/2x/babycat-body-icon-dark-social-media-cover--2x.png "Babycat Logo")

# Babycat is an audio decoding and manipulation library

## Intro
Babycat is a library that makes it easy to decode and manipulate many audio files at once.

### Use Babycat with C, Python, Rust, or JavaScript/WebAssembly.
Babycat is written in Rust, has generated bindings for C, Python, and WebAssembly, and can be compiled to many different target architectures.

Babycat's bindings to Python allow for the concurrent decoding of many audio files without being slowed down by the Python Global Interpreter Lock (GIL), allowing you to get work done faster than you could in pure Python.

### Babycat is permissively-licensed.
Babycat is licensed under the MIT license. Unlike many other libraries in the audio ecosystem, you can use Babycat in any project you want without any restrictions.

### Babycat is battle-tested in industry.
Babycat was built at and is actively maintained by [Neocrym](https://www.neocrym.com/), a record label that use artificial intelligence to find and promote the world's greatest musicians. Neocrym uses Babycat to decode millions of songs as part of audio feature engineering pipelines for training machine learning models.

## Documentation
You can find Babycat's documentation at **[babycat.io](https://babycat.io)**.
### API Reference

- [Rust](https://docs.rs/babycat)
- [Python](https://babycat.io/api/python/)
- [WebAssembly](https://babycat.io/api/wasm/)
- [C](https://babycat.io/api/c/)

### Tutorials
- [Tutorials](https://babycat.io/tutorials/terminology/)
- [Installation and development requirements](https://babycat.io/tutorials/development-requirements/)
- [Using Babycat](https://babycat.io/tutorials/using-babycat/)
- [Contributing to Babycat](https://babycat.io/tutorials/contributing/)

## Acknowledgements
The first version of Babycat was an internal project at Neocrym written by [Ritik Mishra](https://www.linkedin.com/in/ritikmishra). Since then, the code has been extended and open-sourced by [James Mishra](https://www.linkedin.com/in/jamesmishra).

Babycat is built on top of many high-quality open source packages, including:
* [Symphonia](https://github.com/pdeljanov/Symphonia) by Philip Deljanov
* [libsamplerate][1] by Erik de Castro Lopo
* [Hound](https://github.com/ruuda/hound) by Ruud van Asseldonk


[1]: http://www.mega-nerd.com/SRC/index.html
