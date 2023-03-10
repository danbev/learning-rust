#[no_mangle]
pub extern "C" fn increment(x: i32) -> i32 {
    x + 1
}
