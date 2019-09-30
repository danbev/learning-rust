use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let r = io::stdin().read_line(&mut guess);
        //.expect("Failed to read line");
    println!("r is_ok: {}", r.is_ok());
    println!("r is_err: {}", r.is_err());
    match r {
        Ok(b) => println!("read {:?}", b),
        Err(e) => println!("Error reading {:?}", e),
    }

    println!("You guessed: {}", guess);
}
