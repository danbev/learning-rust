
fn main() {
    let count = 100;
    let mut v = Vec::with_capacity(count);
    v.extend(std::iter::repeat(18).take(count));
    println!("{:?}", v);
}
