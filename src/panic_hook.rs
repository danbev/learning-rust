use std::panic;

fn main() {
    // This will replace the default panic hook.
    panic::set_hook(Box::new(|_| {
        println!("Override the default Panic Hook");
    }));

    panic!("Panic :(");
}


