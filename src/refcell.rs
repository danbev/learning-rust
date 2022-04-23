use std::cell::RefCell;
use std::rc::Rc;

struct Something {
    name: String,
    id: RefCell<String>
}

fn main() {
    // The rules for a Box are enforced at compiler time
    let b = Box::new(3);
    // We can have any number of no-mutable references
    let _rb = &b;
    let _rb2 = &b;
    let _rb3 = &b;
    let mut rb4 = b;
    *rb4 = 4;
    // println!("{}", rb3); compiler error.
    //
    let r = RefCell::new(3);
    let _r2 = r.borrow();
    //let _r3 = r.borrow_mut(); // will panic at runtime.

    let s = Rc::new(Something{name: "Fletch".to_string(), id: RefCell::new("1".to_string())});
    let mut id = s.id.borrow_mut();
    println!("s.name: {}, s.id: {:?}", s.name, id);
    id.push_str("12");

    println!("s.name: {}, s.id: {:?}", s.name, id);
}
