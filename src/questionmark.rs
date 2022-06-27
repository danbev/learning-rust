
fn something(input: i32) -> Option<i32> {
    if input == 1 {
        return None;
    }
    return Some(18);
}

// The ? operator can be used with functions that return Result or Option. I waw
// not aware that it could be used with Option which is the main motivation for
// this example.
fn call_something() -> Option<i32> {
    // The question mark will return early if the following call returns None:
    let op = something(1)?;
    println!("Wil not be printed...");
    Some(op)
}

fn main() {
    let s = call_something();
    println!("{:?}", s);
}
