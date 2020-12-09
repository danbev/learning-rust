
#[cfg(test)]
mod tests {
    #[test]
    fn types_test() {
        let nr: i32 = 10;
        let nr2 = &10;
        println!("nr = {} {:p}", nr, &nr);
        println!("nr2 = {} {:p}", nr2, nr2);
        assert_eq!(nr, 10);
        assert_eq!(nr2, &10);
    }
}
