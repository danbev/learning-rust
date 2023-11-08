/// This example shows what the macro #[tokio::main] expands to (not exactly
/// but close enough).
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("bajja");
    });
}
