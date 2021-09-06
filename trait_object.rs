trait Doit {
    fn process(&self);
}

fn call_process(d: &dyn Doit) {
    d.process();
}

struct Once {}
impl Doit for Once {
    fn process(&self) {
        println!("once...");
    }
}

struct Twice {}
impl Doit for Twice {
    fn process(&self) {
        println!("twice...");
    }
}

fn main() {
    let once = &Once{};
    let twice = &Twice{};
    call_process(once);
    call_process(twice);
}
