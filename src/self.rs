
struct Something {
    n: &'static str,
}

impl Something {

    fn name_short(&self) -> &'static str {
        self.n
    }

    fn name_full(self: &Self) -> &'static str {
        self.n
    }
}

fn main() {
    let s = Something{ n: "Fletch" };
    println!("name_short: {}, name_full: {}", s.name_short(), s.name_full());
}
