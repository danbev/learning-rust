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

The last thing that `init` does is spawn a new thread but I though it made sense
to discuss the other parts first and hopefully what this does will make more
sense now:
```rust
  thread::spawn(Self::alarm_thread);                                  
```
So after init has run we will have two threads:
```console
(gdb) info thread
  Id   Target Id                                      Frame 
* 1    Thread 0x7ffff7d7c780 (LWP 280720) "no_macros" embassy::time::driver::set_alarm_callback (alarm=..., 
    callback=0x555555563860 <core::ops::function::FnOnce::call_once<embassy::executor::arch::{impl#0}::new::{closure#0}, (*mut ())>>, 
    ctx=0x5555555afa10) at /home/danielbevenius/.cargo/git/checkouts/embassy-9312dcb0ed774b29/da0c252/embassy/src/time/driver.rs:126
  2    Thread 0x7ffff7d7b640 (LWP 280852) "no_macros" __futex_abstimed_wait_common64 (private=0, cancel=true, abstime=0x7ffff7d7a788, op=137, 
    expected=0, futex_word=0x5555555afc68) at futex-internal.c:57
```

Lets see now how an executor is used. It is used by calling the `run` method:
```rust
pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {               
        init(self.inner.spawner());                                                
                                                                                   
        loop {                                                                     
            unsafe { self.inner.poll() };                                          
            self.signaler.wait()                                                   
        }                                                                          
    }                                      
```
Alright, so with the followwing sample code let try to understand what is
happening:
```rust
    let init = |spawner: Spawner| { 
      println!("spawer init...");
      spawner.spawn(make_task_storage());
    };
    executor.run(init);
```
So the first thing that `run` does is call the closer init that we passed in,
that the argument is taken from calling `spawner` of the Executors raw inner
Executor:
```rust
   pub fn spawner(&'static self) -> super::Spawner {                           
        super::Spawner::new(self)                                               
    }  

pub(crate) fn new(executor: &'static raw::Executor) -> Self {                  
        Self {                                                                     
            executor,                                                              
            not_send: PhantomData,                                                 
        }                                                                          
    }       
```
Spawner is declared as:
```rust
pub struct Spawner {                                                               
    executor: &'static raw::Executor,                                              
    not_send: PhantomData<*mut ()>,                                                
}                                                                                  
```
<a name="not_send"></a>
What is `not_send`?  
PhantomData is marker to so that the compiler see it as Spawner is storing
a raw mutable pointer to (). I'm not understanding this :( The field being
named `not_send` perhaps relates to preventing it from being Send? 
Ulf Lilleengen explained this to me and since the type is `*mut ()` is
automatically !Send so this prevents this a Spawner automatically implementing
Send which it would otherwise.

But he also pointed out that this might note be required, for example if we
comment out the PhantomData field and then pass the executor to another thread:
```rust
    let handler = thread::spawn(|| {
        println!("threadnn....");
        executor.run(init);
    });
```
The there is an error message like this:
```console
error[E0277]: `*mut ()` cannot be sent between threads safely
   --> src/no_macros.rs:32:19
    |
32  |     let handler = thread::spawn(|| {
    |                   ^^^^^^^^^^^^^ `*mut ()` cannot be sent between threads safely
    |
    = help: within `embassy::executor::Executor`, the trait `Send` is not implemented for `*mut ()`
    = note: required because it appears within the type `embassy::executor::raw::Executor`
    = note: required because it appears within the type `embassy::executor::Executor`
    = note: required because of the requirements on the impl of `Send` for `&mut embassy::executor::Executor`
    = note: 1 redundant requirement hidden
    = note: required because of the requirements on the impl of `Send` for `&mut &mut embassy::executor::Executor`
    = note: required because it appears within the type `[closure@src/no_macros.rs:32:33: 35:6]`
note: required by a bound in `spawn`
   --> /home/danielbevenius/.rustup/toolchains/nightly-2021-11-06-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:628:8
    |
628 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`

error[E0277]: `*mut TaskHeader` cannot be sent between threads safely
   --> src/no_macros.rs:32:19
    |
32  |     let handler = thread::spawn(|| {
    |                   ^^^^^^^^^^^^^ `*mut TaskHeader` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `*mut TaskHeader`
    = note: required because of the requirements on the impl of `Send` for `Cell<*mut TaskHeader>`
    = note: required because it appears within the type `embassy::executor::raw::timer_queue::TimerQueue`
    = note: required because it appears within the type `embassy::executor::raw::Executor`
    = note: required because it appears within the type `embassy::executor::Executor`
    = note: required because of the requirements on the impl of `Send` for `&mut embassy::executor::Executor`
    = note: 1 redundant requirement hidden
    = note: required because of the requirements on the impl of `Send` for `&mut &mut embassy::executor::Executor`
    = note: required because it appears within the type `[closure@src/no_macros.rs:32:33: 35:6]`
note: required by a bound in `spawn`
   --> /home/danielbevenius/.rustup/toolchains/nightly-2021-11-06-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:628:8
    |
628 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `embassy-exploration` due to 2 previous errors
```



Anyway, a new Spawner will be returned and this will be passed to our closure.
And we use the Spawner to spawn a task, which is done using a SpawnToken:
```rust
fn make_task_storage() -> SpawnToken<impl ::core::future::Future + 'static> {
    type F = impl ::core::future::Future + 'static;
    static new_task: TaskStorage<F> = TaskStorage::new();
    async fn task() {
        println!("new_task task...");
    }
    TaskStorage::spawn(&new_task, || task())
}
```

```rust
pub struct SpawnToken<F> {                                                      
    raw_task: Option<NonNull<raw::TaskHeader>>,                                 
    phantom: PhantomData<*mut F>,                                               
}
```
Notice in this case that the PhantomData is using the generic type which
was not the case with Spawner above. Also else where in the code base
PhantomData fields are named `_phantom` or just `phantom`. Why are these
different for Spawner and Executor?

So
  




_wip_




