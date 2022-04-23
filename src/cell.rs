use std::rc::Rc;
use std::cell::Cell;

struct Something {
    name: String,
    id: Cell<usize>
}

fn main() {
    println!("Rust Cell example.");
    let s = Rc::new(Something{name: "Fletch".to_string(), id: Cell::new(1)});
    println!("s.name: {}, s.id: {:?}", s.name, s.id);
    // Not possible to modify one field of a Rc object unless we use something
    // like Cell.
    //s.id = 2; // will not compile "cannot assign"
    s.id.set(2);
    println!("s.name: {}, s.id: {:?}", s.name, s.id);
}
