
fn something() {
    println!("something...");
    another();
    fn another() {
        println!("another...");
    }
}
fn main() {
    something();
}
