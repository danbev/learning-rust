use std::{thread, time};
use tokio::time::{sleep, Duration};

fn blocking() -> String {
    println!("blocking function started...");
    thread::sleep(time::Duration::from_secs(10));
    "blocking function done".to_string()
}

async fn async_fn(id: u32) {
    sleep(Duration::from_secs(1)).await;
    println!("async_fn {} done", id);
}

#[tokio::main]
async fn main() {
    let b_h = tokio::task::spawn_blocking(blocking);

    let mut handles = Vec::new();
    for i in 0..10 {
        handles.push(tokio::spawn(async_fn(i)));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let v = b_h.await.unwrap();
    println!("{:?}", v);
}
