#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let mut generator = || {
        println!("in generator, before yield");
        yield 18;
        println!("in generator, before return");
        return "bajja"
    };

    let res = Pin::new(&mut generator).resume(());
    if let GeneratorState::Yielded(i) = res {
        println!("yielded: {}", i);
    }

    /*
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(i) => {
            println!("Yielded: {}", i);
        }
        _ => panic!("unexpected value from resume"),
    }
    */

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Complete(s) => {
            println!("Completed: {}", s);
        }
        _ => panic!("unexpected value from resume"),
    }
}

