FROM quay.io/pypa/manylinux2010_x86_64:2021-05-05-e1501b7

ENV PATH /opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:/opt/python/cp39-cp39/bin/:${PATH}

COPY --from=babycat/rust /usr/local/rustup /usr/local/rustup
COPY --from=babycat/rust /usr/local/cargo /usr/local/cargo
RUN chmod --recursive 777 /usr/local/rustup /usr/local/cargo

ENV PATH /usr/local/cargo/bin:${PATH}
ENV RUST_ARCH x86_64-unknown-linux-gnu
ENV RUSTUP_HOME /usr/local/rustup
ENV CARGO_HOME /usr/local/cargo
