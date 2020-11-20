pub mod iterator {

    pub fn zip() {
        println!("zip testing.... ");
        let a1 = [1, 2, 3];
        let a2 = [4, 5, 6];
        let mut iter = a1.iter().zip(a2.iter());
        let tuple = iter.next().unwrap();
        println!("tuple.0: {}", tuple.0);
        println!("tuple.1: {}", tuple.1);
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
    }

}

#[cfg(test)]
mod tests {
    use super::iterator::*;
    #[test]
    fn zip_test() {
        zip();
    }
}
