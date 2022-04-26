fn main() {
    let v = vec!["one", "two", "three", "four"];
    let skipped = v.into_iter().skip(2).collect::<Vec<&str>>();
    println!("{:?}", skipped);

    let v2 = vec!["one", "two", "three", "four"];
    let skipped = v2.into_iter().skip_while(|s| *s != "three").collect::<Vec<&str>>();
    println!("{:?}", skipped);
}
