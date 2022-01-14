#![allow(dead_code)]
#![deny(unused_must_use)]

use std::io::Write;
use std::{env, fs, path::PathBuf};

use xshell::cmd;

fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    match &args[..] {
        ["something"] => something(),
        _ => {
            println!("USAGE:");
            println!("\tcargo xtask something");
            Ok(())
        }
    }
}

fn something() -> Result<(), anyhow::Error> {
    println!("something...");
    Ok(())
}
