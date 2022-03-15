# Profiles
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


# Features
FEATURE_ARG_BASE?=--no-default-features --features=
FFMPEG_FEATURES?=enable-ffmpeg-build

FRONTEND_RUST_FLAGS?=$(FEATURE_ARG_BASE)frontend-rust
FRONTEND_FFMPEG_RUST_FLAGS?=$(FEATURE_ARG_BASE)frontend-rust,$(FFMPEG_FEATURES)
FRONTEND_BINARY_FLAGS?=--bin=babycat $(FEATURE_ARG_BASE)frontend-binary
FRONTEND_FFMPEG_BINARY_FLAGS?=--bin=babycat $(FEATURE_ARG_BASE)frontend-binary,$(FFMPEG_FEATURES)
FRONTEND_PYTHON_FLAGS?=$(FEATURE_ARG_BASE)frontend-python
FRONTEND_FFMPEG_PYTHON_FLAGS?=$(FEATURE_ARG_BASE)frontend-python,$(FFMPEG_FEATURES)
FRONTEND_C_FLAGS?=$(FEATURE_ARG_BASE)frontend-c
FRONTEND_FFMPEG_C_FLAGS?=$(FEATURE_ARG_BASE)frontend-c,$(FFMPEG_FEATURES)
FRONTEND_WASM_FLAGS?=$(FEATURE_ARG_BASE)frontend-wasm

ifdef FEATURES
	FRONTEND_RUST_FLAGS:=$(FEATURES)
	FRONTEND_FFMPEG_RUST_FLAGS:=$(FEATURES)
	FRONTEND_BINARY_FLAGS:=$(FEATURES)
	FRONTEND_FFMPEG_BINARY_FLAGS:=$(FEATURES)
	FRONTEND_PYTHON_FLAGS:=$(FEATURES)
	FRONTEND_FFMPEG_PYTHON_FLAGS:=$(FEATURES)
	FRONTEND_C_FLAGS:=$(FEATURES)
	FRONTEND_FFMPEG_C_FLAGS:=$(FEATURES)
	FRONTEND_WASM_FLAGS:=-$(FEATURES)
endif

# Target directories
CARGO_TARGET_DIR?=$(PWD)/target
CARGO_TARGET_PROFILE_DIR?=$(CARGO_TARGET_DIR)/$(PROFILE)
WHEEL_DIR?=$(CARGO_TARGET_PROFILE_DIR)/python
MANYLINUX_WHEEL_DIR?=$(CARGO_TARGET_PROFILE_DIR)/python-manylinux
FFMPEG_WHEEL_DIR?=$(CARGO_TARGET_PROFILE_DIR)/python-ffmpeg
WASM_DIR?=$(CARGO_TARGET_PROFILE_DIR)/wasm

# Paths to commands that we depend on
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

# Python venv configuration.
OS?=
ifeq ($(OS),Windows_NT)
	PYTHON?=python
	VENV_SUBDIR?=Scripts
else
	PYTHON?=python3
	VENV_SUBDIR?=bin
endif

DISABLE_VENV?=0
VENV_DIR?=$(PWD)/venv
ifeq ($(DISABLE_VENV), 1)
	CREATE_VENV_CMD?=
	DELETE_VENV_CMD?=
	VENV_BIN?=
else
	CREATE_VENV_CMD?=$(PYTHON) -m venv $(VENV_DIR)
	DELETE_VENV_CMD?=rm -rfv $(VENV_DIR)
	VENV_BIN?=$(VENV_DIR)/$(VENV_SUBDIR)/
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

# The path to the Babycat shared library.
SHARED_LIB_FILENAME?=$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
SHARED_LIB_PATH?=$(CARGO_TARGET_PROFILE_DIR)/$(SHARED_LIB_FILENAME)
CARGO_TARGET_C_DIR=$(CARGO_TARGET_PROFILE_DIR)/c
CARGO_TARGET_FFMPEG_C_DIR=$(CARGO_TARGET_PROFILE_DIR)/ffmpeg-c
FRONTEND_C_LIB?=$(CARGO_TARGET_C_DIR)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
FRONTEND_FFMPEG_C_LIB?=$(CARGO_TARGET_FFMPEG_C_DIR)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)

# Tools in our Python venv
PIP?=$(VENV_BIN)pip
BLACK?=$(VENV_BIN)black
ISORT?=$(VENV_BIN)isort
MATURIN?=$(VENV_BIN)maturin
PYLINT?=$(VENV_BIN)pylint
MYPY?=$(VENV_BIN)mypy
PYTEST?=$(VENV_BIN)pytest
SPHINX_BUILD?=$(VENV_BIN)sphinx-build


PYTHON_CODE_PATHS?=./tests-python ./docs/source/conf.py
JAVASCRIPT_CODE_PATHS?=./tests-wasm-nodejs/test.js

# Commands to execute
# pip
PIP_INSTALL_WHEEL?=$(PIP) install $(WHEEL_DIR)/*.whl
PIP_INSTALL_MANYLINUX_WHEEL?=$(PIP) install $(MANYLINUX_WHEEL_DIR)/*.whl
PIP_FFMPEG_INSTALL_WHEEL?=$(PIP) install $(FFMPEG_WHEEL_DIR)/*.whl

# cargo
CARGO_BUILD?=$(CARGO) build $(PROFILE_FLAG)
CARGO_RUN?=$(CARGO) run $(PROFILE_FLAG)
CARGO_TEST?=$(CARGO) test $(PROFILE_FLAG)

# maturin
MATURIN_FLAGS?=--no-sdist --manifest-path=Cargo.toml
MATURIN_BUILD?=$(MATURIN) build $(MATURIN_FLAGS)

# wasm-pack
WASM_PACK_BUILD?=$(WASM_PACK) build $(WASM_PROFILE_FLAG)

# ===================================================================
# help ==============================================================
# ===================================================================

help:
	@cat makefile-help.txt
.PHONY: help




# ===================================================================
# clean =============================================================
# ===================================================================

clean-cache:
	rm -rfv .b/* .ipynb_checkpoints .mypy_cache .pytest_cache tests-python/__pycache__
	find . -name '.DS_Store' -delete
.PHONY: clean-cache

clean-docs:
	rm -rfv docs/build docs/source/api/python/generated
.PHONY: clean-docs

clean-node-modules:
	rm -rfv $(NODE_MODULES_PATH) tests-wasm-nodejs/node_modules examples-wasm/decode/node_modules
.PHONY: clean-node-modules

clean-target:
	rm -rfv target babycat.h examples-wasm/decode/dist
.PHONY: clean-target

clean-venv:
	$(DELETE_VENV_CMD)
.PHONY: clean-venv

clean: clean-cache clean-docs clean-node-modules clean-target clean-venv
.PHONY: clean




# ===================================================================
# init-python =======================================================
# ===================================================================

init-python-venv:
	$(CREATE_VENV_CMD)
.PHONY: init-python-venv

init-python-build: init-python-venv requirements-build.txt
	$(PIP) install -r requirements-build.txt
.PHONY: init-python-build

init-python-deps: init-python-venv requirements.txt
	$(PIP) install -r requirements.txt
.PHONY: init-python-deps

init-python-dev: init-python-venv requirements-dev.txt
	$(PIP) install -r requirements-dev.txt
.PHONY: init-python-dev

init-python-docs: init-python-venv requirements-docs.txt
	$(PIP) install -r requirements-docs.txt
.PHONY: init-python-docs

init-python: init-python-build init-python-deps init-python-dev init-python-docs




# ===================================================================
# init-npm ==========================================================
# ===================================================================

init-npm-dev: package.json
	$(NPM) rebuild && $(NPM) install
.PHONY: init-npm-dev

init-npm-test: tests-wasm-nodejs/package.json
	cd tests-wasm-nodejs && $(NPM) rebuild && $(NPM) install
.PHONY: init-npm-test

init-npm: init-npm-dev init-npm-test
.PHONY: init-npm




# ===================================================================
# init-cargo ========================================================
# ===================================================================

init-cargo-clippy:
	$(RUSTUP) component add clippy
.PHONY: init-cargo-clippy

init-cargo-fmt:
	$(RUSTUP) component add rustfmt
.PHONY: init-cargo-fmt

init-cargo-wasm32-unknown-unknown:
	$(RUSTUP) target add wasm32-unknown-unknown
.PHONY: init-cargo-wasm32-unknown-unknown

init-cargo-cbindgen:
	$(CARGO) install cbindgen || true
.PHONY: init-cargo-cbindgen

init-cargo: init-cargo-clippy init-cargo-fmt init-cargo-wasm32-unknown-unknown init-cargo-cbindgen
.PHONY: init-cargo

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
.PHONY: fmt-c

fmt-javascript: init-npm-dev
	$(ESLINT) --fix $(JAVASCRIPT_CODE_PATHS)
	$(PRETTIER) --write $(JAVASCRIPT_CODE_PATHS)
.PHONY: fmt-javascript

fmt-python: init-python-dev
	$(BLACK) $(PYTHON_CODE_PATHS)
	$(ISORT) $(PYTHON_CODE_PATHS)
.PHONY: fmt-python

fmt-rust: init-cargo-fmt
	$(CARGO) fmt
.PHONY: fmt-rust

fmt: fmt-c fmt-javascript fmt-python fmt-rust
.PHONY: fmt


# ===================================================================
# fmt-check =========================================================
# ===================================================================

fmt-check-c:
	$(CLANG_FORMAT) --dry-run -Werror tests-c/*
.PHONY: fmt-check-c

fmt-check-javascript: init-npm-dev
	$(ESLINT) $(JAVASCRIPT_CODE_PATHS)
	$(PRETTIER) --check --loglevel=silent $(JAVASCRIPT_CODE_PATHS)
.PHONY: fmt-check-javascript

fmt-check-python: init-python-dev
	$(BLACK) --quiet $(PYTHON_CODE_PATHS)
	$(ISORT) --quiet $(PYTHON_CODE_PATHS)
.PHONY: fmt-check-python

fmt-check-rust: init-cargo-fmt
	$(CARGO) fmt -- --check
.PHONY: fmt-check-rust

fmt-check: fmt-check-c fmt-check-javascript fmt-check-python fmt-check-rust
.PHONY: fmt-check



# ===================================================================
# lint ==============================================================
# ===================================================================

lint-python: init-python-dev
	$(PYLINT) $(PYTHON_CODE_PATHS)
	$(MYPY) $(PYTHON_CODE_PATHS)
.PHONY: lint-python

lint-rust: init-cargo-clippy
	$(ALL_FEATURES_TARGET) $(CARGO) clippy $(PROFILE_FLAG) --all-features
.PHONY: lint-rust

lint: lint-python lint-rust
.PHONY: lint 




# ===================================================================
# build =============================================================
# ===================================================================

build-rust:
	$(CARGO_BUILD) $(FRONTEND_RUST_FLAGS)
.PHONY: build-rust

build-ffmpeg-rust:
	$(CARGO_BUILD) $(FRONTEND_FFMPEG_RUST_FLAGS)
.PHONY: build-ffmpeg-rust

build-rust-all-features:
	$(CARGO_BUILD) --all-features
.PHONY: build-rust-all-features

build-binary:
	$(CARGO_BUILD) $(FRONTEND_BINARY_FLAGS)
.PHONY: build-binary

build-ffmpeg-binary:
	$(CARGO_BUILD) $(FRONTEND_FFMPEG_BINARY_FLAGS)
.PHONY: build-ffmpeg-binary

build-c-header: init-cargo-cbindgen
	$(CBINDGEN) --quiet --output=babycat.h
.PHONY: build-c-header

build-c: build-c-header
	mkdir -p "$(CARGO_TARGET_C_DIR)"
	$(CARGO_BUILD) $(FRONTEND_C_FLAGS) && cp "$(SHARED_LIB_PATH)" "$(FRONTEND_C_LIB)"
.PHONY: build-c

build-ffmpeg-c: build-c-header
	mkdir -p "$(CARGO_TARGET_FFMPEG_C_DIR)"
	$(CARGO_BUILD) $(FRONTEND_FFMPEG_C_FLAGS) && cp "$(SHARED_LIB_PATH)" "$(FRONTEND_FFMPEG_C_LIB)"
.PHONY: build-ffmpeg-c

build-python: init-python-build
	$(MATURIN_BUILD) --out="$(WHEEL_DIR)" --cargo-extra-args="$(PROFILE_FLAG) $(FRONTEND_PYTHON_FLAGS)"
.PHONY: build-python

build-python-manylinux: init-docker-maturin
	mkdir -p $(MANYLINUX_WHEEL_DIR)
	$(DOCKER) run --user=$$(id -u):$$(id -g) -eCARGO_TARGET_DIR=/tmp --volume="$(PWD):/src" --volume="$(MANYLINUX_WHEEL_DIR):/wheels" --workdir=/src babycat/maturin build --out=/wheels $(MATURIN_FLAGS) --cargo-extra-args="$(PROFILE_FLAG) $(FRONTEND_PYTHON_FLAGS)"
.PHONY: build-python-manylinux

build-ffmpeg-python: init-python-build
	$(MATURIN_BUILD) --out="$(FFMPEG_WHEEL_DIR)" --cargo-extra-args="$(PROFILE_FLAG) $(FRONTEND_FFMPEG_PYTHON_FLAGS)"
.PHONY: build-ffmpeg-python

build-wasm-bundler:
	$(WASM_PACK_BUILD) --target=bundler --out-dir="$(WASM_DIR)/bundler" -- $(FRONTEND_WASM_FLAGS)
.PHONY: build-wasm-bundler

build-wasm-nodejs:
	$(WASM_PACK_BUILD) --target=nodejs --out-dir="$(WASM_DIR)/nodejs" -- $(FRONTEND_WASM_FLAGS)
.PHONY: build-wasm-nodejs

build-wasm-web:
	$(WASM_PACK_BUILD) --target=web --out-dir="$(WASM_DIR)/web" -- $(FRONTEND_WASM_FLAGS)
.PHONY: build-wasm-web

build-wasm: build-wasm-bundler build-wasm-nodejs build-wasm-web
.PHONY: build-wasm

build: build-rust build-binary build-c build-python build-wasm
.PHONY: build

build-ffmpeg: build-ffmpeg-rust build-ffmpeg-binary build-ffmpeg-c build-ffmpeg-python
.PHONY: build-ffmpeg
# ===================================================================
# build-ex ==========================================================
# ===================================================================

build-ex-rust-decode:
	$(CARGO_BUILD) $(FRONTEND_RUST_FLAGS) --example=decode
.PHONY: build-ex-rust-decode

build-ex-rust-resampler:
	$(CARGO_BUILD) $(FRONTEND_RUST_FLAGS) --example=resampler_comparison
.PHONY: build-ex-rust-resampler

build-ex-c-decode: build-c
	$(CC) -Wall -o "$(CARGO_TARGET_PROFILE_DIR)/examples-c/decode" examples-c/decode.c "$(FRONTEND_C_LIB)"
.PHONY: build-ex-c-decode

build-ex-wasm-decode: build-wasm-web
	mkdir -p "$(CARGO_TARGET_PROFILE_DIR)/examples-wasm/decode"
	cd examples-wasm/decode/ && $(NPM) rebuild && $(NPM) install && ./node_modules/.bin/webpack
.PHONY: build-ex-wasm-decode

build-ex: build-ex-rust-decode build-ex-rust-resampler build-ex-c-decode build-ex-wasm-decode
.PHONY: build-ex

# ===================================================================
# run-ex ============================================================
# ===================================================================

run-ex-rust-decode:
	$(CARGO_RUN) $(FRONTEND_RUST_FLAGS) --example=decode
.PHONY: run-ex-rust-decode

run-ex-rust-resampler:
	$(CARGO_RUN) $(FRONTEND_RUST_FLAGS) --example=resampler_comparison
.PHONY: run-ex-rust-resampler

run-ex-c-decode: build-ex-c-decode
	"$(CARGO_TARGET_PROFILE_DIR)/examples-c/decode"
.PHONY: run-ex-c-decode

run-ex-wasm-decode: build-wasm-web
	mkdir -p "$(CARGO_TARGET_PROFILE_DIR)/examples-wasm/decode"
	cd examples-wasm/decode/ && $(NPM) rebuild && $(NPM) install && ./node_modules/.bin/webpack
.PHONY: run-ex-wasm-decode

run-ex: run-ex-rust-decode run-ex-rust-resampler run-ex-c-decode run-ex-wasm-decode
.PHONY: run-ex


# ===================================================================
# docs ==============================================================
# ===================================================================

docs-sphinx: clean-docs init-python-docs init-npm-dev build-c-header build-ffmpeg-python build-wasm-bundler
	mkdir docs/build
	$(DOXYGEN)
	$(PIP_INSTALL_WHEEL) && PATH="$(PATH):$(NODE_BIN)" $(SPHINX_BUILD) -M dirhtml docs/source docs/build
.PHONY: docs-sphinx

docs-rustdoc:
	$(CARGO) doc --no-deps $(PROFILE_FLAG) $(FRONTEND_FFMPEG_RUST_FLAGS)
.PHONY: docs-rustdoc

docs: docs-sphinx docs-rustdoc
.PHONY: docs




# ===================================================================
# test ==============================================================
# ===================================================================

test-c: build-c
	$(CC) -g -Wall -Werror=unused-function -o "$(CARGO_TARGET_PROFILE_DIR)/test-c" tests-c/test.c "$(FRONTEND_C_LIB)"
	"$(CARGO_TARGET_PROFILE_DIR)/test-c"
.PHONY: test-c

test-ffmpeg-c: build-ffmpeg-c
	$(CC) -g -Wall -Werror=unused-function -o $(CARGO_TARGET_PROFILE_DIR)/test-ffmpeg-c tests-c/test.c "$(FRONTEND_FFMPEG_C_LIB)"
	"$(CARGO_TARGET_PROFILE_DIR)/test-ffmpeg-c"
.PHONY: test-ffmpeg-c

test-rust:
	$(CARGO_TEST) $(FRONTEND_RUST_FLAGS)
.PHONY: test-rust

test-ffmpeg-rust:
	$(CARGO_TEST) $(FRONTEND_FFMPEG_RUST_FLAGS)
.PHONY: test-ffmpeg-rust

test-python: init-python-dev build-python
	$(PIP_INSTALL_WHEEL) && $(PYTEST)
.PHONY: test-python

test-ffmpeg-python: init-python-dev build-ffmpeg-python
	$(PIP_FFMPEG_INSTALL_WHEEL) && $(PYTEST)
.PHONY: test-ffmpeg-python

test-python-manylinux: init-python-dev build-python-manylinux
	$(PIP_INSTALL_MANYLINUX_WHEEL) && $(PYTEST)
.PHONY: test-python-manylinux

test-wasm-nodejs: build-wasm-nodejs init-npm-test
	cd tests-wasm-nodejs && $(NPM) run test
.PHONY: test-wasm-nodejs

test: test-rust test-python test-c test-wasm-nodejs
.PHONY: test

test-ffmpeg: test-ffmpeg-rust test-ffmpeg-python test-ffmpeg-c
.PHONY: test-ffmpeg




# ===================================================================
# doctest ===========================================================
# ===================================================================

doctest-python: build-ffmpeg-python
	$(PIP_FFMPEG_INSTALL_WHEEL) && $(PYTEST) tests-python/test_doctests.py

doctest-rust:
	$(CARGO_TEST) $(FRONTEND_FFMPEG_RUST_FLAGS) --doc

doctest: doctest-rust doctest-python
.PHONY: doctest doctest-rust doctest-python
