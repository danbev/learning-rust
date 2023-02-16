## Rust related gdb notes

### gdb vector inspection
Lets say we have a vector and one element will look like this:
```console
$11 = Vec(size=20) = {
seedwing_policy_engine::package::Package {
  path: seedwing_policy_engine::runtime::PackagePath {
    is_absolute: true,
    path: Vec(size=1) = {
      seedwing_policy_engine::lang::parser::Located<seedwing_policy_engine::runtime::PackageName> {
        inner: seedwing_policy_engine::runtime::PackageName ("lang"),
        location: seedwing_policy_engine::lang::parser::Location {
          span: core::ops::range::Range<usize> {start: 0, end: 0}
        }
      }
    }
  },
  functions: HashMap(size=6) = {
    ["traverse"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0},
    ["and"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0},
    ["chain"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0},
    ["not"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0},
    ["or"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0},
    ["refine"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}
   },
   sources: Vec(size=0)},
```
Now, how do we index into an array?  
Trying the following will fail:
```console
(gdb) p self.packages[0]
Cannot subscript non-array type
(gdb) p self.packages.get(0)
Could not find function named 'alloc::vec::Vec<seedwing_policy_engine::package::Package, alloc::alloc::Global>::get'
```

But we can access the internals of a vector.
```console
(gdb) p self.packages.len 
$31 = 20
(gdb) p self.packages.buf
$32 = alloc::raw_vec::RawVec<seedwing_policy_engine::package::Package, alloc::alloc::Global> {ptr: core::ptr::unique::Unique<seedwing_policy_engine::package::Package> {pointer: core::ptr::non_null::NonNull<seedwing_policy_engine::package::Package> {pointer: 0x555557950080}, _marker: core::marker::PhantomData<seedwing_policy_engine::package::Package>}, cap: 32, alloc: alloc::alloc::Global}
```
With this information we can try to get a pointer to this first element using:
```console
(gdb) p self.packages.buf.ptr.pointer.pointer
$35 = (*mut seedwing_policy_engine::package::Package) 0x555557950080
```
And we can dereference:
```console
(gdb) p *$35
$36 = seedwing_policy_engine::package::Package {path: seedwing_policy_engine::runtime::PackagePath {is_absolute: true, path: Vec(size=1) = {
      seedwing_policy_engine::lang::parser::Located<seedwing_policy_engine::runtime::PackageName> {inner: seedwing_policy_engine::runtime::PackageName ("lang"), location: seedwing_policy_engine::lang::parser::Location {span: core::ops::range::Range<usize> {start: 0, end: 0}}}}}, functions: HashMap(size=6) = {["traverse"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["and"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["chain"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["not"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["or"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["refine"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}}, sources: Vec(size=0)}
```

`buf.ptr.pointer.pointer` is a pointer so what if we increment it by one:
```console
(gdb) p *(self.packages.buf.ptr.pointer.pointer + 1)
$38 = seedwing_policy_engine::package::Package {path: seedwing_policy_engine::runtime::PackagePath {is_absolute: true, path: Vec(size=1) = {
      seedwing_policy_engine::lang::parser::Located<seedwing_policy_engine::runtime::PackageName> {inner: seedwing_policy_engine::runtime::PackageName ("list"), location: seedwing_policy_engine::lang::parser::Location {span: core::ops::range::Range<usize> {start: 0, end: 0}}}}}, functions: HashMap(size=6) = {["any"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["all"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["none"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["some"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["head"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}, ["tail"] = Arc(strong=1, weak=0) = {
      value = dyn seedwing_policy_engine::core::Function, strong = 1, weak = 0}}, sources: Vec(size=0)}
```
And if we keep doing that we can list a specific index.

### pretty-printers
We can list the pretty-printers that have been registered using:
```console
(gdb) info pretty-printer 
global pretty-printers:
  builtin
    mpx_bound128
objfile /home/danielbevenius/work/security/seedwing/seedwing-policy/target/debug/seedwing-policy-server pretty-printers:
  lookup
```
Notice `lookup`, so where does this come from?  


If we look in rust source tree we can find
`src/etc/gdb_load_rust_pretty_printers.py` which contains:
```python
import gdb                                                                      
import gdb_lookup                                                               
gdb_lookup.register_printers(gdb.current_objfile())
```
The first import it `gdb` module which provides access to the gdb api.
The second import is importing the file `gdb_lookup.py` from the same directory.
And in `src/etc/gdb_lookup.py` have a function named `register_printers`:
```python
def register_printers(objfile):                                                 
    objfile.pretty_printers.append(lookup)
```
And `lookup` is a function in the same file.

So those are the files in rust source tree, where are these installed when we
install Rust using rustup?
To answer that we need to find the `sysroot` of our installed `rustc`:
```console
$ rustc --print=sysroot
/home/danielbevenius/.rustup/toolchains/stable-x86_64-unknown-linux-gnu
```
And we can find the the files in:
```console
$ ls `rustc --print=sysroot`/lib/rustlib/etc/*gdb*.py
~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/etc/gdb_load_rust_pretty_printers.py
~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/etc/gdb_lookup.py
~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/etc/gdb_providers.py
```
So this can be useful if you want to check what is actually happening in
these python scripts.

### Print String
```console
(gdb) p each
$30 = (*mut alloc::string::String) 0x5555586f4ae0

(gdb) p *each
$31 = alloc::string::String {vec: alloc::vec::Vec<u8, alloc::alloc::Global> {buf: alloc::raw_vec::RawVec<u8, alloc::alloc::Global> {ptr: core::ptr::unique::Unique<u8> {pointer: core::ptr::non_null::NonNull<u8> {pointer: 0x5555586f4ab0}, _marker: core::marker::PhantomData<u8>}, cap: 12, alloc: alloc::alloc::Global}, len: 12}}

(gdb) printf "%s\n", (*each).vec.buf.ptr.pointer.pointer
sample-data/
```

