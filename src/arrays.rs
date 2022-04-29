fn main() {
    let a: [i32; 3] = [1, 2, 3];
    println!("{:?}", a);

    let a = [1, 2, 3];
    println!("{:?}", a);

    //               8 is the value, 3 is the size 
    let a:[i32;3] = [8; 3];
    println!("{:?}", a);

    let a = [8; 3];
    println!("{:?}", a);

    const LENGTH: usize = 2;
    let a:[u32; LENGTH] = [1, 2];
    println!("{:?}", a);

    #[derive(Copy, Clone, Debug)]
    struct S {
        id: i32
    }
    let s = S{id: 0};
    let a = [s; 10];
    println!("{:?}", a);
}
