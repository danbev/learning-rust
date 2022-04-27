use std::sync::Mutex;

fn main() {
    println!("Mutex example");
    let m = Mutex::new("bajja");
    println!("{:?}", m);
    println!("is_poisoned: {}", m.is_poisoned());
    let l = m.lock();
    let s = l.unwrap();
    println!("{:?}", s);

    let would_block_err = m.try_lock(); //.unwrap();
    println!("{:?}", would_block_err);
    // The following line will deadlock the current thread as it already holds
    // the lock.
    //let _s2 = m.lock().unwrap();
}
