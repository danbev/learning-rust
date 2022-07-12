#![feature(bench_black_box)]

#[allow(unused_imports)]
use std::hint::black_box;


fn main() {
    let mut vs = Vec::with_capacity(4);
    let start = std::time::Instant::now();
    for i in 0..4 {
        black_box(vs.as_ptr());
        vs.push(i);
        black_box(vs.as_ptr());
    }
    println!("took {:?}", start.elapsed());
}
