#![feature(async_closure)]
use futures::executor::block_on;

fn main() {
    let x = 10;
    let future = async {
        let y = x;
    };
    block_on(future);
}
