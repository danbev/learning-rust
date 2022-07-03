// This example was taken from Puzzle 18 or the book Rust Brain Teasers.
fn main() {
    let nr = &mut 17;
    // The above is equivalent to:
    // let mut n = 17;
    // let nr = &mut n;
    // The surprising thing to me was that we can declare a reference to a
    // literal. My initial though was that this would not compile as the literal
    // would be hard coded into the code (think argument to an assembly
    // instruction. But in Rust case what Rust will create a temporary area of
    // memory containing the value. This is called 'rvalue static promotion`.
    *nr += 1;
    println!("nr: {}", nr);
}
