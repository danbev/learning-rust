use std::{
    future::Future, sync::{ mpsc::{channel, Sender}, Arc, Mutex, Condvar},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker}, mem, pin::Pin,
    thread::{self, JoinHandle}, time::{Duration, Instant}, collections::HashMap
};

#[derive(Clone)]
pub struct Task {
    id: usize,
    reactor: Arc<Mutex<Box<Reactor>>>,
    data: u64,
}

impl Task {
    fn new(id: usize, reactor: Arc<Mutex<Box<Reactor>>>, data: u64) -> Self {
        Task { id, reactor, data }
    }
}

fn main() {
    let start = Instant::now();
    let reactor = Reactor::new();

    let fut1 = async {
        let id = 100;
        let duration = 1;
        let val = Task::new(id, reactor.clone(), duration).await;
        println!("Got id '{}' at time: {:.2}.", val, start.elapsed().as_secs_f32());
        println!();
    };

    let fut2 = async {
        let id = 200;
        let duration = 2;
        let val = Task::new(id, reactor.clone(), duration).await;
        println!("Got id '{}' at time: {:.2}.", val, start.elapsed().as_secs_f32());
    };

    let mainfut = async {
        fut1.await;
        fut2.await;
    };

    block_on(mainfut);
}

#[derive(Default)]
struct Parker(Mutex<bool>, Condvar);

impl Parker {
    fn park(&self) {
        let mut resumable = self.0.lock().unwrap();
            while !*resumable {
                resumable = self.1.wait(resumable).unwrap();
            }
        *resumable = false;
    }

    fn unpark(&self) {
        *self.0.lock().unwrap() = true;
        self.1.notify_one();
    }
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    let parker = Arc::new(Parker::default());
    let mywaker = Arc::new(MyWaker { parker: parker.clone() });
    let waker = mywaker_into_waker(Arc::into_raw(mywaker));
    let mut cx = Context::from_waker(&waker);

    // SAFETY: we shadow `future` so it can't be accessed again.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };
    loop {
        match Future::poll(future.as_mut(), &mut cx) {
            Poll::Ready(val) => break val,
            Poll::Pending => parker.park(),
        };
    }
}

#[derive(Clone)]
struct MyWaker {
    parker: Arc<Parker>,
}

fn mywaker_wake(waker: &MyWaker) {
    let waker_arc = unsafe { Arc::from_raw(waker) };
    waker_arc.parker.unpark();
}

fn mywaker_clone(waker: &MyWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(waker) };
    std::mem::forget(arc.clone()); // increase ref count
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |w| mywaker_clone(&*(w as *const MyWaker)),   // clone
        |w| mywaker_wake(&*(w as *const MyWaker)),    // wake
        |w| (*(w as *const MyWaker)).parker.unpark(), // wake by ref
        |w| drop(Arc::from_raw(w as *const MyWaker)), // decrease refcount
    )
};

fn mywaker_into_waker(waker: *const MyWaker) -> Waker {
    // Note that we are creating a new RawWaker by casting our waker pointer
    // void pointer, and using the VTABLE see above.
    let raw_waker = RawWaker::new(waker as *const (), &VTABLE);

    unsafe { Waker::from_raw(raw_waker) }
}


impl Future for Task {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("future:poll for id: {}", self.id);
        let mut r = self.reactor.lock().unwrap();
        if r.is_ready(self.id) {
            println!("future: is_ready for id: {}", self.id);
            *r.tasks.get_mut(&self.id).unwrap() = TaskState::Finished;
            Poll::Ready(self.id)
        } else if r.tasks.contains_key(&self.id) {
            println!("future: pending");
            r.tasks.insert(self.id, TaskState::NotReady(cx.waker().clone()));
            Poll::Pending
        } else {
            println!("future: register id: {}", self.id);
            r.register(self.data, cx.waker().clone(), self.id);
            Poll::Pending
        }
    }
}

enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}

struct Reactor {
    dispatcher: Sender<Event>,
    handle: Option<JoinHandle<()>>,
    tasks: HashMap<usize, TaskState>,
}

#[derive(Debug)]
enum Event {
    Close,
    Timeout(u64, usize),
}

impl Reactor {

    fn new() -> Arc<Mutex<Box<Self>>> {
        let (tx, rx) = channel::<Event>();
        let reactor = Arc::new(Mutex::new(Box::new(Reactor {
            dispatcher: tx,
            handle: None,
            tasks: HashMap::<usize, TaskState>::new(),
        })));

        let reactor_clone = Arc::downgrade(&reactor);
        let handle = thread::spawn(move || {
            println!("[Event thread spawned]");

            // The following will block waiting for any receiving events sent
            // to the channel.
            let mut handles = vec![];
            for event in rx {
                println!("handle event: {:?}", event);
                let reactor = reactor_clone.clone();
                match event {
                    Event::Close => break,
                    Event::Timeout(duration, id) => {
                        let event_handle = thread::spawn(move || {
                            println!("[Timer thread spawned for id {}]", id);
                            thread::sleep(Duration::from_secs(duration));
                            let reactor = reactor.upgrade().unwrap();
                            // So we lock the reactor, then call map on the
                            // Result which processes the Ok value. This also
                            // returns a Result which we unwap.
                            reactor.lock().map(|mut r| r.wake(id)).unwrap();
                        });
                        handles.push(event_handle);
                    }
                }
            }

            handles.into_iter().for_each( |handle| {
                handle.join().unwrap();
                println!("[Event thread joined]");
            });

        });

        reactor.lock().map(|mut r| r.handle = Some(handle)).unwrap();
        reactor
    }

    fn wake(&mut self, id: usize) {
        let state = self.tasks.get_mut(&id).unwrap();
        // mem::replace(dest, src) and returns the previous dest value.
        // So we update the state and act on the previous state.
        match mem::replace(state, TaskState::Ready) {
            TaskState::NotReady(waker) => waker.wake(),
            TaskState::Finished => panic!("Called 'wake' twice on task: {}", id),
            _ => unreachable!()
        }
    }

    fn register(&mut self, duration: u64, waker: Waker, id: usize) {
        // Notice that the waker is set here.
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("Tried to insert a task with id: '{}', twice!", id);
        }
        // Use Sender::send to an event to the channel. This will end up in
        // the Reactors thread which 
        self.dispatcher.send(Event::Timeout(duration, id)).unwrap();
    }

    fn is_ready(&self, id: usize) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false) // there might not be a match so then return false
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        self.dispatcher.send(Event::Close).unwrap();
        self.handle.take().map(|h| h.join().unwrap()).unwrap();
    }
}
