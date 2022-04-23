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

    let r = panic::catch_unwind(|| { panic!("doh"); }).unwrap_or(18);
    println!("unwrap_or: {}", r);

    let default = 101;
    let r: Result<u32, &str> = Ok(9);
    println!("{:?}", r);
    let x = r.unwrap_or(default);
    println!("{:?}", x);

    let r: Result<u32, &str> = Err("bajja");
    println!("{:?}", r);
    let x = r.unwrap_or(default);
    println!("{:?}", x);
}
