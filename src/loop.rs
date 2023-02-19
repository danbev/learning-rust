fn main() {
    let something = false;
    let result = loop {
        if something {
            break 1;
        }
        break 2;
    };
    println!("result: {}", result);
}
