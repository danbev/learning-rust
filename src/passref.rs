#[derive(Debug)]
struct Something {
    x: i32,
}

fn process_obj(mut o: Something) {
    o.x = 11;
    println!("process_obj. Got a copy of Something: {:?} : {:p}", o, &o);
}

fn process_obj_ref(o: &Something) {
    println!("process_obj_ref. Got a ref of Something: {:?} : {:p}", o, &o);
}

fn process_mut(o: &mut Something) {
    o.x = 12;
    println!("process_mut_obj. Got a mut ref of Something: {:?} : {:p}", o, &o);
}

fn main() {
    println!("Pass references example");

    let s = Something{x: 1};  // Something is stack allocated
    println!("s: {:?} {:p}", s, &s);   
    process_obj_ref(&s);
    process_obj(s);           // the value on the stack is copied to a local variable in process_obj
                              // any changes will not affect the value on this frames stack.
                              
    let mut s2 = Something{x: 1};  // Again stack allocated
    println!("s2: {:?} {:p}", s2, &s2);   
    process_mut(&mut s2);          // Now, we are passing the address on mains stack to the function.

    /*
    let x: i32 = 18;
    let x_ref: &i32 = &x;
    println!("x:     {:p}", &x);
    println!("x_ref: {:p}", x_ref);
    process_ref(x_ref);
    println!("x: {}", x);
    println!("x_ref: {}", x_ref);
    */
}

fn process_ref(r: &i32) {
    println!("r:     {:p}", r);
}
