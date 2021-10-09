
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

struct SomeTask {}

impl Future for SomeTask {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Something poll...");
        Poll::Ready(18)
    }
}

fn main() {
}
