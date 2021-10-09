trait Dr {
    type Target: ?Sized;
    fn deref(self: &Self) -> &Self::Target;
}

#[derive(Debug)]
struct S {
    nr: i32,
}

trait Print {
    fn print(self: &Self);
}

impl Dr for S {
    type Target = S;
    fn deref(self: &Self) -> &Self {
        &self
    }
}

impl Print for S {
    fn print(&self) {
        println!("Printing something");
    }
}

fn main() {
    println!("Deref example");
    let s = S { nr: 18 };
    println!("s: {:?}", s);
    // Now lets use the deref operator
    println!("s: {:?}", *(&s));

    // Show that we can use the dot operator to call a function on s
    let s2 = &s;
    s2.print();
    // This is like syntactic suger for the following:
    (*s2).print();

    let x = 18;
    // so we can get a ref to any value, this is just getting the address
    // to the data which could be on the stack or on the heap.
    // And with a reference we can dereference. Or does the i32 type
    // implement Deref perhaps.
    println!("x: {:?}", *(&x));
}
