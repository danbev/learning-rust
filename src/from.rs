#[derive(Debug)]
struct Something {
    name: String,
}

impl From<String> for Something {
    fn from(s: String) -> Something {
        Something { name: s }
    }
}

fn process(s: Something) -> () {
    println!("processing: {:?}", s)
}

fn main() {
    /*
    let something: Something = Something::from(String::from("bajja"));
    println!("{:?}", something);
    process(something);
    */

    process(String::from("second").into());
}
