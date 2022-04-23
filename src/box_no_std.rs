#![feature(lang_items, box_syntax, start, libc, core_intrinsics, rustc_private)]
#![no_std]
use core::intrinsics;
use core::panic::PanicInfo;

extern crate libc;

#[lang = "owned_box"]
pub struct Box<T>(*mut T);

#[lang = "exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    libc::printf("alloc request size: %d bytes\n\0".as_ptr() as *const i8, size);

    let ptr = libc::malloc(size as libc::size_t) as *mut u8;
    libc::printf("alloc prt: %p\n\0".as_ptr() as *const i8, ptr);

    // Check if `malloc` failed:
    if ptr as usize == 0 {
        intrinsics::abort();
    }

    ptr
}

#[lang = "box_free"]
unsafe fn box_free<T: ?Sized>(ptr: *mut T) {
    libc::printf("box_free prt: %p\n\0".as_ptr() as *const i8, ptr);
    libc::free(ptr as *mut libc::c_void)
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let x = box 1;
    unsafe {
        libc::printf("box_free prt: %p\n\0".as_ptr() as *const i8, x);
    }

    0
}

#[lang = "eh_personality"]
extern fn rust_eh_personality() {
    unsafe {
        libc::printf("rust_eh_personality\n\0".as_ptr() as *const i8);
    }
}

#[lang = "panic_impl"]
extern fn rust_begin_panic(_info: &PanicInfo) -> ! {
    intrinsics::abort()
}
#[no_mangle]
pub extern fn rust_eh_register_frames () {
    unsafe {
        libc::printf("rust_eh_register_frames\n\0".as_ptr() as *const i8);
    }
}

#[no_mangle]
pub extern fn rust_eh_unregister_frames () {
    unsafe {
        libc::printf("rust_eh_unregister_frames\n\0".as_ptr() as *const i8);
    }
}
