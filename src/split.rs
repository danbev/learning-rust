fn main() {
    let s = "something is not right in.";
    let sp: Vec<_> = s.split(" ").collect();
    println!("{:?}", sp);
    something(s.split(" ").collect::<Vec<_>>());

    let sp: Vec<_> = s.split(":").collect();
    println!("{:?}", sp.len());
}

fn something<T>(_v: Vec<T>) {}
