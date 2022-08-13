fn something(_: u32) -> Option<String> {
    Some(String::from("bajja"))
}

fn something_string(_: String) -> Option<String> {
    Some(String::from("string"))
}

fn main() {
    println!("os2: {:?}", Some(18).and_then(something).and_then(something_string));
    println!("os3: {:?}", None.and_then(something).and_then(something_string));

}
