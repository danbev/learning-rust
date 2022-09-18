
struct Something<T> {
    s: T,
}

impl<T> Something<T> {
    fn doit(&self) {
        fn inner_function() {
        }
        inner_function();
    }
}

fn main() {
    let s1: Something<u8> = Something { s: 255 };
    s1.doit();
    let s2: Something<u16> = Something { s: 32000 };
    s2.doit();
}
