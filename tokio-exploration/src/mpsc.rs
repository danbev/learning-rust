use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    for i in 0..10 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            tx_clone.send(i).await.expect("Failed to send message");
        });
    }

    drop(tx); // All senders need to be dropped before we start receiving

    while let Some(message) = rx.recv().await {
        println!("Got: {}", message);
    }
}
