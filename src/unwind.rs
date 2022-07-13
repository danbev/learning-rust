use std::panic;

type SomeResult<T> = std::result::Result<T, SomeError>;
#[derive(Debug, Clone)]
struct SomeError;

fn main() {
    let r = panic::catch_unwind(|| {
        println!("all is well");
    });
    println!("result.is_ok(): {}", r.is_ok());

    let r = panic::catch_unwind(|| {
        panic!("doh");
    });
    println!("result.is_err(): {}", r.is_err());

    let r = panic::catch_unwind(|| {
        panic!("doh2");
    }).unwrap_or(18);

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

    let r = something(false);
    println!("someresult {:?}", r);
    let r = something(true);
    println!("someresult {:?}", r);
}


fn something(r#try: bool) -> SomeResult<i32> {
    if r#try == true {
        let nr= panic::catch_unwind(move || { 18 } ).map_err(|_| SomeError)?;
        Ok(nr)
    } else {
        let err = panic::catch_unwind(move || { SomeError } ).map_err(|_| SomeError)?;
        Err(err)
    }
}
