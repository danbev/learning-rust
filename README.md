## Learning Rust
The sole purpose of this project is to learn the [Rust](http://www.rust-lang.org/) programming language.

### Contents
1. [Debugging](#debugging)
1. [Startup](#startup)
1. [Embedded Rust](./notes/embedded-rust.md)
1. [Pinning](#pin)

### Debugging
Debug symbols are enabled by default when using cargo build or cargo run
without the `--release` flag.

We can use `rust-gdb` or `rust-lldb`

To debug Rust language source code You want to have the rust sources installed:
```console
$ rustup component add rust-src
```

In gdb you might not be able to step into Rust std sources and see an message/
path like this:
```console
/rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/future/future.rs: No such file or directory.
```
This can be worked around by adding the following to a `.gdbinit`:
```console
set substitute-path '/rustc/fc594f15669680fa70d255faec3ca3fb507c3405' '/home/danielbevenius/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust
```
You can find the correct path to use as the second argument using:
```console
$ rustc --print sysroot
/home/danielbevenius/.rustup/toolchains/stable-x86_64-unknown-linux-gnu
```
One thing to note is that we might have to update the hash after updating Rust.

rust-lldb example:
```console
$ rust-lldb -- ./target/debug/main
(lldb) br s -f main.rs -l 47
(lldb) r
```

rust-gdb example:
```console
$ rust-gdb out/atomics
Reading symbols from out/atomics...
(gdb) br atomics.rs:4
Breakpoint 1 at 0x8d47: file src/atomics.rs, line 4.
(gdb) r
Starting program: /home/danielbevenius/work/rust/learning-rust/out/atomics
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib64/libthread_db.so.1".

Breakpoint 1, atomics::main () at src/atomics.rs:4
4	    let a = AtomicIsize::new(0);
Missing separate debuginfos, use: dnf debuginfo-install libgcc-11.2.1-9.fc35.x86_64
(gdb) s
core::sync::atomic::AtomicIsize::new (v=0) at /rustc/0d1754e8bf6942b4c1d24d7c923438782129ba5a/library/core/src/sync/atomic.rs:1401
1401	            #[must_use]
(gdb) list
1396	            #[doc = concat!("let atomic_forty_two = ", stringify!($atomic_type), "::new(42);")]
1397	            /// ```
1398	            #[inline]
1399	            #[$stable]
1400	            #[$const_stable]
1401	            #[must_use]
1402	            pub const fn new(v: $int_type) -> Self {
1403	                Self {v: UnsafeCell::new(v)}
1404	            }
```

### Startup
[main.rs](./startup/src/main.rs) is used in this section to walkthrough the
startup of a Rust program.

The main function that we write is not the entry point of a rust program which
can be seen by inspecting the `start address` using objdump:
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
$ rust-lldb -- ./target/debug/startup
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

Upon startup only the registers `rsp` and `rdx` contain valid data. `rdx` will
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

The rest of the assembly code is setting up argument, 7 of them. 6 are passed in
registers and one on the stack for the  `__libc_start_main` function:
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

So if we inspect the value of `rdi` which is the main function that will be
called by `__libc_start_main` it is:
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
(IR) we can see that a function named `main` is generated for us, and that our
`main` is named just_main::main. 
```console
$ rustc --emit=llvm-ir just-main.rs
```
And we can filter/demangle symbol names using rustfilt:
```
$ cat just-main.ll | rustfilt --input - --output just-main-filtered.ll
```
```
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
This is using `catch_undwind` and there is a standalone example
[unwind.rs](./src/unwind.rs) which might be helpful to take a look at and run to
better understand what is happening here. Taking this apart a little so it is a
little easier to understand we are passing a closure to the first call to
panic::catch_unwind, and this closure will call
panic::catch_unwind(sys_common::rt::init(argc, argv) when it is called.

catch_unwind will call the closure passed in and return Ok with the result of
the closure if there is no panic from that call. If there is a panic then
`catch_unwind` will return Err(cause).

`catch_unwind` can be found in `library/std/src/panic.rs`:
```rust
#[stable(feature = "catch_unwind", since = "1.9.0")]                            
pub fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> {      
    unsafe { panicking::r#try(f) }                                              
}
```
`panicking::r#try` can be found in `library/std/src/panicing.rs`. This name
looked odd to me and I've not come across it before but it is simply to allow
Rust to have the name of this function be `try`, the `r` stands for raw and I
think `try` is a reserved keyword in Rust. I've added an example of this
in [unwind.rs](./src/unwind.rs).
```rust
/// Invoke a closure, capturing the cause of an unwinding panic if one occurs.
pub unsafe fn r#try<R, F: FnOnce() -> R>(f: F) -> Result<R, Box<dyn Any + Send>> {
    ...

    unsafe {
        return if intrinsics::r#try(do_call::<F, R>, data_ptr, do_catch::<F, R>) == 0 {
            Ok(ManuallyDrop::into_inner(data.r))
        } else {
            Err(ManuallyDrop::into_inner(data.p))
        };
    }

    #[inline]                                                                   
    fn do_call<F: FnOnce() -> R, R>(data: *mut u8) {
        unsafe {
            let data = data as *mut Data<F, R>;
            let data = &mut (*data);
            let f = ManuallyDrop::take(&mut data.f);
            data.r = ManuallyDrop::new(f());
        }
    }

    fn do_catch<F: FnOnce() -> R, R>(data: *mut u8, payload: *mut u8) {         
        unsafe {
            let data = data as *mut Data<F, R>;
            let data = &mut (*data);
            let obj = cleanup(payload);
            data.p = ManuallyDrop::new(obj);
        }
    }
}
```
`intrinsics::try` can be found in `library/core/src/intrinsics.rs`:
```rust
    /// Rust's "try catch" construct which invokes the function pointer `try_fn`
    /// with the data pointer `data`.
    ///                                                                             
    /// The third argument is a function called if a panic occurs. This function
    /// takes the data pointer and a pointer to the target-specific exception
    /// object that was caught. For more information see the compiler's
    /// source as well as std's catch implementation.
    pub fn r#try(try_fn: fn(*mut u8), data: *mut u8, catch_fn: fn(*mut u8, *mut u8)) -> i32;
```
So the `try_fn` will be the `do_call` function, and `catch_fn` will be the
`do_catch` function. Now, this is the declaration of the function, but as it
is an intrinsic function it will be implemented by the compiler for the target
architecture.


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

If you have multiple tests that start with the same name one can use `--exact`
to specify that only the test matching should be run and not a substring:
```console
$ cargo test -- --exact boxing::tests::boxing_test
```
To see all the options for the test program:
```console
$ cargo test -- --help
```

Ignore a test:
```rust
#[ignore]
```
You can run just the ignored annotated tests using:
```
$ cargo test -- --ignored
```

Running single integration tests:
Integration tests are tests that exist in the `tests` directory and are each
compiled into separate crates. They should be written to test the external API
interface of the library. These can be run using:
```console
$ cargo t --features="ecdsa, alloc" --test 'public_key' decode_ecdsa_p256_openssh  -- --show-output --exact
```

Running only the unit tests we have to specify `--lib`:
```console
$ cargo t --lib some_tests  -- --show-output --exact
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

### .d files
These are makefile compatible depencency lists.

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
data. When this is done s1 will become "null" or something equivalent and no
longer valid to be referenced leading to a compile time error.

To create a copy you can use `clone`, but note that this will create a copy
of the data on the heap and the two String instances will point to different
places on the heap.

### clone
Clone can be derived for structs, for example:
```rust
#[derive(Clone)]
struct Something {
    name: String,
}
```
And this will get expended into:
```console
$ cargo expand
    Checking exploration v0.1.0 (/home/danielbevenius/work/rust/learning-rust/exploration)
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
struct Something {
    name: String,
}
#[automatically_derived]
impl ::core::clone::Clone for Something {
    #[inline]
    fn clone(&self) -> Something {
        Something {
            name: ::core::clone::Clone::clone(&self.name),
        }
    }
}
```

### str
Is an immutable sequence of utf-8 bytes. So a sequence means [...] and simliar
could be any length they are handled using pointers. So we have a pointer to
the memory and a length.
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
A literal like this is stored in the executable and loaded when the program runs
and it has a life time of `&'static str`
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
then loading the contents of rip+0x2b985 into $rax and then storing that onto
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

We can also have a view into a String that is stored on the heap. This is
because String impleement `Deref<Target = str>`
```rust
    let r2: &str = &String::from("bajja");
```
And this provides us the same type as we saw above for &str. We have a pointer
which in this case now point to the heap, and the length.
```
(&str) $0 = "bajja" {
  data_ptr = 0x00005555555a5bc0 "bajja"
  length = 5
}
```
But a String on the heap is a vector:
```rust
    let s1: String = String::from("bajja");
```

```console
(lldb) expr s1
(alloc::string::String) $0 = "bajja" {
  vec = size=5 {
    [0] = 'b'
    [1] = 'a'
    [2] = 'j'
    [3] = 'j'
    [4] = 'a'
  }
}

(lldb) expr s1.vec.buf
(alloc::raw_vec::RawVec<unsigned char, alloc::alloc::Global>) $12 = {
  ptr = {
    pointer = {
      pointer = 0x00005555555a6bc0 "bajja"
    }
    _marker =
  }
  cap = 5
  alloc =
}
```
And a vector contains a pointer, the size of the allocation, and the number
of elements that have been initialized.
```console
(alloc::vec::Vec<unsigned char, alloc::alloc::Global>) $2 = size=5 {
  [0] = 'b'
  [1] = 'a'
  [2] = 'j'
  [3] = 'j'
  [4] = 'a'
}
```
Compare this to a `&str` and we can see that these are very different.
```console
(lldb) expr r2
(&str) $14 = "bajja" {
  data_ptr = 0x00005555555a6bc0 "bajja"
  length = 5
}
```

### lang_items
Are a way for the stdlib and libcore to define types, traits, functions, and
other items

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
In Rust there is no way to cast a shared reference (&T) to an exclusive
reference (&mut T), except if we use UnsafeCell.

Normally in rust we cannot have multiple mutable references/pointers to the
same location in memory. This is prevented by the compiler. UnsafeCell enables
this rule to be broken.

```rust
    // Multiple *mut pointers are allowed:
    let un = UnsafeCell::new(18);
    let p1: *mut i32 = un.get();
    let p2: *mut i32 = un.get();
    println!("p1: {:?}, *p1: {}", p1, unsafe { *p1 });
    println!("p2: {:?}, *p2: {}", p2, unsafe { *p2 });
```
It is the callers responsibility to ensure that this access is unique. For
example this is what Cell uses and it makes sure that there are no other
pointer accesses.

Just thinking about this a little more; Rust is a frontend for LLVM just like
there are frontends for C. Now, we know that in C we can cast a const pointer
to a normal pointer without any issues. 

Example: [unsafecell.rs](./src/unsafecell.rs).

UnsafeCell can be found in rust/library/core/src/cell.rs and is declared like
this:
```rust
#[lang = "unsafe_cell"]
#[repr(transparent)]
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
```
So looking at the struct is is just one field, `value` of type T.

Also note the usage of a lang item, `unsafe_cell`.
optional and not used for UnsafeCell, but is used for example for `Add(Op)`.
The following is an approximation of what the macro will be expanded into: 
```rust
pub enum LangItem {
  ...
  UnsafeCell,
  ...
}

impl LangItem { 

  pub fn name(self) -> Symbol {                                       
     match self {                                                    
        ...
        LangItem::UnsafeCell => unsafe_cell_type,
        ...
     }                                                               
  }    

  pub fn group(self) -> Option<LangItemGroup> {                       
     use LangItemGroup::*;                                           
         match self {                                                    
	     LangItem::UnsafeCell => expand_group!(sym:unsafe_cell_type,
             $( LangItem::$variant => expand_group!($($group)*), )*         
         }                                                               
  }    

pub struct LanguageItems {
  ...
 $(                                                                  
    #[doc = concat!("Returns the [`DefId`] of the `", stringify!($name), "` lang item if it is defined.")]
    pub fn $method(&self) -> Option<DefId> {                        
        self.items[LangItem::$variant as usize]                     
   }                                                               
 )*   
}
```
If we take a look in `compiler/rustc_middle/src/ty/layout.rs`:
```rust
impl<'tcx> LayoutCx<'tcx, TyCtxt<'tcx>> {
    // FIXME(eddyb) perhaps group the signature/type-containing (or all of them?)
    // arguments of this method, into a separate `struct`.
    fn fn_abi_new_uncached(
        &self,
        sig: ty::PolyFnSig<'tcx>,
        extra_args: &[Ty<'tcx>],
        caller_location: Option<Ty<'tcx>>,
        fn_def_id: Option<DefId>,
        // FIXME(eddyb) replace this with something typed, like an `enum`.
        force_thin_self_ptr: bool,
    ) -> Result<&'tcx FnAbi<'tcx, Ty<'tcx>>, FnAbiError<'tcx>> {
    ...

```

```console
$ RUSTC_LOG=rustc_middle::ty=debug make -B out/unsafecell 1> output 2>&1
```

```rust
#[derive(Copy, Clone, PartialEq, Eq, Debug)]                                    
pub enum PointerKind {                                                          
    /// Most general case, we know no restrictions to tell LLVM.                
    SharedMutable,                                                              
                                                                                
    /// `&T` where `T` contains no `UnsafeCell`, is `dereferenceable`, `noalias` and `readonly`.
    Frozen,                                                                     
                                                                                
    /// `&mut T` which is `dereferenceable` and `noalias` but not `readonly`.   
    UniqueBorrowed,                                                             
                                                                                
    /// `&mut !Unpin`, which is `dereferenceable` but neither `noalias` nor `readonly`.
    UniqueBorrowedPinned,                                                       
                                                                                
    /// `Box<T>`, which is `noalias` (even on return types, unlike the above) but neither `readonly`
    /// nor `dereferenceable`.                                                  
    UniqueOwned,                                                                    
}
```


To create a new instance:
```rust
    #[inline(always)]
    pub const fn new(value: T) -> UnsafeCell<T> {
        UnsafeCell { value }
    }
```
And is this also very simple, just returns a new UnsafeCell with the passed
in value.

There is a function named `into_inner` which returns a copy of the value:
```rust
   pub const fn into_inner(self) -> T {                                        
        self.value                                                              
   }
```

So lets take a closer look at `get`:
```rust
#[rustc_const_stable(feature = "const_unsafecell_get", since = "1.32.0")]
pub const fn get(&self) -> *mut T {
    self as *const UnsafeCell<T> as *const T as *mut T
}
```
Get takes an immutable reference to self (&self), which is then casted to
`*const UnsafeCell<T>`. which is then casted to `*const T` which in trun casted
to `*mut T. We an "unpack" that to hoppfully make it a little clearer:
```rust
    // The following is an example of what UnsafeCell::get does with regards
    // to casting:
    let c = UnsafeCell::new(4);
    // So this an immutable ref to an UnsafeCell:
    let c_ref: &UnsafeCell<i32> = &c;
    // The following if the first cast to a raw const pointer to UnsafeCell<T>:
    let raw_const_un_ptr: *const UnsafeCell<i32> = c_ref as *const UnsafeCell<i32>;
    // The following is casting the raw const pointer to UnsafeCell<T> to a 
    // const pointer to T:
    let raw_const_ptr: *const i32 = raw_const_un_ptr as *const i32;
    // The following is the last cast which is from a raw const pointer to a
    // raw mutable pointer.
    let raw_ptr: *mut i32 = raw_const_ptr as *mut i32;
```
This is casting self into a raw const pointer, and then casts that as const T
and then casts that into a mutable raw pointer. This type of casting is not
unsafe, it is the potential usage of the cast that is and needs an unsafe block.
In the comment for `get` we can find
```rust
// We can just cast the pointer from `UnsafeCell<T>` to `T` because of
// #[repr(transparent)].
```

### repr(transparent)
[Documentation](https://doc.rust-lang.org/1.26.2/unstable-book/language-features/repr-transparent.html)
This is for telling the compiler that a type is only for type safety on the Rust
side.


### Cell
Allows for shared mutable containers in Rust. So normally you can only have a
single mutable reference but this allows multiple mutable pointers to the same
data. This can be done because a reference is never returned by any of the
methods in Cell.

```rust
    let cell = Cell::new(18);
```
The Cell struct looks like this:
```rust
    #[stable(feature = "rust1", since = "1.0.0")]
    #[repr(transparent)]
    pub struct Cell<T: ?Sized> {
        value: UnsafeCell<T>,
    }
```
Notice that is only has a single `value` member of type `UnsafeCell<T>`.

`Cell::new` just calls `UnsafeCell:new`:
```rust

    #[rustc_const_stable(feature = "const_cell_new", since = "1.24.0")]
    #[inline]
    pub const fn new(value: T) -> Cell<T> {
        Cell { value: UnsafeCell::new(value) }
    }
```

Cell is generic so it expects a type to be specified when creating an instance
of it.
```rust
  let something = Something{id: 1, age: Cell::<u32>::new(45)};
```
But the type can also be inferred:
```rust
  let something = Something{id: 1, age: Cell::new(45)};
```

`Cell::set` can be used to set the value in the Cell.
`Cell::get` will return a copy of the contained value.

There is no way to get a pointer to the value inside the cell, all function
that manipulate the contained value done by the Cell. This means that there are
never any other pointers to the Cell value which allows it to be mutated.

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

```console
$ rust-gdb out/rc 
Reading symbols from out/rc...
(gdb) br rc.rs:4
Breakpoint 1 at 0x9347: file src/rc.rs, line 4.
(gdb) r
Starting program: /home/danielbevenius/work/rust/learning-rust/out/rc 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, rc::main () at src/rc.rs:4
4	    let rc = Rc::new(String::from("bajja"));
(gdb) n
5	    println!("{}", rc);
(gdb) p rc
$1 = Rc(strong=1, weak=0) = {value = "bajja", strong = 1, weak = 0}
```
Note that we import `use std::rc::Rc;` but if we inspect the type we will
see `alloc::rc::Rc`. This is because in `library/std/src/lib.rs` there is the
following use statement:
```rust
pub use alloc_crate::rc;
```
Notice that this is done in the crate `std::

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
Notice that we create the struct with parentheses and not brackets. These
structs are called Tuple Structs and are used when you want to have separate
types but the names of the members is not important. 

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


### Exception handling
Happens in two stages, a search phase and a cleanup phase.

Each module (as in an executable of a dynamic library and not a Rust module) has
its own frame unwind info section (usually ".eh_frame")

### eh_personality
Exception Handling personality is a function that determines the how the
exception is to be handled.


```rust
#[lang = "panic_info"]
#[stable(feature = "panic_hooks", since = "1.10.0")]
#[derive(Debug)]
pub struct PanicInfo<'a> {
    payload: &'a (dyn Any + Send),
    message: Option<&'a fmt::Arguments<'a>>,
    location: &'a Location<'a>,
    can_unwind: bool,
}
```

`__cxa_allocate_exception` takes a size_t and allocates enough memory to store
the exception being throw.

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

### Tokio
This is event looping that uses Mio.
TODO:



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
`borrowing`. When doing this we can't modify the value as we don't own it. But
we can specify that it should be a mutable reference and then we can change it.

By default we can think of all pointers as const pointers to const data in Rust
so we can't reassign the pointer itself, nor modify what the pointer points to.

And another difference in Rust is that passing a value copies the value on the
stack and makes the source variable/data invalid and it cannot be used after
that point. If one needs to be able to continue using the variable, the value
can be passed by reference, `&T` to a function which can then read but not
modify the data. If the function needs to modify the data then we can pass it
as & mut T.

For me the best way is to try to remember that these are pointers under the
hood.

Passing-by-value is really copying what is on the stack, which for a primitive
value is the data itself. For a pointer type like an array, vec, slice, or Box
this will be the type with one or more slots of of which is a pointer, the
others slots could be the length, capacity, a pointer to a vtable etc.

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

### Borrowchecker
* When passing a variable to another function one gives up ownership so the
  receiving function now has ownership and the calling function can no longer
  use the variable after that point.
* When passing a reference you can pass as many immutable one as you like, or
  one mutable borrow.

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

### Unit type
`()` is an empty tuple of zero size.

The semicolon `;` can be used to discard the result of an expression at the end
of a block, making the expression (and thus  the block) evaluate to `().


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
build.target               The default target platform to compile to run.
```

### Cargo features
Features are specified in the `features` table in Config.toml, and they are by
default disabled and need to be enabled explicitely.

We can use `#[cfg(feature = "feature name")]` to conditionally compile parts of
the code. An example can be found in [features example](./features).

Optional dependencies are dependencies that will not be compiled by default:
```rust
[dependencies]
something = {version = "0.1.0", optional = true}
```
This will also introduce `something` as a feature which can be used just like
the features mentioned above and used in `cfg` clauses/expressions.
We can use dependencies in features by using the name of the dependency:
```rust
[dependencies]
something = {version = "0.1.0", optional = true}

[features]
all = ["something"]
```
Dependencies can enable features using the `features` key:
```rust
[dependencies]
something = {version = "0.1.0", optional = true, features = ["f1", "f2"]}
```
And we can disable the default features using `default-features = false`.
Dependency features can also be enabled in the features table instead of within
the dependency declaration using the following syntax:
```rust
[dependencies]
something = {version = "0.1.0", optional = true}

[features]
all = ["something/f1", "something/f2"]
```


### Deferred Format (defmt)
Is a logging framework for constrained devices.


### Raw pointers

Example of a mutable raw pointer:
```rust:
    let mut x = 18;
    let r = &mut x as *mut i32;
    println!("r: {:p}", r);
    unsafe {
        println!("*r: {}", *r);
    };
}
```


### Casting
I came a across this syntax which was a little confused about:
```rust
    r as *const _ as _
```
If I'm reading this correctly we are first casting r to a raw unmutable pointer
`*const _` and then casting that into something. The something here is `_` which
is the type placeholder which is us telling Rust to figure out what type this
should be.
[raw_pointers.rs](../src/raw_pointers.rs) contains an example.


### Condvar
This is a mechanism to be able to put a thread to sleep and also wake it up
at some point.
[condvar.rs](../src/condvar.rs) contains an example.

### Impl Trait
An only be used in two locations, as an argument type or as a return type.

### PhantomData
Is a marker type and consumes no space and is intended to signal to the compiler
that our data type, like a struct, acts as though it stores a value of type T,
even though it actually does not. If the struct needs to have a lifetime then
it may be reported as unused if there is only a single pointer in the struct


Take the following example where we have a struct that is generic over T and U
but don't use U:
```rust
struct Something<T, U> {
    first: T,
}
fn main() {}
```
This will generate the following compiler error:
```console
$ make out/phantom_data_unused
rustc "-Copt-level=0" "--edition=2021" -o out/phantom_data_unused -g src/phantom_data_unused.rs
error[E0392]: parameter `U` is never used
 --> src/phantom_data_unused.rs:1:21
  |
1 | struct Something<T, U> {
  |                     ^ unused parameter
  |
  = help: consider removing `U`, referring to it in a field, or using a marker such as `PhantomData`
  = help: if you intended `U` to be a const parameter, use `const U: usize` instead

error: aborting due to previous error
```
We can fix this using:
```rust
struct Something<T, U> {
    first: T,
    _marker: std::marker::PhantomData<U>,
}
```

Another example is when we have an unsued lifetime for a type. Lets say we have
a struct that only holds a raw pointer to a type B, and we want to specify that
the data our type should not outlive the lifetime:
```console
$ rustc -o phantom - <<HERE
struct PhantomStruct<'a, B> {
b: *const B,
}
fn main() {}
HERE
error[E0392]: parameter `'a` is never used
 --> <anon>:1:22
  |
1 | struct PhantomStruct<'a, B> {
  |                      ^^ unused parameter
  |
  = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`

error: aborting due to previous error
```
But by using a PhantomData member we can still get this to compile using:
```rust
struct PhantomStruct<'a B> {
    b: *const B,
    marker: PhantomData<'a B>,
}
```
An example can be found in
[phantom_data_unused.rs](./src/phantom_data_unused.rs).


### Trait Objects
Is a pointer some value that implements a specified trait.
These are implemented as 16 byte fat pointers, the first 8 bytes is a pointer
to the data, and the second 8 bytes is a pointer to a vtable. 

``` 
    Data         Trait Object
   +------+     +--------------+
   |      |<----| ptr to data  |        vtable
   +------+     |--------------|     +---------------+
                | ptr to vtable|---->| ptr to drop   |
                +--------------+     |---------------|
                                     |    size       |
                                     |---------------|
                                     |    align      |
                                     |---------------|
                                     | ptr to func1  |
                                     |---------------|
                                     | ptr to func2  |
                                     +---------------+
```

### extern crate
```rust
extern crate something;
```
This means that we want to link against this external library.
In Rust 2018 this is no longer required and instead we can just write:
```
use something;
```

### Waker API
Is a handle for waking up a task. This is intended for notifiying the Executor
that it has stuff to do and that the executor should poll the task again.
Waker has the following functions:
* as_raw which gives a reference to the underlying RawWaker which a Waker wrapps.
* from_raw create a new Waker from a RawWaker instance.
* wake The actually wake up call which should wake up the task associated with
this waker.
* wake_by_ref same as wake but without consuming the Waker.
* will_wake seems to be used to find out if two wakers would wake the same task.

### Generators
Futures in Rust are implemented in a simlar way that Generators are state
machines.

Notice the similarities between generators and async/await, they both generate
statemachines:
```rust
    let mut generator = || {
        println!("in generator, before yield");
        yield 18;
        println!("in generator, before return");
        return "bajja"
    };
```
```rust
    let mut future = async {
        println!("in future, before await");
        some_future().await;
        println!("in future, after await");
        return "bajja"
    };
```

### Doc-comments
Doc-comments like `/// ...` and `!// ...` are syntax sugar for attributes. They
desugar to `#[doc="..."]` and `#![doc="..."]`.

### rvalue static promotion
This following code is valid and compiles ([immutables.rs](src/immutables.rs):
```rust
    let nr = &mut 17;
    *nr += 1;
    println!("nr: {}", nr);
```
The surprising thing to me was that we can declare a reference to a literal. My
initial though was that this would not compile as the literal would be hard
coded into the code (think argument to an assembly instruction. But in Rust case
 what Rust will create a temporary area of memory containing the value. This is
called 'rvalue static promotion`.

```console
$ rustc +nightly -Zunpretty=mir src/immutables.rs
```

### core::panic!
In library/core/src/macros.rs we have:
```rust
#[rustc_builtin_macro(core_panic)]
#[allow_internal_unstable(edition_panic)]
#[stable(feature = "core", since = "1.6.0")]
#[rustc_diagnostic_item = "core_panic_macro"]
macro_rules! panic {
    // Expands to either `$crate::panic::panic_2015` or `$crate::panic::panic_2021`
    // depending on the edition of the caller.
    ($($arg:tt)*) => {
        /* compiler built-in */
    };
}
```
Notice the use of `rustc_builtin_macro(core_panic)]` which can be found in
compiler/rustc_builtin_macros/src/lib.rs. Builtin macros inject code into the
crate before it is lowered into HIR. So this would be done during the
compilation. 
```rust
pub fn register_builtin_macros(resolver: &mut dyn ResolverExpand) {
...
   // Notice that register is a closure
   let mut register = |name, kind| resolver.register_builtin_macro(name, kind);
   macro register_bang($($name:ident: $f:expr,)*) {
        $(register(sym::$name, SyntaxExtensionKind::LegacyBang(Box::new($f as MacroExpanderFn)));)*
   }

   register_bang! {
      ...
      core_panic: edition_panic::expand_panic,
      std_panic:  edition_panic::expand_panic,
   }
}
```
So if we expand one of those macro calls we should get something like:
```
use rustc_span::symbol::sym;

  resolver.register_builtin_macro(
    sym::core_panic,
    SyntaxExtensionKind::LegacyBang(Box::new(edition_panic::expand_panic as MacroExpanderFn)));
```
compiler/rustc_span/src/symbol.rs has the following macro:
```rust
Symbols {
  ...
  core_panic,
  ...
  panic_2015,
  panic_2021,
  ...
  std_panic,
  ...
}
```
In `compiler/rustc_builtin_macros/src/edition_panic.rs` we find:
```rust
pub fn expand_panic<'cx>(                                                       
    cx: &'cx mut ExtCtxt<'_>,                                                   
    sp: Span,                                                                   
    tts: TokenStream,                                                           
) -> Box<dyn MacResult + 'cx> {                                                 
    let mac = if use_panic_2021(sp) { sym::panic_2021 } else { sym::panic_2015 };
    expand(mac, cx, sp, tts)                                                    
}
```
So this will set `mac` (Symbol) to sym::panic_2021 or sym::panic_2015 which is
then passed to expand.
In library/core/src/panic.rs we have:
```rust
pub macro panic_2021 {                                                             
    () => (
        $crate::panicking::panic("explicit panic")
    ),
    // Special-case the single-argument case for const_panic.
    ("{}", $arg:expr $(,)?) => (
        $crate::panicking::panic_display(&$arg)
    ),
    ($($t:tt)+) => (
        $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    ),
}
```

### std::panic!
Is much like `core::panic!` but can be found in libary/std/src/macros.rs.

### abort vs unwind
```console
$ rustc -C panic=abort -o abort - <<HERE
> fn main() {
> panic!("oh no");
> }
> HERE
$ ./abort 
thread 'main' panicked at 'oh no', <anon>:2:1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Aborted (core dumped)
```

```console
$ rustc -C panic=unwind -o abort - <<HERE
fn main() {
panic!("oh no");
}
HERE
$ ./abort 
thread 'main' panicked at 'oh no', <anon>:2:1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### rustc as lib
To use rustc as a library we need to add `rustc-dev` as a component:
```console
$ rustup component add rustc-dev llvm-tools-preview
```

### sty.rs
Can be found in compiler/rustc_type_ir/src/sty.rs.

### link rustc build
```console
$ rustup toolchain link dev ~/work/rust/rust/build/x86_64-unknown-linux-gnu/stage0
$ rustc +dev --version
rustc 1.63.0-beta.2 (6c1f14289 2022-06-28)
```

### lint listing
```console
$ rustc -W help
```

### MaybeUninit
Think about this is terms of C/C++ if declare a variable:
```c
  int x;
```
The value of x would be an address into the stack memory region. There might be
some data at that address which would then be become the value of x. This is
called uninitialized data, and it would be anything that happens to be on the
stack. 
We can't write something like this in Rust, for example the following will
result in a compilation error:
```rust
$ make out/maybeuninit
rustc +nightly --edition=2021 -o out/maybeuninit -g src/maybeuninit.rs
error[E0381]: used binding `x` isn't initialized
  --> src/maybeuninit.rs:10:20
   |
9  |     let x: i32;
   |         - binding declared here but left uninitialized
10 |     println!("{}", x);
   |                    ^ `x` used here but it isn't initialized

```

### Zero-sized type (ZST)
Is a type that occupies no memory and is optimized away by the compiler.
Examples:
* () (empty tuble/unit type)
* ! (never type)
* structs, unions, and tuples if all of their fields are zero-sized. 
* enums if all variants are zero-sized
* PhantomData


### Generic helper functions
If we have parts of a generic function implementation that is not generic, one
might be able to reduce the size of the binary by adding an inner function that
does not operate on the generic part item/type.

[src/mono.rs](./src/mono.rs) tries to show this by inspecting the generated
llvm ir:
```console
$ make -B out/mono-filtered.ll
```
And then we can inspect the output in `out/mono-filtered.ll` and see that
we have two implementations of the generic function `doit`, one for `u8` and one
for `u16`. But there is only a single function for
`mono::Something<T>::doit::inner_function`.


### std::str::FromStr
When we see this in code:
```rustc
   "string".parse();
```
try to remember that this syntactic suger for:
```rustc
  FromStr::from_str("string")
```
We can implement FromStr for own own type as well,
[from_str.rs](src/from_str.rs) contains an example.
