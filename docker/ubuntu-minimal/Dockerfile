FROM babycat/rust AS base

ARG DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-c"]

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
    # Helps us find shared libraries when compiling code.
    pkg-config \
    # Helps us build libsamplerate and potentially other C/C++ dependencies.
    cmake \
    # Used to generate the automated C header documentation
    # at https://c.babycat.io.
    doxygen \
    # Required by wasm-pack.
    libssl-dev \
    # Required for building the Babycat command line app
    # or any other code that actually plays audio on Linux.
    libasound2-dev \
    # Clean up unused Apt cache data.
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Extract clang-format from LLVM releases.
# The clang-format package in the Ubuntu repositories
# brings in too many dependencies.
RUN cd /tmp \
    && wget https://github.com/llvm/llvm-project/releases/download/llvmorg-12.0.0/clang+llvm-12.0.0-x86_64-linux-gnu-ubuntu-20.04.tar.xz \
    && tar xvf clang+llvm-12.0.0-x86_64-linux-gnu-ubuntu-20.04.tar.xz \
    && mv clang+llvm-12.0.0-x86_64-linux-gnu-ubuntu-20.04/bin/clang-format /usr/local/bin \
    && rm -rfv clang+llvm-12.0.0-x86_64-linux-gnu-ubuntu-20.04.tar.xz clang+llvm-12.0.0-x86_64-linux-gnu-ubuntu-20.04

# Install cbindgen, which we use to build Babycat's C bindings.
RUN cargo install cbindgen

# Install wasm-pack, which we use to build Babycat's WebAssembly bindings.
RUN cargo install wasm-pack
