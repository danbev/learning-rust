#[cfg(test)]
mod tests {
    #[test]
    fn format_test() {
        let port = "8080";
        let addr= format!("0.0.0.0:{}", port);
        println!("{}", addr);
        assert_eq!(addr, "0.0.0.0:8080");
    }
}
