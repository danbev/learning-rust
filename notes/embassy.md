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
Condvar is part of `std::sync` crate and represents the ability to block a
thread such that it consumes no CPU time while waiting for an event to occur.

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
parameter, and in this case a closure is passed in.

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
.

The constructor for `raw::Executor` looks like this
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
If we look in `embassy/src/time/mod.rs` we can find that depending on the
feature, in our case `std` then the driver_std module:
```rust
#[cfg(feature = "std")]
mod driver_std;
```
Notice that `allocate_alarm` is in driver.rs, and that it is calling
`_embassy_time_allocate_alarm` which is generated by a macro. To understand
how this works we need to look in `embassy/src/time/driver_std.rs` and the usage
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
struct TimeDriver {
    alarm_count: AtomicU8,

    once: Once,
    alarms: UninitCell<Mutex<[AlarmState; ALARM_COUNT]>>,
    zero_instant: UninitCell<StdInstant>,
    signaler: UninitCell<Signaler>,
}

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
is what it is used for in this case. So this will only be done once and it will
configure this TimeDriver setting.
Notice that the `alarms` field is UninitCell which is wrapping a Mutex that
guards an array. The Array will be of size `ALARM_COUNT` and each element will
be an ALARM_NEW copy I think. I think that would make sense as each timer will
have it's own values and also separate callbacks.
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
to discuss the other parts first and hopefully what this will make more sense
now:
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

After `driver::set_alarm_callback` has returned Executor::new will create and
return an new instance of itself:
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
RunQueue look like this:
```rust
pub(crate) struct RunQueue {
    head: AtomicPtr<TaskHeader>,
}
```
We can see that it has an atomic pointer that can be shared by threads, which
points to a TaskHeader.
```rust
pub struct TaskHeader {
    pub(crate) state: AtomicU32,
    pub(crate) run_queue_item: RunQueueItem,
    pub(crate) executor: Cell<*const Executor>, // Valid if state != 0
    pub(crate) poll_fn: UninitCell<unsafe fn(NonNull<TaskHeader>)>, // Valid if STATE_SPAWNED

    #[cfg(feature = "time")]
    pub(crate) expires_at: Cell<Instant>,
    #[cfg(feature = "time")]
    pub(crate) timer_queue_item: timer_queue::TimerQueueItem,
}

pub(crate) struct RunQueueItem {
    next: AtomicPtr<TaskHeader>,
}
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
So the first thing that `run` does is call the closer `init` that we passed in,
the argument passed to `init` is taken from calling `spawner` of the Executors
raw inner Executor:
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
From [send-and-sync](https://doc.rust-lang.org/nomicon/send-and-sync.html) we
can read that Send and Sync are automatically derived Traits. So if a type is
made up on only Send or Sync types then the type is Send or Sync.
Send means that it the type is safe to send to another thread. Sync means that
the type is safe to share between threads.
Raw pointers are not Send or Sync (they have no safety guards). UnsafeCell
is not Sync.

So a Spawner instance really just has one member of interest and that is:
a reference to the Executor:
```rust
    executor: &'static raw::Executor,
```
And recall that Executor::run will call:
```rust
        init(self.inner.spawner());
```
And `self.inner.spawner()` will call the raw Executor::spawner` function:
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
And Spawner is declared as:
```rust
pub struct Spawner {
    executor: &'static raw::Executor,
    not_send: PhantomData<*mut ()>,
}

So, a new Spawner will be returned and it will be passed to our closure.
And we use the Spawner to `spawn` a task, which is done using a SpawnToken:
```rust
        spawner.spawn(make_task_storage()).unwrap();
```
And `make_task_storage` will create a new TaskStorate instance:
```rust
    type F = impl ::core::future::Future + 'static;
    static NEW_TASK: TaskStorage<F> = TaskStorage::new();
    async fn task() {
        println!("task...");
    }
    let token: SpawnToken<F> = TaskStorage::spawn(&NEW_TASK, || task());
    token
```
A TaskStorage contains a TaskHeader and a future.
```rust
pub const fn new() -> Self {
        Self {
            raw: TaskHeader::new(),
            future: UninitCell::uninit(),
        }
}

pub fn spawn(&'static self, future: impl FnOnce() -> F) -> SpawnToken<F> {
        if self.spawn_allocate() {
            unsafe { self.spawn_initialize(future) }
        } else {
            SpawnToken::new_failed()
        }
}
```
Notice that `TaskStorage::spawn` take a ref to a self with a static lifetime,
and something that implements FnOnce and returns a F which is of type
`F: Future + 'static'. And this function returns a SpawnToken which is generic
over F. The first thing that happend is that `spawn_allocate` is called which
sets the state of the TaskHeader field. I think I might be missing something
but I'm a little confused about the nameing of this method. Something like
set_spawn_state might be better but I'm not sure (naming is hard). So after
the state has been updated and if it was successful, `spawn_initialized` will
be called:
```rust
unsafe fn spawn_initialize(&'static self, future: impl FnOnce() -> F) -> SpawnToken<F> {
        // Initialize the task
        self.raw.poll_fn.write(Self::poll);
        self.future.write(future());

        SpawnToken::new(NonNull::new_unchecked(&self.raw as *const TaskHeader as _))
}
```
Where we are setting the TaskHeader member's `poll_fn` to Self::poll. and
setting the feature member to our `async task()` function, which is called
and the return value of that is a Future as it is an async function.
After that a new SpawnToken is created passing in the a TaskHeader pointer as a
NonNull. And the SpawnToken is returned to spawn, and then returned to
`make_task_storage` which also returned it and it will then be passed to
spawner.spawn:
```rust
        spawner.spawn(make_task_storage()).unwrap();
```

```rust
pub fn spawn<F>(&self, token: SpawnToken<F>) -> Result<(), SpawnError> {
          let task = token.raw_task;
          mem::forget(token);

          match task {
              Some(task) => {
                  unsafe { self.executor.spawn(task) };
                  Ok(())
              }
              None => Err(SpawnError::Busy),
          }
}
```
Recall that self in this case is Spawner and that it has two member but one
is a PhantomData instance so we can disregard it for now (more about this field
can be found later in this document). That leaves the `executor` field which
we are calling its `spawn` function:
```rust
pub(super) unsafe fn spawn(&'static self, task: NonNull<TaskHeader>) {
        let task = task.as_ref();
        task.executor.set(self);

        critical_section::with(|cs| {
            self.enqueue(cs, task as *const _ as _);
        })
}
```
So this is calling `run_queue.enque`, in src/executor/raw/run_queue.rs, passing
in the critical section and the pointer to the TaskHeader:
```rust
#[inline(always)]
pub(crate) unsafe fn enqueue(&self, _cs: CriticalSection, task: *mut TaskHeader) -> bool {
  let prev = self.head.load(Ordering::Relaxed);
  (*task).run_queue_item.next.store(prev, Ordering::Relaxed);
  self.head.store(task, Ordering::Relaxed);
  prev.is_null()
}
```
RunQueue has an AtomicPtr to a TaskHeader as its only member which is first
retrieved. We are then dereferencing the TaskHeader and setting
`run_queue_item_item.next` to the TaskHeader, adding this to the linked list:
```
  +----------------+
  | Executor       |
  |----------------+
  | +------------+ |
  | | run_queue  | |
  | |------------+ |          +-------------------+
  | | head       |-------+--->| TaskHeader (prev) |
  | +------------+ |     |    | +--------------+  |
  | +  ...       + |     |    | |run_queue_item|  |
  +----------------+     |    | |--------------|  |
                         |    | | next         |-----> NULL
                         |    | +--------------+  |
                         |    | +   ...        |  |
                         |    | +--------------+  |
                         |    +-------------------+
                         |
  +-------------------+  |
  | TaskHeader (task) |  |
  | +--------------+  |  |
  | |run_queue_item|  |  ^
  | |--------------|  |  |
  | | next         |-----+  ((*task).run_queue_item.next.store(prev, Ordering::Relaxed);)
  | +--------------+  |
  | +   ...        |  |
  | +--------------+  |
  +-------------------+


  +----------------+
  | Executor       |
  |----------------+
  | +------------+ |  (self.head.store(task, Ordering::Relaxed);)
  | | run_queue  | |
  | |------------+ |    +-------------------+      +-------------------+
  | | head       |----->| TaskHeader (task) |  +-->| TaskHeader (prev) |
  | +------------+ |    | +--------------+  |  |   | +--------------+  |
  | +  ...       + |    | |run_queue_item|  |  |   | |run_queue_item|  |
  +----------------+    | |--------------|  |  |   | |--------------|  |
                        | | next         |-----+   | | next         |-----> NULL
                        | +--------------+  |      | +--------------+  |
                        | +   ...        |  |      | +   ...        |  |
                        | +--------------+  |      | +--------------+  |
                        +-------------------+      +-------------------+
```
And notice that enqueue will return a boolean value which will be true if
prev now points to null (the end of the linked list).



When we call Executor::run this is done on the Executor of type std in our case
so we have to look in src/executor/arch/std.rs:
```rust
pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {
  init(self.inner.spawner());

  loop {
      unsafe { self.inner.poll() };
      self.signaler.wait()
  }
}
Notice that this method never returns and we are calling the `raw::Executors`
poll method:
```rust
pub unsafe fn poll(&'static self) {
        #[cfg(feature = "time")]
        self.timer_queue.dequeue_expired(Instant::now(), |p| {
            p.as_ref().enqueue();
        });

        self.run_queue.dequeue_all(|p| {
            let task = p.as_ref();
```
Lets start with the call `dequeue_expired` and notice it takes two arguments,
Instant, and a closure. The name tells us that is should probably "remove" all
elements from the timer_queue that have expired. The closure passed into this
function as the `on_task` parameter and specified what should be done with those
tasks that have expired.
`timer_queue` can be found in src/executor/raw/timer_queue.rs:
```rust
pub(crate) unsafe fn dequeue_expired(&self, now: Instant,
  on_task: impl Fn(NonNull<TaskHeader>)) {
    self.retain(|p| {
      let task = p.as_ref();
      if task.expires_at.get() <= now {
        on_task(p);
        false
       } else {
         true
       }
    });
}
```
So this will call `retain` passing in a closure as seen above.


Recall that TimerQueue only has one member fields which is named `head` and is
of type Cell<*mut TaskHeader>. Notice that the type of the contents of this cell
is `*mut TaskHeader` so it is a pointer which is why we this code is checking
that is is not null:
```rust
pub(crate) unsafe fn retain(&self, mut f: impl FnMut(NonNull<TaskHeader>) -> bool) {
  let mut prev = &self.head;
  while !prev.get().is_null() {
    let p = NonNull::new_unchecked(prev.get());
    let task = &*p.as_ptr();
    if f(p) {
	// Skip to next
	prev = &task.timer_queue_item.next;
    } else {
	// Remove it
	prev.set(task.timer_queue_item.next.get());
	task.state.fetch_and(!STATE_TIMER_QUEUED, Ordering::AcqRel);
    }
  }
}
```
So here the initially `prev` will be `self.head` and if it is not null. If it
is not null then the pointer to that task will be passed to the closure passed
in which is:
```rust
    |p| {
      let task = p.as_ref();
      if task.expires_at.get() <= now {
        on_task(p);
        false
      } else {
        true
      }
    }
```
Notice that the above will check if the task has expired, and if so call
on_task, which if of type `impl Fn(NonNull<TaskHeader>` and return false:
```rust
  |p| {
    p.as_ref().enqueue();
  }
```
So p.as_ref() will give a reference to a TaskHeader and it's `enqueue` function
will be called (in src/executor/raw/mod.rs):
```rust
pub(crate) unsafe fn enqueue(&self) {
        critical_section::with(|cs| {
            let state = self.state.load(Ordering::Relaxed);

            // If already scheduled, or if not started,
            if (state & STATE_RUN_QUEUED != 0) || (state & STATE_SPAWNED == 0) {
                return;
            }

            // Mark it as scheduled
            self.state
                .store(state | STATE_RUN_QUEUED, Ordering::Relaxed);

            // We have just marked the task as scheduled, so enqueue it.
            let executor = &*self.executor.get();
            executor.enqueue(cs, self as *const TaskHeader as *mut TaskHeader);
        })
}
```
[critical_section](https://docs.rs/critical-section/latest/critical_section).
First the state is loaded atomitcally. The state is checked and nothing is done
if this task has already been queued, or if it already has a future. Otherwise
the state is set to `STATE_RUN_QUEUED` and the task is enqueued with the
executor:
```rust
    unsafe fn enqueue(&self, cs: CriticalSection, task: *mut TaskHeader) {
        if self.run_queue.enqueue(cs, task) {
            (self.signal_fn)(self.signal_ctx)
        }
    }
```
So this is calling `run_queue.enque`, in src/executor/raw/run_queue.rs, passing
in the critical section and the pointer to the TaskHeader:
```rust
#[inline(always)]
pub(crate) unsafe fn enqueue(&self, _cs: CriticalSection, task: *mut TaskHeader) -> bool {
  let prev = self.head.load(Ordering::Relaxed);
  (*task).run_queue_item.next.store(prev, Ordering::Relaxed);
  self.head.store(task, Ordering::Relaxed);
  prev.is_null()
}
```
RunQueue has an AtomicPtr to a TaskHeader as its only member which is first
retrieved. We are then dereferencing the TaskHeader and setting
`run_queue_item_item.next` to the TaskHeader, adding this to the linked list:
```
  +----------------+
  | Executor       |
  |----------------+
  | +------------+ |
  | | run_queue  | |
  | |------------+ |          +-------------------+
  | | head       |-------+--->| TaskHeader (prev) |
  | +------------+ |     |    | +--------------+  |
  | +  ...       + |     |    | |run_queue_item|  |
  +----------------+     |    | |--------------|  |
                         |    | | next         |-----> NULL
                         |    | +--------------+  |
                         |    | +   ...        |  |
                         |    | +--------------+  |
                         |    +-------------------+
                         |
  +-------------------+  |
  | TaskHeader (task) |  |
  | +--------------+  |  |
  | |run_queue_item|  |  ^
  | |--------------|  |  |
  | | next         |-----+  ((*task).run_queue_item.next.store(prev, Ordering::Relaxed);)
  | +--------------+  |
  | +   ...        |  |
  | +--------------+  |
  +-------------------+


  +----------------+
  | Executor       |
  |----------------+
  | +------------+ |  (self.head.store(task, Ordering::Relaxed);)
  | | run_queue  | |
  | |------------+ |    +-------------------+      +-------------------+
  | | head       |----->| TaskHeader (task) |  +-->| TaskHeader (prev) |
  | +------------+ |    | +--------------+  |  |   | +--------------+  |
  | +  ...       + |    | |run_queue_item|  |  |   | |run_queue_item|  |
  +----------------+    | |--------------|  |  |   | |--------------|  |
                        | | next         |-----+   | | next         |-----> NULL
                        | +--------------+  |      | +--------------+  |
                        | +   ...        |  |      | +   ...        |  |
                        | +--------------+  |      | +--------------+  |
                        +-------------------+      +-------------------+
```
And notice that enqueue will return a boolean value which will be true if
prev now points to null (the end of the linked list). This value will be
passed back and if we will call the signal_fn.
```rust
        if self.run_queue.enqueue(cs, task) {
            (self.signal_fn)(self.signal_ctx)
        }
```
So that was for the timers, which the current case there are non. So this will
return us to `raw::Executor::poll`:
pub unsafe fn poll(&'static self) {
  ...
  self.run_queue.dequeue_all(|p| {
            let task = p.as_ref();

            #[cfg(feature = "time")]
            task.expires_at.set(Instant::MAX);

            let state = task.state.fetch_and(!STATE_RUN_QUEUED, Ordering::AcqRel);
            if state & STATE_SPAWNED == 0 {
                // If task is not running, ignore it. This can happen in the following scenario:
                //   - Task gets dequeued, poll starts
                //   - While task is being polled, it gets woken. It gets placed in the queue.
                //   - Task poll finishes, returning done=true
                //   - RUNNING bit is cleared, but the task is already in the queue.
                return;
            }

            // Run the task
            task.poll_fn.read()(p as _);

            // Enqueue or update into timer_queue
            #[cfg(feature = "time")]
            self.timer_queue.update(p);
        });
        ...
}
```
So this will call `run_queue_dequeue_all`
(again in src/executor/raw/run_queue.rs):
```rust
pub(crate) fn dequeue_all(&self, on_task: impl Fn(NonNull<TaskHeader>)) {
  // Atomically empty the queue.
  let mut ptr = self.head.swap(ptr::null_mut(), Ordering::AcqRel);

   // Iterate the linked list of tasks that were previously in the queue.
   while let Some(task) = NonNull::new(ptr) {
     // If the task re-enqueues itself, the `next` pointer will get overwritten.
     // Therefore, first read the next pointer, and only then process the task.
     let next = unsafe { task.as_ref() }
       .run_queue_item
       .next
       .load(Ordering::Relaxed);
      on_task(task);

    ptr = next
  }
}
```
So we take the current head and swap that for null. Like before `on_task` is
the passed in closure.
```rust
            task.poll_fn.read()(p as _);

```
The `poll_fn` will be `TaskStorage::poll`:
```rust
 unsafe fn poll(p: NonNull<TaskHeader>) {
        let this = &*(p.as_ptr() as *const TaskStorage<F>);

        let future = Pin::new_unchecked(this.future.as_mut());
        let waker = waker::from_task(p);
        let mut cx = Context::from_waker(&waker);
        match future.poll(&mut cx) {
            Poll::Ready(_) => {
                this.future.drop_in_place();
                this.raw.state.fetch_and(!STATE_SPAWNED, Ordering::AcqRel);
            }
            Poll::Pending => {}
        }

        // the compiler is emitting a virtual call for waker drop, but we know
        // it's a noop for our waker.
        mem::forget(waker);
    }
```

```console
(gdb) p cx
$19 = core::task::wake::Context {waker: 0x7fffffffcf10, _marker: core::marker::PhantomData<fn(&()) -> &()>}

(gdb) p task.poll_fn
$12 = embassy::executor::raw::util::UninitCell<unsafe fn(core::ptr::non_null::NonNull<embassy::executor::raw::TaskHeader>)>
(core::mem::maybe_uninit::MaybeUninit<core::cell::UnsafeCell<unsafe fn(core::ptr::non_null::NonNull<embassy::executor::raw::TaskHeader>)>> {
uninit: (),
value: core::mem::manually_drop::ManuallyDrop<core::cell::UnsafeCell<unsafe fn(core::ptr::non_null::NonNull<embassy::executor::raw::TaskHeader>)>> {
value: core::cell::UnsafeCell<unsafe fn(core::ptr::non_null::NonNull<embassy::executor::raw::TaskHeader>)> {
value: 0x55555555f6a0 <embassy::executor::raw::TaskStorage<core::future::from_generator::GenFuture<no_macros::make_task_storage::task::{generator#0}>>::poll<core::future::from_generator::GenFuture<no_macros::make_task_storage::task::{generator#0}>>>}}})
```

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
_wip_


### TaskStorage
This type contains only two fields:
```rust
pub struct TaskStorage<F: Future + 'static> {
    raw: TaskHeader,
    future: UninitCell<F>, // Valid if STATE_SPAWNED
}
```
Creating a new TaskStorage is done with the new constructor function:
```rust
pub const fn new() -> Self {
        Self {
            raw: TaskHeader::new(),
            future: UninitCell::uninit(),
        }
}
```
For example:
```rust
    type F = impl ::core::future::Future + 'static;
    static NEW_TASK: TaskStorage<F> = TaskStorage::new();
```
We can take this newly create TaskStorage and pass it to TaskStorage spawn
```rust
    let token: SpawnToken<F> = TaskStorage::spawn(&NEW_TASK, || async_function());
```
This will first set the state of the TaskHeader to 3
(STATE_SPAWED | STATE_RUN_QUEUED), and if that succeeds spawn will call
`spawn_initialized(future)`:
```rust
    unsafe fn spawn_initialize(&'static self, future: impl FnOnce() -> F) -> SpawnToken<F> {
        // Initialize the task
        self.raw.poll_fn.write(Self::poll);
        self.future.write(future());

        SpawnToken::new(NonNull::new_unchecked(&self.raw as *const TaskHeader as _))
    }
```
And here we can see that it is setting the TaskHeader's poll_fn to the function
Self::poll, and setting this TaskStorage's future field to be the future
returned from calling the passed in `async_function`.
Now, notice that here a SpawnToken is being returned which is created by
passing in a pointer to self.raw which is the first field in TaskStorage. At
first it might seem like we have lost the future which was set here as the
only thing returned was the SpawnToken and it only has a optional TaskHeader
field:
```rust
  pub struct SpawnToken<F> {                                                         
      raw_task: Option<NonNull<raw::TaskHeader>>,                                    
      phantom: PhantomData<*mut F>,                                                  
  }
```
SpawnToken also has TaskHeader as the first field. We are just 
```console
(gdb) p &self.raw
$11 = (*mut embassy::executor::raw::TaskHeader)
0x5555555bb0c0 <no_macros::main::{{closure}}::NEW_TASK>

(gdb) p &self.future
$12 = (*mut embassy::executor::raw::util::UninitCell<core::future::from_generator::GenFuture<no_macros::main::{closure#0}::task::{generator#0}>>)
0x5555555bb0f0 <no_macros::main::{{closure}}::NEW_TASK+48>
```
Now, with only using the token we can do this:
```console
(gdb) p task
$15 = core::option::Option<core::ptr::non_null::NonNull<embassy::executor::raw::TaskHeader>>::Some(core::ptr::non_null::NonNull<embassy::executor::raw::TaskHeader> {pointer: 0x5555555bb0c0 <no_macros::main::{{closure}}::NEW_TASK>})
```
This allows us to cast a SpawnToken into a TaskStorage and hence be able to
get access to the Future. This can be seen in `poll`:
```rust
unsafe fn poll(p: NonNull<TaskHeader>) {
        let this = &*(p.as_ptr() as *const TaskStorage<F>);

        let future = Pin::new_unchecked(this.future.as_mut());
        let waker = waker::from_task(p);
        let mut cx = Context::from_waker(&waker);
        match future.poll(&mut cx) {
            Poll::Ready(_) => {
                this.future.drop_in_place();
                this.raw.state.fetch_and(!STATE_SPAWNED, Ordering::AcqRel);
            }
            Poll::Pending => {}
        }

        // the compiler is emitting a virtual call for waker drop, but we know
        // it's a noop for our waker.
        mem::forget(waker);
    }                  
```

```console
(gdb) br embassy::executor::raw::TaskStorage<F>::poll
```

### TaskHeader
```rust
pub struct TaskHeader {
    pub(crate) state: AtomicU32,
    pub(crate) run_queue_item: RunQueueItem,
    pub(crate) executor: Cell<*const Executor>, // Valid if state != 0
    pub(crate) poll_fn: UninitCell<unsafe fn(NonNull<TaskHeader>)>, // Valid if STATE_SPAWNED

    #[cfg(feature = "time")]
    pub(crate) expires_at: Cell<Instant>,
    #[cfg(feature = "time")]
    pub(crate) timer_queue_item: timer_queue::TimerQueueItem,
}
```
Createing a new TaskHeader can be done using the `new` constructor function but
note that this is only public in the crate so it cannot be called from code
outside of the crate.
```rust
pub(crate) const fn new() -> Self {
        Self {
            state: AtomicU32::new(0),
            run_queue_item: RunQueueItem::new(),
            executor: Cell::new(ptr::null()),
            poll_fn: UninitCell::uninit(),

            #[cfg(feature = "time")]
            expires_at: Cell::new(Instant::from_ticks(0)),
            #[cfg(feature = "time")]
            timer_queue_item: timer_queue::TimerQueueItem::new(),
        }
    }
```
RunQueueItem only has one member which is `next: Cell<*mut TaskHeader>` and
note that this is part of the struct and not a pointer and upon creation it will
be a Cell ptr::null_mut().


