use binaryen_sys::{
    BinaryenModuleDispose, BinaryenModulePrint, BinaryenModuleRead, BinaryenModuleRef,
};

use std::fs::File;
use std::io::Read;

fn read_module(filename: &str) -> BinaryenModuleRef {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = Vec::new();
    f.read_to_end(&mut contents)
        .expect("something went wrong reading the file");

    unsafe { BinaryenModuleRead(contents.as_ptr() as *mut i8, contents.len()) }
}

fn main() {
    unsafe {
        let module = read_module("config.wasm");

        BinaryenModulePrint(module);

        BinaryenModuleDispose(module);
    }
}
