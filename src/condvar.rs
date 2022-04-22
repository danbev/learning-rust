use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::LockResult;
use std::sync::MutexGuard;
use std::thread;

fn main() {
    println!("CondVar example");
    let mutex = Mutex::new(0);
    let condvar = Condvar::new();
    let p = Arc::new((mutex, condvar));
    let p2 = Arc::clone(&p);

    thread::spawn(move || {
        let (m, c) = &*p2;
        let lock_result: LockResult<MutexGuard<i32>> = m.lock();
        let guard = lock_result.unwrap();
        println!("Thread locked: {:p}", &guard);
        println!("Thread CondVar: {:p}", c);
        let mut val = *guard;
        val += 1;
        println!("Thread val: {}", val);
        println!("Thread notify_one...");
        c.notify_one();
    });

    let (m, c) = &*p;
    let lock_result: LockResult<MutexGuard<i32>> = m.lock();
    let guard = lock_result.unwrap();
    println!("main locked: {:p}", &guard);
    println!("main CondVar: {:p}", c);
    println!("main val: {}", *guard);
    // The following will block the current thread until a notification is
    // received using the same Condvar.
    let _ = c.wait(guard);
}
