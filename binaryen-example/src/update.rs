use binaryen_sys::{
    BinaryenExpressionPrint, BinaryenFunctionGetBody, BinaryenFunctionGetName, BinaryenFunctionRef,
    BinaryenGetFunction, BinaryenModuleDispose, BinaryenModulePrint, BinaryenModuleRead,
    BinaryenModuleRef,
};

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;

fn read_module(filename: &str) -> BinaryenModuleRef {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = Vec::new();
    f.read_to_end(&mut contents)
        .expect("something went wrong reading the file");

    unsafe { BinaryenModuleRead(contents.as_ptr() as *mut i8, contents.len()) }
}

fn get_function(name: &str, module: BinaryenModuleRef) -> BinaryenFunctionRef {
    unsafe {
        let name = CString::new(name).unwrap();
        BinaryenGetFunction(module, name.as_ptr())
    }
}

fn get_function_name(func: BinaryenFunctionRef) -> String {
    unsafe {
        let name = BinaryenFunctionGetName(func);
        CStr::from_ptr(name).to_string_lossy().into_owned()
    }
}

fn main() {
    unsafe {
        let module = read_module("config.wasm");

        BinaryenModulePrint(module);

        let model_path_func = get_function("inf:wasi/config#get-model-path", module);
        let model_path_name = get_function_name(model_path_func);
        println!("module_path_func: name: {:?}", model_path_name);
        let body = BinaryenFunctionGetBody(model_path_func);
        BinaryenExpressionPrint(body);

        BinaryenModuleDispose(module);
    }
}
