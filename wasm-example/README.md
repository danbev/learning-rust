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

```console
$ python -m http.server
Serving HTTP on 0.0.0.0 port 8000 (http://0.0.0.0:8000/) ...
```
We can now acces http://0.0.0.0:8000/ and look at the console (CTRL+SHIFT+J) to
see the output:
```console
increment(2) : 3
```

### Notes
This example is very simple and only deals with a single i32 parameter which is
passed from JavaScript to the wasm module. Recall that currently wasm only
as support for for data types namely `i32`, `i64`, `f32`, and `f64`.

For more complex programs there is a project named `wasm-bindgen` which allows
JavaScript/Wasm interaction using strings, JS objects, classes.
