struct A {
    x: i32,
    y: i32
}

struct C(i32, i32);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn struct_a() {
        let a = A {x:1, y:2};
        assert_eq!(a.x, 1);
        assert_eq!(a.y, 2);
    }

    #[test]
    fn struct_c() {
        let c = C(1, 2);
        assert_eq!(c.0, 1);
        assert_eq!(c.1, 2);
    }
}
