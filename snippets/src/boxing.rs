use std::ops::Deref;

// this is a tuple struct with one member
struct B<T>(T);

impl<T> Deref for B<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }

}

impl<T> B<T> {
    fn new(x: T) -> B<T> {
        B(x)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
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

        let i = B::new(x);
        assert_eq!(*i, 18);
    }
}
