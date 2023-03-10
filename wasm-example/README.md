## wasm example
This example is intended to help me understand how compiling a Rust program
manually works and also act as quick reference to try things out in a simple
and familiar setting/application.

### Prerequisites

The wasm tool chain is required:
```console
$ rustup target add wasm32-unknown-unknown
```

wasm-gc is a tool used to compress .wasm files:
```console
$ cargo install wasm-gc
```

### Building
Compile:
```console
$ make build
```

Check the size:
```console
$ make size
1.7M target/wasm32-unknown-unknown/release/wasm_example.wasm
```

Compress:
```console
$ make size
4.0K target/wasm32-unknown-unknown/release/wasm_example.wasm
```
That is a huge difference and I was not expecting that. TODO: take a closer
look at `wasm-gc`.


### Running
So with the wasm module compiled we can run it. To do that we need a wasm
runtime which can be a browser for example, or could be a runtime like
wasmtime (and other runtimes as well).


