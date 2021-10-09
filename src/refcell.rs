use std::cell::RefCell;

fn main() {
    // The rules for a Box are enforced at compiler time
    let b = Box::new(3);
    // We can have any number of no-mutable references
    let rb = &b;
    let rb2 = &b;
    let rb3 = &b;
    let mut rb4 = b;
    *rb4 = 4;
    // println!("{}", rb3); compiler error.
    //
    let r = RefCell::new(3);
    let r2 = r.borrow();
    let r3 = r.borrow_mut(); // will panic at runtime.
}
