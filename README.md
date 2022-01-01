## Learning Rust
The sole purpose of this project is to learn the [Rust](http://www.rust-lang.org/) programming language.

### Contents
1. [Startup](#startup)
1. [Embedded Rust](./notes/embedded-rust.md)
1. [Pinning](#pin)

### Startup
In [main.rs](./startup/src/main.rs) is used in this section to walkthrough the
startup of a Rust program.

The main function that we write is not the entry of a rust program which can be
seen by inspecting the `start address` using objdump:
```console
$ objdump -f ./target/debug/startup 

./target/debug/startup:     file format elf64-x86-64
architecture: i386:x86-64, flags 0x00000150:
HAS_SYMS, DYNAMIC, D_PAGED
start address 0x0000000000007540
```
And we can see the function name of that address using:
```console
$ objdump -Cwd ./target/debug/startup | grep 0000000000007540
0000000000007540 <_start>:
```

So lets set a break point on that `_start`:
```console
$ rust-lldb --  ./target/debug/startup
(lldb) br s -n _start
Breakpoint 1: where = startup`_start, address = 0x0000000000007540
(lldb) r
(lldb) dis -F att
startup`_start:
->  0x55555555b540 <+0>:  endbr64 
    0x55555555b544 <+4>:  xorl   %ebp, %ebp
    0x55555555b546 <+6>:  movq   %rdx, %r9
    0x55555555b549 <+9>:  popq   %rsi
    0x55555555b54a <+10>: movq   %rsp, %rdx
    0x55555555b54d <+13>: andq   $-0x10, %rsp
    0x55555555b551 <+17>: pushq  %rax
    0x55555555b552 <+18>: pushq  %rsp
    0x55555555b553 <+19>: leaq   0x2a9e6(%rip), %r8        ; __libc_csu_fini
    0x55555555b55a <+26>: leaq   0x2a96f(%rip), %rcx       ; __libc_csu_init
    0x55555555b561 <+33>: leaq   0x208(%rip), %rdi         ; main
    0x55555555b568 <+40>: callq  *0x39612(%rip)
    0x55555555b56e <+46>: hlt
(lldb) bt
* thread #1, name = 'startup', stop reason = breakpoint 1.1
  * frame #0: 0x000055555555b540 startup`_start
```

Upon startup only the registers rsp and rdx contain valid data. rdx will
contain:
```text
%rdx         Contains a function pointer to be registered with `atexit'.
             This is how the dynamic linker arranges to have DT_FINI
             functions called for shared libraries that have been loaded
             before this code runs.

%rsp         The stack contains the arguments and environment:
             0(%rsp)                         argc
             LP_SIZE(%rsp)                   argv[0]
             ...
             (LP_SIZE*argc)(%rsp)            NULL
             (LP_SIZE*(argc+1))(%rsp)        envp[0]
             ...
                                             NULL
```
If we inspect rdx we find:
```console
(lldb) re r rdx
     rdx = 0x00007ffff7fdb7b0
(lldb) memory read -f x -c 1 -s 8 $rdx
0x7ffff7fdb7b0: 0xe5894855fa1e0ff3
```
But that does not look like a memory address and there is no function at that
location that can be disassembled:
```console
(lldb) dis -a 0xe5894855fa1e0ff3
error: Could not find function bounds for address 0xe5894855fa1e0ff3
```
Perhaps this is not used or I'm not understanding how it should be used. The
docs above refer to libraries that have been loaded before this code runs.

This is rest of the assembly code is setting up argument, 7 of them. 6 are
passed in registers and one on the stack for the  `__libc_start_main` function:
```c
int __libc_start_main(int *(main) (int, char * *, char * *),   // rdi
                      int argc,                                // rsi
                      char** ubp_av,                           // rdx
                      void (*init) (void),                     // rcx
                      void (*fini) (void),                     // r8
                      void (*rtld_fini) (void),                // r9
                      void (* stack_end));                     // stack
```
The call `callq  *0x39612(%rip)` is the actual call to this function. Now,
`__libc_start_main` does a bunch of things but I've documented that in
[program startup](https://github.com/danbev/learning-linux-kernel#program-startup).

So if we inspect the value of rdi which is the main function that will be called
by `__libc_start_main` it is:
```console
(lldb) register read rdi
     rdi = 0x000055555555ba50  startup`main

(lldb) dis -F att -a $rdi
startup`main:
    0x55555555ba50 <+0>:  pushq  %rax
    0x55555555ba51 <+1>:  movq   %rsi, %rdx
    0x55555555ba54 <+4>:  leaq   0x313a2(%rip), %rax       ; __rustc_debug_gdb_scripts_section__
    0x55555555ba5b <+11>: movb   (%rax), %al
    0x55555555ba5d <+13>: movslq %edi, %rsi
    0x55555555ba60 <+16>: leaq   -0x127(%rip), %rdi        ; startup::main::h737faa4c52471c41 at main.rs:3
    0x55555555ba67 <+23>: callq  0x55555555b870            ; std::rt::lang_start::h06519bdc8ab3e029 at rt.rs:57
    0x55555555ba6c <+28>: popq   %rcx
    0x55555555ba6d <+29>: retq 
```
So we can see that this is not our main function but one provided by the Rust
runtime library. If we inspect the generated llvm intermediate representation
(IR) we can see that a function named `main` is generated for us, and the our
`main` is named just_main::main. 
```console
$ rustc --emit=llvm-ir just-main.rs

; Function Attrs: nonlazybind                                                   
define i32 @main(i32 %0, i8** %1) unnamed_addr #6 {                             
top:                                                                            
  %2 = sext i32 %0 to i64                                                       
; call std::rt::lang_start                                                      
  %3 = call i64 @_ZN3std2rt10lang_start17h4bc6989bc23f981bE(void ()* @_ZN9just_main4main17h2bc67db3f4ca1ef7E, i64 %2, i8** %1)
  %4 = trunc i64 %3 to i32                                                      
  ret i32 %4                                                                    
}
```
Notice that the argument to std::rt::lang_lang is passed in rdi and that is the
main that we wrote:
```console
    0x55555555ba60 <+16>: leaq   -0x127(%rip), %rdi        ; startup::main::h737faa4c52471c41 at main.rs:3
    0x55555555ba67 <+23>: callq  0x55555555b870            ; std::rt::lang_start::h06519bdc8ab3e029 at rt.rs:57
```
So this is the function that will be called by `__libc_start_main` and as the
comment says its from the function std::rt::lang_start in
`library/std/src/rt.rs`.
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
the compiler. So this is calling `_rust_begin_short_backtrace(main).report(), so
what does that do?  
Notice that this is a closure that is passed into `lang_start_internal` and we
are not calling the funcion `rust_begin_stort_backtrace.

We can find that function in `library/std/src/sys_common/backtrace.rs`:
```rust
// Fixed frame used to clean the backtrace with `RUST_BACKTRACE=1`. Note that  
/// this is only inline(never) when backtraces in libstd are enabled, otherwise 
/// it's fine to optimize away.
#[cfg_attr(feature = "backtrace", inline(never))]                               
pub fn __rust_begin_short_backtrace<F, T>(f: F) -> T                            
where F: FnOnce() -> T, {                                                                               
    let result = f();                                                           
                                                                                
    // prevent this frame from being tail-call optimised away                   
    crate::hint::black_box(());                                                 
                                                                                
    result                                                                      
}         
```
So that that closure as the first argument, and argc, followed by argv
`lang_start_internal` will be called (in library/std/src/rt.rs):
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
Note that first sys_common::rt::init(argc, argv) is called which can be found in
library/std/src/sys_common/rt.rs:
```rust
pub unsafe fn init(argc: isize, argv: *const *const u8) {                       
    unsafe {                                                                    
        sys::init(argc, argv);                                                  
                                                                                
        let main_guard = sys::thread::guard::init();                            
        // Next, set up the current Thread with the guard information we just   
        // created. Note that this isn't necessary in general for new threads,  
        // but we just do this to name the main thread and to give it correct   
        // info about the stack bounds.                                         
        let thread = Thread::new(Some("main".to_owned()));                      
        thread_info::set(main_guard, thread);                                   
    }                                                                           
}          
```
So first we call `sys::init(argc, argv)` which in our case will be
std::sys::unit::init and can be found in library/std/src/sys/unix/mod.rs:
```rust
pub unsafe fn init(argc: isize, argv: *const *const u8) {                       
    // The standard streams might be closed on application startup. To prevent  
    // std::io::{stdin, stdout,stderr} objects from using other unrelated file  
    // resources opened later, we reopen standards streams when they are closed.
    sanitize_standard_fds();                                                    
                                                                                
    // By default, some platforms will send a *signal* when an EPIPE error         
    // would otherwise be delivered. This runtime doesn't install a SIGPIPE        
    // handler, causing it to kill the program, which isn't exactly what we        
    // want!                                                                    
    //                                                                          
    // Hence, we set SIGPIPE to ignore when the program starts up in order         
    // to prevent this problem.                                                 
    reset_sigpipe();                                                            
                                                                                
    stack_overflow::init();                                                     
    args::init(argc, argv);    
    ...
}
```
TODO: Digg into the args setup and the rest of the above init function.
Moving on to the next call in lang_start_internal which is:
```rust
    let ret_code = panic::catch_unwind(move || panic::catch_unwind(main).unwrap_or(101) as isize)
        .map_err(move |e| {                                                     
            mem::forget(e);                                                     
            rtprintpanic!("drop of the panic payload panicked");                
            sys::abort_internal()                                               
        });                                                                     
```
This is using `catch_undwind` and there is a standalone example
[unwind.rs](./src/unwind.rs) which might be helpful to take a look at and run to
better understand what is happening here. Taking this apart a little so it is a
little easier to understand we are passing a closure to the first call to
panic::catch_unwind, and this closure will call
panic::catch_unwind(main).unwrap_or(101) as isize) when it is called.

catch_unwind will call the closure passed in and return Ok with the result of
the closure if there is no panic from that call. If there is a panic then
catch_unwind will return Err(cause). In our this case we are also using
unwrap_or(101), so if the closure panics then 101 will be returned by this
cloure. After that map_err is used to just pass through an Ok result, but if the
result contains Err the closure passed in will be run.
So this is the point where the main function that we wrote is called which
was the point of this section!

After that we have:
```rust
    panic::catch_unwind(sys_common::rt::cleanup).map_err(rt_abort)?;            
    ret_code                                                                    
```
`library/std/src/sys_common/rt.rs`:
```rust
#[cfg_attr(test, allow(dead_code))]                                             
pub fn cleanup() {                                                              
    static CLEANUP: Once = Once::new();                                         
    CLEANUP.call_once(|| unsafe {                                               
        // Flush stdout and disable buffering.                                  
        crate::io::cleanup();                                                   
        // SAFETY: Only called once during runtime cleanup.                     
        sys::cleanup();                                                         
    });                                                                         
}
```

When using the standard library in Rust this will link with libc and that means
that start up will follow the [details](https://github.com/danbev/learning-linux-kernel#program-startup)
I've gone through before.

We can override the `start` function and an example can be found in [start.rs](./src/start.rs):
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

So notice that the syntax here is `&str`. That is a type of reference which
as we've seen is a pointer to an array and then a length. So is `str` a
struct?

library/core/src/str/mod.rs:
```rust
#[lang = "str"]                                                                 
#[cfg(not(test))]                                                               
impl str { 
```
The #[lang = "str"] is an attribute. When the compiler sees `str` in code it
knows that is should call this implementation.
This is called a language item.

### lang_items

rustc_hir/src/lang_items.rs:
```rust
language_item_table! {                                                          
//  Variant name,            Name,                     Method name,                Target                  Generic requirements;
...
OwnedBox,                    sym::owned_box,           owned_box,                  Target::Struct,         GenericRequirement::Minimum(1);
..

}
```



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

Lets take a simple example, [box.rs](./src/box.rs) and look at how a create a new
Box. This is done using the `new`method:
```rust
impl<T> Box<T> {                                                                
    /// Allocates memory on the heap and then places `x` into it.               
    ///                                                                         
    /// This doesn't actually allocate if `T` is zero-sized.                    
    ///                                                                         
    /// # Examples                                                              
    ///                                                                         
    /// ```                                                                     
    /// let five = Box::new(5);                                                 
    /// ```                                                                     
    #[cfg(not(no_global_oom_handling))]                                         
    #[inline(always)]                                                           
    #[stable(feature = "rust1", since = "1.0.0")]                               
    pub fn new(x: T) -> Self {                                                  
        box x                                                                   
    }
```
I've not seen this syntax before, `box x`. 
```rust
#[lang = "owned_box"]                                                           
#[fundamental]                                                                  
#[stable(feature = "rust1", since = "1.0.0")]                                   
pub struct Box<                                                                 
    T: ?Sized,                                                                  
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
>(Unique<T>, A);
```
Notice the usage of a language feature and owned_box so the compiler has
special logic to handle owned_box's.

Aparently `box x` should be equivalent to:
```rust
fn new(x: T) -> Box<T> {
    use std::alloc::{alloc, handle_alloc_error, Layout};

    unsafe {
        let ptr = alloc(Layout::new::<T>()).cast::<T>();

        // if allocation failed
        if ptr.is_null() { handle_alloc_error(Layout::new::<T>()) }
    
        ptr.write(x);
        Box::from_raw(ptr)
    }
}
```

```console
$ objdump -C --disassemble=box::create_on_heap ./box

./box:     file format elf64-x86-64
Disassembly of section .text:

0000000000009e60 <box::create_on_heap>:
    9e60:	48 83 ec 28          	sub    $0x28,%rsp
    9e64:	48 8d 7c 24 10       	lea    0x10(%rsp),%rdi
    9e69:	48 8d 35 74 d2 02 00 	lea    0x2d274(%rip),%rsi        # 370e4 <str.0+0x44>
    9e70:	ba 05 00 00 00       	mov    $0x5,%edx
    9e75:	e8 46 f9 ff ff       	callq  97c0 <<str as alloc::string::ToString>::to_string>
    9e7a:	bf 18 00 00 00       	mov    $0x18,%edi
    9e7f:	be 08 00 00 00       	mov    $0x8,%esi
    9e84:	e8 37 e0 ff ff       	callq  7ec0 <alloc::alloc::exchange_malloc>
    9e89:	48 89 c1             	mov    %rax,%rcx
    9e8c:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
    9e91:	48 8b 4c 24 10       	mov    0x10(%rsp),%rcx
    9e96:	48 89 08             	mov    %rcx,(%rax)
    9e99:	48 8b 4c 24 18       	mov    0x18(%rsp),%rcx
    9e9e:	48 89 48 08          	mov    %rcx,0x8(%rax)
    9ea2:	48 8b 4c 24 20       	mov    0x20(%rsp),%rcx
    9ea7:	48 89 48 10          	mov    %rcx,0x10(%rax)
    9eab:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
    9eb0:	48 83 c4 28          	add    $0x28,%rsp
    9eb4:	c3                   	retq   
```
And we can take a look at `alloc::alloc::exchange_malloc`:
```console
$ objdump -C --disassemble=alloc::alloc::exchange_malloc ./box

./box:     file format elf64-x86-64
Disassembly of section .text:

0000000000007ec0 <alloc::alloc::exchange_malloc>:
    7ec0:	48 83 ec 58          	sub    $0x58,%rsp
    7ec4:	48 89 7c 24 28       	mov    %rdi,0x28(%rsp)
    7ec9:	48 89 74 24 30       	mov    %rsi,0x30(%rsp)
    7ece:	e8 bd fb ff ff       	callq  7a90 <core::alloc::layout::Layout::from_size_align_unchecked>
    7ed3:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
    7ed8:	48 89 54 24 10       	mov    %rdx,0x10(%rsp)
    7edd:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
    7ee2:	48 89 54 24 40       	mov    %rdx,0x40(%rsp)
    7ee7:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
    7eec:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
    7ef1:	48 8d 3d 58 f1 02 00 	lea    0x2f158(%rip),%rdi        # 37050 <_fini+0xff8>
    7ef8:	e8 33 04 00 00       	callq  8330 <<alloc::alloc::Global as core::alloc::Allocator>::allocate>
    7efd:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
    7f02:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
    7f07:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
    7f0c:	48 85 c0             	test   %rax,%rax
    7f0f:	0f 94 c0             	sete   %al
    7f12:	0f b6 c0             	movzbl %al,%eax
    7f15:	75 06                	jne    7f1d <alloc::alloc::exchange_malloc+0x5d>
    7f17:	eb 00                	jmp    7f19 <alloc::alloc::exchange_malloc+0x59>
    7f19:	eb 21                	jmp    7f3c <alloc::alloc::exchange_malloc+0x7c>
    7f1b:	0f 0b                	ud2    
    7f1d:	48 8b 7c 24 18       	mov    0x18(%rsp),%rdi
    7f22:	48 8b 74 24 20       	mov    0x20(%rsp),%rsi
    7f27:	48 89 7c 24 48       	mov    %rdi,0x48(%rsp)
    7f2c:	48 89 74 24 50       	mov    %rsi,0x50(%rsp)
    7f31:	e8 4a 17 00 00       	callq  9680 <core::ptr::non_null::NonNull<[T]>::as_mut_ptr>
    7f36:	48 89 04 24          	mov    %rax,(%rsp)
    7f3a:	eb 15                	jmp    7f51 <alloc::alloc::exchange_malloc+0x91>
    7f3c:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
    7f41:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
    7f46:	48 8d 05 43 ee ff ff 	lea    -0x11bd(%rip),%rax        # 6d90 <alloc::alloc::handle_alloc_error>
    7f4d:	ff d0                	callq  *%rax
    7f4f:	0f 0b                	ud2    
    7f51:	48 8b 04 24          	mov    (%rsp),%rax
    7f55:	48 83 c4 58          	add    $0x58,%rsp
    7f59:	c3                   	retq   
```
Global::allocate is a method which takes a Layout. A Layout contains the
requested size and alignement that the program is asking to allocator to find
and allow for it to use.

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

A Struct is declaring what data is stored in memory and the sizes of this data
so that. The size of the above struct would be 4 bytes + 4 bytes for example.
Structs.

Structs don't have function member like we can in C++ (but not C where we can
have function pointers to achieve the same thing) but in C++ methods are
implemented as [free functions](https://github.com/danbev/learning-cpp#function-members)
and this is very simliar to how rust does things with Traits.

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
A panic is not a crash and it is per thread.
It is possible to catch and intercept the stack unwinding using
`std::panic::catch_unwind()`

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
Takes ownership of the values and moves them. Is named Once because the closure
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
Is an async computation that can produce a value.

A Future is a value that implements the trait std::future::Future. Notice that
this says that a future is a value and the value it represents is an in-progress
async computation. A Future in Rust is the computation itself.
```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```

Lets take a look at what a future gets expanded into using the example
[future.rs](./tokio/src/future.rs):
```console
$ cd tokio
$ cargo rustc --bin future -- -Z unpretty=expanded,hygiene
```

Each usage of the async keyword generates a statemachine from the code block.
And each .await in that code block will represent a state.
```rust
#[tokio::main]
async fn main() {
    println!("main: {:?}", thread::current().id());
    let future = Something{end: Instant::now() + Duration::from_millis(10)};
    let result = future.await;
    println!("result: {}", result);
}
```
So the first state would run all the code upto the line with
`let result = furure.await;`
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

enum MainFuture {
    State0,
    State1(Something),
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        use MainFuture::*;
        loop {
            match *self {
                State0 => {
                    let future = Something{end: Instant::now() + Duration::from_millis(10)};
                    *self = State1(future);
                }
                State1(ref mut my_future) => {
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            println!("result: {}", result);
                            *self = Terminated;
                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    }
                }
                Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }
}
```
```console
$ cargo rustc --bin future -- --emit=llvm-ir
$ find target/ -name '*.ll'
target/debug/deps/future-07187fee0c7f789c.ll
```

Lets set a break point in our poll functions:
```console
(lldb) expr self
(core::pin::Pin<future::Something *>) $0 = {
  pointer = 0x00007fffffffc3d0
}
```
A newly create Future can be moved safely as it just contains a resumption point
and the argument values to the Future. The body has not yet begun execution so
nothing as had a chance to borrow these values yet. But when poll is called that
changes. Remember that we pass the Future to a function like block_on or spawn
which take the futures by value.

### Mio (Metal I/O)
It abstracts away the underlying systems select/poll implementations, and
enables I/O multiplexing. For some background around I/O models this
[link](https://github.com/danbev/learning-c#io-models) might be useful.

Poll is the abstraction of of select()/poll()/epoll()/kqueue()/IOCP() and
like those calls (at least for select()/poll()/epoll()) we register file
descriptors that we are interested in events (whether or note performing that
event would cause block or not). In Mio instead of a single function that
performs everything there are multply parts.

Poll is a struct that looks like this:
```rust
pub struct Poll {
    registry: Registry,
}
```
And a Registry is also a struct in the same file (poll.rs):
```rust
pub struct Registry {
    selector: sys::Selector,
}
```
So we we first need to create an instance of Poll:
```rust
    let mut poll = match Poll::new() {
        Ok(poll) => poll,
        Err(e) => panic!("failed to create Poll instance; err={:?}", e),
    };
```
Now, the docs for Poll::new says it will make a system call to create the
system selector.
```console
$ strace ./target/debug/mio-example 
...
write(1, "Metal IO (MIO) example\n", 23) = 23
epoll_create1(EPOLL_CLOEXEC)
```
It might help to take a look at a [standalone epoll example](https://github.com/danbev/learning-c/blob/master/epoll.c)
to get a better understanding of what this doing.
In `src/poll.rs` we find Poll::new():
```rust
	pub fn new() -> io::Result<Poll> {
            sys::Selector::new().map(|selector| Poll {
                registry: Registry { selector },
            })
        }
```
mio::sys::selector is private so we can't create one and explore it.
The type of selector being used is determined by the current operating system
being used, in my case this is unix/linux. The way this works is the
src/lib.rs includes the module `sys` and it has the following conditions in
its attributes:
```rust
#[cfg(unix)]
cfg_os_poll! {
    mod unix;
    pub use self::unix::*;
}
```
So the `Selector` in this case can be found in `src/sys/unix/selector/epoll.rs`:
```rust
#[derive(Debug)]
pub struct Selector {
    #[cfg(debug_assertions)]
    id: usize,
    ep: RawFd,
    #[cfg(debug_assertions)]
    has_waker: AtomicBool,
}
```
`RawFd` is std::os::unix::io::RawFd:
```
/// Raw file descriptors.
#[stable(feature = "rust1", since = "1.0.0")]
pub type RawFd = raw::c_int;

type c_int = i32;
```
So our selector has an id, a file descriptor (which is gets back from
epoll_create:
```c
int epoll_fd = epoll_create1(0);
```
So back to sys::Selector::new() we have the following system call:
```rust
        let flag = libc::EPOLL_CLOEXEC;

	syscall!(epoll_create1(flag)).map(|ep| Selector {
            #[cfg(debug_assertions)]
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            ep,
            #[cfg(debug_assertions)]
            has_waker: AtomicBool::new(false),
        })
```
So we are performing a system call using the syscall! macro. Then calling map
on the Result of that call, which recall will be a file descriptor of type
RawFd. This will be used to create a new Selector instance.
So this is using a syscall! macro that is defined in `src/sys/unix/mod.rs`:
```rust
#[allow(unused_macros)]
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}
```
The name of the macro is syscall (without the !). Then comes the body of the
macro after {. The body contains match expressions where `ident` is an
identifier like a function or a variable.
We can see that if the call is successful then we return Ok(res) where res
in our case should be a RawFd representing underlying file descriptor created.

So we will now return back from the syscall and have a populated
Result<Selector> which the function `map` is called on:
(in src/poll.rs):
```rust
pub fn new() -> io::Result<Poll> {
            sys::Selector::new().map(|selector| Poll {
                registry: Registry { selector },
            })
        }
```
And here Mio is mapping the Selector to a new Poll instance and in process it is
creating a new Registry with the selector that was just created.

__work in progess__


### Tokio
This is event looping that uses Mio.
TODO:

### Pin
```rust
pub struct Pin<P> {
    pointer: P,
}
```
Notice that pointer is not pub but private so we can only access it by using
methods that this type provides.

Example of creating a new Pin (pinned pointer):
```rust
    let p = Pin::new(&10);
    println!("{:?}", p);
```
Take a struct that looks like this:
```rust
struct Something<'a> {
    val: i32,
    ptr: &'a i32,
}

fn main() {
    let dummy = 10;
    let val = 8;
    let mut s = Something{val: val, ptr: &dummy};
    s.ptr = &s.val;
    println!("&s.val: {:p}, &s.ptr: {:p}", &s.val, s.ptr);
}
```
```console
$ rust-lldb pin
(lldb) br s -n main -f pin.rs
(lldb) r
(lldb) expr &s.val
(int *) $2 = 0x00007fffffffccf0
(lldb) expr s.ptr
(int *) $4 = 0x00007fffffffccf0

(lldb) memory read -f x -c 2 -s 8 -l 1 &s
0x7fffffffcce8: 0x00007fffffffccf0
0x7fffffffccf0: 0x00007fff00000008

```
So this would produce something like the following in memory:
```
                    +------------------+
 0x7fffffffcce8     |0x00007fffffffccf0|
                    +------------------+
 0x7fffffffccf0     |0x00007fff00000008|
                    +------------------+
```
Now what happens if we move our strut into another value?  
Well, in theory the values would be copied to a new location on the stack. So
the value in s.val would still be 8, and the value in s.ptr would still be the
address to the old location on the stack. 
I've not been able to create a reproducer of this but this migth be because
it is not allowed.

Now Pin is only of interest where you have types that refer to data items with
in them selves. If you don't have such a situation the type is Unpin. Unpin
is an auto trait, that is if the data types only contains members that are
Unpin your data type will also be Unpin. 

### Trait
Is like an Inteface which can be implemented by multiple types.
Like C++ templates the compiler can generate a separate copy of an abstraction
for each way it is implemented.

### Trait objects
```console
$ objdump -C --disassemble=trait_object::main trait_object

trait_object:     file format elf64-x86-64


Disassembly of section .text:

0000000000007ac0 <trait_object::main>:
    7ac0:	48 83 ec 18          	sub    $0x18,%rsp
    7ac4:	48 8d 05 3d c5 02 00 	lea    0x2c53d(%rip),%rax        # 34008 <_fini+0x3fc>
    7acb:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
    7ad0:	48 8d 05 31 c5 02 00 	lea    0x2c531(%rip),%rax        # 34008 <_fini+0x3fc>
    7ad7:	48 89 44 24 10       	mov    %rax,0x10(%rsp)

    7adc:	48 8d 3d 25 c5 02 00 	lea    0x2c525(%rip),%rdi        # 34008 <_fini+0x3fc>
    7ae3:	48 8d 35 66 8a 03 00 	lea    0x38a66(%rip),%rsi        # 40550 <__dso_handle+0x58>
    7aea:	e8 31 ff ff ff       	callq  7a20 <trait_object::call_process>

    7aef:	48 8d 3d 12 c5 02 00 	lea    0x2c512(%rip),%rdi        # 34008 <_fini+0x3fc>
    7af6:	48 8d 35 73 8a 03 00 	lea    0x38a73(%rip),%rsi        # 40570 <__dso_handle+0x78>
    7afd:	e8 1e ff ff ff       	callq  7a20 <trait_object::call_process>

    7b02:	48 83 c4 18          	add    $0x18,%rsp
    7b06:	c3                   	retq   

Disassembly of section .fini:
```
Notice that we are using instruction relative addresses but what is happening
is that the address of type is loaded into rdi, then the pointer to the
vtable, before calling trait_object::call_process.

The memory layout of a trait object looks something like this:
```
                                   Once
+--------------+               +-------------+
| ptr to type  |-------------> |             |
+--------------+               +-------------+
| ptr to vtable|-----+         vtable Doit for Once      
+--------------+     |         +-------------+ 0
                     +-------->| ptr to drop | 
                               +-------------+ 8
                               |  size       |
                               +-------------+ 16
                               |  align      |  
                               +-------------+ 24
                               |process ptr  |  
                               +-------------+ 32
```
We can take a look at `call_process`:
```console
$ objdump -C --disassemble=trait_object::call_process trait_object

trait_object:     file format elf64-x86-64

Disassembly of section .text:

0000000000007a20 <trait_object::call_process>:
    7a20:	48 83 ec 18          	sub    $0x18,%rsp
    7a24:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
    7a29:	48 89 74 24 10       	mov    %rsi,0x10(%rsp)
    7a2e:	ff 56 18             	callq  *0x18(%rsi)
    7a31:	48 83 c4 18          	add    $0x18,%rsp
    7a35:	c3                   	retq   

Disassembly of section .fini:
```
The first instruction is making room on the stack for local variables, in this
case 24 bytes, next the contents of rdi which is the pointer to the type is 
stored on the stack, followed by storing the vtable pointer. Then rsi (the
vpointer) is referenced using an offset of 0x18 which is the address of the
process function.

We verify this below:
```console
(lldb) register read rdi rsi
     rdi = 0x0000555555588008  
     rsi = 0x0000555555594550  

(lldb) memory read -f x -c 4 -s 8 -l  0x0000555555594550
0x555555594570: 0x000055555555b920 0x0000000000000000 0x0000000000000001 0x000055555555ba80
(lldb) disassemble -a 0x000055555555b920
trait_object`core::ptr::drop_in_place$LT$trait_object..Twice$GT$::h35013cb0c4b55373:
    0x55555555b920 <+0>: push   rax
    0x55555555b921 <+1>: mov    qword ptr [rsp], rdi
    0x55555555b925 <+5>: pop    rax
    0x55555555b926 <+6>: ret    

(lldb) memory read -f x -c 4 -s 8 -l 1 0x0000555555594550
0x555555594550: 0x000055555555b910
0x555555594558: 0x0000000000000000
0x555555594560: 0x0000000000000001
0x555555594568: 0x000055555555ba40

(lldb) disassemble -a 0x000055555555ba40
trait_object`_$LT$trait_object..Once$u20$as$u20$trait_object..Doit$GT$::process::he968e372a4a79e77:
    0x55555555ba40 <+0>:  sub    rsp, 0x38
    0x55555555ba44 <+4>:  mov    qword ptr [rsp + 0x30], rdi
    0x55555555ba49 <+9>:  mov    rdi, rsp
    0x55555555ba4c <+12>: lea    rsi, [rip + 0x38add]
    0x55555555ba53 <+19>: mov    edx, 0x1
    0x55555555ba58 <+24>: lea    rcx, [rip + 0x2c5a9]
    0x55555555ba5f <+31>: xor    eax, eax
    0x55555555ba61 <+33>: mov    r8d, eax
    0x55555555ba64 <+36>: call   0x55555555b940            ; core::fmt::Arguments::new_v1::h00905c6e151ce05e at mod.rs:341
    0x55555555ba69 <+41>: mov    rdi, rsp
    0x55555555ba6c <+44>: call   qword ptr [rip + 0x3b0d6] ; _GLOBAL_OFFSET_TABLE_ + 432
    0x55555555ba72 <+50>: add    rsp, 0x38
    0x55555555ba76 <+54>: ret    
```

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

By default we can think of all pointers as const pointers to const data in Rust
so we can't reassign the pointer itself nor modify what the pointer points to.
And another difference in Rust is that passing a value copies the value on the
stack and makes the source variable/data invalid and it cannot be used after
that pointer. If one needs to be able to continue using the variable the value
can be passed by reference, &T to a function which can then read but not modify the
data. If the function needs to modify the data then we can pass it as & mut T.

For me the best way is to try to remember that these are pointers under the
hood.

Passing-by-value is really copying that is on the stack, which for a primitive
value is the data itself. For a pointer type like an array, vec, slice, Box this
will be the type with one or more slots of of which is a pointer, the others
slots could be the lenght, capacity, a pointer to a vtable etc.

Passing-by-reference is actually passing a memory address. So instead of copying
the type on the stack it just passes the memory address the function.
```rust
$ objdump -C --disassemble='fn::main' fn

fn:     file format elf64-x86-64

0000000000007780 <fn::main>:
    7780:	50                   	push   %rax
    7781:	c7 44 24 04 03 00 00 	movl   $0x3,0x4(%rsp)
    7788:	00 
    7789:	48 8d 7c 24 04       	lea    0x4(%rsp),%rdi
    778e:	e8 dd ff ff ff       	callq  7770 <fn::by_ref>
    7793:	58                   	pop    %rax
    7794:	c3                   	retq   

$ objdump -C --disassemble='fn::by_ref' fn

fn:     file format elf64-x86-64

Disassembly of section .text:

0000000000007770 <fn::by_ref>:
    7770:	50                   	push   %rax
    7771:	48 89 3c 24          	mov    %rdi,(%rsp)
    7775:	58                   	pop    %rax
    7776:	c3                   	retq   
```
Notice that the value 3 is moved onto the stack 0x4(%rsp) and the next
instruction loads the effective address of that location and places it in $rdi
which is the first argument register.

```rust
$ objdump -C --disassemble='fn::main' fn

fn:     file format elf64-x86-64

Disassembly of section .text:

0000000000007790 <fn::main>:
    7790:	50                   	push   %rax
    7791:	c7 44 24 04 03 00 00 	movl   $0x3,0x4(%rsp)
    7798:	00 
    7799:	48 8d 7c 24 04       	lea    0x4(%rsp),%rdi
    779e:	e8 cd ff ff ff       	callq  7770 <fn::by_ref>
    77a3:	8b 7c 24 04          	mov    0x4(%rsp),%edi
    77a7:	e8 d4 ff ff ff       	callq  7780 <fn::by_val>
    77ac:	58                   	pop    %rax
    77ad:	c3                   	retq   
```
Notice that now we are moving the value in 0x4(%rsp) (not the address) into
$edi.

Now, how about passing a struct to a function in a similar manner as the two
examples above. Well, the by_ref is pretty much the same, it will pass the
address to the first member of the struct in $rdi.
The by value case is a little more interesting:
```rust
$ objdump -C --disassemble='fn::main' fn

fn:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .text:

0000000000007790 <fn::main>:
    7790:	50                   	push   %rax
    7791:	c7 04 24 03 00 00 00 	movl   $0x3,(%rsp)
    7798:	c7 44 24 04 04 00 00 	movl   $0x4,0x4(%rsp)
    779f:	00 
    77a0:	48 89 e7             	mov    %rsp,%rdi
    77a3:	e8 c8 ff ff ff       	callq  7770 <fn::by_ref>
    77a8:	8b 3c 24             	mov    (%rsp),%edi
    77ab:	8b 74 24 04          	mov    0x4(%rsp),%esi
    77af:	e8 cc ff ff ff       	callq  7780 <fn::by_val>
    77b4:	58                   	pop    %rax
    77b5:	c3                   	retq   

Disassembly of section .fini:
$ objdump -C --disassemble='fn::by_val' fn

fn:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .text:

0000000000007780 <fn::by_val>:
    7780:	50                   	push   %rax
    7781:	89 3c 24             	mov    %edi,(%rsp)
    7784:	89 74 24 04          	mov    %esi,0x4(%rsp)
    7788:	58                   	pop    %rax
    7789:	c3                   	retq   
```
If we look at main we can see that it first moves the address located on the
content located on the top of the stack, (%rsp) and placed it in rdi (the first
argument), then moves the value located at 0x4(%rsp), into rsi, the second
argument. And if we look at by_val is looks just like a function that takes
two arguments.


### Lifetimes
These are annotations that start with `'` followed by a variable name.
For example `'a` is spoken as 'tick a'. 

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

One thing to notes is that lifetimes on function signatures can tell us what
a function can do with a passed in argument.

The following function cannot store the input reference in a place that would
outlive the function body (like static storage):
```rust
fn something<'a>(input: &'a i32) {
}
```

When a function takes a single ref as an argument and returns a single ref then
Rust assumes that those two refs have the same lifetime.

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
$ echo 'fn main(){}' | rustc +nightly --edition=2018 -Zunpretty=expanded just-main.rs 
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
fn main() { }
```
Notice that the rust compiler includes the prelude.

Show High-Level IR with types:
```console
$ echo 'fn main(){}' | rustc +nightly --edition=2018 -Zunpretty=hir,typed just-main.rs 
```

Show Mid-Level IR:
```console
$ echo 'fn main(){}' | rustc +nightly --edition=2018 -Zunpretty=hir,typed just-main.rs
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
fn main() ({ } as ())
```
Notice that the compiler as not modified the arguments to main asn an empty
struct.

Show LLVM IR:
```console
$ echo 'fn main(){}' | rustc +nightly --emit llvm-ir main/src/simple.rs
```
This will generate a file named `basic.ll`.

### rustc with heredoc
This is useful when you don't need to save the source in a file:
```console
$ rustc -g -o bajja - <<HERE
fn main() {
println!("bajja");
}
HERE
$ ./bajja
```
Lets say we want to inspect what the compiler generates for some code. 
```console
$ rustc +nightly --edition=2018 -Zunpretty=expanded - <<HERE

```

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

### use self
```rust
use std::io::{self, Read, Write, ErrorKind};
```
The `self` in this case means that we can use the name of this crate, which is
`io` as an alias for std::io. So we can write io::Result instead of having to
write `std::io::Result`.

### self
When you see a function take a single `&self` this is just syntactic suger for
```rust
self: &Self
```
This just means that the first argument to a method is an instance of the
implementing type.

### Unit struct
This is a struct without any members:
```rust
struct Something;
```
This will be a new type but of now size so would be a noop in a program.
These are called zero sized types (ZST)s.


### rustc_driver
First thing to do is add the `rustc-dev` component:
```console
$ rustup component add rustc-dev llvm-tools-preview
info: component 'rustc-dev' for target 'x86_64-unknown-linux-gnu' is up to date
info: downloading component 'llvm-tools-preview'
info: installing component 'llvm-tools-preview'
 21.6 MiB /  21.6 MiB (100 %)  13.7 MiB/s in  1s ETA:  0s
```

```console
$ rustc -g compiler.rs
$ ./compiler 
./compiler: error while loading shared libraries: libLLVM-12-rust-1.56.0-nightly.so: cannot open shared object file: No such file or directory
```
```console
$ ldd compiler
	linux-vdso.so.1 (0x00007fffe5f66000)
	libLLVM-12-rust-1.56.0-nightly.so => not found
	libgcc_s.so.1 => /usr/lib64/libgcc_s.so.1 (0x00007fe502961000)
	libpthread.so.0 => /usr/lib64/libpthread.so.0 (0x00007fe502940000)
	libm.so.6 => /usr/lib64/libm.so.6 (0x00007fe5027fc000)
	libdl.so.2 => /usr/lib64/libdl.so.2 (0x00007fe5027f5000)
	libc.so.6 => /usr/lib64/libc.so.6 (0x00007fe502626000)
	/lib64/ld-linux-x86-64.so.2 (0x00007fe503712000)
```
Find the library:
```console
$ find ~/.rustup -name libLLVM-12-rust-1.56.0-nightly.so
```
And then we can set `LD_LIBRARY_PATH`:
```console
$ LD_LIBRARY_PATH=~/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/ ldd compiler
	linux-vdso.so.1 (0x00007fff701d6000)
	libLLVM-12-rust-1.56.0-nightly.so => /home/danielbevenius/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/libLLVM-12-rust-1.56.0-nightly.so (0x00007f2dfdf99000)
	libgcc_s.so.1 => /usr/lib64/libgcc_s.so.1 (0x00007f2dfdf66000)
	libpthread.so.0 => /usr/lib64/libpthread.so.0 (0x00007f2dfdf45000)
	libm.so.6 => /usr/lib64/libm.so.6 (0x00007f2dfde01000)
	libdl.so.2 => /usr/lib64/libdl.so.2 (0x00007f2dfddfa000)
	libc.so.6 => /usr/lib64/libc.so.6 (0x00007f2dfdc2b000)
	/lib64/ld-linux-x86-64.so.2 (0x00007f2e03ecc000)
	librt.so.1 => /usr/lib64/librt.so.1 (0x00007f2dfdc1e000)
	libz.so.1 => /usr/lib64/libz.so.1 (0x00007f2dfdc04000)
```
And to run the compiler example:
```console
$ LD_LIBRARY_PATH=~/.rustup/toolchains/nightly-2021-08-03-x86_64-unknown-linux-gnu/lib/ ./compiler
```
You can export LD_LIBRARY_PATH as well but just don't forget to unset it later
or you migth run into issues.

### rustc_ast
This crate contains the AST definition.

### AST Span
Is used to link a particular AST node back to its source text:
In the [compiler.rs(./src/compiler.rs) example our input source looks like this:
```rust
   input: r###"fn main() { println!("Bajja{}"); }"###.to_string(),
```
And if we take a look at part of the output we can see a few examples of span:
```console
$ ./compiler 
Crate {
    attrs: [],
    items: [
        Item {
            attrs: [],
            id: NodeId(4294967040),
            span: <main.rs>:1:1: 1:35 (#0),  // first line, first column.
            vis: Visibility {
                kind: Inherited,
                span: no-location (#0),
                tokens: None,
            },
            ...
            TokenStream(
               [
                 (
                     Token(
                         Token {
                             kind: Literal(
                                 Lit {
                                   kind: Str,
                                   symbol: "Bajja{}",
                                   suffix: None,
                                 },
                          ),
                          span: <main.rs>:1:22: 1:31 (#0), // Bajja starts at column 22
}
```

###
Hygiene relates to how to handle names defined within a macro. In particular, a
hygienic macro system prevents errors due to names introduced within a macro


### move
This section takes a look at what a move does, for example
```rust
    let s = [0; 1024];
    let t = s;
```
That will compile into:
```console
$ objdump -C --disassemble=move::main move

move:     file format elf64-x86-64
Disassembly of section .text:

00000000000076b0 <move::main>:
    76b0:	b8 08 20 00 00       	mov    $0x2008,%eax
    76b5:	e8 0b 15 03 00       	callq  38bc5 <__rust_probestack>
    76ba:	48 29 c4             	sub    %rax,%rsp
    76bd:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
    76c2:	31 f6                	xor    %esi,%esi
    76c4:	ba 00 10 00 00       	mov    $0x1000,%edx
    76c9:	e8 62 d9 ff ff       	callq  5030 <memset@plt>
    76ce:	48 8d bc 24 08 10 00 	lea    0x1008(%rsp),%rdi
    76d5:	00 
    76d6:	48 8d 74 24 08       	lea    0x8(%rsp),%rsi
    76db:	ba 00 10 00 00       	mov    $0x1000,%edx
    76e0:	e8 8b d9 ff ff       	callq  5070 <memcpy@plt>
    76e5:	48 81 c4 08 20 00 00 	add    $0x2008,%rsp
    76ec:	c3                   	retq   

Disassembly of section .fini:
```

### Cargo config
This section contains notes about Cargo's configuration system.

A "personal" configuration file exist in $CARGO_HOME which is usually the users
home directory on UNIX:
```
$HOME/.cargo/config.toml
```
Projects can `./cargo/config.toml` files in the root and subdirectores.
```
```
build.target               The default target platform to compile to run.
```
