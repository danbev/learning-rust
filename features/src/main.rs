
#[cfg(feature = "something")]
fn something() {
    println!("with something feature...");
}

#[cfg(not(feature = "something"))]
fn something() {
    println!("without something feature...");
}

fn main() {
    something();
}
