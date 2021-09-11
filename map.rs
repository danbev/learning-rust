
fn something(file_name: &str) -> Option<&str> {
    match file_name.find('.') {
        None => None,                       // if find returned None, just return None
        Some(i) => Some(&file_name[i+1..]), // add a slice as Some
    }
}

fn something2(file_name: &str) -> Option<&str> {
    // this does the same as the match expression in something, and it allows
    // for "mapping" a function onto the Option<T> if there is something in it
    // and if not just return None.
    file_name.find('.').map(|i| &file_name[i+1..]) 
}

fn main() {
    let s = "one.two";
    let sub = &s[1..];
    println!("{}", sub);
    let s1: &str = something(s).unwrap();
    println!("s: {}", s1);
    let s2: &str = something2(s).unwrap();
    println!("s2: {}", s2);
}
