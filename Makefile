# These are the Rust files being tracked by Git.
RUST_SRC_FILES ?= $(shell git ls-files --cached --deleted --modified --others src)


# These variables set the path for Rust or system tools.
CBINDGEN ?= cbindgen
CARGO ?= cargo
DOXYGEN ?= doxygen
RUSTUP ?= rustup
CLANG_FORMAT ?= clang-format
DOCKER_COMPOSE ?= docker-compose
WASM_PACK ?= wasm-pack
VALGRIND ?= valgrind


# These variables set the paths for Node/NPM/JavaScript tools.
NPM ?= npm
ESLINT ?= ./node_modules/.bin/eslint
PRETTIER ?= ./node_modules/.bin/prettier
JAVASCRIPT_CODE_PATHS ?= ./tests-wasm-nodejs/test.js


# These variables set the paths for Python tools.
PYTHON ?= python3
WHEEL_DIR ?= target/python
WHEEL_CMD ?= wheel --no-cache-dir --no-deps --wheel-dir=$(WHEEL_DIR) .
VENV_PATH ?= venv
CREATE_VENV_CMD ?= $(PYTHON) -m venv $(VENV_PATH)
PYTHON_CODE_PATHS ?= ./tests-python ./docs/source/conf.py


# Windows and Unix have different paths for activating
# Python virtualenvs.
# Note that once we have activated the venv, we do not need
# to use the Python path in $(PYTHON). The "python" command
# will automatically point to the right Python.
# TODO(jamesmishra): Handle the distinction between bash and cmd.
ifeq ($(OS),Windows_NT)
	ACTIVATE_VENV_PATH ?= $(VENV_PATH)/Scripts/activate
	ACTIVATE_VENV_CMD ?= . $(ACTIVATE_VENV_PATH)
else
	ACTIVATE_VENV_PATH ?= $(VENV_PATH)/bin/activate
	ACTIVATE_VENV_CMD ?= . $(ACTIVATE_VENV_PATH)
endif


# This is the shared library filename
# (excluding the extension, see SHARED_LIB_EXT below)
# that `cargo build` creates.
ifeq ($(OS),Windows_NT)
	BABYCAT_SHARED_LIB_NAME ?= babycat
else
	BABYCAT_SHARED_LIB_NAME ?= libbabycat
endif


# This is the filename for the babycat binary.
ifeq ($(OS),Windows_NT)
	BABYCAT_BINARY_NAME ?= babycat.exe
else
	BABYCAT_BINARY_NAME ?= babycat
endif


# This sets the file extension for linking to shared libraries.
# We typically use this when testing Babycat's C FFI bindings.
ifeq ($(OS),Windows_NT)
	SHARED_LIB_EXT ?= lib
else
	ifeq ($(shell uname -s),Darwin)
		SHARED_LIB_EXT ?= dylib
	else
		SHARED_LIB_EXT ?= so
	endif
endif



# help ==============================================================

help:
	@cat makefile-help.txt
.PHONY: help


# clean =============================================================

clean-caches:
	rm -rfv docker/main/.ti docker/pip/.ti docker/rust/.ti .ipynb_checkpoints .mypy_cache .pytest_cache tests-python/__pycache__
	find . -name '.DS_Store' -delete
.PHONY: clean-caches

clean-docs:
	rm -rfv docs/build docs/source/api/python/generated
.PHONY: clean-docs

clean-node-modules:
	rm -rfv node_modules tests-wasm-nodejs/node_module examples-wasm/decode/node_modules
.PHONY: clean-node-modules

clean-vendor:
	rm -rfv Cargo.lock vendor
.PHONY: clean-vendor

clean-target:
	rm -rfv target babycat.h examples-wasm/decode/dist
.PHONY: clean-target

clean-venv:
	rm -rfv $(VENV_PATH)
.PHONY: clean-venv

clean: clean-caches clean-docs clean-node-modules clean-vendor clean-target clean-venv
.PHONY: clean


# vendor ============================================================

vendor/.ti: Cargo.toml .cargo/config.toml
	$(CARGO) vendor --versioned-dirs --quiet
	@touch vendor/.ti

vendor: vendor/.ti


# init ==============================================================

# Set up the Python virtualenv
$(VENV_PATH)/.ti: requirements-dev.txt requirements-docs.txt
	$(CREATE_VENV_CMD)
	$(ACTIVATE_VENV_CMD) && python -m pip install --upgrade pip
	$(ACTIVATE_VENV_CMD) && python -m pip install --requirement requirements-dev.txt
	$(ACTIVATE_VENV_CMD) && python -m pip install --requirement requirements-docs.txt
	@touch $(VENV_PATH)/.ti

# Wrapper command for setting up the Python virtualenv
init-python: $(VENV_PATH)/.ti
.PHONY: init-python

# Set up our main npm node_modules, containing developer tools
node_modules/.ti: package.json package-lock.json
	$(NPM) rebuild && $(NPM) install
	@touch node_modules/.ti

init-javascript-tools: node_modules/.ti
.PHONY: init-javascript-tools

# Set up our npm node_modules for testing
tests-wasm-nodejs/node_modules/.ti: tests-wasm-nodejs/package.json tests-wasm-nodejs/package-lock.json
	cd tests-wasm-nodejs && $(NPM) rebuild && $(NPM) install

# Wrapper command for setting up npm
init-javascript-tests: node_modules/.ti tests-wasm-nodejs/node_modules/.ti
.PHONY: init-javascript-tests

# All of the Rust tools needed for development.
init-rust: vendor/.ti
.PHONY: init-rust

# only needed if linting code. not needed if we are only compiling.
init-rustup-clippy:
	$(RUSTUP) component add clippy
.PHONY: init-rustup-clippy

# only needed if formatting code. not needed if we are only compiling.
init-rustup-rustfmt:
	$(RUSTUP) component add rustfmt
.PHONY: init-rustup-rustfmt

# only needed when compilin to WebAssembly.
init-rustup-wasm32-unknown-unknown:
	$(RUSTUP) target add wasm32-unknown-unknown
.PHONY: init-rustup-wasm32-unknown-unknown

# Only needed when generating headers for the C bindings.
init-cargo-cbindgen:
	$(CBINDGEN) --version > /dev/null || $(CARGO) install cbindgen
.PHONY: init-cargo-cbindgen

# enable the environment variable OPENSSL_NO_VENDOR=1 to
# use a pre-compiled OpenSSL already on the system.
init-cargo-wasm-pack:
	$(WASM_PACK) --version > /dev/null || $(CARGO) install wasm-pack
.PHONY: init-cargo-wasm-pack

# Only needed if compiling and generating flamegraphs on this machine.
init-cargo-flamegraph:
	$(CARGO) flamegraph --version > /dev/null || $(CARGO) install flamegraph
.PHONY: init-cargo-flamegraph

# Only needed if compiling and testing code using a pre-installed
# Valgrind binary on this machine.
init-cargo-valgrind:
	$(CARGO) valgrind --version > /dev/null || $(CARGO) install cargo-valgrind
.PHONY: init-cargo-valgrind


# fmt ===============================================================

fmt-c:
	$(CLANG_FORMAT) -i tests-c/*.c examples-c/*.c
.PHONY: fmt-c

fmt-javascript: init-javascript-tools
	$(ESLINT) --fix $(JAVASCRIPT_CODE_PATHS)
	$(PRETTIER) --write $(JAVASCRIPT_CODE_PATHS)
.PHONY: fmt-javascript

fmt-python: init-python
	$(ACTIVATE_VENV_CMD) && black $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && isort $(PYTHON_CODE_PATHS)
.PHONY: fmt-python

fmt-rust: init-rustup-rustfmt
	$(CARGO) fmt
.PHONY: fmt-rust

fmt: fmt-c fmt-javascript fmt-python fmt-rust
.PHONY: fmt


# fmt-check =========================================================

fmt-check-c:
	$(CLANG_FORMAT) --dry-run -Werror tests-c/*
.PHONY: fmt-check-c

fmt-check-javascript: init-javascript-tools
	$(ESLINT) $(JAVASCRIPT_CODE_PATHS)
	$(PRETTIER) --check --loglevel=silent $(JAVASCRIPT_CODE_PATHS)
.PHONY: fmt-check-javascript

fmt-check-python: init-python
	$(ACTIVATE_VENV_CMD) && black --quiet $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && isort --quiet $(PYTHON_CODE_PATHS)
.PHONY: fmt-check-python

fmt-check-rust: init-rustup-rustfmt
	$(CARGO) fmt -- --check
.PHONY: fmt-check-rust

fmt-check: fmt-check-c fmt-check-javascript fmt-check-python fmt-check-rust
.PHONY: fmt-check


# lint ==============================================================

lint-python: init-python
	$(ACTIVATE_VENV_CMD) && pylint $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && mypy $(PYTHON_CODE_PATHS)
.PHONY: lint-python

lint-rust: init-rustup-clippy init-rust
	CARGO_TARGET_DIR=target/all-features $(CARGO) clippy --release --all-features
.PHONY: lint-rust

lint: lint-rust lint-python
.PHONY: lint


# cargo build commands ==============================================

## all features
target/all-features/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT): init-rust
	CARGO_TARGET_DIR=target/all-features $(CARGO) build --release --all-features

cargo-build-release-all-features: target/all-features/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
.PHONY: cargo-build-release-all-features

## frontend-rust
target/frontend-rust/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT): init-rust
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) build --release --features=frontend-rust

cargo-build-release-frontend-rust: target/frontend-rust/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
.PHONY: cargo-build-release-frontend-rust

## frontend-wasm
target/frontend-wasm/release/$(BABYCAT_SHARED_LIB_NAME).${SHARED_LIB_EXT}: init-rust init-rustup-wasm32-unknown-unknown
	CARGO_TARGET_DIR=target/frontend-wasm $(CARGO) build --release --features=frontend-wasm

cargo-build-release-frontend-wasm: target/frontend-wasm/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
.PHONY: cargo-build-release-frontend-wasm

## frontend-c
target/frontend-c/release/$(BABYCAT_SHARED_LIB_NAME).${SHARED_LIB_EXT}: init-rust
	CARGO_TARGET_DIR=target/frontend-c $(CARGO) build --release --features=frontend-c

cargo-build-release-frontend-c: target/frontend-c/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
.PHONY: cargo-build-release-frontend-c

## frontend-binary
target/frontend-binary/release/$(BABYCAT_BINARY_NAME): init-rust
	CARGO_TARGET_DIR=target/frontend-binary $(CARGO) build --release --features=frontend-binary --bin=babycat

cargo-build-release-frontend-binary: target/frontend-binary/release/$(BABYCAT_BINARY_NAME)
.PHONY: cargo-build-release-frontend-binary


# docs ==============================================================

docs: init-javascript-tools build-python-and-install build-wasm-bundler babycat.h $(RUST_SRC_FILES)
	rm -rf docs/build
	mkdir docs/build
	doxygen
	$(ACTIVATE_VENV_CMD) && export PATH=$(PWD)/node_modules/.bin:$$PATH && $(MAKE) -C docs dirhtml
.PHONY: docs

# This is the command we use to build docs on Netlify.
# The Netlify build image has Python 3.8 installed,
# but does not come with the virtualenv extension.
docs-netlify: init-rust init-javascript-tools build-wasm-bundler babycat.h
# Clean any previous builds.
	rm -rf docs/build
	mkdir docs/build
# Generate Doxygen XML to document Babycat's C bindings.
	doxygen
# Install Python dependencies for building the docs.
	python3 -m pip install -r requirements-docs.txt
# Install Babycat's Python bindings.
	python3 -m pip install --force-reinstall .
# Generate the docs.
	export PATH=$(PWD)/node_modules/.bin:$$PATH && $(MAKE) -C docs dirhtml
.PHONY: docs-netlify


# build =============================================================

babycat.h: init-rust init-cargo-cbindgen cbindgen.toml $(RUST_SRC_FILES)
	$(CBINDGEN) --quiet --output babycat.h

$(WHEEL_DIR)/*.whl: init-rust $(RUST_SRC_FILES)
	$(PYTHON) -m pip $(WHEEL_CMD)

build-python: $(WHEEL_DIR)/*.whl
.PHONY: build-python

build-python-and-install: build-python init-python
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --no-deps --force-reinstall $(WHEEL_DIR)/*.whl
.PHONY: build-python-and-install

build-python-manylinux: docker-build-pip
	$(DOCKER_COMPOSE) run --rm --user=$$(id -u):$$(id -g) pip $(WHEEL_CMD)
.PHONY: build-python-manylinux

build-rust: cargo-build-release-frontend-rust
.PHONY: build-rust

build-wasm-bundler: init-rust init-cargo-wasm-pack init-rustup-wasm32-unknown-unknown
	CARGO_TARGET_DIR=target/frontend-wasm $(WASM_PACK) build --release --target=bundler --out-dir=./target/wasm/bundler -- --no-default-features --features=frontend-wasm
	cp .npmrc-example ./target/wasm/bundler/.npmrc
.PHONY: build-wasm-bundler

build-wasm-nodejs: init-rust init-cargo-wasm-pack init-rustup-wasm32-unknown-unknown
	CARGO_TARGET_DIR=target/frontend-wasm $(WASM_PACK) build --release --target=nodejs --out-dir=./target/wasm/nodejs -- --no-default-features --features=frontend-wasm
	cp .npmrc-example ./target/wasm/nodejs/.npmrc
.PHONY: build-wasm-nodejs

build-wasm-web: init-rust init-cargo-wasm-pack init-rustup-wasm32-unknown-unknown
	CARGO_TARGET_DIR=target/frontend-wasm $(WASM_PACK) build --release --target=web --out-dir=./target/wasm/web -- --no-default-features --features=frontend-wasm
	cp .npmrc-example ./target/wasm/web/.npmrc
.PHONY: build-wasm-web

build: build-python build-rust build-wasm-bundler build-wasm-nodejs build-wasm-web
.PHONY: build

# For now, we are going to purposely exclude `build-binary` from running
# in the general `build`  command. This is because the babycat command line
# app depends on dynamically linking to ALSA libraries on Ubuntu.
# We don't want to make `make build` fail if the user does not have
# those libraries.
build-binary: cargo-build-release-frontend-binary
.PHONY: build-binary


# test ==============================================================

test-c: babycat.h cargo-build-release-frontend-c
	$(CC) -g -Wall -Werror=unused-function -o target/test_c tests-c/test.c target/frontend-c/release/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
	./target/test_c
.PHONY: test-c

test-c-valgrind: init-cargo-valgrind babycat.h cargo-build-release-frontend-c
	$(CC) -g -Wall -Werror=unused-function -o target/test_c tests-c/test.c target/frontend-c/release/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
	$(VALGRIND) --leak-check=full --show-leak-kinds=all ./target/test_c
.PHONY: test-c-valgrind

test-python: build-python-and-install
	$(ACTIVATE_VENV_CMD) && pytest
.PHONY: test-python

test-rust: init-rust
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) test --release --features=frontend-rust
.PHONY: test-rust

test-wasm-nodejs: init-javascript-tests build-wasm-nodejs
	cd tests-wasm-nodejs && $(NPM) run test
.PHONY: test-wasm-nodejs

test: test-rust test-python test-wasm-nodejs test-c
.PHONY: test


# doctest ==========================================================

doctest-python: build-python-and-install
	$(ACTIVATE_VENV_CMD) && pytest tests-python/test_doctests.py
.PHONY: doctest-python

doctest-rust: init-rust
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) test --release --doc
.PHONY: doctest-rust

doctest: doctest-rust doctest-python
.PHONY: doctest


# bench =============================================================

bench-rust: init-rust
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) bench
.PHONY: bench-rust

bench: bench-rust
.PHONY: bench


# example ===========================================================

example-resampler-comparison: init-rust
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) run --release --example resampler_comparison
.PHONY: example-resampler-comparison

example-decode-rust: init-rust
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) run --release --example decode
.PHONY: example-decode-rust

example-decode-python: build-python-and-install
	$(ACTIVATE_VENV_CMD) && python3 examples-python/decode.py
.PHONY: example-decode-python

example-decode-c: babycat.h cargo-build-release-frontend-c
	$(CC) -Wall -o target/decode_c examples-c/decode.c target/frontend-c/release/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
	./target/decode_c
PHONY: example-decode-c

example-decode-wasm: build-wasm-bundler
	cd examples-wasm/decode/ && $(NPM) install
	cd examples-wasm/decode/ && ./node_modules/.bin/webpack
PHONY: example-decode-wasm


# docker build ======================================================

docker/rust/.ti: docker-compose.yml docker/rust/Dockerfile
	$(DOCKER_COMPOSE) build cargo
	@touch docker/rust/.ti

docker/ubuntu-minimal/.ti: docker/rust/.ti docker-compose.yml docker/ubuntu-minimal/Dockerfile
	$(DOCKER_COMPOSE) build ubuntu-minimal
	@touch docker/ubuntu-minimal/.ti

docker/main/.ti: docker/ubuntu-minimal/.ti docker-compose.yml docker/main/Dockerfile
	$(DOCKER_COMPOSE) build main
	@touch docker/main/.ti

docker/pip/.ti: docker/rust/.ti docker-compose.yml docker/pip/Dockerfile
	$(DOCKER_COMPOSE) build pip
	@touch docker/pip/.ti

docker-build-cargo: docker/rust/.ti
.PHONY: docker-build-cargo

docker-build-ubuntu-minimal: docker/ubuntu-minimal/.ti
.PHONY: docker-build-ubuntu-minimal

docker-build-main: docker/main/.ti
.PHONY: docker-build-main

docker-build-pip: docker/pip/.ti
.PHONY: docker-build-pip

docker-build: docker-build-cargo docker-build-ubuntu-minimal docker-build-main docker-build-pip
.PHONY: docker-build


# docker run ========================================================

docker-run-docs-netlify:
	$(DOCKER_COMPOSE) run --rm netlify
.PHONY: docker-run-docs-netlify
