Development requirements
=========================

Babycat has numerous dependencies. Most of them are automatically installed
via a language package manager:

- Rust dependencies are installed via ``cargo``.
- Python dependencies are installed via ``pip``.
- JavaScript dependencies are installed via ``npm``.

However, in order to compile Babycat and run tests, there are some software packages that you will need to manually install.

Requirements for compiling Babycat
----------------------------------

Rust 1.56 or newer
^^^^^^^^^^^^^^^^^^
Babycat is written for stable Rust, targeting 1.56.0 or newer. You can install Rust with `rustup <https://www.rust-lang.org/tools/install>`_.

cbindgen
^^^^^^^^
`cbindgen <https://github.com/eqrion/cbindgen>`_ is a tool that we use to generate Babycat's C bindings. You can install it with the command ``cargo install cbindgen``.

wasm-pack
^^^^^^^^^
`wasm-pack <https://rustwasm.github.io/docs/wasm-pack/>`_ is a tool that we use to generate Babycat's WebAssembly bindings. You can install it with the command ``cargo install wasm-pack``.

GNU Make
^^^^^^^^
GNU Make is a build system that Babycat uses to coordinate tasks.

If youare building Babycat on Windows, you should make sure that you have a Bash-compatible shell and GNU Make to run Babycat build commands with. On Windows, you can install GNU Make from `the GNUWin32 project <http://gnuwin32.sourceforge.net/install.html>`_.

Many macOS computer have the BSD version of Make, which might not properly understand Babycat's Makefile. To see which variant of Make you have installed, run the command ``make --version``. You should see ``GNU`` on the first line.

CMake
^^^^^
The CMake build system is necessary to compile Babycat with `libsamplerate <http://www.mega-nerd.com/SRC/index.html>`_ and other C and C++ dependencies.

A C compiler
^^^^^^^^^^^^
Babycat is written in Rust, but a C compiler is needed to:

- compile certain Babycat dependencies, such as libsamplerate.
- compile the tests for Babycat's C bindings.

Any recent C compiler should work, but we prefer Clang/LLVM.

You can install a C compiler and build tools from your system package manager:

- **Ubuntu**: ``apt-get install build-essential``
- **Fedora**: ``dnf install @development-tools``
- **macOS**: ``xcode-select --install``

On Windows, try installing the LLVM toolchain. There are Windows installers listed on the *Pre-Built Binaries* section of the `LLVM releases page <https://releases.llvm.org/download.html>`_.

Python 3.6 or newer
^^^^^^^^^^^^^^^^^^^
Python 3.6 or newer is necessary for the following tasks:

- compiling and using the Babycat Python bindings.
- linting and formatting Python code.
- generating documentation using Sphinx.

Node.js and NPM
^^^^^^^^^^^^^^^
You should have Node.js version 14 or newer. `The Node.js website has installers <https://nodejs.org/en/download/>`_ for all of the major operating systems. Node.js and NPM are necessary for:

- testing the Babycat WebAssembly bindings.
- linting and formatting the WebAssembly binding unit tests.

OpenSSL development headers
^^^^^^^^^^^^^^^^^^^^^^^^^^^
wasm-pack and other dependencies need these headers. You can install these headers from your system package manager:

- **Ubuntu**: ``apt-get install libssl-dev``
- **Fedora**: ``dnf install openssl-devel``
- **macOS**: ``brew install openssl``

libffi development headers
^^^^^^^^^^^^^^^^^^^^^^^^^^
The CPython runtime, the Python cryptography package, and various other bits of code depend on the `libffi <https://sourceware.org/libffi/>`_ development headers. You can install them from your system package manager:

- **Ubuntu**: ``apt-get install libffi-dev``
- **Fedora**: ``dnf install libffi-devel``
- **MacOS**: ``brew install libffi``

pkg-config (Linux-only)
^^^^^^^^^^^^^^^^^^^^^^^
On Linux, the program `pkg-config <https://www.freedesktop.org/wiki/Software/pkg-config/>`_ helps compilers find shared libraries. You can install pkg-config from your system package manager:

- **Ubuntu**: ``apt-get install pkg-config``
- **Fedora**: ``dnf install pkgconf-pkg-config``
- **MacOS**: ``brew install pkg-config``

ALSA headers (Linux only)
^^^^^^^^^^^^^^^^^^^^^^^^^
The Babycat command line program has the ability to play audio through your system speakers. In order to compile this program on Linux, you will need the development headers for `Advanced Linux Sound Architecture (ALSA) <https://www.alsa-project.org/wiki/Main_Page>`_.

You can install the ALSA development headers using your system package manager:
- **Ubuntu**: ``apt-get install libasound2-dev``
- **Fedora**: ``dnf install alsa-lib-devel``

Optional dependencies that make development easier
--------------------------------------------------

Docker and Docker Compose
^^^^^^^^^^^^^^^^^^^^^^^^^
Docker is not necessary to build or use Babycat, but the Babycat repository contains a few Docker images that are useful for testing.

Docker is also required for building a Babycat Python wheel that obeys the `manylinux <https://github.com/pypa/manylinux>`_ protocol--which is a requirement for releasing a Python Linux wheel that is compatible with most Linux systems.

To use these images, install Docker and Docker Compose. The Docker website has installers for `Docker Desktop here <https://docs.docker.com/get-docker/>`_ and `Docker Compose here <https://docs.docker.com/compose/install/>`_.

clang-format
^^^^^^^^^^^^
`clang-format <https://clang.llvm.org/docs/ClangFormat.html>`_ is a tool that ships with the Clang compiler. Babycat uses it to automatically format C code. If you have Clang installed, you likely already have clang-format. If not, you can install clang-format from your system package manager:

- **Ubuntu**: ``apt-get install clang-format``
- **Fedora**: ``dnf install clang-tools``
- **macOS**: ``brew install clang-format``

On Windows, try installing the LLVM toolchain. There are Windows installers listed on the *Pre-Built Binaries* section of the `LLVM releases page <https://releases.llvm.org/download.html>`_.

Doxygen
^^^^^^^
`Doxygen <https://www.doxygen.nl>`_ is a tool for generating documentation for Babycat's C bindings. The Doxygen website has `pre-built binaries <https://www.doxygen.nl/download.html#srcbin>`_ for macOS, Linux, and Windows. You can also install Doxygen from your system package manager:

- **Ubuntu**: ``apt-get install doxygen``
- **Fedora**: ``dnf install doxygen``
- **macOS**: ``brew install doxygen``

Valgrind
^^^^^^^^
`Valgrind <https://valgrind.org/>`_ is a tool for debugging memory errors in computer programs. You can also install Valgrind from your system package manager:

- **Ubuntu**: ``apt-get install valgrind``
- **Fedora**: ``dnf install valgrind``
- **macOS**: ``brew install valgrind``

Babycat also uses the `cargo-valgrind <https://crates.io/crates/cargo-valgrind>`_ Rust crate that makes it easier to use Valgrind to debug Rust programs. You can install it with the command ``cargo install valgrind``.
