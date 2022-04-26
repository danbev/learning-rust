//#![feature(type_name_of_val)]
//use std::any::type_name_of_val;

fn main() {
    let v = vec!["1", "2", "3"];
    let mut p = v.into_iter().peekable();
    println!("{:?}", p.peek());
    p.next();
    // peek() returns a shared reference, and the type we have stored in our
    // vector is &str, so we need to declare this as & for the shared reference
    // and then &str for the reference to the str. We don't have to specify
    // this at all but it was not obvious to me what the type should be if we
    // did want to write it out which is the reason for including it here.
    let i: Option<&&str> = p.peek();
    //println!("{:?}", type_name_of_val(&i));
    println!("{:?}", i);
    p.next();
    let i = p.peek();
    println!("{:?}", i);
    p.next();
    println!("{:?}", p.peek());
}
