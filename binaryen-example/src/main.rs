use binaryen_sys::{
    BinaryenAddDataSegment, BinaryenAddFunction, BinaryenAddFunctionExport, BinaryenConst,
    BinaryenExpressionPrint, BinaryenIndex, BinaryenLiteralInt32, BinaryenModuleCreate,
    BinaryenModuleDispose, BinaryenModulePrint, BinaryenModuleWrite, BinaryenSetMemory,
    BinaryenStringConst, BinaryenTypeCreate, BinaryenTypeInt32,
};

use std::ffi::{c_char, CString};
use std::fs::File;
use std::io::Write;
use std::ptr;

fn main() {
    unsafe {
        let module = BinaryenModuleCreate();

        let data = CString::new("FirstStr").expect("CString::new failed");
        let data_ptr = data.as_ptr();
        let data_len = data.as_bytes().len();
        println!("data_len: {:?}", data_len);

        let data2 = CString::new("What is LoRA?").expect("CString::new failed");
        let data2_ptr = data2.as_ptr();
        let data2_len = data2.as_bytes().len();
        println!("data2_len: {:?}", data2_len);

        let data_offset = BinaryenConst(module, BinaryenLiteralInt32(0));
        let data2_offset = BinaryenConst(module, BinaryenLiteralInt32(data_len as i32));
        let mut segment_offsets = vec![data_offset, data2_offset];

        let segment1_name = CString::new("0").unwrap();
        let segment2_name = CString::new("1").unwrap();
        let segment_names = vec![segment1_name.as_ptr(), segment2_name.as_ptr()];

        let segment_data = vec![data_ptr as *const c_char, data2_ptr as *const c_char];
        let mut segment_passive = vec![false, false];

        let segment_sizes = vec![data_len as BinaryenIndex, data2_len as BinaryenIndex];
        let memory_name = CString::new("mem").unwrap();

        BinaryenSetMemory(
            module,
            1,   // initial pages
            256, // maximum pages
            memory_name.as_ptr(),
            segment_names.as_ptr() as *mut _, // segment names
            segment_data.as_ptr() as *mut _,  // segment data
            segment_passive.as_mut_ptr(),     // segment passive
            segment_offsets.as_mut_ptr(),     // segment offsets
            segment_sizes.as_ptr() as *mut _, // segment sizes
            2,                                // number of segments
            false,                            // shared memory (false)
            false,                            // memory64 (false)
            ptr::null(),                      // name (optional, null for this example)
        );

        let func_name = CString::new("getPrompt").unwrap();
        let func = BinaryenStringConst(module, func_name.as_ptr() as *mut _);
        BinaryenExpressionPrint(func);

        let params = BinaryenTypeCreate(ptr::null_mut(), 0);
        let results = BinaryenTypeInt32();
        let index = data2_offset;
        let _ = BinaryenAddFunction(
            module,
            func_name.as_ptr() as *mut _,
            params,
            results,
            ptr::null_mut(),
            0,
            index,
        );
        BinaryenAddFunctionExport(
            module,
            func_name.as_ptr() as *mut _,
            func_name.as_ptr() as *mut _,
        );

        let data3 = CString::new("Third").expect("CString::new failed");
        let data3_ptr = data3.as_ptr();
        let data3_len = data3.as_bytes().len();
        let segment3_name = CString::new("3").unwrap();
        let segment3_offset =
            BinaryenConst(module, BinaryenLiteralInt32((data_len + data2_len) as i32));

        BinaryenAddDataSegment(
            module,
            segment3_name.as_ptr() as *mut _,
            std::ptr::null_mut(),
            false,
            segment3_offset,
            data3_ptr as *mut _,
            data3_len as BinaryenIndex,
        );

        BinaryenModulePrint(module);

        let output_file = "sample.wasm";
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(1024 * 1024 * 1024, 0);

        let written_size =
            BinaryenModuleWrite(module, buffer.as_mut_ptr() as *mut i8, buffer.len());
        buffer.truncate(written_size);
        let mut file = File::create(output_file).expect("Unable to create file");
        file.write(&buffer).expect("Unable to write data to file");
        println!("Wrote {} bytes to {}", written_size, output_file);

        println!("We should now be able to run wasm2wat {}", output_file);

        BinaryenModuleDispose(module);
    }
}
