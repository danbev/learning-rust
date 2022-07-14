#![feature(once_cell)]

use std::cell::LazyCell;

fn main() {
    let nr: LazyCell<i32> = LazyCell::new(|| {
        println!("init nr LazyCell...");
        18
    });
    println!("LazyCell example...");
    println!("{}", *nr);
    println!("{}", *nr);
}
