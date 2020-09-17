all: build

build:
	cargo build

docs:
	cargo doc --all --no-deps --target wasm32-unknown-unknown && cargo doc --open
