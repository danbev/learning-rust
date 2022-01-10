fn something() {
    println!("something...");
}

fn main() {
    let s = [0; 1024];
    let t = s;

    // Note that the following is a closure, and something() is the body.
    let closure = move || something();
    closure();
}
