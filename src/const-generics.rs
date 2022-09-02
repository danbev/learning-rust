
struct Something<const N: usize> {
    array: [i32; N],
}


fn main() {
    //    type; number of elements (which can be through of as a const
    //      ↓    ↓     parameter.
    let a: [i32; 3] = [8; 3];
    println!("Arrays have always been const generics: {:?}", a);
    let a: [i32; 3] = [8, 8, 8];
    println!("Arrays have always been const generics: {:?}", a);

    type Array<T, const N: usize> = [T; N];
    let a: Array<i32, 3> = [8; 3];
    println!("Arrays have always been const generics: {:?}", a);

    let s1 = Something { array: [1, 2, 3, 4]};
    println!("s1: {:?}", s1.array);
    let s1 = Something { array: [1, 2, 3]};
    println!("s1: {:?}", s1.array);
}
