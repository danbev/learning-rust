struct Something;

impl Something {
    fn doit(self: &Self) {
        println!("Something::doit");
    }
}

fn main() {
    let s = Something;
    s.doit();
    Something::doit(&s);

    println!("{}", "bajja".to_string());
    println!("{}", str::to_string("bajja"));
    println!("{}", ToString::to_string("bajja"));
    println!("{}", <str as ToString>::to_string("bajja"));
}
