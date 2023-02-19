### std::convert::From
[from.rs](./src/from.rs)

How does this actually work. Like if I have implemented From then I can write
```rust
#[derive(Debug)]
struct Something {
    name: String,
}

impl From<String> for Something {
    fn from(s: String) -> Something {
        Something { name: s }
    }
}

fn process(s: Something) -> () {
    println!("processing: {:?}", s)
}

fn main() {
    process(String::from("second").into());
}
```
So, String::from will return a String from a `&'static str` which we are the
calling .into on. And the fuction being called takes a `Something` struct.

Lets take a look at this in a debugger.
```console
$ rust-gdb --args out/from 
Reading symbols from out/from...

(gdb) shell make out/from
rustc "-Copt-level=0" "--edition=2021" -o out/from -g src/from.rs

(gdb) r
The program being debugged has been started already.
Start it from the beginning? (y or n) y
`/home/danielbevenius/work/rust/learning-rust/out/from' has changed; re-reading symbols.
Starting program: /home/danielbevenius/work/rust/learning-rust/out/from 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, from::main () at src/from.rs:23
23	    process(String::from("second").into());

(gdb) disassemble 
Dump of assembler code for function _ZN4from4main17h221e1a30e9e5d27dE:
   0x000055555555d1e0 <+0>:	    sub    $0x38,%rsp
=> 0x000055555555d1e4 <+4>:	    lea    0x20(%rsp),%rdi
   0x000055555555d1e9 <+9>:	    lea    0x36e1d(%rip),%rsi        # 0x55555559400d
   0x000055555555d1f0 <+16>:	mov    $0x6,%edx
   0x000055555555d1f5 <+21>:	call   0x55555555cfc0 <alloc::string::{impl#53}::from>
   0x000055555555d1fa <+26>:	lea    0x8(%rsp),%rdi
   0x000055555555d1ff <+31>:	lea    0x20(%rsp),%rsi
   0x000055555555d204 <+36>:	call   0x55555555cf20 <core::convert::{impl#3}::into<alloc::string::String, from::Something>>
   0x000055555555d209 <+41>:	lea    0x8(%rsp),%rdi
   0x000055555555d20e <+46>:	call   0x55555555d140 <from::process>
   0x000055555555d213 <+51>:	add    $0x38,%rsp
   0x000055555555d217 <+55>:	ret    
End of assembler dump.
```
First thing first, lets check what the `flavor` this output is in:
```console
(gdb) show disassembly-flavor 
The disassembly flavor is "att".
```
So this is AT&T format which means the format is `op source destination`.

Lets set a break point on the first instruction:
```console
(gdb) br *0x000055555555d1e0
Breakpoint 2 at 0x55555555d1e0: file src/from.rs, line 16.

(gdb) r
The program being debugged has been started already.
Start it from the beginning? (y or n) y
Starting program: /home/danielbevenius/work/rust/learning-rust/out/from 

Breakpoint 2, from::main () at src/from.rs:16
16	fn main() {

(gdb) disassemble 
Dump of assembler code for function _ZN4from4main17h221e1a30e9e5d27dE:
=> 0x000055555555d1e0 <+0>:	    sub    $0x38,%rsp
   0x000055555555d1e4 <+4>:	    lea    0x20(%rsp),%rdi
   0x000055555555d1e9 <+9>:	    lea    0x36e1d(%rip),%rsi        # 0x55555559400d
   0x000055555555d1f0 <+16>:	mov    $0x6,%edx
   0x000055555555d1f5 <+21>:	call   0x55555555cfc0 <alloc::string::{impl#53}::from>
   0x000055555555d1fa <+26>:	lea    0x8(%rsp),%rdi
   0x000055555555d1ff <+31>:	lea    0x20(%rsp),%rsi
   0x000055555555d204 <+36>:	call   0x55555555cf20 <core::convert::{impl#3}::into<alloc::string::String, from::Something>>
   0x000055555555d209 <+41>:	lea    0x8(%rsp),%rdi
   0x000055555555d20e <+46>:	call   0x55555555d140 <from::process>
   0x000055555555d213 <+51>:	add    $0x38,%rsp
   0x000055555555d217 <+55>:	ret    
End of assembler dump.
```
First we substract from the stack pointer to make room for local variables. This
is reserving 56 bytes by subtracting that from the stack pointer (rsp).

Notice that this first instruction, `lea` computes the effective address of
the second operand (rdi), and then stores the result in the first operand which
in this case is a location on the stack.
```console
(gdb) disassemble 0x55555555d1e0
or 
(gdb) disassemble $rdi
Dump of assembler code for function from::main:
   0x000055555555d1e0 <+0>:	    sub    $0x38,%rsp
=> 0x000055555555d1e4 <+4>:	    lea    0x20(%rsp),%rdi
   0x000055555555d1e9 <+9>:   	lea    0x36e1d(%rip),%rsi        # 0x55555559400d
   0x000055555555d1f0 <+16>:	mov    $0x6,%edx
   0x000055555555d1f5 <+21>:	call   0x55555555cfc0 <alloc::string::{impl#53}::from>
   0x000055555555d1fa <+26>:	lea    0x8(%rsp),%rdi
   0x000055555555d1ff <+31>:	lea    0x20(%rsp),%rsi
   0x000055555555d204 <+36>:	call   0x55555555cf20 <core::convert::{impl#3}::into<alloc::string::String, from::Something>>
   0x000055555555d209 <+41>:	lea    0x8(%rsp),%rdi
   0x000055555555d20e <+46>:	call   0x55555555d140 <from::process>
   0x000055555555d213 <+51>:	add    $0x38,%rsp
   0x000055555555d217 <+55>:	ret    
End of assembler dump.
```
This instruction is taking the address of first operand, 0x20(%rsp) which is
a memory location on the stack, and storing that address in $rdi (the first
argument to the coming function).
Next we have:
```
   0x000055555555d1e9 <+9>:   	lea    0x36e1d(%rip),%rsi        # 0x55555559400d
```
And this is loading the address of the string "second" which would be in the
data section of our program.
```console
(gdb) x/1xg 0x36e1d + $rip
0x55555559400d:	0x6f53646e6f636573
```
Notice that is the lea instruction loads the address which is the left-most
value `0x55555559400d` into `rsi`.
We can inspect the memory location to verify that this is indeed our string:
```console
(gdb) x/6cb 0x55555559400d
0x55555559400d:	115 's'	101 'e'	99 'c'	111 'o'	110 'n'	100 'd'
```
Next we have:
```
=> 0x000055555555d1f0 <+16>:	mov    $0x6,%edx
```
And this is the length of our string. Recall that a String in rust consist
of a pointer and a length.
So to recap, we are passing a memory address to the stack in `rdi`, and then
the pointer to our string in `rsi`, and the length in `edx` which are the the
arguments to String::from("second").

Next we have the call to 
```console
=> 0x000055555555d1f5 <+21>:	call   0x55555555cfc0 <alloc::string::{impl#53}::from>
```
This was interesting and also a good refresher for me as I've mainly been using
lldb to inspect memory and things like that. But what I'm really interesed in
is the initial quiestion above. 

The part I was missing is the second call instruction:
```console
   0x000055555555d204 <+36>:	call   0x55555555cf20 <core::convert::{impl#3}::into<alloc::string::String, from::Something>>
   0x000055555555d209 <+41>:	lea    0x8(%rsp),%rdi
   0x000055555555d20e <+46>:	call   0x55555555d140 <from::process>
   0x000055555555d213 <+51>:	add    $0x38,%rsp
   0x000055555555d217 <+55>:	ret    
```
If we just look at the function being called part:
```
<core::convert::{impl#3}::into<alloc::string::String, from::Something>>
```
We can see that we are calling a function that is generic over a
`alloc::string::String`, and a `from::Something`.
Notice that we did not write this function. Lets set a breakpoint on the line
with the call instruction:
```console
(gdb) br *0x000055555555d204
Breakpoint 2 at 0x55555555d204: file src/from.rs, line 23.
```
This functions source looks like this:
```console
(gdb) l 715,728
715	#[stable(feature = "rust1", since = "1.0.0")]
716	#[rustc_const_unstable(feature = "const_convert", issue = "88674")]
717	impl<T, U> const Into<U> for T
718	where
719	    U: ~const From<T>,
720	{
721	    /// Calls `U::from(self)`.
722	    ///
723	    /// That is, this conversion is whatever the implementation of
724	    /// <code>[From]&lt;T&gt; for U</code> chooses to do.
725	    fn into(self) -> U {
726	        U::from(self)
727	    }
728	}
```
In our case `T` is alloc::string::String, and `U` is `from::Something. So that
could be read as:
```console
725	    fn into(self) -> from::Something {
726	        from::Something::from(self)
727	    }
```
So I think that answers my question about how `into` actually calls a custom
`from` implementation.
