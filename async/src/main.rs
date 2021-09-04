use futures::executor::block_on;

async fn something() -> i32 {
    println!("something...");
    return 2;
}

fn main() {
    println!("Async exploration..");
    let future = something();
    block_on(future);
}
