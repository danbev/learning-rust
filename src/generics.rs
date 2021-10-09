trait SomeTrait {
    fn doit(&self, a: &str, b: &str) -> String;
}

fn process(input: &dyn SomeTrait) -> () {
    println!("doit1");
}

struct SomeTraitImpl {}
impl SomeTrait for SomeTraitImpl {
    fn doit(&self, a: &str, b: &str) -> String {
        "SomeTraitImpl".to_string()
    }
}

/*
fn do_this<T>(input: &T) -> String
where T: SomeTrait + std::fmt::Debug {
    input.doit("first", "second");
}
*/

/*
fn something<C>(input: C) 
where C:  std::fmt::Debug {
    println!("something<C>: input: {}", input);
}
*/

// So we are saying that T must implement the Trait Add and Sub and that
// their output should be of the same types that are added/substracted.
fn add<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug>(a: T, b: T) -> T {
    println!("a: {:?}, b: {:?}", a, b);
    a - b
}

fn add2<T>(a: T, b: T) -> T 
where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug {
    println!("a: {:?}, b: {:?}", a, b);
    a - b
}


fn main() {
    println!("{}", add(1,2));
    println!("{}", add2(3,2));

    let s1 = SomeTraitImpl{};
    process(&s1);
    let r = s1.doit("input1", "input2");
    println!("{}", r);
}
