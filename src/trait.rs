use std::fmt::Debug;

trait Print {
    const WORD: u32;
}

struct Something {}

impl Print for Something {
    const WORD: u32 = 18;
}


fn print<T: Debug>(val: T) {
    println!("{:?}", val);
}

fn print_impl_trait(val: impl Debug) {
    print(val);
}

fn main() {
    print("bajja");
    print_impl_trait("bajja");
}
