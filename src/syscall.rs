#![feature(llvm_asm)]

fn main() {
    let msg = String::from("bajja\n");
    syscall(msg);
}

#[cfg(target_os = "linux")]
#[inline(never)]
fn syscall(msg: String) {
    let msg_ptr = msg.as_ptr();
    let msg_len = msg.len();

    unsafe {
        // Notice the double $ for specifying constants
        // A single $ refers to input params
        llvm_asm!(r#"
        mov $$1, %rax
        mov $$1, %rdi
        mov $0, %rsi
        mov $1, %rdx
        syscall
        "#
        :                            // no output params
        : "r"(msg_ptr), "r"(msg_len) // input params
        : "rax", "rdi", "rsi", "rdx" // clobbers these registers
        :                            // options
        )
    }
}
