## Rust Python wasi example
This is an example of a wasi module that is written in Rust and evaluates and
Python script/snippet.

The goal is just to verify that it is possible to run Python code from Rust
in a wasi module.

### Building
```console
$ make build
```

### Running
```console
$ make run 
wasmtime \
--dir=./target/wasm32-wasi/wasi-deps/usr::/usr \
        target/wasm32-wasi/release/python_example.wasm
Going to eval the following Python code:  print('Printing from Python!')
Printing from Python!
Python code executed successfully!
```
