pub mod owner {
    pub fn run() {
      let mut stack_allocated = "stack allocated";
      println!("{:?}", stack_allocated);
      let x = 22;
      let y = x;
      println!("x = {} {:p}, y = {} {:p}", x, &x, y, &y);

      let s = create_string();
      println!("created s = {} {:p}", s, &s);
      let s = use_string(s);
      println!("used s = {} {:p}", s, &s);
    }

    pub fn create_string() -> String {
        let s = String::from("bajja");
        s  // will move the string because it's a return value.
    }

    pub fn use_string(s : String) -> String {
        // use s
        s  // will move the string because it's a return value.
    }
}

#[cfg(test)]
mod tests {
    use super::owner::run;
    #[test]
    fn run_test() {
        run();
    }
}
