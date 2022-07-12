use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
     let v = vec![String::from("M"), String::from("Fletch")];
    /* Just using the following will produce an error:
     * `Vec<String>` cannot be formatted with the default formatter
     * let v = vec![String::from("M"), String::from("Fletch")];
     * println!("{}", v);
    */
    let w = Wrapper(v);
    println!("{}", w);
}
