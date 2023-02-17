## Pin
Rust types fall into 2 different categories, types that can be moved around
in memory like primitives (numbers, bools, etc, and all types made up on these
primitive types). These are called Unpinned types. Unpin is an autotrait which
all types that are moveble get by default.

Then there are also types that cannot be moved around in memory as this will
cause them to misbehave. An example follows below were we have a struct with
a field/member pointer, that points to another field/member of the struct.
These types are !Unpin (does not implement Unpin).

```rust
pub struct Pin<P> {
    pointer: P,
}
```

```
  +------------+     +---------+      +--------+
  |Pin<Pointer>|---->| Pointer |----->| Data   |
  +------------+     +---------+      +--------+
```
When I first saw this I thought that perhaps this indirection does something
fancy like updating the pointer if moved, but that does not seem to be the case.
Pinning does nothing special with memory allocation like putting it into some
"read only" memory or anything like that. It only uses the type system to
prevent certain operations on this value. But I think it does provide the
ability to use swap, as that would only swap the Pin's pointer.

Pin prevents the Pointer from being moved and this allows for using it in
self-referencial structs.

Notice that pointer is not public (pub) but private so we can only access it by
using methods that this type provides.

Example of creating a new Pin (pinned pointer):
```rust
    let p = Pin::new(&10);
    println!("{:?}", p);
```
Take a struct that looks like this:
```rust
struct Something<'a> {
    val: i32,
    ptr_to_val: &'a i32,
}
```
So we have a member `val` which is just a i32, and then we have a reference
to that that member.
```rust
fn main() {
    let dummy = 10;
    let val = 8;
    let mut s = Something{val: val, ptr_to_val: &dummy};
    s.ptr_to_val = &s.val;
    println!("&s.val: {:p}, &s.ptr_to_val: {:p}", &s.val, s.ptr_to_val);
}
```

```console
$ rust-lldb pin
(lldb) br s -n main -f pin.rs
(lldb) r
(lldb) expr &s.val
(int *) $2 = 0x00007fffffffccf0
(lldb) expr s.ptr_to_val
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
Now what happens if we move our struct into another value?  
Well, in theory the values would be copied to a new location on the stack. So
the value in `s.val` would still be 8, and the value in `s.ptr_to_val` would
still be the address to the old location on the stack. There is an example of
this in [Pin2.rs](./src/pin2.rs).
```console
$ make out/pin2
rustc --edition 2021 -o out/pin2 -g src/pin2.rs

$ ./out/pin2
t.a: 0x7ffc4a2e8c68 first
t.b: 0x7ffc4a2e8c80 0x7ffc4a2e8c68 first

t2.a: 0x7ffc4a2e8d78 second
t2.b: 0x7ffc4a2e8d90 0x7ffc4a2e8d78 second

Now swap t and t2)

t.a: 0x7ffc4a2e8c68 second
t.b: 0x7ffc4a2e8c80 0x7ffc4a2e8d78 first

t2.a: 0x7ffc4a2e8d78 first
t2.b: 0x7ffc4a2e8d90 0x7ffc4a2e8c68 second
```
So notice that this has just copied the bytes when swaping and we can see
that the address of `t.b` stayed the same but the value was replaced with the
value of t2.b:
```console
t.b: 0x7ffc4a2e8c80 0x7ffc4a2e8c68 first
t.b: 0x7ffc4a2e8c80 0x7ffc4a2e8d78 first
```
And the same goes for t2:
```console
t2.b: 0x7ffc4a2e8d90 0x7ffc4a2e8d78 second
t2.b: 0x7ffc4a2e8d90 0x7ffc4a2e8c68 second
```

Now Pin is only of interest where you have types that refer to data items within
themselves. If you don't have such a situation the type is `Unpin`. `Unpin`
is an auto trait, that is if the data types only contains members that are
`Unpin` your data type will also be Unpin. 

```rust
#[stable(feature = "pin", since = "1.33.0")]
#[lang = "pin"]
#[fundamental]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Pin<P> {
    #[unstable(feature = "unsafe_pin_internals", issue = "none")]
    #[doc(hidden)]
    pub pointer: P,
}
```
### Pin::Box
Just like in the normal pin situation we don't want the thing in Box (on the
heap) to be moved. So we wrap the Box in a Pin which sounds reaonable.

Lets take a look at an example:
```rust
    let pinned_x = Box::pin(18);
```
And we can start a debugging session and step-through the code using:
```console
$ rust-gdb --args out/pin-box 
Reading symbols from out/pin-box...
(gdb) br pin-box.rs:4
Breakpoint 1 at 0x88f7: file src/pin-box.rs, line 4.

5	    let pinned_x = Box::pin(18);
(gdb) s
alloc::boxed::{impl#0}::pin<i32> (x=18) at /rustc/0416b1a6f6d5c42696494e1a3a33580fd3f669d8/library/alloc/src/boxed.rs:286
286	    pub fn pin(x: T) -> Pin<Box<T>> {
(gdb) l
281	    /// construct a (pinned) `Box` in a different way than with [`Box::new`].
282	    #[cfg(not(no_global_oom_handling))]
283	    #[stable(feature = "pin", since = "1.33.0")]
284	    #[must_use]
285	    #[inline(always)]
286	    pub fn pin(x: T) -> Pin<Box<T>> {
287	        (#[rustc_box]
288	        Box::new(x))
289	        .into()
290	    }
```
So notice that first a new Box is created and we are copying our value `x` into
a location on the heap. After that `into()` is called.

```console
(gdb) l
1472	    ///
1473	    /// Constructing and pinning a `Box` with <code><Pin<Box\<T>>>::from([Box::new]\(x))</code>
1474	    /// can also be written more concisely using <code>[Box::pin]\(x)</code>.
1475	    /// This `From` implementation is useful if you already have a `Box<T>`, or you are
1476	    /// constructing a (pinned) `Box` in a different way than with [`Box::new`].
1477	    fn from(boxed: Box<T, A>) -> Self {
1478	        Box::into_pin(boxed)
1479	    }
1480	}
1481	
```
And `Box::into_pin` looks like this:
```console
(gdb) l -
1224	    pub const fn into_pin(boxed: Self) -> Pin<Self>
1225	    where
1226	        A: 'static,
1227	    {
1228	        // It's not possible to move or replace the insides of a `Pin<Box<T>>`
1229	        // when `T: !Unpin`, so it's safe to pin it directly without any
1230	        // additional requirements.
1231	        unsafe { Pin::new_unchecked(boxed) }
1232	    }
1233	}
1234	
```

```console
(gdb) p pinned_x
$5 = core::pin::Pin<alloc::boxed::Box<i32, alloc::alloc::Global>> {pointer: 0x5555555a5ba0}
(gdb) p *pinned_x.pointer 
$7 = 18
```


