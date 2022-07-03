// The lifetime annotation refers to references that instances of this struct
// refer to. The stuct itself does not have a lifetime per say.
struct SomeStruct<'a> {
    name: &'a str,
}

impl<'a> SomeStruct<'a> {
    // because we this is a method which takes a ref to self the output
    // lifetime annotation will be that of self which is tick a.
    //fn something(self: &Self, s: &str) -> &str {
    fn something(self: &'a Self, s: &str) -> &'a str {
        println!("{}", s);
        return self.name
    }
}

fn main() {
    println!("Struct lifetimes examples");
    let s_ref: &str = &String::from("Fletch");
    let s = SomeStruct{ name: s_ref};
    let input = "bajja";
    s.something(input);
}
