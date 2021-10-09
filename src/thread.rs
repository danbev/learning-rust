use std::thread;

fn main() {
    println!("Thread example.");
    // || = closure
    let s1 = "Hello";
    let th = thread::spawn(move ||  { 
        s1
    });
    let s = th.join().unwrap();
    println!("{}", s);

}
