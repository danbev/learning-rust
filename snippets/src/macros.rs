macro_rules! fact {
    ($x:expr) => {
        println!("macro expr: {}", $x);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_test() {
        fact!(10);
        assert_eq!(true, true);
    }
}
