## Learning Rust
The sole purpose of this project is to learn the [Rust](http://www.rust-lang.org/) programming language.

### Installing rust
Install and use rustup which is similar to nvm.

### Rustup
Install Rust using rustup:
` `console
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

### Manual testing
```console
$ rustc snippets/src/readline.rs
$ ./readline
```

### Crate
A crate is a binary or library.
A package can have multiple binary crates by placing files in the src/bin
directory: each file will be a separate binary crate.

### Packages
A package contains one or more crate, it packages crates. Each package has a
Cargo.toml which describes how to package these crates.

### Modules
Allows for organizing code in a crate and can be used for making code private/
public.

src/main.rs and src/lib.rs are called crate roots. The reason for their name is
that the contents of either of these two files form a module named crate at the
root of the crate’s module structure, known as the module tree.

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
are called Tuple Structs and are used whey you want to have separate types but
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

### Error handling
`panic!` macro will by default unwind the program walking up the stack and
releasing resources as needed. This can be avoided if you are ok letting 
the OS to this (the process will just go away and you don't really have any
external resources that need cleaning). The you can add the following to your
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

