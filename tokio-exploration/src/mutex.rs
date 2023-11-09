use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

#[tokio::main]
async fn main() {
    // Create a Tokio Mutex wrapped around some data
    let data = Arc::new(Mutex::new(0));

    // Clone the Arc to share ownership with the spawned tasks
    let data_clone1 = Arc::clone(&data);
    let data_clone2 = Arc::clone(&data);

    // Spawn two tasks that will access the data
    let handle1 = task::spawn(async move {
        let mut data = data_clone1.lock().await;
        *data += 1;
        println!("Task 1: data: {}", *data);
    });

    let handle2 = task::spawn(async move {
        let mut data = data_clone2.lock().await;
        *data += 2;
        println!("Task 2: data: {}", *data);
    });

    // Wait for both tasks to complete
    let _ = handle1.await;
    let _ = handle2.await;

    // Final value of the shared data
    println!("Final value: {}", *data.lock().await);
}
