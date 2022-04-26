fn main() {
    println!("flatten example");
    let v = vec![Some("one"), None, Some("three"), None];
    println!("{:?}", v);
    let iter = v.into_iter();
    //let some = iter.flatten().collect::<Vec<_>>();
    let some = iter.flatten()
        .inspect(|s| println!("before uppder: {}", s))
        .map(|s| s.to_uppercase())
        .collect::<Vec<String>>();
    println!("{:?}", some);
}
