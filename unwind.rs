use std::panic;

fn f() {
    struct S {
    }
}

fn main() {
    let r = panic::catch_unwind(|| { println!("all is well"); });
    println!("result.is_ok(): {}", r.is_ok());
    let r = panic::catch_unwind(|| { panic!("doh"); });
    println!("result.is_err(): {}", r.is_err());
}
