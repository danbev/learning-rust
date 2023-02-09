use chumsky::prelude::*;
use chumsky::Parser;

#[derive(Debug)]
enum Item {
    Num(u32),
}

fn parser() -> impl Parser<char, Item, Error = Simple<char>> {
    // Here we are using one of Chumsky primitive parsers imported by the
    // prelude, filter which will only accept that the passed in closure return
    // true for.
    // The filter function is imported via the prelude.
    filter(|c: &char| c.is_ascii_digit()).map(|c| Item::Num(c.to_digit(10).unwrap()))
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let output = parser().parse(src).unwrap();
    println!("{:?}", output);
}
