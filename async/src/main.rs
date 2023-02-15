#![feature(async_closure)]
use futures::executor::block_on;

async fn something() -> i32 {
    println!("async function something...");
    return 2;
}

fn main() {
    println!("Async exploration..");
    let future = something();
    block_on(future);

    block_on(async move {
        println!("async block ...");
    });

    let closure = async || {
        println!("async closure");
    };
    println!("Before calling async closure...");

    block_on(closure());
}
