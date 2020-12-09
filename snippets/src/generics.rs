#[derive(Debug)]
struct Something<T: ?Sized> {
    name: T,
}

impl Something<String> {
    fn print(&self) -> () {
      println!("{:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::Something;
    #[test]
    fn run_test() {
        let s = Something { name: String::from("Fletch") };
        s.print();
    }

}
