use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Error> {
    let f = File::open("bogus.txt")?;
    Ok(())
}
