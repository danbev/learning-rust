
fn something() -> i32 {
    for i in 0..1000 {
        println!("{}", i);
    }
    18
}


fn main() {
    let s = Some(18);
    // Notice that something will get called regardless of the value of s.
    let t = s.unwrap_or(something());
    println!("t: {}", t);
}
