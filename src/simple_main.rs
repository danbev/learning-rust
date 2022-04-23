#![feature(no_core)]
#![feature(lang_items)]

#![no_core]

#[cfg(target_os = "linux")]
#[link(name = "c")]
extern {}

// Compiler needs these to proceed
#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}

// `main` isn't the actual entry point, `start` is.
#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    18
}

fn main() {
}
