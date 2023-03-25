.PHONY: build test format

build:
	cargo build --release

test:
	cargo test

format:
	rustfmt -v --edition 2021 src/main.rs
