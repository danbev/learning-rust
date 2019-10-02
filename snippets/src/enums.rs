
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
        #[test]
        fn enums_test() {
            let s = super::Something::ONE(String::from("first"));
            super::func(s);
            assert_eq!(2 + 2, 4);
    }

}
