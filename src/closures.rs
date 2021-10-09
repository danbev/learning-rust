
fn main() {
    let c = |x, y| {
        println!("In closure x: {}, y: {}", x, y);
    };
    c(2, 3);

    let x = 22;
    let cl = || dbg!(x);
    cl();

    let mut name = String::from("Fletch");
    // Example of Fn
    let r = || println!("{}", name);
    r();

    // Example of FnMut
    let mut m = || { 
        name.push('!');
        println!("{}", name);
    };
    m();

    // Example of FnOnce
    let drop_it = || drop(name);
    drop_it();

}
