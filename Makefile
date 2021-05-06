CBINDGEN ?= cbindgen
CARGO ?= cargo
CLANG_FORMAT ?= clang-format
NPM ?= npm
PYTHON ?= python3
WASM_PACK ?= wasm-pack

WHEEL_CMD ?= wheel --no-cache-dir --no-deps --wheel-dir=target/python .
VENV_PATH ?= venv
CREATE_VENV_CMD ?= $(PYTHON) -m venv $(VENV_PATH)
PYTHON_CODE_PATHS ?= tests-python

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

.PHONY: help clean init-nodejs init-rust init vendor fmt-c fmt-rust fmt fmt-check-rust fmt-check lint-rust lint docs-rust docs babycat.h build-rust build-wasm-nodejs build-wasm-web build test-c test-rust test-wasm-nodejs test bench-rust bench example-resampler-comparison

# help ==============================================================

help:
	@cat makefile-help.txt

# clean =============================================================

clean:
	rm -rf target venv

# init ==============================================================

$(VENV_PATH)/.t:
	$(CREATE_VENV_CMD)
	$(ACTIVATE_VENV_CMD) && python -m pip install --upgrade pip
	$(ACTIVATE_VENV_CMD) && python -m pip install --requirement requirements-dev.txt
	@touch $(VENV_PATH)/.t

init-nodejs:
	cd tests-wasm-nodejs && $(NPM) rebuild && $(NPM) install

init-python: $(VENV_PATH)/.t

init-rust:
	rustup component add clippy rustfmt
	rustup target add wasm32-unknown-unknown
	cargo install cargo-valgrind cbindgen flamegraph wasm-pack

init: init-nodejs init-python init-rust

# vendor ============================================================

vendor/.t: Cargo.toml $(wildcard */Cargo.toml)
	$(CARGO) vendor --versioned-dirs --quiet
	@touch vendor/.t

vendor: vendor/.t

# fmt ===============================================================

fmt-c:
	$(CLANG_FORMAT) -i tests-c/*.c

fmt-python:
	$(ACTIVATE_VENV_CMD) && black $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && isort $(PYTHON_CODE_PATHS)

fmt-rust:
	$(CARGO) fmt

fmt: fmt-c fmt-python fmt-rust

# fmt-check =========================================================

fmt-check-c:
	$(CLANG_FORMAT) --dry-run -Werror tests-c/*

fmt-check-python:
	$(ACTIVATE_VENV_CMD) && black --quiet $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && isort --quiet $(PYTHON_CODE_PATHS)

fmt-check-rust:
	$(CARGO) fmt -- --check

fmt-check: fmt-check-c fmt-check-python fmt-check-rust

# lint ==============================================================

lint-python: init-python
	$(ACTIVATE_VENV_CMD) && pylint $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && mypy $(PYTHON_CODE_PATHS)

lint-rust: vendor
	$(CARGO) clippy --all-features

lint: lint-rust lint-python

# docs ==============================================================

docs-rust: vendor
	$(CARGO) doc --all-features --no-deps

docs: docs-rust

# build =============================================================

babycat.h:
	$(CBINDGEN) --quiet --output babycat.h
	$(CLANG_FORMAT) -i babycat.h

build-python: vendor init-python
	$(PYTHON) -m pip $(WHEEL_CMD)

build-rust: vendor
	$(CARGO) build --release --features=frontend-rust

build-wasm-nodejs: vendor
	$(WASM_PACK) build --release --target=nodejs --out-dir=./target/wasm/nodejs -- --no-default-features --features=frontend-wasm

build-wasm-web: vendor
	$(WASM_PACK) build --release --target=web --out-dir=./target/wasm/web -- --no-default-features --features=frontend-wasm

build: build-rust build-wasm-nodejs build-wasm-web

# test ==============================================================

test-c: vendor babycat.h
	$(CARGO) build --release --no-default-features --features=frontend-c
	$(CC) -g -Wall -Werror=unused-function -o target/release/test_c tests-c/test.c target/release/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
	./target/release/test_c

test-python: vendor init-python
	$(ACTIVATE_VENV_CMD) && python3 -m pip install .
	$(ACTIVATE_VENV_CMD) && pytest

test-rust: vendor
	$(CARGO) test --features=frontend-rust

test-wasm-nodejs: build-wasm-nodejs
	cd tests-wasm-nodejs && $(NPM) run test

test: test-rust test-python test-wasm-nodejs test-c


# bench =============================================================

bench-rust:
	$(CARGO) bench

bench: bench-rust

# example ===========================================================

example-resampler-comparison: vendor
	$(CARGO) run --release --example resampler_comparison
