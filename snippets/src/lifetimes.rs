pub mod lifetimes {
    pub fn run() {
      println!("Lifetimes exploration.");
      let x;
      {
          let y = 2;
          x = &y;
      }
      //println!("x = {}", x);
      let mut s = "bajja";
      s = doit(s);
      s = doit2(s, s);
      println!("s = {}", s);

      let v = vec![1, 2, 3, 4];
      something(&v);
      let nr = v.get(0);
      println!("v[0] = {}", nr.unwrap());
    }

    pub fn doit(s: &str) -> &str {
        s
    }

    pub fn doit2<'a, 'b>(s: &'a str, s2: &'b str) -> &'b str {
        s2
    }

    pub fn something<T>(v : &Vec<T>) {}
}

#[cfg(test)]
mod tests {
    use super::lifetimes::run;
    #[test]
    fn run_test() {
        run();
    }
}
