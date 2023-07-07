## Compiling Rust programatically
This is some prototyping code to programatically compile a Rust source file
using `--target=wasm32-wasi` and then turing it into a webassembly component
model module, followed by composing it with another, pre-built component.

### Setup
```console
$ rustup component add rustc-dev --toolchain nightly
```

### Building
```console
$ cargo b
```

### Running
```console
$ cargo r
```
This will generate the composed.wasm file in the root directory. This file can
be inspected using the following command:
```console
$ wasm-tools component wit composed.wasm 
package root:component

world root {
  import wasi:io/streams
  import wasi:filesystem/filesystem
  import wasi:cli-base/environment
  import wasi:cli-base/preopens
  import wasi:cli-base/exit
  import wasi:cli-base/stdin
  import wasi:cli-base/stdout
  import wasi:cli-base/stderr
  export run: func() -> string
}
```
