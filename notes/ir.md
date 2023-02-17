## Intermediate representation
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

Show High-Level IR (HIR) with types:
```console
$ echo 'fn main(){}' | rustc +nightly --edition=2018 -Zunpretty=hir,typed just-main.rs 
```

Show Mid-Level IR (MIR):
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

### Control Flow Graphs (CFG)
Is structured as a set of basic blocks (bb) which are connected by edges. A BB
consists of a set of statements and when you branch to a BB you start executing
the statement in order one after the other. At the end of the block there can
be a branch to another block which is called a terminator.

```rust
bb0:  { // basic block 0
  statement0;
  statement1;
  ...
  terminator;
}
```


## Middle Intermedieate Representation (MIR)
This representation is generated from the HIR and will be transformed into
LLVM-IR. It was described in a presentation as "Rust's simple core" and has
explicit types, explicit panics, not loops. It's a control flow graph.

Local variables are specified using a `_` prefix followed by a number:
```
let mut _1: i32;
```
The local `_0` is reserved for storing the return value:
```
let mut _0: ();
```
Notice that the type is the unit type.

I variable can har fields which is denoted as `_1.f` where for example:
```
let mut _1: (i32, bool);
```
Notice here that the type is a tuple, and `_1.0` is the i32 value, and `_1.1` is
the bool value.

`StorageLive(var)` denotes that storage is to be allocated for _1 on the stack.
`StorageDead(var)` denotes deallocating from the stack.

Lets take a very simple example ([mir.rs](src/mir.rs):
```rust
$ rustc +nightly -Zunpretty=mir src/mir.rs 
fn main() -> () {
    let mut _0: ();                      // return place in scope 0 at src/mir.rs:1:11: 1:11
    let mut _1: i32;                     // in scope 0 at src/mir.rs:2:9: 2:15
    scope 1 {
        debug _x => _1;                  // in scope 1 at src/mir.rs:2:9: 2:15
    }

    bb0: {
        _1 = const 16_i32;               // scope 0 at src/mir.rs:2:18: 2:20
        _1 = const 18_i32;               // scope 1 at src/mir.rs:3:5: 3:16
        return;                          // scope 0 at src/mir.rs:4:2: 4:2
    }
}
```
Here we can see that `_0` is a variable that will be used as the return value
of the main function, notice that the main function in MIR actually returns
`()`.
After that we have `_1` which is the `_x` variable in the source
[mir.rs](src/mir.rs).
We only have one scope in our main function, and the `debug` keyword is to
associate our variable name which with variable "index" in MIR.
Following that we have the first and only basic block, `bb0`. Now, when we use
rustc above the default value for
[opt-level](https://doc.rust-lang.org/rustc/codegen-options/index.html#opt-level)
(optimization level) is 0.

The optimization levels can be set using the `-C` which stands for `Codegen`
flag to rustc. We can turn on all optimizations using opt-level 3:
```console
$ rustc +nightly -Zunpretty=mir -C opt-level=3 src/mir.rs 
fn main() -> () {
    let mut _0: ();                      // return place in scope 0 at src/mir.rs:1:11: 1:11
    let mut _1: i32;                     // in scope 0 at src/mir.rs:2:9: 2:15
    scope 1 {
        debug _x => _1;                  // in scope 1 at src/mir.rs:2:9: 2:15
    }

    bb0: {
        StorageLive(_1);                 // scope 0 at src/mir.rs:2:9: 2:15
        StorageDead(_1);                 // scope 0 at src/mir.rs:4:1: 4:2
        return;                          // scope 0 at src/mir.rs:4:2: 4:2
    }
}
```
In this case since we don't actually use the variable it has been optimized out.

One thing to keep in mind when looking at MIR is that this is intended to be
used for optimizations, and will then be lowered into llvm-ir.


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
