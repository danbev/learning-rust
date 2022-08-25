fn main() {
    let x: u32 = 0x00010031;
    let y: u32 = x.rotate_left(16);
    println!("x = {:b} {:x}", x, x);
    println!("y = {:b} {:x}", y, y);

    let cmd: u32 = 0b11000000000000000000000000000100;
    println!("cmd = {:b}, rotated(1)  = {:#034b}", cmd, cmd.rotate_left(1));
    println!("cmd = {:b}, rotated(2)  = {:#034b}", cmd, cmd.rotate_left(2));
    println!("cmd = {:b}, rotated(3)  = {:#034b}", cmd, cmd.rotate_left(3));
    println!("cmd = {:b}, rotated(4)  = {:#034b}", cmd, cmd.rotate_left(4));
    println!("cmd = {:b}, rotated(5)  = {:#034b}", cmd, cmd.rotate_left(5));
    println!("cmd = {:b}, rotated(6)  = {:#034b}", cmd, cmd.rotate_left(6));
    println!("cmd = {:b}, rotated(7)  = {:#034b}", cmd, cmd.rotate_left(7));
    println!("cmd = {:b}, rotated(8)  = {:#034b}", cmd, cmd.rotate_left(8));
    println!("cmd = {:b}, rotated(9)  = {:#034b}", cmd, cmd.rotate_left(9));
    println!("cmd = {:b}, rotated(10) = {:#034b}", cmd, cmd.rotate_left(10));
    println!("cmd = {:b}, rotated(11) = {:#034b}", cmd, cmd.rotate_left(11));
    println!("cmd = {:b}, rotated(12) = {:#034b}", cmd, cmd.rotate_left(12));
    println!("cmd = {:b}, rotated(13) = {:#034b}", cmd, cmd.rotate_left(13));
    println!("cmd = {:b}, rotated(14) = {:#034b}", cmd, cmd.rotate_left(14));
    println!("cmd = {:b}, rotated(15) = {:#034b}", cmd, cmd.rotate_left(15));
    println!("cmd = {:b}, rotated(16) = {:#034b}", cmd, cmd.rotate_left(16));

    let word_length: u32 = 0x00000001;
    println!("word_lenght= {:#034b}, rotated(16)  = {:#034b}", word_length, word_length.rotate_left(16));
}
