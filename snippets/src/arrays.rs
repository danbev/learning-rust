pub mod arrays {
    pub fn print(a: &[i32]) {
        println!("arrays::print: {:?}", a);
    }

    pub fn print2(a: &[i32]) {
        println!("arrays::print: {:?}", a);
    }
}

#[cfg(test)]
mod tests {
    use super::arrays::{print, print2};
    #[test]
    fn run_test() {
        let a: [i32; 3] = [1, 2, 3];
        let b: [i32; 3] = [4, 5, 6];
        print(&a);
        //print2(b);
    }
}
