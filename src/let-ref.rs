fn main() {
    let ref r1 = 20;
    let r2 = r1;
    println!("{:?}", r1 as *const _);
    println!("{:?}", r2 as *const _);

    // The above is same as writing the following:
    let r1 = &18;
    let r2 = r1;
    println!("{:?}", r1 as *const _);
    println!("{:?}", r2 as *const _);
}
