use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorOne;
impl Error for ErrorOne { }
impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "First error (ErrorOne)!")
    }
}

#[derive(Debug)]
struct ErrorTwo;
impl Error for ErrorTwo { }
impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Second error (ErrorTwo)!")
    }
}

// The 'impl Error' means that we are chosing to return anything that
// implements Error but the match is done of two different types ErrorOne and
// ErrorTwo. But if we use dynamic dispatch 
//fn returns_errors(input: u8) -> Result<String, impl Error> {
fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
    match input {
        //0 => Err(ErrorOne),
        //1 => Err(ErrorTwo),
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("worked".to_string())
    }
}

fn main() {
    let vec = vec![0, 1, 2, 4];
    for nr in vec {
        match returns_errors(nr) {
            Ok(message) => println!("{}", message),
            Err(message) => println!("{}", message),
        }
    }
}

