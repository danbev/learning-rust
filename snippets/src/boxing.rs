
#[cfg(test)]
mod tests {
    #[test]
    fn boxing_test() {
        let x = 18;
        let y = &x;
        let mut z = Box::new(x);
        println!("--------->x: {}, y: {}, &y: {:p}, z: {}, &z:{:p}", x, y, y, z, z);
        assert_eq!(x, 18);
        assert_eq!(*y, 18);
        assert_eq!(*z, 18);
        *z += 1;
        println!("--------->x: {}, y: {}, &y: {:p}, z: {}, &z:{:p}", x, y, y, z, z);
    }
}
