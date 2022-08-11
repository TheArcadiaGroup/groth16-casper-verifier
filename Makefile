prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p contract --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/contract.wasm

copy-wasm-file-to-client:
	mkdir -p client/wasm
	cp target/wasm32-unknown-unknown/release/*.wasm client/wasm

build: build-contract copy-wasm-file-to-client

clippy:
	cargo clippy --all-targets --all -- -A clippy::ptr_arg

check-lint: clippy
	cargo fmt --all -- --check

format:
	cargo fmt --all

lint: clippy format
	
clean:
	cargo clean
	rm -rf client/wasm/*.wasm