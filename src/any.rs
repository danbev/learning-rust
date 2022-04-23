
fn doit(a: fn()) {
    a();
}

fn print_bajja() {
    println!("bajja");
}

fn main() {
    println!("Example of function accepting function");
    let s = Box::new(String::new());
    let p = Box::into_raw(s);
    println!("Raw pointer: {:p}", p);
    let f_ptr = print_bajja;
    let ptr = f_ptr as *const ();

    println!("Function pointer: {:p}", ptr);
    // transmute is like memcpy
    //let code: extern "C" fn() = unsafe { std::mem::transmute(ptr) };
    //let code: fn() = unsafe { std::mem::transmute(ptr) };
    //(code)();
    //doit(code);
    doit(print_bajja);

}
