struct SomeThing {
    name: String,
    age: i32,
}

impl SomeThing {
    fn new() -> Self {
        Self { name: "Fletch".to_string(), age: 46}
    }

    fn new_with_args(name: String, age: i32) -> Self {
        Self {name, age}
    }

    fn print(&self) {
        println!("name: {}, age: {}", self.name, self.age);
    }

    fn print2(self: &Self) {
        println!("name: {}, age: {}", self.name, self.age);
    }
}

struct Something2(SomeThing);

fn main() {
    let s = SomeThing::new();
    println!("Size of s: {}", std::mem::size_of::<SomeThing>());
    println!("Size of s.name: {}", std::mem::size_of::<String>());
    println!("Size of s.age: {}", std::mem::size_of::<i32>());
    s.print();

    let x = SomeThing {age: 30, name: "bajja".to_string()};
    x.print();

    let s = Something2(x);
    println!("{}", s.0.name);
    println!("{}", s.0.age);

}
