
fn process(nr: i8) -> () {
    println!("{}", nr);
}

fn main() {
    let v = vec![1, 2, 3];
    let mut iterator = IntoIterator::into_iter(v);
    loop {
        match Iterator::next(&mut iterator) {
            Some(elem) => process(elem),
            None => break,
        }
    }
    println!("simple example to show IR");
}
