FROM ubuntu:focal-20210401 AS base

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
    build-essential \
    git \
    curl \
    wget \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_VERSION 1.23.1
ENV RUST_VERSION 1.53.0
ENV RUST_ARCH x86_64-unknown-linux-gnu
ENV RUSTUP_HOME /usr/local/rustup
ENV CARGO_HOME /usr/local/cargo

ADD https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/${RUST_ARCH}/rustup-init rustup-init

RUN chmod +x rustup-init \
    && ./rustup-init -y --no-modify-path --profile minimal --default-toolchain ${RUST_VERSION} --default-host ${RUST_ARCH} \
    && rm rustup-init

ENV PATH ${CARGO_HOME}/bin:${PATH}

RUN rustup component add rustfmt clippy
RUN rustup target add x86_64-unknown-linux-musl wasm32-unknown-unknown
RUN chmod -R a+w ${RUSTUP_HOME} ${CARGO_HOME}