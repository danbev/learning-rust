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

    let mut future = async {
        println!("in future, before await");
        some_future().await;
        println!("in future, after await");
        return "bajja"
    };

    match Pin::new(&mut generator).resume(()) {
      GeneratorState::Yielded(value) => { println!("Generator yielded: {}", value); }
      GeneratorState::Complete(value) => { println!("Generator completed: {}", value); }
    }
    match Pin::new(&mut generator).resume(()) {
      GeneratorState::Yielded(value) => { println!("Generator yielded: {}", value); }
      GeneratorState::Complete(value) => { println!("Generator completed: {}", value); }
    }

    let mut x = 0;
    let mut g = move |mut x| {
        println!("Before yield: {}", x);
        x = yield x;
        println!("After yield:  {}", x);
        return x;
    };

    Pin::new(&mut g).resume(1);
    match Pin::new(&mut g).resume(2) {
      GeneratorState::Complete(value) => { println!("Generator completed: {}", value); }
      GeneratorState::Yielded(value) => { println!("Generator yielded: {}", value); }
    }
    
}

async fn some_future() {
    let f = std::future::ready(18);
    f.await;
}
