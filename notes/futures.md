## Future
Is an async computation that can produce a value.

A Future is a value that implements the trait std::future::Future. Notice that
this says that a future is a value and the value it represents is an in-progress
async computation. A Future in Rust is the computation itself.
```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```

Lets take a look at what a future gets expanded into using the example
[future.rs](./tokio/src/future.rs):
```console
$ cd tokio
$ cargo rustc --bin future -- -Z unpretty=expanded,hygiene
```

Each usage of the async keyword generates a state machine from the code block.
And each .await in that code block will represent a state.
```rust
#[tokio::main]
async fn main() {
    println!("main: {:?}", thread::current().id());
    let future = Something{end: Instant::now() + Duration::from_millis(10)};
    let result = future.await;
    println!("result: {}", result);
}
```
So the first state would run all the code up to the line with
`let result = furure.await;`:
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

enum MainFuture {
    State0,
    State1(Something),
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        use MainFuture::*;
        loop {
            match *self {
                State0 => {
                    let future = Something{end: Instant::now() + Duration::from_millis(10)};
                    *self = State1(future);
                }
                State1(ref mut my_future) => {
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            println!("result: {}", result);
                            *self = Terminated;
                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    }
                }
                Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }
}
```
```console
$ cargo rustc --bin future -- --emit=llvm-ir
$ find target/ -name '*.ll'
target/debug/deps/future-07187fee0c7f789c.ll
```

Lets set a break point in our poll functions:
```console
(lldb) expr self
(core::pin::Pin<future::Something *>) $0 = {
  pointer = 0x00007fffffffc3d0
}
```
A newly created Future can be moved safely as it just contains a resumption
point and the argument values to the Future. The body has not yet begun
execution so nothing has had a chance to borrow these values yet. But when poll
is called that changes. Remember that we pass the Future to a function like
`block_on` or `spawn` which take the futures by value.

### Mio (Metal I/O)
It abstracts away the underlying systems select/poll implementations, and
enables I/O multiplexing. For some background around I/O models this
[link](https://github.com/danbev/learning-c#io-models) might be useful.

Poll is the abstraction of of select()/poll()/epoll()/kqueue()/IOCP() and
like those calls (at least for select()/poll()/epoll()) we register file
descriptors that we are interested in events (whether or note performing that
event would cause block or not). In Mio instead of a single function that
performs everything there are multply parts.

Poll is a struct that looks like this:
```rust
pub struct Poll {
    registry: Registry,
}
```
And a Registry is also a struct in the same file (poll.rs):
```rust
pub struct Registry {
    selector: sys::Selector,
}
```
So we we first need to create an instance of Poll:
```rust
    let mut poll = match Poll::new() {
        Ok(poll) => poll,
        Err(e) => panic!("failed to create Poll instance; err={:?}", e),
    };
```
Now, the docs for Poll::new says it will make a system call to create the
system selector.
```console
$ strace ./target/debug/mio-example 
...
write(1, "Metal IO (MIO) example\n", 23) = 23
epoll_create1(EPOLL_CLOEXEC)
```
It might help to take a look at a [standalone epoll example](https://github.com/danbev/learning-c/blob/master/epoll.c)
to get a better understanding of what this doing.
In `src/poll.rs` we find Poll::new():
```rust
	pub fn new() -> io::Result<Poll> {
            sys::Selector::new().map(|selector| Poll {
                registry: Registry { selector },
            })
        }
```
mio::sys::selector is private so we can't create one and explore it.
The type of selector being used is determined by the current operating system
being used, in my case this is unix/linux. The way this works is the
src/lib.rs includes the module `sys` and it has the following conditions in
its attributes:
```rust
#[cfg(unix)]
cfg_os_poll! {
    mod unix;
    pub use self::unix::*;
}
```
So the `Selector` in this case can be found in `src/sys/unix/selector/epoll.rs`:
```rust
#[derive(Debug)]
pub struct Selector {
    #[cfg(debug_assertions)]
    id: usize,
    ep: RawFd,
    #[cfg(debug_assertions)]
    has_waker: AtomicBool,
}
```
`RawFd` is std::os::unix::io::RawFd:
```
/// Raw file descriptors.
#[stable(feature = "rust1", since = "1.0.0")]
pub type RawFd = raw::c_int;

type c_int = i32;
```
So our selector has an id, a file descriptor (which is gets back from
epoll_create:
```c
int epoll_fd = epoll_create1(0);
```
So back to sys::Selector::new() we have the following system call:
```rust
        let flag = libc::EPOLL_CLOEXEC;

	syscall!(epoll_create1(flag)).map(|ep| Selector {
            #[cfg(debug_assertions)]
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            ep,
            #[cfg(debug_assertions)]
            has_waker: AtomicBool::new(false),
        })
```
So we are performing a system call using the syscall! macro. Then calling map
on the Result of that call, which recall will be a file descriptor of type
RawFd. This will be used to create a new Selector instance.
So this is using a syscall! macro that is defined in `src/sys/unix/mod.rs`:
```rust
#[allow(unused_macros)]
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}
```
The name of the macro is syscall (without the !). Then comes the body of the
macro after {. The body contains match expressions where `ident` is an
identifier like a function or a variable.
We can see that if the call is successful then we return Ok(res) where res
in our case should be a RawFd representing underlying file descriptor created.

So we will now return back from the syscall and have a populated
Result<Selector> which the function `map` is called on:
(in src/poll.rs):
```rust
pub fn new() -> io::Result<Poll> {
            sys::Selector::new().map(|selector| Poll {
                registry: Registry { selector },
            })
        }
```
And here Mio is mapping the Selector to a new Poll instance and in process it is
creating a new Registry with the selector that was just created.

__work in progess__
