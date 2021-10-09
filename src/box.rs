
fn create_on_heap() -> Box<String> {
    Box::new("bajja".to_string())
}

fn main() {
    let s = create_on_heap();
    println!("{:?} {:p}", s, s);
}


