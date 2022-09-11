fn main() {
    let zero: u8 = 0;
    println!("{}", zero);
    //if zero - 10 < 3 {
    let x = zero.wrapping_sub(1);
    println!("{}", x);
    if zero.wrapping_sub(1) > 64 {
        println!("not sure we should be doing this...");
    }
}
