use std::str::FromStr;

struct Something {
    nr: u32,
}

#[derive(Debug)]
struct SomethingError;

impl FromStr for Something {
    type Err = SomethingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nr = s.parse::<u32>().unwrap();
        Ok(Something { nr })
    }
}

fn main() {
    println!("From::Str examples");
    let s = "18";
    let something: Something = s.parse().unwrap();
    println!("something.nr: {}", something.nr);
}
