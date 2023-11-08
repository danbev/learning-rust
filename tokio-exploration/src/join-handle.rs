use std::future::Future;
use tokio::task::JoinHandle;

async fn async_fn() -> u32 {
    1
}

fn desugered_async_fn() -> impl Future<Output = u32> {
    async { 18 }
}

#[tokio::main]
async fn main() {
    let jh: JoinHandle<u32> = tokio::spawn(async_fn());
    let v = jh.await.unwrap();
    println!("{}", v);

    let jh: JoinHandle<u32> = tokio::spawn(desugered_async_fn());
    let v = jh.await.unwrap();
    println!("{}", v);
}
