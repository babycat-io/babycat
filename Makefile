CBINDGEN ?= cbindgen
CARGO ?= cargo
CLANG_FORMAT ?= clang-format
NPM ?= npm
WASM_PACK ?= wasm-pack

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
	rm -rf target

# init ==============================================================

init-nodejs:
	cd tests-wasm-nodejs && $(NPM) rebuild && $(NPM) install

init-rust:
	rustup component add clippy rustfmt
	rustup target add wasm32-unknown-unknown
	cargo install cargo-valgrind cbindgen flamegraph wasm-pack

init: init-nodejs init-rust

# vendor ============================================================

vendor/.t: Cargo.toml $(wildcard */Cargo.toml)
	$(CARGO) vendor --versioned-dirs --quiet
	@touch vendor/.t

vendor: vendor/.t

# fmt ===============================================================

fmt-c:
	$(CLANG_FORMAT) -i tests-c/*.c

fmt-rust:
	$(CARGO) fmt

fmt: fmt-c fmt-rust

# fmt-check =========================================================

fmt-check-c:
	$(CLANG_FORMAT) --dry-run -Werror tests-c/*

fmt-check-rust:
	$(CARGO) fmt -- --check

fmt-check: fmt-check-rust

# lint ==============================================================

lint-rust: vendor
	$(CARGO) clippy --all-features

lint: lint-rust

# docs ==============================================================

docs-rust: vendor
	$(CARGO) doc --all-features --no-deps

docs: docs-rust

# build =============================================================

babycat.h:
	$(CBINDGEN) --quiet --output babycat.h
	$(CLANG_FORMAT) -i babycat.h

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

test-rust: vendor
	$(CARGO) test --features=frontend-rust

test-wasm-nodejs: build-wasm-nodejs
	cd tests-wasm-nodejs && $(NPM) run test

test: test-rust test-wasm-nodejs test-c


# bench =============================================================

bench-rust:
	$(CARGO) bench

bench: bench-rust

# example ===========================================================

example-resampler-comparison: vendor
	$(CARGO) run --release --example resampler_comparison
