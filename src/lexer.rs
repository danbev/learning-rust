#![feature(rustc_private)]

extern crate rustc_lexer;
use rustc_lexer::{tokenize};
use std::path::Path;
use std::fs;

fn main() {
    let path = Path::new("./src/only_main.rs");
    let src: String = fs::read_to_string(path).unwrap().parse().unwrap();
    let iter = tokenize(&src);
    for token in iter {
        println!("Token.kind: {:?}, Token.len: {:?}", token.kind, token.len);
    }
}

