fn main() {
    let v = vec!["one", "two", "three", "four"];
    let taken = v.into_iter().take(2).collect::<Vec<&str>>();
    println!("{:?}", taken);

    let v2 = vec!["one", "two", "three", "four"];
    let taken = v2.into_iter().take_while(|s| *s != "four").collect::<Vec<&str>>();
    println!("{:?}", taken);
}
