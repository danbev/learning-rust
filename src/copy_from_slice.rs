fn main() {
    let a1: [u8; 4] = [1, 2, 3, 4];
    println!("a1: {:#?}", a1);

    // Copy all the values from the specified slice
    let mut a2: [u8; 4] = [0; 4];
    a2.copy_from_slice(&a1);
    println!("a2: {:#?}", a2);

}
