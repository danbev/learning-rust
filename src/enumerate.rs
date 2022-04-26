fn main() {
    let v = vec!["a", "b", "c", "d"];
    println!("{:?}", v);
    let e = v.into_iter().enumerate();
    for (index, letter) in e {
        println!("{}: {}", index, letter);
    }
}
