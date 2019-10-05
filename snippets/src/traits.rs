
pub trait Doit {
    fn doit(&self);
}

struct Something {
    name: String,
}

impl Something {
    fn print(&self) -> () {
    }
}

impl Doit for Something {
    fn doit(&self) {
      println!("something.name = {}", self.name);
    }
}


#[cfg(test)]
mod tests {
    use super::Something;
    use super::Doit;
    #[test]
    fn traits_test() {
        let s = Something { name: String::from("Fletch") };
        s.doit();
        assert_eq!(1, 1);
    }

}
