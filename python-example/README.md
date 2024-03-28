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

### Wasi Python
There is a build.rs file that will download the python dependencies:
```rust
use wlr_libpy::bld_cfg::configure_static_libs;

fn main() {
    configure_static_libs().unwrap().emit_link_flags();
}
```
[configure_static_libs](https://github.com/vmware-labs/webassembly-language-runtimes/blob/5a40c308763309d227dd1dd85fe3e9397282c6d9/python/tools/wlr-libpy/src/bld_cfg.rs#L41).
This will download a wasi-sysroot, libclang_rt.builtins-wasm32-wasi.a, and

### Wasi sysroot
In traditional development for operating systems like Linux, Windows, or macOS,
the system root (often referred to as /usr) contains the libraries and tools
necessary for compiling and running applications. These include standard
libraries, header files, and other resources.
Similarly, a wasi-sysroot provides these components but tailored for WebAssembly
applications targeting WASI. 
* Standard C library implementations
* Header files for compiling code that uses WASI APIs

### libclang_rt
The compiler-rt project is a part of the LLVM project. It is a static library
that is part of the LLVM compiler infrastructure project. Specifically, it is a
runtime library for LLVM's Clang compiler that provides implementations of
low-level operations for the WebAssembly (wasm32) target. These low-level
operations include built-in functions that the compiler may use for various
purposes, such as arithmetic operations, bit manipulation, and other fundamental
operations that are not directly provided by the WebAssembly instruction set.


### Python Packages
A Python package can be distributed as a wheel which is basically an archive
of a build package, and there are also source distributions (sdist). The built
wheels can be dependent/compatible with a specific python version, OS, or
hardware arch.

When performing `pip install <packge>` pip will try to install a built wheel
and fall backe to a source distribution if a wheel is not available (at least
that is my understanding).
