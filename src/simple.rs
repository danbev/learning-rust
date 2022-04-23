trait Something {
    fn speak(&self);
}

struct S1 { }

impl Something for S1 {
    fn speak(&self) {
        println!("S1 speak");
    }
}

fn main() {
    let one = 8;
    println!("simple main function");

    let s = {
        println!("in block..{}", 10);
        10
    };

    let s = S1{};
    s.speak();


    println!("{:b}", 10);
    let s = "bajja";
    let s2 = &s;

    let (name, age) = ("Fletch", "46");
    println!("{} : {}", name, age);


    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:?}", v);
}

