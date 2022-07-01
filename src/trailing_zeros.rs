
fn main() {
    println!("trailing_zeros example");
    println!("nr: {}", 0b1_i32.trailing_zeros());
    println!("nr: {}", 0b10_i32.trailing_zeros());
    println!("nr: {}", 0b11_i32.trailing_zeros());
    println!("nr: {}", 0b100_i32.trailing_zeros());

    let b = BitIter(0b101_u32);
    for v in b {
        println!("bit {} was set", v);
    }
}

struct BitIter(u32);

impl Iterator for BitIter {
    type Item = u32;

    // What this is doing is working backwards through a binary number
    // starting from the least significant bit and only returning
    // the bits that have are set. 
    //
    // For example if we have 101 this function will first
    // return 0 as that bit position was set.
    // After that self.0 will be 100 and the next call will return
    // 2 as that is the next bit that is set.
    fn next(&mut self) -> Option<Self::Item> {
        println!("trailing_zeros: {}", self.0.trailing_zeros());
        match self.0.trailing_zeros() {
            32 => None,
            // remember that this can be 0 as well
            b => {
                //println!("self.0: {:b}, b: {}", self.0, b);
                self.0 &= !(1 << b);
                Some(b)
            }
        }
    }
}
