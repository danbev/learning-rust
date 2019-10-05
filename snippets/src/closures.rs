
#[cfg(test)]
mod tests {
    #[test]
    fn closure_test() {
        let c = |name: String| -> String {
            println!("Closure name: {}", name);
            String::from("done")
        };
        let r = c(String::from("Fletch"));
        assert_eq!(r, "done");
    }
}
