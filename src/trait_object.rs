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

    let one = Once{};
    let to: &dyn Doit = &one;
    to.process();
    Doit::process(to);
}

// The following two functions are equivalent. One might prefer the generic
// version (the first one) if the function uses T in multiple parameters.
#[allow(dead_code)]
fn process_long<T: Doit>(d: &T) {
    d.process();
}

#[allow(dead_code)]
fn process_long_where<T>(d: &T) 
    where T: Doit {
    d.process();
}

#[allow(dead_code)]
fn process_short(d: &impl Doit) {
    d.process();
}
