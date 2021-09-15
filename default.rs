//#[derive(Default)]
struct Something {
    one: i32,
    two: i32,
}

impl Default for Something {
    fn default() -> Self {
        Something {one: 1, two :2}
    }
}

fn main() {
    let s = Something::default();
    println!("s.one: {}", s.one);
}
