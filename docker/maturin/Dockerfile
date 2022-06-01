# Use a Docker container to build the manylinux wheel.
FROM quay.io/pypa/manylinux2014_x86_64
ENV DEBIAN_FRONTEND noninteractive


# Fix permissions issues with our pip cache.
RUN mkdir /.cache && chmod --recursive 777 /.cache


# Install Rust so we can build the wheel.
COPY --from=rust:1.61.0-slim /usr/local/cargo /usr/local/cargo
COPY --from=rust:1.61.0-slim /usr/local/rustup /usr/local/rustup
RUN chmod --recursive 777 /usr/local/cargo /usr/local/rustup


# Configure our build environment.
ENV PATH /usr/local/cargo/bin:/opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:/opt/python/cp39-cp39/bin/:${PATH}
ENV RUST_ARCH x86_64-unknown-linux-gnu
ENV CARGO_HOME /usr/local/cargo
ENV RUSTUP_HOME /usr/local/cargo


# Install maturin.
COPY requirements-build.txt .
RUN python3.8 -m pip install -r requirements-build.txt \
    && rm requirements-build.txt

ENTRYPOINT [ "maturin" ]
