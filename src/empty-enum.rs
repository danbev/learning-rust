enum Something{}

impl Something {
    fn get_value() -> i32 {
        18
    }
}

fn something() -> i32 {
    println!("something function returnes Something Enum");
    return Something::get_value();
}

fn main() {
    println!("Empty Enum example");
    println!("something(): {}", something());
}
