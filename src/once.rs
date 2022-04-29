use std::sync::Once;

fn main() {
    let once = Once::new();
    once.call_once( || {
        println!("Running intialization code...");
    });
    once.call_once( || {
        println!("Running intialization code...");
    });
}
