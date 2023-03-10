MODULE = wasm_example
build: 
	cargo build --target wasm32-unknown-unknown --release

size:
	@ls -sh target/wasm32-unknown-unknown/release/${MODULE}.wasm

compress: build
	wasm-gc target/wasm32-unknown-unknown/release/${MODULE}.wasm
