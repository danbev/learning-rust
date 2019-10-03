
    #[derive(Debug)]
    enum Something {
        ONE(String),
        TWO(String),
    }

    fn func(s :Something) {
      println!("{:?}", s);
    }


    #[cfg(test)]
    mod tests {
        use super::Something;
        use super::func;
        #[test]
        fn enums_test() {
            let s = Something::ONE(String::from("first"));
            func(s);
            assert_eq!(2 + 2, 4);
    }

}
