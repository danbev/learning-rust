## Linking


### Rust Codegen Units (RCGU)
```console
$ rustc --print link-args src/phantom_data_unused.rs
LC_ALL="C"

VSLANG="1033"

"cc" "-m64"
"/tmp/rustcfsFjN5/symbols.o"
"phantom_data_unused.phantom_data_unused.19238de8-cgu.0.rcgu.o"
"phantom_data_unused.phantom_data_unused.19238de8-cgu.1.rcgu.o"
"phantom_data_unused.phantom_data_unused.19238de8-cgu.2.rcgu.o"
"phantom_data_unused.phantom_data_unused.19238de8-cgu.3.rcgu.o"
"phantom_data_unused.phantom_data_unused.19238de8-cgu.4.rcgu.o"
"phantom_data_unused.2qi44crxxlknf84s.rcgu.o"
...
```
These files are `rust codegen units (RCGU)` which are then linked into the
executable. But we can tell rustc to keep these files:
```console
$ rustc -v -Csave-temps=yes --print link-args src/phantom_data_unused.rs
```
```console
$ ls phantom_data_unused.phantom_data_unused.19238de8-cgu.*
phantom_data_unused.phantom_data_unused.19238de8-cgu.0.rcgu.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.0.rcgu.no-opt.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.0.rcgu.o
phantom_data_unused.phantom_data_unused.19238de8-cgu.1.rcgu.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.1.rcgu.no-opt.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.1.rcgu.o
phantom_data_unused.phantom_data_unused.19238de8-cgu.2.rcgu.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.2.rcgu.no-opt.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.2.rcgu.o
phantom_data_unused.phantom_data_unused.19238de8-cgu.3.rcgu.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.3.rcgu.no-opt.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.3.rcgu.o
phantom_data_unused.phantom_data_unused.19238de8-cgu.4.rcgu.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.4.rcgu.no-opt.bc
phantom_data_unused.phantom_data_unused.19238de8-cgu.4.rcgu.o
```
The `.bc` files are LLVM [bitcode](https://github.com/danbev/learning-llvm#bitcode)
files.
```console
$ llvm-dis phantom_data_unused.phantom_data_unused.19238de8-cgu.0.rcgu.bc -o phantom.ll
```
And we can inspect the object files using `nm`:
```console
$ nm phantom_data_unused.phantom_data_unused.19238de8-cgu.0.rcgu.o | rustfilt 
0000000000000000 T <() as std::process::Termination>::report

$ nm phantom_data_unused.phantom_data_unused.19238de8-cgu.1.rcgu.o | rustfilt 
0000000000000000 V DW.ref.rust_eh_personality
0000000000000000 r GCC_except_table1
                 U rust_eh_personality
                 U _Unwind_Resume
                 U std::rt::lang_start::{{closure}}
0000000000000000 T core::ops::function::FnOnce::call_once{{vtable.shim}}
0000000000000000 t core::ops::function::FnOnce::call_once
0000000000000000 T core::ops::function::FnOnce::call_once
0000000000000000 T core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>

$ nm phantom_data_unused.phantom_data_unused.19238de8-cgu.2.rcgu.o | rustfilt 
0000000000000000 T std::sys_common::backtrace::__rust_begin_short_backtrace
                 U core::ops::function::FnOnce::call_once

$ nm phantom_data_unused.phantom_data_unused.19238de8-cgu.3.rcgu.o | rustfilt 
                 U std::sys_common::backtrace::__rust_begin_short_backtrace
0000000000000000 T std::rt::lang_start
0000000000000000 T std::rt::lang_start::{{closure}}
                 U std::rt::lang_start_internal
                 U core::ops::function::FnOnce::call_once{{vtable.shim}}
                 U core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
                 U <() as std::process::Termination>::report

$ nm phantom_data_unused.phantom_data_unused.19238de8-cgu.4.rcgu.o | rustfilt 
0000000000000000 T main
0000000000000000 t phantom_data_unused::main
                 U std::rt::lang_start
```

### libraries
```console
$ rustc --print link-args src/phantom_data_unused.rs
```
"-L"
  "libstd-67e0fe4bfa018a5e.rlib"
  "libpanic_unwind-6e25273444177929.rlib"
  "libobject-05da49d3cca73bff.rlib"
  "libmemchr-616c9bd2710f0982.rlib"
  "libaddr2line-d9df84ec1a8a7a8f.rlib"
  "libgimli-0242ef3eea1e9db2.rlib"
  "librustc_demangle-127f477a16f3f8f8.rlib"
  "libstd_detect-eb235cc34134320b.rlib"
  "libhashbrown-c5f20f2274212453.rlib"
  "libminiz_oxide-4483c8bc4648568f.rlib"
  "libadler-94da6a76998341a3.rlib"
  "librustc_std_workspace_alloc-22a9646e8f27a6e4.rlib"
  "libunwind-eb91273024ac0258.rlib"
  "libcfg_if-323da837c64ef472.rlib"
  "liblibc-6d46d38f739892fe.rlib"
  "liballoc-8212dcd77adfe144.rlib"
  "librustc_std_workspace_core-522518611024dce5.rlib"
  "libcore-05898138a596088a.rlib"
  "libcompiler_builtins-b78d27aa9e5e005b.rlib"
```
Recall that `-L`adds a directory to the library search path.

### libstd
```console
$ mkdir stdlib-archive
$ ar x /home/danielbevenius/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-67e0fe4bfa018a5e.rlib --output stdlib-archive
$ ls stdlib-archive/
lib.rmeta  std-67e0fe4bfa018a5e.std.b53c8129-cgu.0.rcgu.o
```
The `lib.rmeta` file contains [metadata](https://rustc-dev-guide.rust-lang.org/backend/libs-and-metadata.html#metadata) for the crate. 


To inspect all the symbols in std lib use:
```console
$ nm std-67e0fe4bfa018a5e.std.b53c8129-cgu.0.rcgu.o | rustfilt
```

### libgimli
[gimli](https://docs.rs/gimli/latest/gimli/) is a library for reading DWARF
debugging formats and the name is not so odd when you think about what it does.

### libminiz_oxide
Is a rust impl of [miniz](https://github.com/richgel999/miniz) which is a
zlib encoder decoder.

### libaddr2line
Is a library/binary for translating cross-platform addresses into function
name, file name, and line number. So an address in an executable can be passed
to add2line and will produce the filename and line number by using debugging
information stored in the executable.

### libobject
Is a library that provides cross platform support for reading object and
executable files. The lib can
also write object files (COFF/ELF/Mach-O) and execuble files (ELF/PE).

### emit object file
```
$ rustc src/phantom_data_unused.rs --emit=obj
$ file phantom_data_unused.o 
phantom_data_unused.o: ELF 64-bit LSB relocatable, x86-64, version 1 (SYSV), not stripped
$ nm phantom_data_unused.o | rustfilt 
0000000000000000 V DW.ref.rust_eh_personality
0000000000000000 r GCC_except_table4
0000000000000000 T main
                 U rust_eh_personality
                 U _Unwind_Resume
0000000000000000 t phantom_data_unused::main
0000000000000000 t std::sys_common::backtrace::__rust_begin_short_backtrace
0000000000000000 T std::rt::lang_start
0000000000000000 t std::rt::lang_start::{{closure}}
                 U std::rt::lang_start_internal
0000000000000000 t core::ops::function::FnOnce::call_once{{vtable.shim}}
0000000000000000 t core::ops::function::FnOnce::call_once
0000000000000000 t core::ops::function::FnOnce::call_once
0000000000000000 t core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
0000000000000000 t <() as std::process::Termination>::report
```
