
fn main() {
    println!("flatten example");

    let v = vec![Some("one"), None, Some("three"), None];
    println!("{:?}", v);
    let iter = v.into_iter();
    //let some = iter.flatten().collect::<Vec<_>>();
    let some = iter.flatten().collect::<Vec<&str>>();
    println!("{:?}", some);

    let op_some = Some("one").into_iter();
    println!("{:?}", op_some.len());
    let op_none: std::option::IntoIter<&str> = None.into_iter();
    println!("{:?}", op_none.len());
}
