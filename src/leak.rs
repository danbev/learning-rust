struct Something<'a> {
    name: &'a str,
}

impl<'a> Drop for Something<'a> {
    fn drop(&mut self) {
        println!("Something drop {}", self.name);
    }
}

fn main() {
    println!("Leak example...");
    let s1 = Something{name: "one"};
    // Drop will not be called for s2 which is the why leak is used.
    let s2 = Box::leak(Box::new(Something{name: "two"}));
}
