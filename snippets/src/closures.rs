
#[cfg(test)]
mod tests {
    #[test]
    fn closure_test() {
        let s = String::from("bajja");
        let c = |name: String| -> String {
            assert_eq!(name, "Fletch");
            assert_eq!(s, "bajja");
            String::from("done")
        };
        let r = c(String::from("Fletch"));
        assert_eq!(r, "done");
    }
}
