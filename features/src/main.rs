
#[cfg(feature = "something")]
fn something() {
    println!("with something feature...");
}

#[cfg(feature = "all")]
fn something() {
    println!("with all feature...");
}

#[cfg(not(feature = "something"))]
fn something() {
    println!("without something feature...");
}

fn main() {
    something();

    let s = if cfg!(feature = "bajja") {
        "bajja"
    } else {
        "none"
    };
    println!("{}", s);
}
