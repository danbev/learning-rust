use std::mem::size_of;

struct Unsized {}

fn main() {
    let s: Unsized = Unsized{};
    println!("Size of s: {}", size_of::<Unsized>());
    let s2: &str = "bajja";
}
