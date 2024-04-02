build:
	PYO3_NO_PYTHON=1 cargo b --lib --target wasm32-wasi --release

# This is needed to preopen the CPython standard libraries.
run:
	wasmtime \
	--dir=./target/wasm32-wasi/wasi-deps/usr::/usr \
        target/wasm32-wasi/release/python_example.wasm


dep: 
	PIP_REQUIRE_VIRTUALENV=0 python -m pip install --target deps \
	--only-binary :all: \
	--implementation py \
	--abi none \
       	--platform any \
	--python-version "3.11" \
       	-r requirements-convert.txt


download-sysroot:
	wget -P ./tmp https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/wasi-sysroot-20.0.tar.gz

download-clangrt:
	wget -P ./tmp https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/libclang_rt.builtins-wasm32-wasi-20.0.tar.gz

download-python:
	wget -P ./tmp https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/python%2F3.11.4%2B20230714-11be424/libpython-3.11.4-wasi-sdk-20.0.tar.gz
