FEATURES ?=

FS_NAMESPACE ?=


# You can enable Babycat's FFmpeg integration with these command line variables.
#
# To compile a copy of FFmpeg and statically link to it:
#	ENABLE_FFMPEG_BUILD_LINK_STATIC=1 make build-rust
#
# To statically link to an existing copy of FFmpeg on your system:
#	ENABLE_FFMPEG_LINK_STATIC=1 make build-rust
#
# To dynamically link to FFmpeg on your end user's system:
#	ENABLE_FFMPEG_LINK_DYNAMIC=1 make build-rust
#
# You can replace "build-rust" with any other Makefile command.
ifdef ENABLE_FFMPEG_BUILD_LINK_STATIC
	FEATURES += enable-ffmpeg-build-link-static,
	FS_NAMESPACE += ffmpeg-build-link-static/
else
	ifdef ENABLE_FFMPEG_LINK_STATIC
		FEATURES += enable-ffmpeg-link-static,
		FS_NAMESPACE += ffmpeg-link-static/
	else
		ifdef ENABLE_FFMPEG_LINK_DYNAMIC
			FEATURES += enable-ffmpeg-link-dynamic,
			FS_NAMESPACE += ffmpeg-link-dynamic/
		else
			FS_NAMESPACE += no-ffmpeg/
		endif
	endif
endif


# This is the Cargo/Rust build profile.
# You can specify a different value on the command line.
# To make a build with the "debug" profile, run the shell command:
#		PROFILE=debug make build-rust
PROFILE ?= release
ifeq ($(PROFILE),release)
	PROFILE_ARG := --release
else
	ifeq ($(PROFILE),debug)
		PROFILE_ARG :=
	else
		PROFILE_ARG := --profile=$(PROFILE)
	endif
endif


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
WHEEL_DIR ?= target/frontend-python/$(FS_NAMESPACE)$(PROFILE)
MANYLINUX_WHEEL_DIR ?= target/frontend-python--manylinux/$(FS_NAMESPACE)$(PROFILE)
PYTHON_CODE_PATHS ?= ./tests-python ./docs/source/conf.py


# Windows and Unix have different paths for activating
# Python virtualenvs.
# Note that once we have activated the venv, we do not need
# to use the Python path in $(PYTHON). The "python" command
# will automatically point to the right Python.
# TODO(jamesmishra): Handle the distinction between bash and cmd.
VENV_PATH ?= venv
ifeq ($(OS),Windows_NT)
	PYTHON ?= python
	CREATE_VENV_CMD ?= $(PYTHON) -m venv $(VENV_PATH)
	ACTIVATE_VENV_PATH ?= $(VENV_PATH)/Scripts/activate
	ACTIVATE_VENV_CMD ?= . $(ACTIVATE_VENV_PATH)
else
	PYTHON ?= python3
	CREATE_VENV_CMD ?= $(PYTHON) -m venv $(VENV_PATH)
	ACTIVATE_VENV_PATH ?= $(VENV_PATH)/bin/activate
	ACTIVATE_VENV_CMD ?= . $(ACTIVATE_VENV_PATH)
endif

# These are the Rust files being tracked by Git.
RUST_SRC_FILES ?= $(shell $(PYTHON) .listfiles.py src)

# These are the documentation files tracked by Git.
DOCS_FILES ?= $(shell $(PYTHON) .listfiles.py docs)

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


ifdef PYTHON_TEST_FILTER
	PYTEST_CMD ?= pytest -k $(PYTHON_TEST_FILTER)
else
	PYTEST_CMD ?= pytest
endif


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



# ===================================================================
# vendor ============================================================
# ===================================================================

## vendor-rust
.b/vendor-rust: Cargo.toml .cargo/config.toml
	$(CARGO) vendor --versioned-dirs --quiet
	@touch .b/vendor-rust
vendor-rust: .b/vendor-rust
.PHONY: vendor-rust

## vendor
vendor: vendor-rust
.PHONY: vendor



# ===================================================================
# init ==============================================================
# ===================================================================

## init-python
# Set up the Python virtualenv
$(VENV_PATH)/.ti:
	$(CREATE_VENV_CMD)
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --upgrade pip
	@touch $(VENV_PATH)/.ti

# Wrapper command for setting up the Python virtualenv
init-python: $(VENV_PATH)/.ti
.PHONY: init-python

# Install Python packages required to build Babycat's source into a Python extension wheel.
$(VENV_PATH)/.requirements-build.txt.ti: $(VENV_PATH)/.ti
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --requirement requirements-build.txt
	@touch $(VENV_PATH)/.requirements-build.txt.ti


init-python-requirements-build: $(VENV_PATH)/.requirements-build.txt.ti
.PHONY: init-python-requirements-build

# Install packages required to use Babycat.
$(VENV_PATH)/.requirements.txt.ti: $(VENV_PATH)/.ti
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --requirement requirements.txt
	@touch $(VENV_PATH)/.requirements.txt.ti

init-python-requirements: $(VENV_PATH)/.requirements.txt.ti
.PHONY: init-python-requirements

# Install packages required to lint and test Babycat's source code.
$(VENV_PATH)/.requirements-dev.txt.ti: $(VENV_PATH)/.ti
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --requirement requirements-dev.txt
	@touch $(VENV_PATH)/.requirements-dev.txt.ti

init-python-requirements-dev: $(VENV_PATH)/.requirements-dev.txt.ti
.PHONY: init-python-requirements-dev

# Install packages required to build Babycat's documentation.
$(VENV_PATH)/.requirements-docs.txt.ti: $(VENV_PATH)/.ti
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --requirement requirements-docs.txt
	@touch $(VENV_PATH)/.requirements-docs.txt.ti

init-python-requirements-docs: $(VENV_PATH)/.requirements-docs.txt.ti
.PHONY: init-python-requirements-docs


# Set up our main npm node_modules, containing developer tools
.b/init-javascript-tools: package.json	
	$(NPM) rebuild && $(NPM) install
	@touch .b/init-javascript-tools
init-javascript-tools: .b/init-javascript-tools
.PHONY: init-javascript-tools

## init-javascript-tests
# Set up our npm node_modules for testing
.b/init-javascript-tests: tests-wasm-nodejs/package.json
	cd tests-wasm-nodejs && $(NPM) rebuild && $(NPM) install
	@touch .b/init-javascript-tests
init-javascript-tests: .b/init-javascript-tests
.PHONY: init-javascript-tests

## init-javascript
init-javascript: .b/init-javascript-tools .b/init-javascript-tests
.PHONY: init-javascript

## init-rust
# All of the Rust tools needed for development.
.b/init-rust: .b/vendor-rust
	@touch .b/init-rust
init-rust: .b/init-rust
.PHONY: init-rust

## init-rustup-clippy
# only needed if linting code. not needed if we are only compiling.
.b/init-rustup-clippy:
	$(RUSTUP) component add clippy
	@touch .b/init-rustup-clippy
init-rustup-clippy: .b/init-rustup-clippy
.PHONY: init-rustup-clippy

## init-rustup-rustfmt
# only needed if formatting code. not needed if we are only compiling.
.b/init-rustup-rustfmt:
	$(RUSTUP) component add rustfmt
	@touch .b/init-rustup-rustfmt
init-rustup-rustfmt: .b/init-rustup-rustfmt
.PHONY: init-rustup-rustfmt

## init-rustup-wasm32-unknown-unknown
# only needed when compiling to WebAssembly.
.b/init-rustup-wasm32-unknown-unknown:
	$(RUSTUP) target add wasm32-unknown-unknown
	@touch .b/init-rustup-wasm32-unknown-unknown
init-rustup-wasm32-unknown-unknown: .b/init-rustup-wasm32-unknown-unknown
.PHONY: init-rustup-wasm32-unknown-unknown

# init-cargo-cbindgen
# Only needed when generating headers for the C bindings.
.b/init-cargo-cbindgen:
	$(CBINDGEN) --version > /dev/null 2>&1 || $(CARGO) install cbindgen
	@touch .b/init-cargo-cbindgen
init-cargo-cbindgen: .b/init-cargo-cbindgen
.PHONY: init-cargo-cbindgen

## init-cargo-wasm-pack
# enable the environment variable OPENSSL_NO_VENDOR=1 to
# use a pre-compiled OpenSSL already on the system.
.b/init-cargo-wasm-pack:
	$(WASM_PACK) --version > /dev/null 2>&1 || $(CARGO) install wasm-pack
	@touch .b/init-cargo-wasm-pack
init-cargo-wasm-pack: .b/init-cargo-wasm-pack
.PHONY: init-cargo-wasm-pack

## init-cargo-flamegraph
# Only needed if compiling and generating flamegraphs on this machine.
.b/init-cargo-flamegraph:
	$(CARGO) flamegraph --version > /dev/null 2>&1 || $(CARGO) install flamegraph
	@touch .b/init-cargo-flamegraph
init-cargo-flamegraph: .b/init-cargo-flamegraph
.PHONY: init-cargo-flamegraph

## init-cargo-valgrind
# Only needed if compiling and testing code using a pre-installed
# Valgrind binary on this machine.
.b/init-cargo-valgrind:
	$(CARGO) valgrind --version > /dev/null 2>&1 || $(CARGO) install cargo-valgrind
	@touch .b/init-cargo-valgrind
init-cargo-valgrind: .b/init-cargo-valgrind
.PHONY: init-cargo-valgrind

init: init-python init-javascript-tools init-javascript-tests init-rust

# ===================================================================
# fmt ===============================================================
# ===================================================================

## fmt-c
fmt-c:
	@$(CLANG_FORMAT) -i tests-c/*.c examples-c/*.c
.PHONY: fmt-c

## fmt-javascript
fmt-javascript: .b/init-javascript-tools
	@$(ESLINT) --fix $(JAVASCRIPT_CODE_PATHS)
	@$(PRETTIER) --write $(JAVASCRIPT_CODE_PATHS)
.PHONY: fmt-javascript

fmt-python: init-python-requirements-dev
	$(ACTIVATE_VENV_CMD) && black $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && isort $(PYTHON_CODE_PATHS)
.PHONY: fmt-python

## fmt-rust
fmt-rust: .b/init-rustup-rustfmt
	@$(CARGO) fmt
.PHONY: fmt-rust

## fmt
fmt: fmt-c fmt-javascript fmt-python fmt-rust
.PHONY: fmt



# ===================================================================
# fmt-check =========================================================
# ===================================================================

## fmt-check-c
fmt-check-c:
	@$(CLANG_FORMAT) --dry-run -Werror tests-c/*
.PHONY: fmt-check-c

## fmt-check-javascript
fmt-check-javascript: .b/init-javascript-tools
	@$(ESLINT) $(JAVASCRIPT_CODE_PATHS)
	@$(PRETTIER) --check --loglevel=silent $(JAVASCRIPT_CODE_PATHS)
.PHONY: fmt-check-javascript

fmt-check-python: init-python-requirements-dev
	$(ACTIVATE_VENV_CMD) && black --quiet $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && isort --quiet $(PYTHON_CODE_PATHS)
.PHONY: fmt-check-python

## fmt-check-rust
fmt-check-rust: .b/init-rustup-rustfmt
	@$(CARGO) fmt -- --check
.PHONY: fmt-check-rust

## fmt-check
fmt-check: fmt-check-c fmt-check-javascript fmt-check-python fmt-check-rust
.PHONY: fmt-check



# ===================================================================
# lint ==============================================================
# ===================================================================

lint-python: init-python-requirements-dev
	$(ACTIVATE_VENV_CMD) && pylint $(PYTHON_CODE_PATHS)
	$(ACTIVATE_VENV_CMD) && mypy $(PYTHON_CODE_PATHS)
.PHONY: lint-python

## lint-rust
lint-rust: .b/init-rustup-clippy .b/init-rust
	@CARGO_TARGET_DIR=target/all-features $(CARGO) clippy $(PROFILE_ARG) --all-features
.PHONY: lint-rust

## lint
lint: lint-python lint-rust
.PHONY: lint



# ===================================================================
# build =============================================================
# ===================================================================

## build-rust
target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT): .b/init-rust $(RUST_SRC_FILES)
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) build $(PROFILE_ARG) --no-default-features --features=frontend-rust,$(FEATURES)
	@touch target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
build-rust: target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
.PHONY: build-rust

## build-c
target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT): .b/init-rust $(RUST_SRC_FILES)
	CARGO_TARGET_DIR=target/frontend-c/$(FS_NAMESPACE) $(CARGO) build $(PROFILE_ARG) --no-default-features --features=frontend-c,$(FEATURES)
	@touch target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).${SHARED_LIB_EXT}
build-c: target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
.PHONY: build-c

## build-python
$(WHEEL_DIR)/*.whl: .b/init-rust $(VENV_PATH)/.requirements-build.txt.ti $(RUST_SRC_FILES)
	$(ACTIVATE_VENV_CMD) && maturin build --no-sdist --manifest-path=Cargo.toml --out=$(WHEEL_DIR) --cargo-extra-args="$(PROFILE_ARG) --no-default-features --features=frontend-python,$(FEATURES)"
#$(PYTHON) -m pip wheel --no-cache-dir --no-deps --wheel-dir=$(WHEEL_DIR) .
	@touch $(WHEEL_DIR)/*.whl
.b/build-python: $(WHEEL_DIR)/*.whl
	@touch .b/build-python
build-python: .b/build-python
.PHONY: build-python

target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT): .b/init-rust .b/init-rustup-wasm32-unknown-unknown $(RUST_SRC_FILES)
	CARGO_TARGET_DIR=target/frontend-wasm/$(FS_NAMESPACE) $(CARGO) build $(PROFILE_ARG) --no-default-features --features=frontend-wasm,$(FEATURES)
	@touch target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).${SHARED_LIB_EXT}

## build-wasm-bundler
target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/bundler/babycat_bg.wasm: .b/init-cargo-wasm-pack .npmrc-example target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
	CARGO_TARGET_DIR=target/frontend-wasm/$(FS_NAMESPACE) $(WASM_PACK) build $(PROFILE_ARG) --target=bundler --out-dir=./target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/bundler -- --no-default-features --features=frontend-wasm,$(FEATURES)
	cp .npmrc-example ./target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/bundler/.npmrc
build-wasm-bundler: target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/bundler/babycat_bg.wasm
.PHONY: build-wasm-bundler

## build-wasm-nodejs
target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/nodejs/babycat_bg.wasm: .b/init-cargo-wasm-pack .npmrc-example target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
	CARGO_TARGET_DIR=target/frontend-wasm/$(FS_NAMESPACE) $(WASM_PACK) build $(PROFILE_ARG) --target=nodejs --out-dir=./target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/nodejs -- --no-default-features --features=frontend-wasm,$(FEATURES)
	cp .npmrc-example ./target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/nodejs/.npmrc
build-wasm-nodejs: target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/nodejs/babycat_bg.wasm
.PHONY: build-wasm-nodejs

## build-wasm-web
target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/web/babycat_bg.wasm: .b/init-cargo-wasm-pack .npmrc-example target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
	CARGO_TARGET_DIR=target/frontend-wasm/$(FS_NAMESPACE) $(WASM_PACK) build $(PROFILE_ARG) --target=web --out-dir=./target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/web -- --no-default-features --features=frontend-wasm,$(FEATURES)
	cp .npmrc-example ./target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/nodejs/.npmrc
build-wasm-web: target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/web/babycat_bg.wasm
.PHONY: build-wasm-web

## build
build: build-python build-rust build-c build-wasm-bundler build-wasm-nodejs build-wasm-web
.PHONY: build

# ===================================================================
# extra build commands ==============================================
# ===================================================================

## babycat.h
babycat.h: .b/init-rust .b/init-cargo-cbindgen cbindgen.toml $(RUST_SRC_FILES)
	$(CBINDGEN) --quiet --output babycat.h && touch babycat.h

## build-python-manylinux
$(MANYLINUX_WHEEL_DIR)/*.whl: .b/docker-build-pip $(RUST_SRC_FILES)
	$(DOCKER_COMPOSE) run --rm --user=$$(id -u):$$(id -g) pip wheel --no-cache-dir --no-deps --wheel-dir=$(MANYLINUX_WHEEL_DIR) .
build-python-manylinux: $(MANYLINUX_WHEEL_DIR)/*.whl
.PHONY: build-python-manylinux

## build-binary
# For now, we are going to purposely exclude `build-binary` from running
# in the general `build`  command. This is because the babycat command line
# app depends on dynamically linking to ALSA libraries on Ubuntu.
# We don't want to make `make build` fail if the user does not have
# those libraries.
target/frontend-binary/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_BINARY_NAME): .b/init-rust $(RUST_SRC_FILES)
	CARGO_TARGET_DIR=target/frontend-binary/$(FS_NAMESPACE) $(CARGO) build $(PROFILE_ARG) --no-default-features --features=frontend-binary,$(FEATURES) --bin=babycat
	@touch target/frontend-binary/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_BINARY_NAME)
build-binary: target/frontend-binary/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_BINARY_NAME)
.PHONY: build-binary



# ===================================================================
# docs ==============================================================
# ===================================================================

.b/docs-sphinx: init-javascript-tools build-python-and-install init-python-requirements-docs build-wasm-bundler babycat.h $(RUST_SRC_FILES)
	rm -rf docs/build
	mkdir docs/build
	$(DOXYGEN)
	$(ACTIVATE_VENV_CMD) && export PATH=$(PWD)/node_modules/.bin:$$HOME/.bin:$$PATH && $(MAKE) -C docs dirhtml
	@touch .b/docs-sphinx
docs-sphinx: .b/docs-sphinx
.PHONY: docs-sphinx

## docs-sphinx-netlify
# This is the command we use to build docs on Netlify.
# The Netlify build image has Python 3.8 installed,
# but does not come with the virtualenv extension.
.b/docs-sphinx-netlify: .b/init-javascript-tools target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/bundler/babycat_bg.wasm babycat.h .b/build-python $(DOCS_FILES)
# Clean any previous builds.
	rm -rf docs/build
	mkdir docs/build
# Generate Doxygen XML to document Babycat's C bindings.
	$(DOXYGEN)
# Install Python dependencies for building the docs.
	python -m pip install -r requirements-docs.txt
# Install Babycat's Python bindings.
	python -m pip install --force-reinstall $(WHEEL_DIR)/*.whl
# Generate the docs.
	export PATH=$(PWD)/node_modules/.bin:$$PATH && $(MAKE) -C docs dirhtml
	@touch .b/docs-sphinx-netlify
docs-sphinx-netlify: .b/docs-sphinx-netlify
.PHONY: docs-sphinx-netlify


## docs-rustdoc
.b/docs-rustdoc: .b/init-rust $(RUST_SRC_FILES)
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) doc $(PROFILE_ARG) --no-default-features --features=frontend-rust,$(FEATURES) --no-deps
	@touch .b/docs-rustdoc
docs-rustdoc: .b/docs-rustdoc
.PHONY: docs-rustdoc

build-python-and-install: build-python init-python-requirements
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --no-deps --force-reinstall $(WHEEL_DIR)/*.whl
.PHONY: build-python-and-install

docs: .b/docs-sphinx .b/docs-rustdoc
.PHONY: docs


# ===================================================================
# install ===========================================================
# ===================================================================

## install-python-wheel
.b/install-python-wheel: .b/build-python $(VENV_PATH)/.requirements.txt.ti
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --no-deps --force-reinstall $(WHEEL_DIR)/*.whl
	@touch .b/install-python-wheel
install-python-wheel: .b/install-python-wheel
.PHONY: install-python-wheel

## install-python-wheel-manylinux
.b/install-python-wheel-manylinux: $(MANYLINUX_WHEEL_DIR)/*.whl $(VENV_PATH)/.requirements.txt.ti
	$(ACTIVATE_VENV_CMD) && $(PYTHON) -m pip install --no-deps --force-reinstall $(MANYLINUX_WHEEL_DIR)/*.whl
	@touch .b/install-python-wheel-manylinux
install-python-wheel-manylinux: .b/install-python-wheel-manylinux
.PHONY: install-python-wheel-manylinux



# ===================================================================
# test ==============================================================
# ===================================================================

## test-c
target/test_c: babycat.h tests-c/test.c target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
	$(CC) -g -Wall -Werror=unused-function -o target/test_c tests-c/test.c target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
test-c: target/test_c
	./target/test_c
.PHONY: test-c

## test-c-valgrind
test-c-valgrind: target/test_c
	$(VALGRIND) --leak-check=full --show-leak-kinds=all ./target/test_c
.PHONY: test-c-valgrind

## test-python
test-python: $(VENV_PATH)/.requirements-dev.txt.ti .b/install-python-wheel
	$(ACTIVATE_VENV_CMD) && $(PYTEST_CMD)
.PHONY: test-python

## test-python-manylinux
test-python-manylinux: $(VENV_PATH)/.requirements-dev.txt.ti .b/install-python-wheel-manylinux
	$(ACTIVATE_VENV_CMD) && $(PYTEST_CMD)
.PHONY: test-python-manylinux

## test-rust
test-rust: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) test $(PROFILE_ARG) --no-default-features --features=frontend-rust,$(FEATURES)
.PHONY: test-rust

## test-wasm-nodejs
test-wasm-nodejs: .b/init-javascript-tests target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/nodejs/babycat_bg.wasm
	cd tests-wasm-nodejs && $(NPM) run test
.PHONY: test-wasm-nodejs


## test
test: test-rust test-python test-wasm-nodejs test-c
.PHONY: test



# ===================================================================
# doctest ===========================================================
# ===================================================================

## doctest-python
doctest-python: $(VENV_PATH)/.requirements-dev.txt.ti .b/install-python-wheel
	$(ACTIVATE_VENV_CMD) && pytest tests-python/test_doctests.py
.PHONY: doctest-python

## doctest-rust
doctest-rust: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) test $(PROFILE_ARG) --doc
.PHONY: doctest-rust

## doctest
doctest: doctest-rust doctest-python
.PHONY: doctest



# ===================================================================
# bench =============================================================
# ===================================================================

bench-rust-decoding-batch-misc: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) bench --no-default-features --features=frontend-rust,$(FEATURES) --bench=decoding_batch_misc
.PHONY: bench-rust-decoding-batch-misc

## bench-rust-decoding-log-sweep
bench-rust-decoding-log-sweep: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) bench --no-default-features --features=frontend-rust,$(FEATURES) --bench=decoding_log_sweep
.PHONY: bench-rust-decoding-log-sweep

## bench-rust-decoding
bench-rust-decoding: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) bench --no-default-features --features=frontend-rust,$(FEATURES) --bench=decoding_benchmark
.PHONY: bench-rust-decoding

## bench-rust-resample-bc-lanczos
bench-rust-resample-bc-lanczos: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) bench --no-default-features --features=frontend-rust,$(FEATURES) --bench=resample_babycat_lanczos
.PHONY: bench-rust-resample-bc-lanczos

## bench-rust-resample-bc-sinc
bench-rust-resample-bc-sinc: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) bench --no-default-features --features=frontend-rust,$(FEATURES) --bench=resample_babycat_sinc
.PHONY: bench-rust-resample-bc-sinc

## bench-rust-resample-libsamplerate
bench-rust-resample-libsamplerate: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) bench --no-default-features --features=frontend-rust,$(FEATURES) --bench=resample_libsamplerate
.PHONY: bench-rust-resample-libsamplerate

## bench-rust-resample
bench-rust-resample: bench-rust-resample-bc-lanczos bench-rust-resample-bc-sinc bench-rust-resample-libsamplerate
.PHONY: bench-rust-resample

## bench-rust
bench-rust: bench-rust-decoding bench-rust-resample
.PHONY: bench-rust

## bench
bench: bench-rust
.PHONY: bench



# ===================================================================
# build-ex ==========================================================
# ===================================================================

## build-ex-resampler-comparison
target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/resampler_comparison: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) build $(PROFILE_ARG) --example resampler_comparison
	@touch target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/resampler_comparison
build-ex-resampler-comparison: target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/resampler_comparison
.PHONY: build-ex-resampler-comparison

## build-ex-decode-rust
target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/decode: .b/init-rust
	CARGO_TARGET_DIR=target/frontend-rust/$(FS_NAMESPACE) $(CARGO) build $(PROFILE_ARG) --example decode
	@touch target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/decode
build-ex-decode-rust: target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/decode
.PHONY: build-ex-decode-rust

## build-ex-decode-c
target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/examples/decode: babycat.h target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/$(BABYCAT_SHARED_LIB_NAME).$(SHARED_LIB_EXT)
	$(CC) -Wall -o target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/examples/decode examples-c/decode.c target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/${BABYCAT_SHARED_LIB_NAME}.${SHARED_LIB_EXT}
	@touch target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/examples/decode
build-ex-decode-c: target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/examples/decode
.PHONY: build-ex-decode-c

## build-ex-decode-wasm
target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/examples/decode/index.bundle.js: target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/bundler/babycat_bg.wasm
	mkdir -p target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/examples/decode
	cd examples-wasm/decode/ && $(NPM) rebuild && $(NPM) install && ./node_modules/.bin/webpack
	@touch target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/examples/decode/index.bundle.js
build-ex-decode-wasm: target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/examples/decode/index.bundle.js
.PHONY: build-ex-decode-wasm

## build-ex
build-ex: target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/resampler_comparison target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/decode target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/examples/decode target/frontend-wasm/$(FS_NAMESPACE)/$(PROFILE)/examples/decode/index.bundle.js
.PHONY: build-ex


# ===================================================================
# run-ex ============================================================
# ===================================================================

## run-ex-resampler-comparison
run-ex-resampler-comparison: target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/resampler_comparison
	target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/resampler_comparison
.PHONY: run-ex-resampler-comparison

## run-ex-decode-rust
run-ex-decode-rust: target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/decode
	target/frontend-rust/$(FS_NAMESPACE)/$(PROFILE)/examples/decode
.PHONY: run-ex-decode-rust

## run-ex-decode-c
run-ex-decode-c: target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/examples/decode
	target/frontend-c/$(FS_NAMESPACE)/$(PROFILE)/examples/decode
.PHONY: run-ex-decode-c

## run-ex-decode-python
run-ex-decode-python: .b/install-python-wheel
	$(ACTIVATE_VENV_CMD) && python examples-python/decode.py
.PHONY: build-ex-decode-python



# ===================================================================
# docker build ======================================================
# ===================================================================

## docker-build-cargo
.b/docker-build-cargo: docker-compose.yml docker/rust/Dockerfile
	$(DOCKER_COMPOSE) build cargo
	@touch .b/docker-build-cargo
docker-build-cargo: .b/docker-build-cargo
.PHONY: docker-build-cargo

## docker-build-ubuntu-minimal
.b/docker-build-ubuntu-minimal: .b/docker-build-cargo docker-compose.yml docker/ubuntu-minimal/Dockerfile
	$(DOCKER_COMPOSE) build ubuntu-minimal
	@touch .b/docker-build-ubuntu-minimal
docker-build-ubuntu-minimal: .b/docker-build-ubuntu-minimal
.PHONY: docker-build-ubuntu-minimal

## docker-build-main
.b/docker-build-main: .b/docker-build-ubuntu-minimal docker-compose.yml docker/main/Dockerfile
	$(DOCKER_COMPOSE) build main
	@touch .b/docker-build-main
docker-build-main: .b/docker-build-main
.PHONY: docker-build-main

## docker-build-pip
.b/docker-build-pip: .b/docker-build-cargo docker-compose.yml docker/pip/Dockerfile
	$(DOCKER_COMPOSE) build pip
	@touch .b/docker-build-pip
docker-build-pip: .b/docker-build-pip
.PHONY: docker-build-pip

## docker-build
docker-build: .b/docker-build-cargo .b/docker-build-ubuntu-minimal .b/docker-build-main .b/docker-build-pip
.PHONY: docker-build


# ===================================================================
# docker-run ========================================================
# ===================================================================

## docker-run-docs-netlify
docker-run-docs-netlify:
	$(DOCKER_COMPOSE) run --rm netlify
.PHONY: docker-run-docs-netlify
