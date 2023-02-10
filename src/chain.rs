use std::collections::HashMap;
use std::iter::Chain;
use std::slice::Iter;

fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    //let iter: Chain<Iter<_>, Iter<_>> = a1.iter().chain(a2.iter());
    //println!("{:?}", iter);

    let sum: i32 = a1.iter().map(|x| x * x).sum();
    println!("{:?}", sum);

    let v2: Vec<_> = a1.iter().map(|x| x).chain(a2.iter()).collect();
    println!("{:?}", v2);

    //let v3: Vec<_> = a1.iter().map(|x| (x, x)).chain(a2.iter()).collect();
    //println!("{:?}", v3);

    let h: HashMap<i32, i32> = a1.iter().map(|&n| (n, n * 10)).collect();
    println!("{:?}", h);

    let h: HashMap<i32, i32> = a1
        .iter()
        .map(|&n| (n, n * 10))
        .chain(a2.iter().map(|&n| (n, n * 10)))
        .collect();
    println!("{:?}", h);
}
