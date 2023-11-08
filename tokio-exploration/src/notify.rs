use std::sync::Arc;
use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    let handle = tokio::spawn(async move {
        notify2.notified().await;
        println!("notified");
    });

    println!("Sleeping for 1 second");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Now awake and notifying");
    notify.notify_one();

    handle.await.unwrap();
}
