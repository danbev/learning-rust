use binaryen_sys::{
    BinaryenExpressionPrint, BinaryenFunctionGetBody, BinaryenFunctionGetName,
    BinaryenFunctionGetNumLocals, BinaryenFunctionGetParams, BinaryenFunctionRef,
    BinaryenFunctionSetBody, BinaryenGetFunction, BinaryenModuleDispose, BinaryenModulePrint,
    BinaryenModuleRead, BinaryenModuleRef, BinaryenModuleWrite, BinaryenStore,
};

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read, Write};

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
    let filename = "config.wasm";
    unsafe {
        let module = read_module(filename);

        BinaryenModulePrint(module);

        let func = get_function("inf:wasi/config#get-name", module);
        let func_name = get_function_name(func);
        println!("module_path_func: name: {:?}", func_name);
        let body = BinaryenFunctionGetBody(func);
        println!("body:");
        BinaryenExpressionPrint(body);
        let params = BinaryenFunctionGetParams(func);
        println!("params: {}", params);
        let locals = BinaryenFunctionGetNumLocals(func);
        println!("nr of locals: {}", locals);

        let rust_alloc = get_function("__rust_alloc", module);
        let rust_alloc_body = BinaryenFunctionGetBody(rust_alloc);
        println!("__rust_alloc:");
        BinaryenExpressionPrint(rust_alloc_body);

        //BinaryenStore(module, 9, 1049240, 1, ĸkk

        /*

        let new_body = body;
        BinaryenFunctionSetBody(func, new_body);
        */

        /*
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(1024 * 1024 * 1024, 0);

        let written_size =
            BinaryenModuleWrite(module, buffer.as_mut_ptr() as *mut i8, buffer.len());
        buffer.truncate(written_size);
        let mut file = File::create(filename).expect("Unable to create file");
        file.write(&buffer).expect("Unable to write data to file");
        println!("Wrote {} bytes to {}", written_size, filename);
        */

        BinaryenModuleDispose(module);
    }
}
