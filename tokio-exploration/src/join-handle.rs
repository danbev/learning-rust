use std::future::Future;
use tokio::task::JoinHandle;

async fn async_fn() -> u32 {
    1
}

fn desugered_async_fn() -> impl Future<Output = u32> {
    async { 18 }
}

async fn async_string(s: &str) -> String {
    s.to_string()
}

#[tokio::main]
async fn main() {
    let jh: JoinHandle<u32> = tokio::spawn(async_fn());
    let v = jh.await.unwrap();
    println!("{}", v);

    let jh: JoinHandle<u32> = tokio::spawn(desugered_async_fn());
    let v = jh.await.unwrap();
    println!("{}", v);

    // tokio::task::spawn is reexported in/by tokio::spawn so it is the same
    // functions that is being called.
    let jh: JoinHandle<String> = tokio::task::spawn(async_string("bajja"));
    let v = jh.await.unwrap();
    println!("{}", v);
}
