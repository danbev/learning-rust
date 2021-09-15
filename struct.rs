struct Some {
    name: String,
    age: i32,
}

impl Some {
    fn new() -> Some {
        Some { name: "Fletch".to_string(), age: 46}
    }

    fn print(&self) {
        println!("name: {}, age: {}", self.name, self.age);
    }

    fn print2(self: &Self) {
        println!("name: {}, age: {}", self.name, self.age);
    }
}

fn main() {
    let s = Some::new();
    println!("Size of s: {}", std::mem::size_of::<Some>());
    println!("Size of s.name: {}", std::mem::size_of::<String>());
    println!("Size of s.age: {}", std::mem::size_of::<i32>());
    s.print();
}
