use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();
    let tx: tokio::sync::oneshot::Sender<&str> = tx;
    let rx: tokio::sync::oneshot::Receiver<&str> = rx;
    println!("oneshot channel created: tx = {:?}, rx = {:?}", tx, rx);

    tokio::spawn(async move {
        if let Err(_) = tx.send("oneshot from spawned task") {
            println!("The receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("Got {}", v),
        Err(_) => println!("The sender dropped"),
    }
}
