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
