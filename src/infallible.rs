use std::convert::Infallible;

trait Something {
    type Error;

    fn doit(&self) -> Result<i32, Self::Error>;
}

struct S { }

impl Something for S {
    type Error = Infallible;

    fn doit(&self) -> Result<i32, Self::Error> {
        Ok(10)
    }
}

fn main() {
    println!("Infallible example");
    let s = S{};
    let nr = s.doit().unwrap();
    println!("{}", nr);
}
