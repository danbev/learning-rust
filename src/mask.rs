fn main() {
    let addr: u32 = 0x180041;
    let mask: u32 = 0x7FFF;
    println!("addr     = {:#034b}, dec = {}", addr, addr);
    println!("mask     = {:#034b}, dec = {}", mask, mask);
    println!("!mask    = {:#034b}, dec = {}", !mask, !mask);
    let masked = addr & !mask;
    println!("masked   = {:#034b}, dec= {}", masked, masked);

    let shifted: u8 = (masked >> 24) as u8;
    println!("shifted  = {:#010b}, dec= {}", shifted, shifted);

    let bp_window: u32 = 0xAAAA_AAAA;
    println!("bp_window= {:#034b}, dec= {}", bp_window, bp_window);

    let bp_shifted: u8 = (bp_window >> 24) as u8;
    println!("bp_shifted 24 = {:#010b}, dec= {}", bp_shifted, bp_shifted);

    let bp_shifted: u8 = (bp_window >> 16) as u8;
    println!("bp_shifted 16 = {:#010b}, dec= {}", bp_shifted, bp_shifted);

    let bp_shifted: u8 = (bp_window >> 8) as u8;
    println!("bp_shifted 8  = {:#010b}, dec= {}", bp_shifted, bp_shifted);

    /*
     * addr          = 00000000000110000000000001000001
     * mask          = 00000000000000000111111111111111
     * !mask         = 11111111111111111000000000000000
     *
     * masked        = 00000000000110000000000000000000
     * shifted >> 24 = 00000000000000000000000000000000
     *
     * So in this case it only gives 0 but what we instead had
     * masked        = 10000000000110000000000000000000
     * shifted >> 24 = 00000000000000000000000010000000
     * shifted_as_u8 = 
     */
    let masked: u32 = 0b10000000000110000000000000000000;
    let shifted: u8 = (masked >> 24) as u8;
    println!("shifted  = {:#010b}, dec= {}", shifted, shifted);
}
