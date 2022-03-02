# Use a Docker container to build the manylinux wheel.
FROM quay.io/pypa/manylinux2014_x86_64

# Install Rust so we can build the wheel.
COPY --from=rust:1.56.0-slim /usr/local/cargo /usr/local/cargo
COPY --from=rust:1.56.0-slim /usr/local/rustup /usr/local/rustup
RUN chmod --recursive 777 /usr/local/cargo /usr/local/rustup

# Configure our build environment.
ENV PATH /usr/local/cargo/bin:/opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:/opt/python/cp39-cp39/bin/:${PATH}
ENV RUST_ARCH x86_64-unknown-linux-gnu
ENV CARGO_HOME /usr/local/cargo
ENV RUSTUP_HOME /usr/local/cargo