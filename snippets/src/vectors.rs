
#[cfg(test)]
mod tests {
    #[test]
    fn vector_test() {
        let mut v1: Vec<i32> = Vec::new();
        v1.push(1);
        v1.push(2);
        v1.push(3);
        let v2 = vec![1, 2, 3];
        assert_eq!(v1, v2);
        assert_eq!(v2[2], 3);
        assert_eq!(v2.get(2).unwrap(), &3);
    }
}
