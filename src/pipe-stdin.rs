use std::io::{self, BufRead};

/*
fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        println!("{}", line);
    }
}
*/

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();
        if last_input.len() == 0 {
            break;
        }
        if user_input.len() > 0 {
            user_input.push_str("\n");
        }
        user_input.push_str(&last_input);
    }

    println!("{}", user_input);

    // the lock is released after it goes out of scope
    Ok(())
}
