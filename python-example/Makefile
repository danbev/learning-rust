build:
	PYO3_NO_PYTHON=1 cargo b --lib --target wasm32-wasi --release

run:
	wasmtime \
	--dir=./target/wasm32-wasi/wasi-deps/usr::/usr \
        target/wasm32-wasi/release/python_example.wasm
