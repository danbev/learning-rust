mod default {
    pub struct Something<'a> {
        pub value: &'a str,
    }

    impl Default for Something<'_> {
        fn default() -> Self {
            Something { value: "Fletch" }
        }
    }
}

#[cfg(test)]
mod test {
    use super::default::Something;
    #[test]
    pub fn run_test() {
        let s = Something{value: "bajja"};
        println!("default runtest...{}", s.value);
        let s2: Something = Default::default();
        println!("default runtest...{}", s2.value);
    }
}

