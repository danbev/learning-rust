pub mod cell {
    use std::cell::Cell;

    #[derive(Debug)]
    pub struct Something {
        pub id: i32,
        pub age: Cell<u32> 
    }

}

#[cfg(test)]
mod test {
    use super::cell::Something;
    use std::cell::Cell;
    #[test]
    pub fn run_test() {
        // Notice that something is immutable (no mut keyword)
        let something = Something{id: 1, age: Cell::new(45)};
        println!("something: {:?}", something);
        something.age.set(46);
        println!("something updated: {:?}", something);
    }
}


