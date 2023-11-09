use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[tokio::main]
async fn main() {
    let (tx, mut rx): (UnboundedSender<i32>, UnboundedReceiver<i32>) = mpsc::unbounded_channel();

    for i in 0..10 {
        let tx_clone: UnboundedSender<i32> = tx.clone();
        tokio::spawn(async move {
            tx_clone.send(i).expect("Failed to send message");
        });
    }

    // Dropping the original sender so the receiver knows when to stop
    drop(tx);

    while let Some(message) = rx.recv().await {
        println!("Got: {}", message);
    }
}
