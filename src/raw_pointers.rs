
fn something(f: fn(*mut ())) {
    let m: *mut () = &mut ();
    f(m);
}

fn print(n: *mut ()) {
    println!("print...{:?}", n);
}

fn print2(n: *mut i32) {
    println!("print2...{:?}", n);
}

fn main() {
    println!("Raw pointers example");
    //print_type_of(&print_type_of::<()>);
    println!("{}", std::any::type_name::<()>());
    //println!("{}", std::any::type_name::<T>())

    let mut x = 18;
    //let raw: *const i32 = &x as *const i32;
    let raw: *const i32 = &x;
    unsafe {
        println!("raw: {}", *raw);
    }

    let mut r = &mut x as *mut i32;
    
    let y:i32 = 20;
    print(y as *const i32 as _);
    print(y as *const i32 as *mut ());
    print(r as *const _ as *mut ());
    print(r as *const _ as _);
    print2(r as *const _ as _);
    // _ causes the compiler to infer the type
    println!("r: {:p}", r);
    unsafe {
        println!("*r: {}", *r);
    };

    let m: *mut () = &mut ();
    something(print);
}
