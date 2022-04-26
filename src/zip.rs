fn main() {
    let v1 = vec!["a", "b", "c", "d"];
    let v2 = vec!["1", "2", "3", "4"];

    let zipped = v1.into_iter().zip(v2);
    for (a, b) in zipped {
        println!("{}:{}", a, b);
    }
}
