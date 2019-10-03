
#[cfg(test)]
mod tests {
    //use super::Doit;
    #[test]
    fn result_test() {
        
        let good: Result<String, &str> = Ok(String::from("Working"));
        assert_eq!(good.is_ok(), true);
        assert_eq!(good.unwrap(), "Working");
        let bad: Result<String, &str> = Err("Not Working");
        assert_eq!(bad.is_ok(), false);
    }
}
