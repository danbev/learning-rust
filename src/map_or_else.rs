fn main() {
    let k = 1;
    //let x = Some(8);
    let x: Option<u32> = None;
    println!("{}", x.map_or_else(|| default_fn(), |i| something(i)));
}

fn default_fn() -> u32 {
    18
}

fn something(s: u32) -> u32 {
    println!("in something...");
    2 * s
}
