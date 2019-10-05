
#[cfg(test)]
mod tests {
    #[test]
    fn types_test() {
        let nr: i32 = 10;
        let nr2 = &10;
        println!("nr = {}", nr);
        assert_eq!(nr, 10);
    }
}
