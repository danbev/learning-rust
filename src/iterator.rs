use std::iter::IntoIterator;
use std::ops::Range;

// rustc +nightly --edition=2018 -Zunpretty=expanded iterator.rs 
fn main() {
    for i in 0..3 {
        println!("{}", i);
    }

    let range = Range{ start: 0, end: 3};
    let mut iter = IntoIterator::into_iter(range);

    while let Some(i) = iter.next() {
        println!("{}", i);
    }

    loop {
        match iter.next() {
            Some(i) => println!("{}", i),
            None => break,
        }
    }

}
