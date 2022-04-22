## Embassy 


### Executor
Lets start by creating an Executor and take a look at what is involved and
what an Executor in Embassy is. This first section will only consider the
following call:
```rust
    let mut executor = ::embassy::executor::Executor::new();
```
This can be compiled and started in the debugger using:
```console
$ cargo build --bin no_macros
$ rust-lldb target/debug/no_macros
(lldb) br s -n no_macros::main
(lldb) s
```
Setting in we land us in embassy/src/executor/arch/std.rs. This is because we
have specified a feature in our dependency declaration in Cargo.toml:
```toml
embassy = { git = "https://github.com/embassy-rs/embassy.git", features = ["log", "std", "time"] }
```
And in embassy/src/executor/mod.rs we find:
```rust
cfg_if::cfg_if! {                                                                  
    if #[cfg(cortex_m)] {                                                          
        #[path="arch/cortex_m.rs"]                                                 
        mod arch;                                                                  
        pub use arch::*;                                                           
    }                                                                              
    else if #[cfg(feature="wasm")] {                                               
        #[path="arch/wasm.rs"]                                                     
        mod arch;                                                                  
        pub use arch::*;                                                           
    }                                                                              
    else if #[cfg(feature="std")] {                                                
        #[path="arch/std.rs"]                                                      
        mod arch;                                                                  
        pub use arch::*;                                                           
    }                                                                              
}          
```
So in our case embassy/src/executor/arch/std.rs will be included:
```rust
    /// Create a new Executor.                                                  
    pub fn new() -> Self {                                                      
        let signaler = &*Box::leak(Box::new(Signaler::new()));                  
        Self {                                                                  
            inner: raw::Executor::new(                                             
                |p| unsafe {                                                       
                    let s = &*(p as *const () as *const Signaler);                 
                    s.signal()                                                     
                },                                                                 
                signaler as *const _ as _,                                         
            ),                                                                     
            not_send: PhantomData,                                                 
            signaler,                                                              
        }                                                                          
    }
```
So Box::leak is saying that we want this new Signaler instance to exist for the
entire lifetime of the program.
```
struct Signaler {                                                                  
    mutex: Mutex<bool>,                                                            
    condvar: Condvar,                                                              
} 
```
Condvar is part of std::sync crate and represent the ability to block a thread
such that it consumes no CPU time while waiting for an event to occur.

Next in the constructor we have:
```rust
    /// Create a new Executor.                                                  
    pub fn new() -> Self {                                                      
        let signaler = &*Box::leak(Box::new(Signaler::new()));                  
        Self {                                                                  
            inner: raw::Executor::new(                                             
                |p| unsafe {                                                       
                    let s = &*(p as *const () as *const Signaler);                 
                    s.signal()                                                     
                },                                                                 
                signaler as *const _ as _,                                         
            ),                                                                     
            not_send: PhantomData,                                                 
            signaler,                                                              
        }                                                                          
    }
```
We are calling the constructor raw::Executor::new:
```rust
pub fn new(signal_fn: fn(*mut ()), signal_ctx: *mut ()) -> Self
...
```
Notice the type of the first parameter is a function that takes a single 
parameter. The type of that parameter is a mutable raw pointer to a unit type
`()`. I'm thinking this is like a pointer to void, so we could cast anything to
this type, for example:
```rust
fn print(n: *mut ()) {
    println!("print...{:?}", n);
}

fn main() {
    let y:i32 = 20;
    print(y as *const i32 as _);
    print(y as *const i32 as *mut ());
}
```
There is an example in [raw_pointers.rs](../src/raw_pointers.rs) that I played
around with to try to figure this out.

Alright, back to Embassy. The actual argument passed to that function will be
a raw pointer to the signaler which is the second argument and notice that it
is casted first to a raw pointer and then to `*mut ()`.
The rest of the constructor is setting the other fields that make up an
Executor:
```rust
/// Single-threaded std-based executor.                                         
pub struct Executor {                                                           
    inner: raw::Executor,                                                       
    not_send: PhantomData<*mut ()>,                                             
    signaler: &'static Signaler,                                                
}
```
Lets take a closer look at `raw::Executor`. This struct has the following
members:
```rust
pub struct Executor {                                                           
    run_queue: RunQueue,                                                        
    signal_fn: fn(*mut ()),                                                     
    signal_ctx: *mut (),                                                        
                                                                                
    #[cfg(feature = "time")]                                                    
    pub(crate) timer_queue: timer_queue::TimerQueue,                            
    #[cfg(feature = "time")]                                                    
    alarm: AlarmHandle,                                                         
}
```
We saw `signal_fn` and `signal_ctx` above which were passed into the constructor
. The constructor for `raw::Executor` looks like this
(in embassy/src/executor/raw/mod.rs):
```rust
pub fn new(signal_fn: fn(*mut ()), signal_ctx: *mut ()) -> Self {           
        #[cfg(feature = "time")]                                                
        let alarm = unsafe { unwrap!(driver::allocate_alarm()) };               
        #[cfg(feature = "time")]                                                
        driver::set_alarm_callback(alarm, signal_fn, signal_ctx);               
                                                                                
        Self {                                                                  
            run_queue: RunQueue::new(),                                         
            signal_fn,                                                          
            signal_ctx,                                                         
                                                                                
            #[cfg(feature = "time")]                                            
            timer_queue: timer_queue::TimerQueue::new(),                        
            #[cfg(feature = "time")]                                            
            alarm,                                                              
        }                                                                       
    }                        
```
This example as the feature `time` configured so this will call
`driver::allocate_alarm()` which can be found in embassy/src/time/driver.rs):
Stepping into this call will land us in:
```console
   120 	/// Safety: it is UB to make the alarm fire before setting a callback.
   121 	pub(crate) unsafe fn allocate_alarm() -> Option<AlarmHandle> {
-> 122 	    _embassy_time_allocate_alarm()
   123 	}
```
If we look in embassy/src/time/mod.rs we can find that depending on the
feature, in our case `std` then the driver_std module:
```rust
#[cfg(feature = "std")]                                                            
mod driver_std;
```
_wip_




