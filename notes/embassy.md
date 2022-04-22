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
For more a standalone example of using Condvar see
[condvar.rs](../src/condvar.rs), but basically it can be used to block and the
used to notify.

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
This example has the feature `time` configured so this will call
`driver::allocate_alarm()` which can be found in embassy/src/time/driver.rs).
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
Notice that `allocate_alarm` is in driver.rs, and that it is calling
`_embassy_time_allocate_alarm` which is generated by a macro. To understand
how this works we need to look in embassy/src/time/driver_std.rs and the usage
of the macro `time_driver_impl`:
```rust
crate::time_driver_impl!(static DRIVER: TimeDriver = TimeDriver {                  
    alarm_count: AtomicU8::new(0),                                                 
                                                                                   
    once: Once::new(),                                                             
    alarms: UninitCell::uninit(),                                                  
    zero_instant: UninitCell::uninit(),                                            
    signaler: UninitCell::uninit(),                                                
});
```
```rust
#[macro_export]                                                                    
macro_rules! time_driver_impl {                                                    
    (static $name:ident: $t: ty = $val:expr) => {                                  
        static $name: $t = $val;                                                   
                                                                                   
        #[no_mangle]                                                               
        unsafe fn _embassy_time_allocate_alarm() -> Option<$crate::time::driver::AlarmHandle> {
            <$t as $crate::time::driver::Driver>::allocate_alarm(&$name)           
        }                                                                          
                                                                                   
    };                                                                             
}
```
```rust
static DRIVER: TimeDriver = TimeDriver {
        alarm_count: AtomicU8::new(0),
        once: Once::new(),
        alarms: UninitCell::uninit(),
        zero_instant: UninitCell::uninit(),
        signaler: UninitCell::uninit()};
...

#[no_mangle]
unsafe fn _embassy_time_allocate_alarm() -> Option<crate::time::driver::AlarmHandle> {
    <TimeDriver as crate::time::driver::Driver>::allocate_alarm(&DRIVER)
}
```
So the above call will land us in embassy/src/time/driver_std.rs and the
allocate_alarm function.
```rust
impl Driver for TimeDriver {                                                    
  ...
    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {                    
        let id = self                                                           
            .alarm_count                                                        
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |x| {            
                if x < ALARM_COUNT as u8 {                                      
                    Some(x + 1)                                                 
                } else {                                                        
                    None                                                        
                }                                                               
            });                                                                 
                                                                                
        match id {                                                              
            Ok(id) => Some(AlarmHandle::new(id)),                               
            Err(_) => None,                                                     
        }                                                                       
    }                                  
```
After this function call returns we will have an id for the alarm and return it
which will return us to raw::Executor::new:
```rust

let alarm = unsafe { unwrap!(driver::allocate_alarm()) };
driver::set_alarm_callback(alarm, signal_fn, signal_ctx);
```
`set_alarm_callback` is similar to `allocate_alarm` with regards to the macro
handling so we can look in embassy/src/time/driver_std.rs to see the
implementation of this function:
```rust
    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        self.init();                                                            
        let mut alarms = unsafe { self.alarms.as_ref() }.lock().unwrap();       
        let alarm = &mut alarms[alarm.id() as usize];                           
        alarm.callback = callback as *const ();                                 
        alarm.ctx = ctx;                                                        
    }
```

TimerDriver::init is in the same file and look like this:
```rust
const ALARM_COUNT: usize = 4;

impl TimeDriver {                                                               
    fn init(&self) {                                                            
        self.once.call_once(|| unsafe {                                         
            self.alarms.write(Mutex::new([ALARM_NEW; ALARM_COUNT]));            
            self.zero_instant.write(StdInstant::now());                         
            self.signaler.write(Signaler::new());                               
                                                                                
            thread::spawn(Self::alarm_thread);                                  
        });                                                                     
    }
```
Here the std::once member which can be used for one time initialization which
is what it is used for in this case.
Notice that alarms is being set to an array of type AlarmState of each element
and ALARM_COUNT is the number of elements.
```rust
const ALARM_NEW: AlarmState = AlarmState::new();

impl AlarmState {                                                               
    const fn new() -> Self {                                                    
        Self {                                                                  
            timestamp: u64::MAX,                                                
            callback: ptr::null(),                                              
            ctx: ptr::null_mut(),                                               
        }                                                                       
    }                                                                           
} 
```
And if we turn our attention back to `TimeDriver::set_alarm_callback` and look
at the code following `init`:
```rust
    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        self.init();                                                            
        let mut alarms = unsafe { self.alarms.as_ref() }.lock().unwrap();       
        let alarm = &mut alarms[alarm.id() as usize];                           
        alarm.callback = callback as *const ();                                 
        alarm.ctx = ctx;                                                        
```
We can see that TimeDriver alarms member is a Mutex that guards the array
of AlarmState's, and we are indexing that array using the passed-in `id` that
was returned from the call to `allocate_alarm`. We are then setting the callback
of this alarm to the signal funtion and the argument for that function as the
context (ctx). To refresh my memory this is what the callback looks like,
followed by the argument:
```rust
impl Executor {                                                                 
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
Signaler's signal function looks like this and only takes a reference to self
as it is method:
```rust
    fn signal(&self) {                                                          
        let mut signaled = self.mutex.lock().unwrap();                          
        *signaled = true;                                                       
        self.condvar.notify_one();                                              
    } 
```
I was a little confused when looking at this but I think this is done because
when `signal` is called it will be called using something like this:
```rust
  let f: fn(*mut ()) = unsafe { mem::transmute(alarm.callback) };
  f(alarm.ctx);
```
In this the closure takes an argument which is the signaler.


_wip_




