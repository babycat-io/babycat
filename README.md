![Babycat Logo](https://static.neocrym.com/images/babycat/v1/2x/babycat-body-icon-dark-social-media-cover--2x.png "Babycat Logo")

# Babycat is an audio decoding and manipulation library

## Intro
Babycat is a library that makes it easy to decode and manipulate many audio files at once.

### Use Babycat with C, Python, Rust, or JavaScript/WebAssembly.
Babycat is written in Rust, has generated bindings for C, Python, and WebAssembly, and can be compiled to many different target architectures.

Babycat's bindings to Python allow for the concurrent decoding of many audio files without being slowed down by the Python Global Interpreter Lock (GIL), allowing you to get work done faster than you could in pure Python.

### Babycat is permissively-licensed.
Babycat is licensed under the MIT license. Unlike many other libraries in the audio ecosystem,you can use Babycat in any project you want without any restrictions.

### Babycat is battle-tested in industry.
Babycat was built at and is actively maintained by [Neocrym](https://www.neocrym.com/), a record label that use artificial intelligence to find and promote the world's greatest musicians. Neocrym uses Babycat to decode millions of songs as part of audio feature engineering pipelines for training machine learning models.

## Terminology
Babycat uses the [same audio terminology as the Apple Core Audio API](https://developer.apple.com/documentation/coreaudiotypes/audiostreambasicdescription?language=objc):

* An audio _**stream**_ is a continuous series of data that represents a sound, such as a song.
* A _**channel**_ is a discrete track of monophonic audio. A monophonic stream has one channel. A stereo stream has two channels. An audio stream that contains 5.1 surround sound will have five normal channels and one Low Frequency Enhancement (LFE) channel.
* A _**sample**_ is single numerical value in a single audio channel in an audio stream.
* A _**frame**_ is a collection of samples from the same point in time--one sample for each channel.
* The _**frame rate**_ (or _**sample rate**_) for a stream is the number of frames per second (hertz) of uncompressed audio.

Babycat stores audio as a single array of _**interleaved**_ samples from channels. A waveform with two channels (e.g. left and right) are stored as:
```
[L R L R L R...]
```
We give every sample a two-dimensional index--first by frame and then by channel.

## System requirements for compiling Babycat
Here is the software you will need if you want to compile your own changes to Babycat:

### Rust 1.50 or newer
Babycat is written for stable Rust, targeting version 1.50.0 or newer.

### GNU Make
GNU Make is a build system that Babycat uses to coordinate tasks.

If you are building Babycat on Windows, you should make sure that you have a Bash-compatible shell and GNU Make to run Babycat build commands with. On Windows, you can install GNU Make from [the GNUWin32 project](http://gnuwin32.sourceforge.net/install.html).

Many MacOS computers have the BSD version of Make, which might not parse Babycat's Makefile. To see which version of Make you have installed, run the command `make --version`. You should see `GNU` on the first line.

### CMake
The CMake build system is necessary to compile Babycat with [libsamplerate][1] and other C/C++ dependencies.

### A C compiler
Babycat is written in Rust, but a C compiler is needed to:
* compile certain Babycat dependencies, such as [libsamplerate][1].
* compile and run the tests for Babycat's C bindings.

Any recent C compiler should work, but we prefer Clang/LLVM.

You can install a C compiler and build tools from your system package manager:
* **Ubuntu**: `apt-get install build-essential`
* **Fedora**: `dnf install @development-tools`
* **MacOS**: `xcode-select --install`

On Windows, try installing the LLVM toolchain. There are Windows installers listed on the *Pre-Built Binaries* section of the [LLVM releases page](https://releases.llvm.org/download.html).

### Python 3.6 or newer
Python 3.6 or newer is necessary for the following tasks:
* Compiling the Babycat Python bindings
* Using Babycat Python bindings
* Linting and formatting Python code
* Generating documentation using Sphinx

### Node.js and NPM
Node.js and NPM are only necessary for running the Babycat WebAssembly tests. You should have Node.JS version 14 or newer. Node.js and NPM are only necessary for the following tasks:
* Testing the Babycat WebAssembly bindings.
* Linting and formatting the WebAssembly binding test source code.

### OpenSSL development headers
wasm-pack and other dependencies need these headers. You can install these headers from your system package manager:
* **Ubuntu**: `apt-get install libssl-dev`
* **Fedora**: `dnf install openssl-devel`
* **MacOS**: `brew install openssl`

### libffi development headers
The CPython runtime, the Python cryptography package, and various other bits of code depend on the libffi development headers. You can install them from your system package manager:
* **Ubuntu**: `apt-get install libffi-dev`
* **Fedora**: `dnf install libffi-devel`
* **MacOS**: `brew install libffi`

### pkg-config (Linux only)
On Linux, the program pkg-config helps compilers find shared libraries. You can install pkg-config from your system package manager:
* **Ubuntu**: `apt-get install pkg-config`
* **Fedora**: `dnf install pkgconf-pkg-config`
* **MacOS**: `brew install pkg-config`

### ALSA headers (Linux only)
The Babycat command line program has the ability to paly audio through your system speakers. In order to compile this program on Linux, you will need the development headers for Advanced Linux Sound Architecture (ALSA).

You can install the ALSA development headers using your system package manager:
* **Ubuntu**: `apt-get install libasound2-dev`
* **Fedora**: `dnf install alsa-lib-devel`

### cbindgen
[cbindgen](https://github.com/eqrion/cbindgen) is a tool that we use to generate Babycat's C bindings. You can install it with the command `cargo install cbindgen`.

### wasm-pack
[wasm-pack](https://rustwasm.github.io/docs/wasm-pack/) is a tool that we use to generate Babycat's WebAssembly bindings. You can install it with the command `cargo install wasm-pack`.

Before you install wasm-pack, make sure you have already installed the OpenSSL development headers (see above).

### Docker and Docker Compose
Docker is not necessary to build or use Babycat, but the Babycat repository contains a few Docker images that are useful for testing.

Docker is also required for building a Babycat Python wheel that obeys the [`manylinux`](https://github.com/pypa/manylinux) protocol--which is a requirement for building a binary Python wheel that is compatible with most Linux systems.

To use these images, install Docker and Docker Compose. The Docker website has installers for Docker Desktop [here](https://docs.docker.com/get-docker/) and Docker Compose [here](https://docs.docker.com/compose/install/).

### clang-format
clang-format is a tool that ships with the Clang compiler. Babycat uses it to automatically format the code for our C binding tests. If you already use Clang, you might already have clang-format installed. If not, you can install clang-format from your system package manager:
* **Ubuntu**: `apt-get install clang-format`
* **Fedora**: `dnf install clang-tools`
* **MacOS**: `brew install clang-format`

On Windows, the best way to get clang-format is to download and install the LLVM toolchain. You can find Windows installers under the *Pre-Built Binaries* section of the [LLVM releases page](https://releases.llvm.org/download.html).

## Acknowledgements
The first version of Babycat was an internal project at Neocrym written by [Ritik Mishra](https://www.linkedin.com/in/ritikmishra). Since then, the code has been extended and open-sourced by [James Mishra](https://www.linkedin.com/in/jamesmishra).

Babycat is built on top of many high-quality open source packages, including:
* [Symphonia](https://github.com/pdeljanov/Symphonia) by Philip Deljanov
* [libsamplerate][1] by Erik de Castro Lopo
* [Hound](https://github.com/ruuda/hound) by Ruud van Asseldonk


[1]: http://www.mega-nerd.com/SRC/index.html
