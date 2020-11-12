## Learning Rust
The sole purpose of this project is to learn the [Rust](http://www.rust-lang.org/) programming language.

### Installing rust
Install and use rustup which is similar to nvm.

### Rustup
Install Rust using rustup:
```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup install nightly-x86_64-apple-darwin
```
Install nightly channel:
```console
$ rustup install nightly
```
Add wasi target to nightly:
```console
$ rustup target add wasm32-wasi --toolchain nightly
```
Build using nightly:
```console
$ cargo +nightyly build --target=wasm32-wasi
```

### Compiling
```console
$ cargo build --tests
```

### Running
```console
$ cargo run learning-rust
```

### Running tests
To run a test you have to have compiled using the `--tests` flag.
```console
$ cargo test -- --nocapture
```

Run a single test
```
$ cargo test boxing::tests::boxing_test
```

Ignore a test:
```rust
#[ignore]
```
You can run just the ignored annotated tests using:
```
$ cargo test -- --ignored
```

### Manual testing
```console
$ rustc snippets/src/readline.rs
$ ./readline
```

### Debugging
Debug symbols are enabled by default when using cargo build or cargo run 
without the `--release` flag.
```console
$ lldb -- ./target/debug/main
(lldb) br s -f main.rs -l 47
(lldb) r
```

You want to have the rust sources installed:
```console
$ rustup component add rust-src
```

### Crate
A crate is a binary or library.
A package can have multiple binary crates by placing files in the src/bin
directory: each file will be a separate binary crate.

### Packages
A package contains one or more crate, it packages crates. A crate is a binary
or a library.
Each package has a Cargo.toml which describes how to package these crates.

If the package directory contains src/lib.rs Cargo knows this is s library crate
with the same name as the package, and src/lib.rs is its crate root. Cargo
will pass src/lib.rs to rustc to build the library.

### Modules
Allows for organizing code in a crate and can be used for making code private/
public.

src/main.rs and src/lib.rs are called crate roots. The reason for their name is
that the contents of either of these two files form a module named crate at the
root of the crate’s module structure, known as the module tree.

For an example of a module see [module_path.rs](snippets/src/module_path.rs).

For example:
```rust
pub mod namespace {
  pub fun doit() {}
  fun doit() {}
}
```
So you can have public modules which are given a name, kind of like a namespace
in C++. The function defined in a module can be public or private.

The files src/lib.rs or src/main.rs are also modules which are named `crate` and
this is the reason they are called root modules. If you need to refer to a
module you can use `crate::module::function_name();` for example. This is called
an absoute path. You can also use relative paths using `self` or `super`

### use
Is used to bring module into scope so that we don't have to use the whole
path for it (similar to `using` in C++ I guess):
```rust
use crate::module_path::something_private::private_function;                
```
And after this we can just call `private_function();`. This also works with
a wildcard so the above could have been written as:
```rust
use crate::module_path::something_private::*;
```

### as
If there is a conflict when the same symbols are in a file because of using
`use` then one can alias them:
```rust
use crate::module_path::something_private::private_function;                   
use crate::module_path::something_private::private_function as bajja; 
```


### Path
A path how we identify functions/variables in modules. These paths can be absolute
or relative. An absolute path starts from the root; `crate::`, and a relative
path starts with `self::`, or `super::`. Super is like doing `cd ..` in a terminal.

### build.rs
By default Cargo looks for a "build.rs" file in a package root (even if you do
not specify a value for build). 
This file will be compiled and invoked before anything else is compiled in the
package.

If you want to print something that gets displayed when building you can use
`cargo:warning`:
```console
    println!("cargo:warning=BEVE................out_dir: {:?}", out_dir);
```

The output from build.rs (usage of println!) can be found in `target/debug/<pkg>output`.

As an example, in wasmtime there is a build.rs file that generates a file
that runs tests `target/debug/build/wasmtime-cli-83cc8a2a072b3d0d/out/wast_testsuite_tests.rs`.


### Smart pointers
Similar to smart pointers in C++. Smart pointers are ordinary structs that
implement the Deref and Drop traits.

First, we have references which just borrow the the value they point to:
```rust
let x = 18;
let y = &x;
```
This would be the same as if you did this in C/C++. You can print the memory 
address using:
```rust
println!("x: {}, y: {}, address: {:p}", x, y, y);
```
```console
x: 18, y: 18, &y: 0x700008ddc484
```

For heap allocated objects, like String, there is no deep copying done
automatically.
```rust
let s1 = String::from("hello");
let s2 = s1;
```
So both s1 and s2 are stack allocated String objects that point to the same
data. When this is done s1 will become null or something equivalent and no
longer valid to be referenced leading to a compile time error.
To create a copy you can use `clone`, but note that this will create a copy
of the data on the heap and the two String instances will point to different
places on the heap.


### String literals
String literals are stored inside the binary (text or data section?)
```rust
let s:&str = "Hello, world!";
```


#### Box<T>
In C++ we also have `std::unique_ptr` and Rust has something similar named Box.
This is for anything heap based, only the pointer itself is on the stack.
When the box goes out of scope, the pointer on the stack is cleaned up, as well
as the value on the heap. This is done by calling the Drop trait.

### Rc<T>
Reference counting type for multiple ownerships

### Ref<T> RefMut<T>


### Structs
We can declare a struct like this:
```rust
struct A {
    x: i32,
    y: i32
}
```
And we would create and access the members of such a struct like this:
```rust
let a = A {x:1, y:2};
assert_eq!(a.x, 1);
assert_eq!(a.y, 2);
```

Next we can also declare a struct like this:

```rust
struct C(i32, i32);
```
This struct will have two member named `0` and `1`.
```rust
let c = C(1, 2);
assert_eq!(c.0, 1);
assert_eq!(c.1, 2);
```
Notice that we create the struct with parentheses and not brackets. These structs
are called Tuple Structs and are used when you want to have separate types but
the names of the members is not important. 

### Troubleshooting
Wasmtime build issue:
```console
$ cargo build
error: failed to read `/work/wasm/wasmtime/crates/wasi-common/WASI/tools/witx/Cargo.toml`

Caused by:
  No such file or directory (os error 2)
```
```console
$ git pull --recurse-submodules
Already up to date.
```
```console
$ git submodule update --init --recursive
```
This last one did the trick. The issue might have been that this repository
in question has been moved to a different org (not 100% sure here)

### prelude
Rust includes:
```rust
extern crate std;
use std::prelude::v1::*;
```
This contents of v1 can be found [here](https://doc.rust-lang.org/std/prelude/v1/index.html).

### doc comment
You can add comments to a crate/module/functions using `//!` which will then be generated
using `cargo doc`.
```
$ cargo doc --open
```

Adding an examples section to a document comment will allow this example code
to be run using `cargo test`:
```rust
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
```

### Cargo install
This will install a binary into `$HOME/.cargo/bin`. 

### Cargo extensions
If a binary in your $PATH is named cargo-something, you can run it as if it was
a Cargo subcommand by running cargo something. So you could do cargo install to
install an extension and the be able to run it.

Use list to show all commands
```console
$ cargo --list
```

### Error handling
`panic!` macro will by default unwind the program walking up the stack and
releasing resources as needed. This can be avoided if you are ok letting 
the OS do this (the process will just go away and you don't really have any
external resources that need cleaning). Then you can add the following to your
Cargo.toml file:
```
[profile.release]
panic = 'abort'
```
Panic is used like this:
```rust
panic!("doh!");
```
You can use `RUST_BACKTRACE=1` to get a list of all functions that have been
called to get to the point where the panic happened.

Rather than `panic!` on an error, `?` will return the error value from the
current function for the caller to handle. For example:
```rust
let contents = fs::read_to_string(config.filename)?;
```

```rust
return Ok(());
or just
OK(())
```
This Ok(()) syntax might look a bit strange at first, but using () like this is
the idiomatic way to indicate that we’re calling run for its side effects only;
it doesn’t return a value we need.


### Closures

#### Fn trait
Borrows the values from the closure env immutably.

#### FnMut trait
Mutably borrows values from the closure env and can hence change them.

#### FnOnce trait
Takes ownership of the values and moves them. Is named once because the closure
cannot take ownership of the same variables more than once.



### Building rustc from source
```console
$ cp config.toml.example config.toml
```
I've set the following configuration options:
```
targets = "WebAssembly;X86"
```
When updating the configuration you (might) have to remove the `build` directory
for an updated configuration to take effect.

```console
$ ./x.py build -i --stage 2
```
`-i` specifies an incremental build

In the docs they mention that to have multiple toolchains installed you can
use rustup to link them. I'm still trying to figure out how I can build a compiler
with support for a wasm32-unknown-unknown, or wasm32-wasi target.

### Target triple
These are in the format:
```
<architecture-vendor-sys-abi>
```
Arcitectur: on linux systems uname -m
Vendor: unknown on linux, `pc` for Windows, and `apple` for OSX.
System: uname -s
ABI: On Linux, this refers to the libc implementation which you can find out with ldd --version. 

So for `wasm32-unknown-unknown`, `wasm32` is the arcitecture, no vendor is
specified, and so system is specified.
For `wasm32-wasi`

To see the supported targets:
```console
$ rustc --print target-list 
```


#### Troubleshooting
```console
Caused by:
  process didn't exit successfully: `/home/danielbevenius/work/wasm/enarx/demo/target/debug/build/wasmtime-basic-308ab90e55f39614/build-script-build` (exit code: 101)
--- stdout
Compiling Rust source to WASM...

--- stderr
error: linker `rust-lld` not found
  |
  = note: No such file or directory (os error 2)

error: aborting due to previous error

thread 'main' panicked at 'assertion failed: Command::new("rustc").arg("-C").arg("lto").arg("-C").arg("opt-level=3").arg("--target").arg("wasm32-unknown-unknown").arg("-o").arg(format!("{}/add.wasm",
                                                                                                                                            out)).arg("src/add.rs").status().expect("failed to compile WASM module").success()', wasmtime-basic/build.rs:39:9

```
I've not specified that lld should be compiled and made available in the sysroot,
perhaps doing that will allow for the 
I can see this executable is compiled:
```console
$ file ./build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld
```


```console
signalhandlers/SignalHandlers.hpp:5:10: fatal error: 'setjmp.h' file not found
signalhandlers/SignalHandlers.hpp:5:10: fatal error: 'setjmp.h' file not found, err: true
thread 'main' panicked at 'Unable to generate bindings: ()', /home/danielbevenius/.cargo/git/checkouts/wasmtime-5c699c1a3ee5d368/b7d86af/wasmtime-runtime/build.rs:32:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
```console
export BINDGEN_EXTRA_CLANG_ARGS="-I/usr/include" 
```

### Deno exploration
Building deno
```console
$ cargo build
```

Verifying the build:
```console
$ cargo run --bin deno -- --version
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/deno --version`
deno 1.4.6
v8 8.7.220.3
typescript 4.0.3
```

Run a script:
```console
$ ./target/debug/deno run std/examples/welcome.ts 
Check file:///home/danielbevenius/work/deno/deno/std/examples/welcome.ts
Welcome to Deno 🦕
```

Lets step through this and see how it works:
```console
$ rust-lldb -- ./target/debug/deno run std/examples/welcome.ts 
(lldb) br s -n main
(lldb) r
```

```console
$ file target/debug/deno
target/debug/deno: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=2eeb497fb5461d2d4fad67a430a9702e1cf91db7, for GNU/Linux 3.2.0, with debug_info, not stripped
$ ldd target/debug/deno
	linux-vdso.so.1 (0x00007fffb0796000)
	libdl.so.2 => /lib64/libdl.so.2 (0x00007f2629db6000)
	libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x00007f2629d9c000)
	librt.so.1 => /lib64/librt.so.1 (0x00007f2629d91000)
	libpthread.so.0 => /lib64/libpthread.so.0 (0x00007f2629d6f000)
	libm.so.6 => /lib64/libm.so.6 (0x00007f2629c29000)
	libc.so.6 => /lib64/libc.so.6 (0x00007f2629a60000)
	/lib64/ld-linux-x86-64.so.2 (0x00007f262f539000)
```

### Build scripts
One can place a build.rs file in the root of a project and cargo will compile
it and run it before the build. This can be used to compile C/C++ libraries.
For example, rusty-v8 uses a build script.


### async/await
Remember that asynchronous programming allows us to run multiple task concurrently
on the `same` thread. The goal is to reduce the overhead of using multiple threads.
Threads are supported at the operating system level and are easy to use (perhaps
minus synchronization issues) but that is not the case for async code which is
why the language or a library is required.

`async` 

To block and wait for an async function `block_on` can be used which will block
the current thread. `await` on the other hand will not block the current thread
but instead will asynchronously wait for the future to complete. So await will
allow other tasks to be scheduled on the same thread if the current task is
blocked and cannot make progress.

### Future
Is an async computation that can produce a value
