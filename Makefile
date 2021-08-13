# These are the Rust files being tracked by Git.
RUST_SRC_FILES ?= $(shell git ls-files src)


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


.PHONY: help clean init-javascript init-rust-wasm-pack init-rust-cargo-valgrind init-rust-flamegraph init-rust-minimal init-rust init vendor fmt-c fmt-javascript fmt-python fmt-rust fmt fmt-check-javascript fmt-check-python fmt-check-rust fmt-check lint-rust lint cargo-build-release-all-features cargo-build-release-frontend-rust cargo-build-release-frontend-wasm cargo-build-release-frontend-c babycat.h build-python install-babycat-python build-rust build-wasm-bundler build-wasm-nodejs build-wasm-web build test-c test-c-valgrind test-rust test-wasm-nodejs test doctest-python doctest-rust doctest bench-rust bench example-resampler-comparison example-decode-rust example-decode-python example-decode-c example-decode-wasm docker-build-cargo docker-build-ubuntu-minimal docker-build-main docker-build-pip docker-build docker-run-docs-netlify


# help ==============================================================

help:
	@cat makefile-help.txt


# clean =============================================================

clean:
	rm -rf target node_modules tests-wasm-nodejs/node_modules venv docker/main/.ti docker/pip/.ti docker/rust/.ti .ipynb_checkpoints .mypy_cache .pytest_cache Cargo.lock babycat.h tests-python/__pycache__ examples-wasm/decode/dist docs/build docs/source/api/python/generated
	find . -name '.DS_Store' -delete


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

# Set up our main npm node_modules, containing developer tools
node_modules/.ti: package.json package-lock.json
	$(NPM) rebuild && $(NPM) install
	@touch node_modules/.ti

init-javascript-minimal: node_modules/.ti

# Set up our npm node_modules for testing
tests-wasm-nodejs/node_modules/.ti: tests-wasm-nodejs/package.json tests-wasm-nodejs/package-lock.json
	cd tests-wasm-nodejs && $(NPM) rebuild && $(NPM) install

# Wrapper command for setting up npm
init-javascript: node_modules/.ti tests-wasm-nodejs/node_modules/.ti

init-rust-cbindgen:
	$(CBINDGEN) --version || $(CARGO) install cbindgen

init-rust-wasm-pack:
	$(WASM_PACK) --version || $(CARGO) install wasm-pack

init-rust-cargo-valgrind:
	$(CARGO) valgrind --version || $(CARGO) install cargo-valgrind

init-rust-flamegraph:
	$(CARGO) flamegraph --version || $(CARGO) install flamegraph

# Install a minimal set of Rust tools to build documentation.
init-rust-minimal: init-rust-cbindgen init-rust-wasm-pack
	$(RUSTUP) target add wasm32-unknown-unknown

# All of the Rust tools needed for development.
init-rust: init-rust-minimal init-rust-cargo-valgrind init-rust-flamegraph
	$(RUSTUP) component add clippy rustfmt

init: init-javascript init-python init-rust


# vendor ============================================================

vendor/.ti: Cargo.toml
	$(CARGO) vendor --versioned-dirs --quiet
	@touch vendor/.ti

vendor: vendor/.ti


# fmt ===============================================================

fmt-c:
	$(CLANG_FORMAT) -i tests-c/*.c examples-c/*.c

fmt-javascript: init-javascript-minimal
	$(ESLINT) --fix $(JAVASCRIPT_CODE_PATHS)
	$(PRETTIER) --write $(JAVASCRIPT_CODE_PATHS)

fmt-python: init-python
	$(ACTIVATE_VENV_CMD) && black $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && isort $(PYTHON_CODE_PATHS)

fmt-rust: init-rust vendor
	$(CARGO) fmt

fmt: fmt-c fmt-javascript fmt-python fmt-rust


# fmt-check =========================================================

fmt-check-c:
	$(CLANG_FORMAT) --dry-run -Werror tests-c/*

fmt-check-javascript: init-javascript-minimal
	$(ESLINT) $(JAVASCRIPT_CODE_PATHS)
	$(PRETTIER) --check --loglevel=silent $(JAVASCRIPT_CODE_PATHS)

fmt-check-python: init-python
	$(ACTIVATE_VENV_CMD) && black --quiet $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && isort --quiet $(PYTHON_CODE_PATHS)

fmt-check-rust: init-rust vendor
	$(CARGO) fmt -- --check

fmt-check: fmt-check-c fmt-check-javascript fmt-check-python fmt-check-rust


# lint ==============================================================

lint-python: init-python
	$(ACTIVATE_VENV_CMD) && pylint $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && mypy $(PYTHON_CODE_PATHS)

lint-rust: init-rust vendor
	CARGO_TARGET_DIR=target/all-features $(CARGO) clippy --release --all-features

lint: lint-rust lint-python


# cargo build commands ==============================================

## all features
target/all-features/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT): vendor
	CARGO_TARGET_DIR=target/all-features $(CARGO) build --release --all-features

cargo-build-release-all-features: target/all-features/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)

## frontend-rust
target/frontend-rust/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT): vendor
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) build --release --features=frontend-rust

cargo-build-release-frontend-rust: target/frontend-rust/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)

## frontend-wasm
target/frontend-wasm/release/$(BABYCAT_SHARED_LIB_NAME).${SHARED_LIB_EXT}: vendor
	CARGO_TARGET_DIR=target/frontend-wasm $(CARGO) build --release --features=frontend-wasm

cargo-build-release-frontend-wasm: target/frontend-wasm/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)

## frontend-c
target/frontend-c/release/$(BABYCAT_SHARED_LIB_NAME).${SHARED_LIB_EXT}: vendor
	CARGO_TARGET_DIR=target/frontend-c $(CARGO) build --release --features=frontend-c

cargo-build-release-frontend-c: target/frontend-c/release/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)

## frontend-binary
target/frontend-binary/release/$(BABYCAT_BINARY_NAME): vendor
	CARGO_TARGET_DIR=target/frontend-binary $(CARGO) build --release --features=frontend-binary --bin=babycat

cargo-build-release-frontend-binary: target/frontend-binary/release/$(BABYCAT_BINARY_NAME)


# docs ==============================================================

docs: init-javascript-minimal install-babycat-python build-wasm-bundler babycat.h $(shell git ls-files src)
	rm -rf docs/build
	mkdir docs/build
	doxygen
	$(ACTIVATE_VENV_CMD) && export PATH=$(PWD)/node_modules/.bin:$$PATH && $(MAKE) -C docs dirhtml

# This is the command we use to build docs on Netlify.
# The Netlify build image has Python 3.8 installed,
# but does not come with the virtualenv extension.
docs-netlify: init-javascript-minimal build-wasm-bundler babycat.h
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


# build =============================================================

babycat.h: init-rust-minimal cbindgen.toml $(RUST_SRC_FILES)
	$(CBINDGEN) --quiet --output babycat.h
	@$(CLANG_FORMAT) -i babycat.h || true

$(WHEEL_DIR)/*.whl: vendor/.ti $(RUST_SRC_FILES)
	$(PYTHON) -m pip $(WHEEL_CMD)

build-python: $(WHEEL_DIR)/*.whl

install-babycat-python: build-python init-python
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --no-deps --force-reinstall $(WHEEL_DIR)/*.whl

build-python-manylinux: docker-build-pip
	$(DOCKER_COMPOSE) run --rm --user=$$(id -u):$$(id -g) pip $(WHEEL_CMD)

build-rust: cargo-build-release-frontend-rust

build-wasm-bundler: vendor init-rust-minimal
	CARGO_TARGET_DIR=target/frontend-wasm $(WASM_PACK) build --release --target=bundler --out-dir=./target/wasm/bundler -- --no-default-features --features=frontend-wasm
	cp .npmrc-example ./target/wasm/bundler/.npmrc

build-wasm-nodejs: vendor init-javascript-minimal
	CARGO_TARGET_DIR=target/frontend-wasm $(WASM_PACK) build --release --target=nodejs --out-dir=./target/wasm/nodejs -- --no-default-features --features=frontend-wasm
	cp .npmrc-example ./target/wasm/nodejs/.npmrc

build-wasm-web: vendor init-javascript-minimal
	CARGO_TARGET_DIR=target/frontend-wasm $(WASM_PACK) build --release --target=web --out-dir=./target/wasm/web -- --no-default-features --features=frontend-wasm
	cp .npmrc-example ./target/wasm/web/.npmrc

build: build-python build-rust build-wasm-bundler build-wasm-nodejs build-wasm-web

# For now, we are going to purposely exclude `build-binary` from running
# in the general `build`  command. This is because the babycat command line
# app depends on dynamically linking to ALSA libraries on Ubuntu.
# We don't want to make `make build` fail if the user does not have
# those libraries.
build-binary: cargo-build-release-frontend-binary


# test ==============================================================

test-c: babycat.h cargo-build-release-frontend-c
	$(CC) -g -Wall -Werror=unused-function -o target/test_c tests-c/test.c target/frontend-c/release/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
	./target/test_c

test-c-valgrind: babycat.h cargo-build-release-frontend-c
	$(CC) -g -Wall -Werror=unused-function -o target/test_c tests-c/test.c target/frontend-c/release/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
	$(VALGRIND) --leak-check=full --show-leak-kinds=all ./target/test_c

test-python: install-babycat-python
	$(ACTIVATE_VENV_CMD) && pytest

test-rust: vendor
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) test --release --features=frontend-rust

test-wasm-nodejs: init-javascript build-wasm-nodejs
	cd tests-wasm-nodejs && $(NPM) run test

test: test-rust test-python test-wasm-nodejs test-c


# doctest ==========================================================

doctest-python: install-babycat-python
	$(ACTIVATE_VENV_CMD) && pytest tests-python/test_doctests.py

doctest-rust: vendor
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) test --release --doc

doctest: doctest-rust doctest-python


# bench =============================================================

bench-rust: vendor
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) bench

bench: bench-rust


# example ===========================================================

example-resampler-comparison: vendor
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) run --release --example resampler_comparison

example-decode-rust: vendor
	CARGO_TARGET_DIR=target/frontend-rust $(CARGO) run --release --example decode

example-decode-python: install-babycat-python
	$(ACTIVATE_VENV_CMD) && python3 examples-python/decode.py

example-decode-c: babycat.h cargo-build-release-frontend-c
	$(CC) -Wall -o target/decode_c examples-c/decode.c target/frontend-c/release/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
	./target/decode_c

example-decode-wasm: build-wasm-bundler
	cd examples-wasm/decode/ && $(NPM) install
	cd examples-wasm/decode/ && ./node_modules/.bin/webpack

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

docker-build-ubuntu-minimal: docker/ubuntu-minimal/.ti

docker-build-main: docker/main/.ti

docker-build-pip: docker/pip/.ti

docker-build: docker-build-cargo docker-build-ubuntu-minimal docker-build-main docker-build-pip


# docker run ========================================================

docker-run-docs-netlify:
	$(DOCKER_COMPOSE) run --rm netlify
