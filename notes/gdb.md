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

