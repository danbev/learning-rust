use chumsky::error::Cheap;
use chumsky::prelude::*;

fn main() {
    println!("chumsky primitive parser examples\n");
    /// any is a parser that accepts any input, but not the end of input.
    /// Cheap is an error type.
    let any = any::<char, Cheap<char>>();

    println!("any parser:");
    println!("any accepts any input: {:?}", any.parse("a"));
    /// any accepts any input but only one
    println!("any only parses on character: {:?}", any.parse("ajakflj"));
    println!("any errors on end/empty string: {:?}", any.parse(""));
    println!("");

    println!("end parser:");
    let e = end::<Simple<char>>();
    println!("end accepts end/empty string: {:?}", e.parse(""));
    println!("end errors on non empty string: {:?}", e.parse("something"));
    println!("");

    println!("filter parser:");
    /// Filter only accepts inputs that match the filer.
    let filter = filter::<_, _, Cheap<char>>(char::is_ascii_digit)
        .repeated()
        .at_least(1)
        .padded()
        .then_ignore(end())
        .collect::<String>();
    println!(
        "filter accepts any digits as input: {:?}",
        filter.parse("1111")
    );
    println!(
        "filter accepts whitespace before/after input thanks to padded(): {:?}",
        filter.parse(" 1111 ")
    );
    println!(
        "filter does not allow non digits as input: {:?}",
        filter.parse("111a")
    );

    println!("just parser:");
    let just = just::<_, _, Cheap<char>>('a');
    println!(
        "just does not allow a single char (a) {:?}",
        just.parse("a")
    );
    println!(
        "just does not allow a single char (a) but not \"b\": {:?}",
        just.parse("b")
    );
}
