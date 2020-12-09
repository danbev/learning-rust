#[cfg(test)]
mod tests {
    #[test]
    fn format_test() {
        let x = 10;
        let y = x.clone();
        println!("clone y: {}", y);

        let mut s = "bajja1".to_string();
        let mut s2 = String::from("bajja2");
        println!("{}, {}", s, s2);

        let s3:&str = "bajja3";
        println!("{}", s3);
        println!("clone {}", &s3[..]);
    }
}
