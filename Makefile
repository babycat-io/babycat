PROFILE?=release
ifeq ($(PROFILE),release)
	PROFILE_FLAG:=--release
	WASM_PROFILE_FLAG:=$(PROFILE_FLAG)
else
	ifeq ($(PROFILE),debug)
		PROFILE_FLAG :=
		WASM_PROFILE_FLAG:=--dev
	else
		PROFILE_FLAG:=--profile=$(PROFILE)
		WASM_PROFILE_FLAG:=$(PROFILE_FLAG)
	endif
endif


FEATURE_ARG_BASE?=--no-default-features --features=
FFMPEG_FEATURES?=enable-ffmpeg-build

FRONTEND_RUST_FLAGS?=$(FEATURE_ARG_BASE)frontend-rust
FRONTEND_RUST_FFMPEG_FLAGS?=$(FEATURE_ARG_BASE)frontend-rust,$(FFMPEG_FEATURES)
FRONTEND_BINARY_FLAGS?=--bin=babycat $(FEATURE_ARG_BASE)frontend-binary
FRONTEND_BINARY_FFMPEG_FLAGS?=--bin=babycat $(FEATURE_ARG_BASE)frontend-binary,$(FFMPEG_FEATURES)
FRONTEND_PYTHON_FLAGS?=$(FEATURE_ARG_BASE)frontend-python
FRONTEND_PYTHON_FFMPEG_FLAGS?=$(FEATURE_ARG_BASE)frontend-python,$(FFMPEG_FEATURES)
FRONTEND_C_FLAGS?=$(FEATURE_ARG_BASE)frontend-c
FRONTEND_C_FFMPEG_FLAGS?=$(FEATURE_ARG_BASE)frontend-c,$(FFMPEG_FEATURES)
FRONTEND_WASM_FLAGS?=$(FEATURE_ARG_BASE)frontend-wasm

ifdef FEATURES
	FRONTEND_RUST_FLAGS:=$(FEATURES)
	FRONTEND_RUST_FFMPEG_FLAGS:=$(FEATURES)
	FRONTEND_BINARY_FLAGS:=$(FEATURES)
	FRONTEND_BINARY_FFMPEG_FLAGS:=$(FEATURES)
	FRONTEND_PYTHON_FLAGS:=$(FEATURES)
	FRONTEND_PYTHON_FFMPEG_FLAGS:=$(FEATURES)
	FRONTEND_C_FLAGS:=$(FEATURES)
	FRONTEND_C_FFMPEG_FLAGS:=$(FEATURES)
	FRONTEND_WASM_FLAGS:=-$(FEATURES)
endif

CARGO_TARGET_DIR?=$(PWD)/target

ALL_FEATURES_TARGET?=$(CARGO_TARGET_DIR)/all-features
FRONTEND_RUST_TARGET?=$(CARGO_TARGET_DIR)/frontend-rust
FRONTEND_RUST_FFMPEG_TARGET?=$(CARGO_TARGET_DIR)/frontend-rust-ffmpeg
FRONTEND_BINARY_TARGET?=$(CARGO_TARGET_DIR)/frontend-binary
FRONTEND_BINARY_FFMPEG_TARGET?=$(CARGO_TARGET_DIR)/frontend-binary-ffmpeg
FRONTEND_PYTHON_TARGET?=$(CARGO_TARGET_DIR)/frontend-python
FRONTEND_PYTHON_MANYLINUX_TARGET?=$(CARGO_TARGET_DIR)/frontend-python-manylinux
FRONTEND_PYTHON_FFMPEG_TARGET?=$(CARGO_TARGET_DIR)/frontend-python-ffmpeg
FRONTEND_C_TARGET?=$(CARGO_TARGET_DIR)/frontend-c
FRONTEND_C_FFMPEG_TARGET?=$(CARGO_TARGET_DIR)/frontend-c-ffmpeg
FRONTEND_WASM_TARGET?=$(CARGO_TARGET_DIR)/frontend-wasm

WHEEL_DIR?=$(FRONTEND_PYTHON_TARGET)/$(PROFILE)
MANYLINUX_WHEEL_DIR?=$(FRONTEND_PYTHON_MANYLINUX_TARGET)/$(PROFILE)
FFMPEG_WHEEL_DIR?=$(FRONTEND_PYTHON_FFMPEG_TARGET)/$(PROFILE)
WASM_DIR?=$(FRONTEND_WASM_TARGET)/$(PROFILE)

CARGO?=cargo
RUSTUP?=rustup
CBINDGEN?=cbindgen
CLANG_FORMAT?=clang-format
DOXYGEN?=doxygen
DOCKER?=docker
WASM_PACK?=wasm-pack
NPM?=npm
NODE_MODULES_PATH?=$(PWD)/node_modules
NODE_BIN?=$(NODE_MODULES_PATH)/.bin
ESLINT?=$(NODE_BIN)/eslint
PRETTIER?=$(NODE_BIN)/prettier

OS?=
ifeq ($(OS),Windows_NT)
	PYTHON?=python
else
	PYTHON?=python3
endif


# This is the shared library filename
# (excluding the extension, see SHARED_LIB_EXT below)
# that `cargo build` creates.
ifeq ($(OS),Windows_NT)
	BABYCAT_SHARED_LIB_NAME?=babycat
else
	BABYCAT_SHARED_LIB_NAME?=libbabycat
endif


# This is the filename for the babycat binary.
ifeq ($(OS),Windows_NT)
	BABYCAT_BINARY_NAME?=babycat.exe
else
	BABYCAT_BINARY_NAME?=babycat
endif


# This sets the file extension for linking to shared libraries.
# We typically use this when testing Babycat's C FFI bindings.
ifeq ($(OS),Windows_NT)
	SHARED_LIB_EXT?=lib
else
	ifeq ($(shell uname -s),Darwin)
		SHARED_LIB_EXT?=dylib
	else
		SHARED_LIB_EXT?=so
	endif
endif



FRONTEND_C_LIB?=$(FRONTEND_C_TARGET)/$(PROFILE)/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
FRONTEND_C_FFMPEG_LIB?=$(FRONTEND_C_FFMPEG_TARGET)/$(PROFILE)/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}

ENABLE_VENV?=1
VENV_DIR?=$(PWD)/venv
ifeq ($(ENABLE_VENV), 1)
	CREATE_VENV_CMD?=$(PYTHON) -m venv $(VENV_DIR)
	DELETE_VENV_CMD?=rm -rfv $(VENV_DIR)
	VENV_BIN?=$(VENV_DIR)/bin/
else
	CREATE_VENV_CMD?=
	DELETE_VENV_CMD?=
	VENV_BIN?=
endif

PIP?=$(VENV_BIN)pip
BLACK?=$(VENV_BIN)black
ISORT?=$(VENV_BIN)isort
MATURIN?=$(VENV_BIN)maturin
PYTEST?=$(VENV_BIN)pytest
SPHINX_BUILD?=$(VENV_BIN)sphinx-build

MATURIN_FLAGS?=--no-sdist --manifest-path=Cargo.toml

PYTHON_CODE_PATHS?=./tests-python ./benches-python ./docs/source/conf.py
JAVASCRIPT_CODE_PATHS?=./tests-wasm-nodejs/test.js

# ===================================================================
# help ==============================================================
# ===================================================================

help:
	@cat makefile-help.txt
.PHONY: help




# ===================================================================
# clean =============================================================
# ===================================================================

clean-caches:
	rm -rfv .b/* .ipynb_checkpoints .mypy_cache .pytest_cache tests-python/__pycache__
	find . -name '.DS_Store' -delete

clean-docs:
	rm -rfv docs/build docs/source/api/python/generated

clean-node-modules:
	rm -rfv $(NODE_MODULES_PATH) tests-wasm-nodejs/node_modules examples-wasm/decode/node_modules

clean-target:
	rm -rfv target babycat.h examples-wasm/decode/dist

clean-venv:
	$(DELETE_VENV_CMD)

clean: clean-caches clean-docs clean-node-modules clean-target clean-venv
.PHONY: clean clean-caches clean-docs clean-node-modules clean-target clean-venv




# ===================================================================
# init-python =======================================================
# ===================================================================

init-python-venv:
	$(CREATE_VENV_CMD)

init-python-build: init-python-venv requirements-build.txt
	$(PIP) install -r requirements-build.txt

init-python-deps: init-python-venv requirements.txt
	$(PIP) install -r requirements.txt

init-python-dev: init-python-venv requirements-dev.txt
	$(PIP) install -r requirements-dev.txt

init-python-docs: init-python-venv requirements-docs.txt
	$(PIP) install -r requirements-docs.txt

init-python: init-python-build init-python-deps init-python-dev init-python-docs
.PHONY: init-python init-python-venv init-python-build init-python-deps init-python-dev init-python-docs




# ===================================================================
# init-npm ==========================================================
# ===================================================================

init-npm-dev: package.json
	$(NPM) rebuild && $(NPM) install

init-npm-test: tests-wasm-nodejs/package.json
	cd tests-wasm-nodejs && $(NPM) rebuild && $(NPM) install

init-npm: init-npm-dev init-npm-test
.PHONY: init-npm init-npm-dev init-npm-test




# ===================================================================
# init-cargo ========================================================
# ===================================================================

init-cargo-clippy:
	$(RUSTUP) component add clippy

init-cargo-fmt:
	$(RUSTUP) component add rustfmt

init-cargo-wasm32-unknown-unknown:
	$(RUSTUP) target add wasm32-unknown-unknown

init-cargo-cbindgen:
	$(CARGO) install cbindgen

init-cargo: init-cargo-clippy init-cargo-fmt init-cargo-wasm32-unknown-unknown init-cargo-cbindgen
.PHONY: init-cargo init-cargo-clippy init-cargo-fmt init-cargo-wasm32-unknown-unknown init-cargo-cbindgen

init: init-python init-npm init-cargo
.PHONY: init




# ===================================================================
# init-docker =======================================================
# ===================================================================

init-docker-maturin:
	$(DOCKER) build -t babycat/maturin -f docker/maturin/Dockerfile .




# ===================================================================
# fmt ===============================================================
# ===================================================================

fmt-c:
	$(CLANG_FORMAT) -i tests-c/*.c examples-c/*.c

fmt-javascript: init-npm-dev
	$(ESLINT) --fix $(JAVASCRIPT_CODE_PATHS)
	$(PRETTIER) --write $(JAVASCRIPT_CODE_PATHS)

fmt-python: init-python-dev
	$(BLACK) $(PYTHON_CODE_PATHS)
	$(ISORT) $(PYTHON_CODE_PATHS)

fmt-rust: init-cargo-fmt
	$(CARGO) fmt

fmt: fmt-c fmt-javascript fmt-python fmt-rust
.PHONY: fmt fmt-c fmt-javascript fmt-python fmt-rust




# ===================================================================
# fmt-check =========================================================
# ===================================================================

fmt-check-c:
	$(CLANG_FORMAT) --dry-run -Werror tests-c/*

fmt-check-javascript: init-npm-dev
	$(ESLINT) $(JAVASCRIPT_CODE_PATHS)
	$(PRETTIER) --check --loglevel=silent $(JAVASCRIPT_CODE_PATHS)

fmt-check-python: init-python-dev
	$(BLACK) --quiet $(PYTHON_CODE_PATHS)
	$(ISORT) --quiet $(PYTHON_CODE_PATHS)

fmt-check-rust: init-cargo-fmt
	$(CARGO) fmt -- --check

fmt-check: fmt-check-c fmt-check-javascript fmt-check-python fmt-check-rust
.PHONY: fmt-check fmt-check-c fmt-check-javascript fmt-check-python fmt-check-rust




# ===================================================================
# lint ==============================================================
# ===================================================================

lint-python: init-python-dev
	$(PYLINT) $(PYTHON_CODE_PATHS)
	$(MYPY) $(PYTHON_CODE_PATHS)

lint-rust: init-cargo-clippy
	$(ALL_FEATURES_TARGET) $(CARGO) clippy $(PROFILE_FLAG) --all-features

lint: lint-python lint-rust
.PHONY: lint lint-python lint-rust




# ===================================================================
# build =============================================================
# ===================================================================

build-rust:
	CARGO_TARGET_DIR=$(FRONTEND_RUST_TARGET) $(CARGO) build $(PROFILE_FLAG) $(FRONTEND_RUST_FLAGS)

build-rust-ffmpeg:
	CARGO_TARGET_DIR=$(FRONTEND_RUST_FFMPEG_TARGET) $(CARGO) build $(PROFILE_FLAG) $(FRONTEND_RUST_FFMPEG_FLAGS)

build-rust-all-features:
	CARGO_TARGET_DIR=$(ALL_FEATURES_TARGET) $(CARGO) build $(PROFILE_FLAG) --all-features

build-binary:
	CARGO_TARGET_DIR=$(FRONTEND_BINARY_TARGET) $(CARGO) build $(PROFILE_FLAG) $(FRONTEND_BINARY_FLAGS)

build-binary-ffmpeg:
	CARGO_TARGET_DIR=$(FRONTEND_BINARY_FFMPEG_TARGET) $(CARGO) build $(PROFILE_FLAG) $(FRONTEND_BINARY_FFMPEG_FLAGS)

build-c-header: init-cargo-cbindgen
	$(CBINDGEN) --quiet --output=babycat.h

build-c: build-c-header
	CARGO_TARGET_DIR=$(FRONTEND_C_TARGET) $(CARGO) build $(PROFILE_FLAG) $(FRONTEND_C_FLAGS)

build-c-ffmpeg: build-c-header
	CARGO_TARGET_DIR=$(FRONTEND_C_FFMPEG_TARGET) $(CARGO) build $(PROFILE_FLAG) $(FRONTEND_C_FFMPEG_FLAGS)

build-python: init-python-build
	CARGO_TARGET_DIR=$(FRONTEND_PYTHON_TARGET) $(MATURIN) build --out="$(WHEEL_DIR)" $(MATURIN_FLAGS) --cargo-extra-args="$(PROFILE_FLAG) $(FRONTEND_PYTHON_FLAGS)"
build-python-manylinux: init-docker-maturin
	mkdir -p $(MANYLINUX_WHEEL_DIR)
	$(DOCKER) run --user=$$(id -u):$$(id -g) -eCARGO_TARGET_DIR=/tmp --volume="$(PWD):/src" --volume="$(MANYLINUX_WHEEL_DIR):/wheels" --workdir=/src babycat/maturin build --out=/wheels $(MATURIN_FLAGS) --cargo-extra-args="$(PROFILE_FLAG) $(FRONTEND_PYTHON_FLAGS)"

build-python-ffmpeg: init-python-build
	CARGO_TARGET_DIR=$(FRONTEND_PYTHON_FFMPEG_TARGET) $(MATURIN) build --out="$(FFMPEG_WHEEL_DIR)" $(MATURIN_FLAGS) --cargo-extra-args="$(PROFILE_FLAG) $(FRONTEND_PYTHON_FFMPEG_FLAGS)"

build-wasm-bundler:
	CARGO_TARGET_DIR=$(FRONTEND_WASM_TARGET) $(WASM_PACK) build $(WASM_PROFILE_FLAG) --target=bundler --out-dir=$(WASM_DIR)/bundler -- $(FRONTEND_WASM_FLAGS)

build-wasm-nodejs:
	CARGO_TARGET_DIR=$(FRONTEND_WASM_TARGET) $(WASM_PACK) build $(WASM_PROFILE_FLAG) --target=nodejs --out-dir=$(WASM_DIR)/nodejs -- $(FRONTEND_WASM_FLAGS)

build-wasm-web:
	CARGO_TARGET_DIR=$(FRONTEND_WASM_TARGET) $(WASM_PACK) build $(WASM_PROFILE_FLAG) --target=web --out-dir=$(WASM_DIR)/web -- $(FRONTEND_WASM_FLAGS)

build: build-rust build-rust-ffmpeg build-rust-all-features build-binary build-binary-ffmpeg build-c-header build-c build-c-ffmpeg build-python build-python-manylinux build-python-ffmpeg build-wasm-bundler build-wasm-nodejs build-wasm-web
.PHONY: build build-rust build-rust-ffmpeg build-rust-all-features build-binary build-binary-ffmpeg build-c-header build-c build-c-ffmpeg build-python build-python-manylinux build-python-ffmpeg build-wasm-bundler build-wasm-nodejs build-wasm-web




# ===================================================================
# build-ex ==========================================================
# ===================================================================

build-ex-rust-decode:
	CARGO_TARGET_DIR=$(FRONTEND_RUST_TARGET) $(CARGO) build $(PROFILE_FLAG) $(FRONTEND_RUST_FLAGS) --example=decode

build-ex-rust-resampler:
	CARGO_TARGET_DIR=$(FRONTEND_RUST_TARGET) $(CARGO) build $(PROFILE_FLAG) $(FRONTEND_RUST_FLAGS) --example=resampler_comparison

build-ex-c-decode: build-c
	$(CC) -Wall -o $(FRONTEND_C_TARGET)/$(PROFILE)/examples/decode examples-c/decode.c $(FRONTEND_C_LIB)

build-ex-wasm-decode: build-wasm-web
	mkdir -p $(FRONTEND_WASM_TARGET)/$(PROFILE)/examples/decode
	cd examples-wasm/decode/ && $(NPM) rebuild && $(NPM) install && ./node_modules/.bin/webpack

build-ex: build-ex-rust-decode build-ex-rust-resampler build-ex-c-decode build-ex-wasm-decode
.PHONY: build-ex build-ex-rust-decode build-ex-rust-resampler build-ex-c-decode build-ex-wasm-decode




# ===================================================================
# docs ==============================================================
# ===================================================================

docs-sphinx: clean-docs init-python-docs init-npm-dev build-c-header build-python-ffmpeg build-wasm-bundler
	mkdir docs/build
	$(DOXYGEN)
	$(PIP) install $(WHEEL_DIR)/*.whl && PATH=$(PATH):$(NODE_BIN) $(SPHINX_BUILD) -M dirhtml docs/source docs/build

docs-rustdoc:
	CARGO_TARGET_DIR=$(FRONTEND_RUST_FFMPEG_TARGET) $(CARGO) doc --no-deps $(PROFILE_FLAG) $(FRONTEND_RUST_FFMPEG_FLAGS)

docs: docs-sphinx docs-rustdoc
.PHONY: docs docs-sphinx docs-rustdoc




# ===================================================================
# test ==============================================================
# ===================================================================

test-c: build-c
	$(CC) -g -Wall -Werror=unused-function -o $(FRONTEND_C_TARGET)/test-c tests-c/test.c $(FRONTEND_C_LIB)
	$(FRONTEND_C_TARGET)/test-c

test-c-ffmpeg: build-c-ffmpeg
	$(CC) -g -Wall -Werror=unused-function -o $(FRONTEND_C_FFMPEG_TARGET)/test-c tests-c/test.c $(FRONTEND_C_FFMPEG_LIB)
	$(FRONTEND_C_FFMPEG_TARGET)/test-c

test-rust:
	CARGO_TARGET_DIR=$(FRONTEND_RUST_TARGET) $(CARGO) test $(PROFILE_FLAG) $(FRONTEND_RUST_FLAGS)

test-rust-ffmpeg:
	CARGO_TARGET_DIR=$(FRONTEND_RUST_FFMPEG_TARGET) $(CARGO) test $(PROFILE_FLAG) $(FRONTEND_RUST_FFMPEG_FLAGS)

test-python: init-python-dev build-python
	$(PIP) install $(WHEEL_DIR)/*.whl && $(PYTEST)

test-python-ffmpeg: init-python-dev build-python-ffmpeg
	$(PIP) install $(FFMPEG_WHEEL_DIR)/*.whl && $(PYTEST)

test-python-manylinux: init-python-dev build-python-manylinux
	$(PIP) install $(MANYLINUX_WHEEL_DIR)/*.whl && $(PYTEST)

test-wasm-nodejs: build-wasm-nodejs init-npm-test
	cd tests-wasm-nodejs && $(NPM) run test

test: test-rust test-rust-ffmpeg test-python test-python-ffmpeg test-wasm-nodejs test-c test-c-ffmpeg
.PHONY: test test-rust test-rust-ffmpeg test-python test-python-ffmpeg test-wasm-nodejs test-c test-c-ffmpeg




# ===================================================================
# doctest ===========================================================
# ===================================================================

doctest-python: build-python-ffmpeg
	$(PIP) install $(FFMPEG_WHEEL_DIR)/*.whl && $(PYTEST) tests-python/test_doctests.py

doctest-rust:
	CARGO_TARGET_DIR=$(FRONTEND_RUST_FFMPEG_TARGET) $(CARGO) test $(PROFILE_FLAG) $(FRONTEND_RUST_FFMPEG_FLAGS)

doctest: doctest-rust doctest-python
.PHONY: doctest doctest-python doctest-rust
