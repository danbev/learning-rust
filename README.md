## Learning Rust
The sole purpose of this project is to learn the [Rust](http://www.rust-lang.org/) programming language.

### Startup
In [start.rs](./start.rs) there is an example of what the `start`
function. The main function that we write is note the entry of a rust program
which can be seen when setting a break point in main:
```console
(lldb) br s -n main
(lldb) r
(lldb) r
(lldb) bt
(lldb) bt
* thread #1, name = 'size', stop reason = breakpoint 1.1
  * frame #0: 0x000055555555ba74 size`size::main::hda5f8c9912e648ba at size.rs:7:5
    frame #1: 0x000055555555ba0b size`core::ops::function::FnOnce::call_once::h54200b7b2701b699((null)=(size`size::main::hda5f8c9912e648ba at size.rs:5), (null)=<unavailable>) at function.rs:227:5
    frame #2: 0x000055555555b96e size`std::sys_common::backtrace::__rust_begin_short_backtrace::hb77fb4371938fee9(f=(size`size::main::hda5f8c9912e648ba at size.rs:5)) at backtrace.rs:125:18
    frame #3: 0x000055555555b881 size`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbdbcf615a6363c6a at rt.rs:49:18
    frame #4: 0x000055555556ccb9 size`std::rt::lang_start_internal::h22ac7383c516f93e [inlined] core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h2aabc384aab89b7b at function.rs:259:13
    frame #5: 0x000055555556ccb2 size`std::rt::lang_start_internal::h22ac7383c516f93e [inlined] std::panicking::try::do_call::hc5fcacb7a85fc7b1 at panicking.rs:401
    frame #6: 0x000055555556ccb2 size`std::rt::lang_start_internal::h22ac7383c516f93e [inlined] std::panicking::try::hb5d9603af3abbe3a at panicking.rs:365
    frame #7: 0x000055555556ccb2 size`std::rt::lang_start_internal::h22ac7383c516f93e [inlined] std::panic::catch_unwind::h98fe6ac3925e64b4 at panic.rs:434
    frame #8: 0x000055555556ccb2 size`std::rt::lang_start_internal::h22ac7383c516f93e at rt.rs:34
    frame #9: 0x000055555555b860 size`std::rt::lang_start::h9f00871bee7a1abc(main=(size`size::main::hda5f8c9912e648ba at size.rs:5), argc=1, argv=0x00007fffffffd0b8) at rt.rs:48:5
    frame #10: 0x000055555555bb1c size`main + 28
    frame #11: 0x00007ffff7dbf1a3 libc.so.6`.annobin_libc_start.c + 243
    frame #12: 0x000055555555b76e size`_start + 46
```
By looking at the above we can see that `std::rt::lang_start` is called and
the file is `rt.rs` line 48. Which can be found in the Rust source code
reporistory in `./library/std/src/rt.rs':

```rust
#[cfg(not(test))]                                                               
#[lang = "start"]                                                               
fn lang_start<T: crate::process::Termination + 'static>(                        
    main: fn() -> T,                                                            
    argc: isize,                                                                
    argv: *const *const u8,                                                     
) -> isize {                                                                    
    lang_start_internal(                                                        
        &move || crate::sys_common::backtrace::__rust_begin_short_backtrace(main).report(),
        argc,                                                                   
        argv,                                                                   
    )                                                                           
    .into_ok()                                                                  
}                    
```
There are two attributes above which start with the `#` character. `lang` is
a language item which are special functions and types required internally by
the compiler.

```rust
fn lang_start_internal(                                                         
    main: &(dyn Fn() -> i32 + Sync + crate::panic::RefUnwindSafe),              
    argc: isize,                                                                
    argv: *const *const u8,                                                     
) -> Result<isize, !> {                                                         
    use crate::{mem, panic, sys, sys_common};                                   
    let rt_abort = move |e| {                                                   
        mem::forget(e);                                                         
        rtabort!("initialization or cleanup bug");                              
    };                                                                          
    // Guard against the code called by this function from unwinding outside of the Rust-controlled
    // code, which is UB. This is a requirement imposed by a combination of how the
    // `#[lang="start"]` attribute is implemented as well as by the implementation of the panicking
    // mechanism itself.                                                        
    //                                                                          
    // There are a couple of instances where unwinding can begin. First is inside of the
    // `rt::init`, `rt::cleanup` and similar functions controlled by libstd. In those instances a
    // panic is a libstd implementation bug. A quite likely one too, as there isn't any way to
    // prevent libstd from accidentally introducing a panic to these functions. Another is from
    // user code from `main` or, more nefariously, as described in e.g. issue #86030.
    // SAFETY: Only called once during runtime initialization.                  
    panic::catch_unwind(move || unsafe { sys_common::rt::init(argc, argv) }).map_err(rt_abort)?;
    let ret_code = panic::catch_unwind(move || panic::catch_unwind(main).unwrap_or(101) as isize)
        .map_err(move |e| {                                                     
            mem::forget(e);                                                     
            rtprintpanic!("drop of the panic payload panicked");                
            sys::abort_internal()                                               
        });                                                                     
    panic::catch_unwind(sys_common::rt::cleanup).map_err(rt_abort)?;            
    ret_code                                                                    
}                                              
```

```console
$ rustc -g start.rs 
$ ./start 
$ echo $?
18
```

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
The above tests is in the crate boxing, and in the module tests.

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

If the package directory contains src/lib.rs Cargo knows this is a library crate
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

### library
When you have a lib.rs and have included tests in those files there will be
an executable created that will contains the tests. This can be run manually:
```console
$ ./target/debug/snippets-fdf89e874d36062f

running 19 tests
test boxing::tests::boxing_test ... ok
test closures::tests::closure_test ... ok
test enums::tests::enums_test ... ok
test envvar::tests::envar_test ... ok
test factorial::fac_test ... ok
test clone::tests::format_test ... ok
test factorial::facrec_test ... ok
test hashmap::tests::options_test ... ok
test macros::tests::macro_test ... ok
test module_path::tests::module_path ... ok
test results::tests::result_test ... ok
test selection::selection_sort_test ... ok
test owner::tests::run_test ... ok
test structs::tests::struct_a ... ok
test string::tests::format_test ... ok
test structs::tests::struct_c ... ok
test traits::tests::traits_test ... ok
test vectors::tests::vector_test ... ok
test types::tests::types_test ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```
You can see the options using `--help`:
```console
$ ./target/debug/snippets-fdf89e874d36062f --help
```
Listing all the available tests:
```console
$ ./target/debug/snippets-fdf89e874d36062f --list
```

The archive for the library is a rlib file:
```console
$ ar t target/debug/libsnippets.rlib

```

### rlib
Is an archive for statically linked object files.

To list the contents:
```console
$ ar t target/debug/libsnippets.rlib
```
It will contains a number of object files, and a `lib.rmeta` file.

```console
$ cargo install rustfilt
$ readelf -s  target/debug/libsnippets.rlib | rustfilt
```

### use
Is used to bring a module into scope so that we don't have to use the whole
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
A path is how we identify functions/variables in modules. These paths can be absolute
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

First, we have references which just borrow the value it points to:
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

### str
```rust
let s = "bajja"
```
```console
(&str) $0 = "bajja" {
  data_ptr = 0x000055555558a034 "bajja"
  length = 5
}

(lldb) expr &s
(&str *) $1 = 0x00007fffffffcc80
```
So this is stack allocated.
```console
(lldb) disassemble 
stack`stack::main::h338f866a487ef61e:
    0x55555555b670 <+0>:  sub    rsp, 0x18
->  0x55555555b674 <+4>:  lea    rax, [rip + 0x2b985]
    0x55555555b67b <+11>: mov    qword ptr [rsp], rax
    0x55555555b67f <+15>: mov    qword ptr [rsp + 0x8], 0x5
    0x55555555b688 <+24>: mov    dword ptr [rsp + 0x14], 0xa
    0x55555555b690 <+32>: add    rsp, 0x18
    0x55555555b694 <+36>: ret
```
So we can first see that we are making room on the stack for 24 bytes (0x18),
the loading the contenst or rip+0x2b985 into $rax and then storing that onto
the stack

```console
(lldb) memory read --force -f x -c 5 -s 8 $rsp --num-per-line 1
0x7fffffffcca0: 0x00007ffff7d96c00
0x7fffffffcca8: 0x00007ffff7d96c00
0x7fffffffccb0: 0x0000555555596a70
0x7fffffffccb8: 0x000055555555b75b
0x7fffffffccc0: 0x0000555555596a50

(lldb) register read rax
     rax = 0x0000555555587000  
(lldb) memory read -f s 0x0000555555587000
0x555555587000: "bajja"
```
Notice that the value in $rax will be saved onto the stack:
```console
(lldb) memory read -f x -s 8 -c 1 --num-per-line 1 $rsp
0x7fffffffcca0: 0x0000555555587000
```
The next assembly instruction is storing the contant 5 into the next location
on the stack, which is the length of the string pointed to be. And this is
constistent with the contents of a str, there is a pointer to the string, and
there is the lenght of the string on the stack.

### String literals
String literals are stored inside the binary (text or data section?)
```rust
let s:&str = "Hello, world!";
```

### Inherited mutability
This is the case when you take a &mut reference to some struct allowing
modifying any fields of the struct.

### Interior mutability
This allows certain fields of a struct, those with type Cell<T> or RefCell<T>
to be modified using a normal reference.

### Box<T>
In C++ we also have `std::unique_ptr` and Rust has something similar named Box.
This is for anything heap based, only the pointer itself is on the stack.
When the box goes out of scope, the pointer on the stack is cleaned up, as well
as the value on the heap. This is done by calling the Drop trait.

### UnsafeCell
TODO:

### Cell
Allows for shared mutable containers in Rust. So normally you can only have a
single mutable reference but this allows multiple mutable pointers to the same
data.
Cell is generic so it expects a type to be specified when creating an instance
of it.
```rust
  let something = Something{id: 1, age: Cell::<u32>::new(45)};
```
But the type can also be inferred:
```rust
  let something = Something{id: 1, age: Cell::new(45)};
```

Cell::set can be used to set the value in the Cell.
Cell::get will return a copy of the contained value.

There is no way to get a pointer to the value inside the cell, all function
that manipulate the contained value done by the Cell. This means that there is
never any other pointers to the Call value which allows it to be mutated.

Notice that Call does not implement Sync which is declared like this:
```rust
impl<T: ?Sized> !Sync for Cell<T> {}
```

Example: [cell.rs](./snippets/src/cell.rs).

### Ref

### RefCell

### RefMut

### Rc<T>
Reference counting type for multiple ownerships. When you take a new refererence
using clone() the reference count will be incremented. Internally it uses a
Cell

### Ref<T> RefMut<T>


### Sync
Is a trait that specifies that pointer/reference to this type can be shared
between threads. If this should be prohibited the one can use !Sync.

To support access from multiple threads:
```rust
impl<T> Sync for Cell<T> {}
```
And to disable:
```rust
impl<T> !Sync for Cell<T> {}
```

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

### Foreign Function Interface

Example can be found in [ffi](./ffi).

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
You can add comments to a crate/module/functions using `//!` which will then be
generated using `cargo doc`.
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
a Cargo subcommand by running `cargo something`. So you could do cargo install to
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


### Native threads
Tasks provided by the OS, like 1:1 between a task and a thread. The OS can
handle the scheduling. A thread (task in linux) can be quite heavy and there
is a limit on the number of threads that can be created.

### Green threads
Is where a single OS thread can run multiple tasks. Are not part of the
overall system, instead the runtime handles the scheduling. Lighter weight than
native thread and you can create more of them.

### async/await
Remember that asynchronous programming allows us to run multiple task concurrently
on the `same` thread. The goal is to reduce the overhead of using multiple threads.
Threads are supported at the operating system level and are easy to use (perhaps
minus synchronization issues) but that is not the case for async code which is
why the language or a library is required.
Remember that Rust does not have a runtime like JavaScript for example. In
JavaScript promises can be run when they get created, but in Rust there has to
be something that does this. This is where the need for an executor for futures
need to be specified. One that would work is Tokio for example.


`async` 

To block and wait for an async function `block_on` can be used which will block
the current thread. `await` on the other hand will not block the current thread
but instead will asynchronously wait for the future to complete. So await will
allow other tasks to be scheduled on the same thread if the current task is
blocked and cannot make progress.

### Future
Is an async computation that can produce a value


### Trait
Is like an Inteface which can be implemented by multiple types.
Like C++ templates the compiler can generate a separate copy of an abstraction
for each way it is implemented.

### Ownership
Every value in Rust has a variable called its owner and there can only be one
owner at a time. When the variables goes out of scope the value will be dropped.
This sounds very much like a unique_ptr in C++ (RAII).

Values that are stored on the stack have a known size at compile time and can
be copied without any problems:
```rust
  let x = 22;                                                               
  let y = x;                                                                
  println!("x = {} {:p}, y = {} {:p}", x, &x, y, &y); 
```
```console
x = 22 0x7fadf91d9550, y = 22 0x7fadf91d9554
```
Notice that these are different memory locations and contain different values,
in this case both contain 22 but changing one will not affect the other.

All types that implement the `Copy` trait can be used as above and assigned to
other variables and the contents will be copied.

#### References
Allows for passing a value without taking ownership of it, the ownership stays
with the calling value outside of a function call for example. This is called
borrowing. When doing this we can't modify the value as we don't own it. But we
can specify that it should be a mutable reference and then we can change it.

### Lifetimes
These are annotations that start with `'` followed by a variable name.

```console
error[E0597]: `y` does not live long enough
 --> snippets/src/lifetimes.rs:7:15
  |
7 |           x = &y;
  |               ^^ borrowed value does not live long enough
8 |       }
  |       - `y` dropped here while still borrowed
9 |       println!("x = {}", x);
  |                          - borrow later used here

```
The rust compiler can figure out lifetimes so that we don't have to specify them
but only if the following are true:
* The functions does not return a reference
* There is exactly one reference input parameter

When we pass a variable as opposed to a reference we are giving up ownership.
When passing a variable as a reference you are lending it to the function. You
can pass around as many immutable references as you like with out any issue.

### Underscore
This can be used when one needs to specify a type but can let Rust determine
the template type, for example HashMap<_, _>. We might need to specify the type
of collection Vec, HashMap, but we still want Rust to determine the types the
collection holds.

### Dynamically sized types (DTS)
When the sizes of types in Rust are known at compile time they implement the
Sized trait. But not all type's sizes are known at compile type, for example
an array which is a member of a struct. 
Unsized struct pointers are double-width (fat-pointers) because they store a
pointer to the struct data and the size of the struct.
Unsized structs can only have 1 unsized field and it must be the last field in
the struct

### Sized trait
Is an autotrait which means that it gets implemented automatically if the trait
meets certain conditions.

```
?Sized
```
This means optionally sized or maybe sized.


trait object pointers are double-width because they store a pointer to the data
and a pointer to a vtable

### Intermediate representation
Rust has 3 intermediate representations, `High-Level (HIR)`, `Mid-Level (MIR)`,
and `LLVM IR (Low-Level)`.
```
                 (type checking) (borrow checking) (optimizations)
+------+     +------+     +-------+     +--------+     +--------------+
|Source|---->|  HIR |---->|  MIR  |---->| LLVM IR|---->| Machine code |
+------+     +------+     +-------+     +--------+     +--------------+
```


To see these intermediate representations the following command can be used:
```console
$ rustc +nightly -Zunpretty=normal main/src/basic.rs
```
This will just output the source as-is.

To show expanded macros and syntax extensions:
```console
$ ~/.cargo/bin/rustc +nightly -Zunpretty=expanded main/src/basic.rs
```

Show High-Level IR with types:
```console
$ ~/.cargo/bin/rustc +nightly -Zunpretty=hir,typed main/src/simple.rs
```

Show Mid-Level IR:
```console
$ ~/.cargo/bin/rustc +nightly -Zunpretty=mir,typed main/src/simple.rs
```

Show LLVM IR:
```console
$ rustc +nightly --emit llvm-ir main/src/simple.rs
```
This will generate a file named `basic.ll`.


### Building rustc manually
Build a specific branch
```console
$ git co -b 1.47.0 1.47.0
$ git submodule deinit -f .
$ git submodule update --init
```

```
### binutils
This will give cargo subcommands like `nm`, `objdump`, `readobj`:
```console
$ cargo install cargo-binutils
```

### Concurrency
The ability for a program to execute interleave and complete successfully
I/O bound
This is where async await comes into play.

### Parallism
The ability to run on multiple hardware threads at the same time
CPU bound
Rayon library migth be a good option?


### References
```rust
fn main() {
    let r: &str = &String::from("bajja");
}
```
```console
$ rustc -g reference.rs
```
```console
$ objdump -C --disassemble=reference::main reference

reference:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .text:

0000000000008410 <reference::main>:
    8410:	48 83 ec 28          	sub    $0x28,%rsp
    8414:	48 89 e7             	mov    %rsp,%rdi
    8417:	48 8d 35 9b bc 02 00 	lea    0x2bc9b(%rip),%rsi        # 340b9 <str.0+0x19>
    841e:	ba 05 00 00 00       	mov    $0x5,%edx
    8423:	e8 08 0f 00 00       	callq  9330 <<alloc::string::String as core::convert::From<&str>>::from>
    8428:	48 89 e7             	mov    %rsp,%rdi
    842b:	e8 c0 0e 00 00       	callq  92f0 <<alloc::string::String as core::ops::deref::Deref>::deref>
    8430:	48 89 c1             	mov    %rax,%rcx
    8433:	48 89 d6             	mov    %rdx,%rsi
    8436:	eb 00                	jmp    8438 <reference::main+0x28>
    8438:	48 89 e7             	mov    %rsp,%rdi
    843b:	e8 f0 0b 00 00       	callq  9030 <core::ptr::drop_in_place<alloc::string::String>>
    8440:	eb 26                	jmp    8468 <reference::main+0x58>
    8442:	48 89 e7             	mov    %rsp,%rdi
    8445:	e8 e6 0b 00 00       	callq  9030 <core::ptr::drop_in_place<alloc::string::String>>
    844a:	eb 10                	jmp    845c <reference::main+0x4c>
    844c:	48 89 c1             	mov    %rax,%rcx
    844f:	89 d0                	mov    %edx,%eax
    8451:	48 89 4c 24 18       	mov    %rcx,0x18(%rsp)
    8456:	89 44 24 20          	mov    %eax,0x20(%rsp)
    845a:	eb e6                	jmp    8442 <reference::main+0x32>
    845c:	48 8b 7c 24 18       	mov    0x18(%rsp),%rdi
    8461:	e8 2a cc ff ff       	callq  5090 <_Unwind_Resume@plt>
    8466:	0f 0b                	ud2    
    8468:	48 83 c4 28          	add    $0x28,%rsp
    846c:	c3                   	retq   

Disassembly of section .fini:
```
```console
$ rust-lldb -- reference
(lldb) br s -n main -f reference.rs
```

### Dynamic dispatch



### Size


### Set link args
```
$ rustc -C link-arg="-Wl,--verbose" size.rs
```

### Show link arguments
```console
$ RUSTC_LOG=rustc_codegen_ssa::back::link=info rustc -Z print-link-args -C link-arg='-Wl,--verbose' size.rs
 INFO rustc_codegen_ssa::back::link preparing Executable to "size"
"cc" "-m64" "size.size.ad1dfb40-cgu.0.rcgu.o" "size.size.ad1dfb40-cgu.1.rcgu.o" "size.size.ad1dfb40-cgu.2.rcgu.o" "size.size.ad1dfb40-cgu.3.rcgu.o" "size.size.ad1dfb40-cgu.4.rcgu.o" "size.size.ad1dfb40-cgu.5.rcgu.o" "size.size.ad1dfb40-cgu.6.rcgu.o" "size.size.ad1dfb40-cgu.7.rcgu.o" "size.size.ad1dfb40-cgu.8.rcgu.o" "size.size.ad1dfb40-cgu.9.rcgu.o" "size.4uog3iw28kovxxf1.rcgu.o" "-Wl,--as-needed" "-L" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5665011a98b2dd1d.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-292c1c33e047c187.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-1a282f8b292d9e3f.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-a54ae5159230894d.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-2de76061bb6a7faf.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-f144b5114d626180.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3ada49b85ba5941b.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-9d83ac27f983a7d6.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-943ba0c1e3f87a89.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0dbc7e011696d844.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-78b2343cc72ff57a.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-cc736a7495779f4b.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-72cab8079f9b3b1e.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4688b763605c6a0e.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-9ee1d5d15e6abbeb.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-52d5241975807511.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-9924c22ae1efcf66.rlib" "-Wl,--end-group" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-96219fb718f2f3e8.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "size" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-Wl,--verbose"
 INFO rustc_codegen_ssa::back::link "cc" "-m64" "size.size.ad1dfb40-cgu.0.rcgu.o" "size.size.ad1dfb40-cgu.1.rcgu.o" "size.size.ad1dfb40-cgu.2.rcgu.o" "size.size.ad1dfb40-cgu.3.rcgu.o" "size.size.ad1dfb40-cgu.4.rcgu.o" "size.size.ad1dfb40-cgu.5.rcgu.o" "size.size.ad1dfb40-cgu.6.rcgu.o" "size.size.ad1dfb40-cgu.7.rcgu.o" "size.size.ad1dfb40-cgu.8.rcgu.o" "size.size.ad1dfb40-cgu.9.rcgu.o" "size.4uog3iw28kovxxf1.rcgu.o" "-Wl,--as-needed" "-L" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5665011a98b2dd1d.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-292c1c33e047c187.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-1a282f8b292d9e3f.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-a54ae5159230894d.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-2de76061bb6a7faf.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-f144b5114d626180.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3ada49b85ba5941b.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-9d83ac27f983a7d6.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-943ba0c1e3f87a89.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0dbc7e011696d844.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-78b2343cc72ff57a.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-cc736a7495779f4b.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-72cab8079f9b3b1e.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4688b763605c6a0e.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-9ee1d5d15e6abbeb.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-52d5241975807511.rlib" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-9924c22ae1efcf66.rlib" "-Wl,--end-group" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-96219fb718f2f3e8.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "size" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-Wl,--verbose"
 INFO rustc_codegen_ssa::back::link linker stderr:

 INFO rustc_codegen_ssa::back::link linker stdout:
GNU ld version 2.32-33.fc31
  Supported emulations:
   elf_x86_64
   elf32_x86_64
   elf_i386
   elf_iamcu
   elf_l1om
   elf_k1om
   i386pep
   i386pe
using internal linker script:
==================================================
/* Script for -pie -z combreloc -z now -z relro -z separate-code: position independent executable, combine & sort relocs with separate code segment */
/* Copyright (C) 2014-2019 Free Software Foundation, Inc.
   Copying and distribution of this script, with or without modification,
   are permitted in any medium without royalty provided the copyright
   notice and this notice are preserved.  */
OUTPUT_FORMAT("elf64-x86-64", "elf64-x86-64",
	      "elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)
SEARCH_DIR("=/usr/x86_64-redhat-linux/lib64"); SEARCH_DIR("=/usr/lib64"); SEARCH_DIR("=/usr/local/lib64"); SEARCH_DIR("=/lib64"); SEARCH_DIR("=/usr/x86_64-redhat-linux/lib"); SEARCH_DIR("=/usr/local/lib"); SEARCH_DIR("=/lib"); SEARCH_DIR("=/usr/lib");
SECTIONS
{
  PROVIDE (__executable_start = SEGMENT_START("text-segment", 0)); . = SEGMENT_START("text-segment", 0) + SIZEOF_HEADERS;
  .interp         : { *(.interp) }
  .note.gnu.build-id  : { *(.note.gnu.build-id) }
  .hash           : { *(.hash) }
  .gnu.hash       : { *(.gnu.hash) }
  .dynsym         : { *(.dynsym) }
  .dynstr         : { *(.dynstr) }
  .gnu.version    : { *(.gnu.version) }
  .gnu.version_d  : { *(.gnu.version_d) }
  .gnu.version_r  : { *(.gnu.version_r) }
  .rela.dyn       :
    {
      *(.rela.init)
      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)
      *(.rela.fini)
      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)
      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)
      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)
      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)
      *(.rela.ctors)
      *(.rela.dtors)
      *(.rela.got)
      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)
      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)
      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)
      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)
      *(.rela.ifunc)
    }
  .rela.plt       :
    {
      *(.rela.plt)
      PROVIDE_HIDDEN (__rela_iplt_start = .);
      *(.rela.iplt)
      PROVIDE_HIDDEN (__rela_iplt_end = .);
    }
  . = ALIGN(CONSTANT (MAXPAGESIZE));
  .init           :
  {
    KEEP (*(SORT_NONE(.init)))
  }
  .plt            : { *(.plt) *(.iplt) }
.plt.got        : { *(.plt.got) }
.plt.sec        : { *(.plt.sec) }
  .text           :
  {
    *(.text.unlikely .text.*_unlikely .text.unlikely.*)
    *(.text.exit .text.exit.*)
    *(.text.startup .text.startup.*)
    *(.text.hot .text.hot.*)
    *(.text .stub .text.* .gnu.linkonce.t.*)
    /* .gnu.warning sections are handled specially by elf32.em.  */
    *(.gnu.warning)
  }
  .fini           :
  {
    KEEP (*(SORT_NONE(.fini)))
  }
  PROVIDE (__etext = .);
  PROVIDE (_etext = .);
  PROVIDE (etext = .);
  . = ALIGN(CONSTANT (MAXPAGESIZE));
  /* Adjust the address for the rodata segment.  We want to adjust up to
     the same address within the page on the next page up.  */
  . = SEGMENT_START("rodata-segment", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));
  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }
  .rodata1        : { *(.rodata1) }
  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }
  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }
  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }
  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }
  /* These sections are generated by the Sun/Oracle C++ compiler.  */
  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }
  /* Adjust the address for the data segment.  We want to adjust up to
     the same address within the page on the next page up.  */
  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));
  /* Exception handling  */
  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }
  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }
  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }
  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }
  /* Thread Local Storage sections  */
  .tdata	  :
   {
     PROVIDE_HIDDEN (__tdata_start = .);
     *(.tdata .tdata.* .gnu.linkonce.td.*)
   }
  .tbss		  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }
  .preinit_array    :
  {
    PROVIDE_HIDDEN (__preinit_array_start = .);
    KEEP (*(.preinit_array))
    PROVIDE_HIDDEN (__preinit_array_end = .);
  }
  .init_array    :
  {
    PROVIDE_HIDDEN (__init_array_start = .);
    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))
    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))
    PROVIDE_HIDDEN (__init_array_end = .);
  }
  .fini_array    :
  {
    PROVIDE_HIDDEN (__fini_array_start = .);
    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))
    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))
    PROVIDE_HIDDEN (__fini_array_end = .);
  }
  .ctors          :
  {
    /* gcc uses crtbegin.o to find the start of
       the constructors, so we make sure it is
       first.  Because this is a wildcard, it
       doesn't matter if the user does not
       actually link against crtbegin.o; the
       linker won't look for a file to match a
       wildcard.  The wildcard also means that it
       doesn't matter which directory crtbegin.o
       is in.  */
    KEEP (*crtbegin.o(.ctors))
    KEEP (*crtbegin?.o(.ctors))
    /* We don't want to include the .ctor section from
       the crtend.o file until after the sorted ctors.
       The .ctor section from the crtend file contains the
       end of ctors marker and it must be last */
    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))
    KEEP (*(SORT(.ctors.*)))
    KEEP (*(.ctors))
  }
  .dtors          :
  {
    KEEP (*crtbegin.o(.dtors))
    KEEP (*crtbegin?.o(.dtors))
    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))
    KEEP (*(SORT(.dtors.*)))
    KEEP (*(.dtors))
  }
  .jcr            : { KEEP (*(.jcr)) }
  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }
  .dynamic        : { *(.dynamic) }
  .got            : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }
  . = DATA_SEGMENT_RELRO_END (0, .);
  .data           :
  {
    *(.data .data.* .gnu.linkonce.d.*)
    SORT(CONSTRUCTORS)
  }
  .data1          : { *(.data1) }
  _edata = .; PROVIDE (edata = .);
  . = .;
  __bss_start = .;
  .bss            :
  {
   *(.dynbss)
   *(.bss .bss.* .gnu.linkonce.b.*)
   *(COMMON)
   /* Align here to ensure that the .bss section occupies space up to
      _end.  Align after .bss to ensure correct alignment even if the
      .bss section disappears because there are no input sections.
      FIXME: Why do we need it? When there is no .bss section, we do not
      pad the .data section.  */
   . = ALIGN(. != 0 ? 64 / 8 : 1);
  }
  .lbss   :
  {
    *(.dynlbss)
    *(.lbss .lbss.* .gnu.linkonce.lb.*)
    *(LARGE_COMMON)
  }
  . = ALIGN(64 / 8);
  . = SEGMENT_START("ldata-segment", .);
  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :
  {
    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)
  }
  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :
  {
    *(.ldata .ldata.* .gnu.linkonce.l.*)
    . = ALIGN(. != 0 ? 64 / 8 : 1);
  }
  . = ALIGN(64 / 8);
  _end = .; PROVIDE (end = .);
  . = DATA_SEGMENT_END (.);
  /* Stabs debugging sections.  */
  .stab          0 : { *(.stab) }
  .stabstr       0 : { *(.stabstr) }
  .stab.excl     0 : { *(.stab.excl) }
  .stab.exclstr  0 : { *(.stab.exclstr) }
  .stab.index    0 : { *(.stab.index) }
  .stab.indexstr 0 : { *(.stab.indexstr) }
  .comment       0 : { *(.comment) }
  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }
  /* DWARF debug sections.
     Symbols in the DWARF debugging sections are relative to the beginning
     of the section so we begin them at 0.  */
  /* DWARF 1 */
  .debug          0 : { *(.debug) }
  .line           0 : { *(.line) }
  /* GNU DWARF 1 extensions */
  .debug_srcinfo  0 : { *(.debug_srcinfo) }
  .debug_sfnames  0 : { *(.debug_sfnames) }
  /* DWARF 1.1 and DWARF 2 */
  .debug_aranges  0 : { *(.debug_aranges) }
  .debug_pubnames 0 : { *(.debug_pubnames) }
  /* DWARF 2 */
  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }
  .debug_abbrev   0 : { *(.debug_abbrev) }
  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }
  .debug_frame    0 : { *(.debug_frame) }
  .debug_str      0 : { *(.debug_str) }
  .debug_loc      0 : { *(.debug_loc) }
  .debug_macinfo  0 : { *(.debug_macinfo) }
  /* SGI/MIPS DWARF 2 extensions */
  .debug_weaknames 0 : { *(.debug_weaknames) }
  .debug_funcnames 0 : { *(.debug_funcnames) }
  .debug_typenames 0 : { *(.debug_typenames) }
  .debug_varnames  0 : { *(.debug_varnames) }
  /* DWARF 3 */
  .debug_pubtypes 0 : { *(.debug_pubtypes) }
  .debug_ranges   0 : { *(.debug_ranges) }
  /* DWARF Extension.  */
  .debug_macro    0 : { *(.debug_macro) }
  .debug_addr     0 : { *(.debug_addr) }
  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }
  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }
}


==================================================
/usr/bin/ld: mode elf_x86_64
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/Scrt1.o succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/Scrt1.o
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/crti.o succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/crti.o
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/crtbeginS.o succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/crtbeginS.o
attempt to open size.size.ad1dfb40-cgu.0.rcgu.o succeeded
size.size.ad1dfb40-cgu.0.rcgu.o
attempt to open size.size.ad1dfb40-cgu.1.rcgu.o succeeded
size.size.ad1dfb40-cgu.1.rcgu.o
attempt to open size.size.ad1dfb40-cgu.2.rcgu.o succeeded
size.size.ad1dfb40-cgu.2.rcgu.o
attempt to open size.size.ad1dfb40-cgu.3.rcgu.o succeeded
size.size.ad1dfb40-cgu.3.rcgu.o
attempt to open size.size.ad1dfb40-cgu.4.rcgu.o succeeded
size.size.ad1dfb40-cgu.4.rcgu.o
attempt to open size.size.ad1dfb40-cgu.5.rcgu.o succeeded
size.size.ad1dfb40-cgu.5.rcgu.o
attempt to open size.size.ad1dfb40-cgu.6.rcgu.o succeeded
size.size.ad1dfb40-cgu.6.rcgu.o
attempt to open size.size.ad1dfb40-cgu.7.rcgu.o succeeded
size.size.ad1dfb40-cgu.7.rcgu.o
attempt to open size.size.ad1dfb40-cgu.8.rcgu.o succeeded
size.size.ad1dfb40-cgu.8.rcgu.o
attempt to open size.size.ad1dfb40-cgu.9.rcgu.o succeeded
size.size.ad1dfb40-cgu.9.rcgu.o
attempt to open size.4uog3iw28kovxxf1.rcgu.o succeeded
size.4uog3iw28kovxxf1.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5665011a98b2dd1d.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5665011a98b2dd1d.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5665011a98b2dd1d.rlib)std-5665011a98b2dd1d.std.1836a641-cgu.0.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-292c1c33e047c187.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-292c1c33e047c187.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-292c1c33e047c187.rlib)panic_unwind-292c1c33e047c187.panic_unwind.3b3487cb-cgu.0.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-1a282f8b292d9e3f.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-1a282f8b292d9e3f.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-1a282f8b292d9e3f.rlib)miniz_oxide-1a282f8b292d9e3f.miniz_oxide.f4a3f7e6-cgu.0.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-a54ae5159230894d.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-a54ae5159230894d.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-a54ae5159230894d.rlib)adler-a54ae5159230894d.adler.bbc74789-cgu.0.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-2de76061bb6a7faf.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-2de76061bb6a7faf.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-2de76061bb6a7faf.rlib)object-2de76061bb6a7faf.object.1e43aa9b-cgu.0.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-f144b5114d626180.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-f144b5114d626180.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-f144b5114d626180.rlib)addr2line-f144b5114d626180.addr2line.9866fa2d-cgu.0.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3ada49b85ba5941b.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3ada49b85ba5941b.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3ada49b85ba5941b.rlib)gimli-3ada49b85ba5941b.gimli.58841ec5-cgu.0.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-9d83ac27f983a7d6.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-9d83ac27f983a7d6.rlib
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-943ba0c1e3f87a89.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-943ba0c1e3f87a89.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-943ba0c1e3f87a89.rlib)rustc_demangle-943ba0c1e3f87a89.rustc_demangle.577b3c67-cgu.0.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0dbc7e011696d844.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0dbc7e011696d844.rlib
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-78b2343cc72ff57a.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-78b2343cc72ff57a.rlib
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-cc736a7495779f4b.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-cc736a7495779f4b.rlib
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-72cab8079f9b3b1e.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-72cab8079f9b3b1e.rlib
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4688b763605c6a0e.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4688b763605c6a0e.rlib
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-9ee1d5d15e6abbeb.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-9ee1d5d15e6abbeb.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-9ee1d5d15e6abbeb.rlib)alloc-9ee1d5d15e6abbeb.alloc.9413cecc-cgu.0.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-52d5241975807511.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-52d5241975807511.rlib
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-9924c22ae1efcf66.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-9924c22ae1efcf66.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-9924c22ae1efcf66.rlib)core-9924c22ae1efcf66.core.7ca205ef-cgu.0.rcgu.o
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5665011a98b2dd1d.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-292c1c33e047c187.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-1a282f8b292d9e3f.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-a54ae5159230894d.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-2de76061bb6a7faf.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-f144b5114d626180.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3ada49b85ba5941b.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-9d83ac27f983a7d6.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-943ba0c1e3f87a89.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-0dbc7e011696d844.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-78b2343cc72ff57a.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-cc736a7495779f4b.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-72cab8079f9b3b1e.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4688b763605c6a0e.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-9ee1d5d15e6abbeb.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-52d5241975807511.rlib
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-9924c22ae1efcf66.rlib
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-96219fb718f2f3e8.rlib succeeded
/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-96219fb718f2f3e8.rlib
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-96219fb718f2f3e8.rlib)compiler_builtins-96219fb718f2f3e8.compiler_builtins.45456e86-cgu.107.rcgu.o
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-96219fb718f2f3e8.rlib)compiler_builtins-96219fb718f2f3e8.compiler_builtins.45456e86-cgu.110.rcgu.o
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-96219fb718f2f3e8.rlib)compiler_builtins-96219fb718f2f3e8.compiler_builtins.45456e86-cgu.111.rcgu.o
(/home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-96219fb718f2f3e8.rlib)compiler_builtins-96219fb718f2f3e8.compiler_builtins.45456e86-cgu.66.rcgu.o
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgcc_s.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgcc_s.a failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgcc_s.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgcc_s.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libgcc_s.so succeeded
opened script file /usr/lib/gcc/x86_64-redhat-linux/9/libgcc_s.so
/usr/lib/gcc/x86_64-redhat-linux/9/libgcc_s.so
opened script file /usr/lib/gcc/x86_64-redhat-linux/9/libgcc_s.so
attempt to open /lib64/libgcc_s.so.1 succeeded
/lib64/libgcc_s.so.1
attempt to open libgcc.a failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgcc.a failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgcc.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libgcc.a succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/libgcc.a
/usr/lib/gcc/x86_64-redhat-linux/9/libgcc.a
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libutil.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libutil.a failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libutil.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libutil.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libutil.so failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libutil.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libutil.so succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libutil.so
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librt.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librt.a failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librt.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librt.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/librt.so failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/librt.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/librt.so succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/librt.so
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpthread.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpthread.a failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpthread.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpthread.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libpthread.so failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libpthread.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libpthread.so succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libpthread.so
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libm.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libm.a failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libm.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libm.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libm.so failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libm.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libm.so succeeded
opened script file /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libm.so
/usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libm.so
opened script file /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libm.so
attempt to open /lib64/libm.so.6 succeeded
/lib64/libm.so.6
attempt to open /usr/lib64/libmvec_nonshared.a succeeded
/usr/lib64/libmvec_nonshared.a
attempt to open /lib64/libmvec.so.1 succeeded
/lib64/libmvec.so.1
/usr/lib64/libmvec_nonshared.a
/lib64/libmvec.so.1
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libdl.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libdl.a failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libdl.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libdl.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libdl.so failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libdl.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libdl.so succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libdl.so
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libc.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libc.a failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libc.so failed
attempt to open /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libc.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libc.so failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/libc.a failed
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libc.so succeeded
opened script file /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libc.so
/usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libc.so
opened script file /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/libc.so
attempt to open /lib64/libc.so.6 succeeded
/lib64/libc.so.6
attempt to open /usr/lib64/libc_nonshared.a succeeded
/usr/lib64/libc_nonshared.a
(/usr/lib64/libc_nonshared.a)elf-init.oS
(/usr/lib64/libc_nonshared.a)stat64.oS
(/usr/lib64/libc_nonshared.a)fstat64.oS
(/usr/lib64/libc_nonshared.a)lstat64.oS
(/usr/lib64/libc_nonshared.a)fstatat64.oS
attempt to open /lib64/ld-linux-x86-64.so.2 succeeded
/lib64/ld-linux-x86-64.so.2
/usr/lib64/libc_nonshared.a
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/crtendS.o succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/crtendS.o
attempt to open /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/crtn.o succeeded
/usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/crtn.o

warning: 2 warnings emitted


```
