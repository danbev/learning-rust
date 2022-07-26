#![feature(yeet_expr)] // yeet is slang for discarding an item with high velocity

fn something(b: bool) -> Option<u32> {
    if b == true {
        // exit early from function and return None in this case.
        do yeet;
    } else {
        Some(18)
    }
}

fn main() {
    println!("yeet example");
    println!("something(true): {:?}", something(true));
    println!("something(false): {:?}", something(false));
}
