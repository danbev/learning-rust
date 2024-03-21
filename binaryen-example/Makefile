build-lib:
	cargo b --lib --target wasm32-unknown-unknown --release

build: 
	cargo b --release

run-basic: 
	cargo r --bin basic --release --features "bin-deps"
	@echo -e "\nTest sample.wasm using Node.js"
	@node test-basic-wasm.js

run-update: 
	cargo r --bin update --release --features "bin-deps"
