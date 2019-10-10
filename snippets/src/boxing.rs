
#[cfg(test)]
mod tests {
    #[test]
    fn box_test() {
        let x = 18;
        let y = &x;
        let z = Box::new(x);
        assert_eq!(x, 18);
        assert_eq!(*y, 18);
        assert_eq!(*z, 18);
    }
}
