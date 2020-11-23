pub mod refs {
    pub fn run() {
        let v = vec![1, 2, 3];
        let r = &v;
        print(r);
        print(&v);

        let mut x = 18;
        let rx = &x;
        println!("rx: {}", *rx);
        let mx = &mut x;
        *mx += 18;
        println!("mx: {}", *mx);

        refs(&x);
    }

    pub fn print(v: &Vec<u8>) -> () {
        println!("{:?}", v);
    }

    pub fn refs<'a>(n: &'a u8) -> () {
        let r1 = &n;
        let r2 = &r1;
        let r3 = &r2;
        println!("{}", ***r3);
    }

    struct Something<'a> {
        n: &'a u8
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ref_test() {
        refs::run();
    }
}
