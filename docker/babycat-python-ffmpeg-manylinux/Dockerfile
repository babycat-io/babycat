ARG PYTHON=python3.6
ARG ENABLE_FFMPEG_BUILD_LINK_STATIC=1
ARG DISABLE_VENV=1
ARG WHEEL_DIR=/wheels



# =========================================================
FROM quay.io/pypa/manylinux2014_x86_64 AS manylinux_stage

# Silence interactivity.
ENV DEBIAN_FRONTEND noninteractive

# Set up the paths to the various Python installations.
ENV PATH /opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:/opt/python/cp39-cp39/bin/:${PATH}

# Set up directories and permissions.
RUN mkdir /.cache /.local /wheels /babycat && chmod --recursive 777 /.cache /wheels /babycat
WORKDIR /babycat



# =========================================================
FROM manylinux_stage AS build_stage
ARG PYTHON
ARG ENABLE_FFMPEG_BUILD_LINK_STATIC
ARG DISABLE_VENV
ARG WHEEL_DIR

# Install Rust.
COPY --from=rust:1.56.0-slim /usr/local/cargo /usr/local/cargo
COPY --from=rust:1.56.0-slim /usr/local/rustup /usr/local/rustup
RUN chmod --recursive 777 /usr/local/cargo /usr/local/rustup
ENV PATH /usr/local/cargo/bin:${PATH}
ENV RUST_ARCH x86_64-unknown-linux-gnu
ENV CARGO_HOME /usr/local/cargo
ENV RUSTUP_HOME /usr/local/cargo
RUN rustup default 1.56.0-${RUST_ARCH}

# Install libclang and other tools that are required for building FFmpeg.
RUN yum update -y \
    && yum install -y centos-release-scl \
    && yum install -y llvm-toolset-7

# Vendor Rust dependencies
COPY Cargo.toml Makefile .listfiles.py ./
COPY .cargo .cargo
RUN mkdir .b src && touch src/lib.rs
RUN make vendor

# Install Maturin and any other Python dependencies that we need to build Babycat.
COPY requirements-build.txt .
RUN make init-python-requirements-build

# Build FFmpeg, Babycat, and Babycat's Python bindings.
COPY . .
RUN scl enable llvm-toolset-7 'make build-python'



# =========================================================
# Start over in a fresh container that doesn't have clang.
FROM manylinux_stage AS run_stage
ARG PYTHON

# Install dependencies for running tests.
COPY --from=build_stage /babycat/requirements-dev.txt .
RUN ${PYTHON} -m pip install -r requirements-dev.txt

# Install our compiled wheel from the previous container.
COPY --from=build_stage /wheels /wheels
RUN ${PYTHON} -m pip install /wheels/*
