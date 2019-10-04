#[cfg(test)]
mod tests {
    use std::env;
    #[test]
    #[should_panic]
    fn envar_test() {
        env::var("PORT").expect("PORT not set");
    }
}
