CARGO ?= cargo
NPM ?= npm
WASM_PACK ?= wasm-pack

.PHONY: help clean init-nodejs init-rust init vendor fmt-rust fmt fmt-check-rust fmt-check lint-rust lint docs-rust docs build-rust build-wasm-nodejs build-wasm-web build test-rust test-wasm-nodejs test bench-rust bench example-resampler-comparison

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

fmt-rust:
	$(CARGO) fmt

fmt: fmt-rust

# fmt-check =========================================================

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

build-rust: vendor
	$(CARGO) build --release --features=frontend-rust

build-wasm-nodejs: vendor
	$(WASM_PACK) build --release --target=nodejs --out-dir=./target/wasm/nodejs -- --no-default-features --features=frontend-wasm

build-wasm-web: vendor
	$(WASM_PACK) build --release --target=web --out-dir=./target/wasm/web -- --no-default-features --features=frontend-wasm

build: build-rust build-wasm-nodejs build-wasm-web

# test ==============================================================

test-rust: vendor
	$(CARGO) test --features=frontend-rust

test-wasm-nodejs: build-wasm-nodejs
	cd tests-wasm-nodejs && $(NPM) run test

test: test-rust test-wasm-nodejs


# bench =============================================================

bench-rust:
	$(CARGO) bench

bench: bench-rust

# example ===========================================================

example-resampler-comparison: vendor
	$(CARGO) run --release --all-features --package babycat-lib-resample --example resampler_comparison
