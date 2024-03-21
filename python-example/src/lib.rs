use pyo3::prelude::*;

#[export_name = "_start"]
pub fn print() {
    println!("Python Wasm Example (Printed from Rust)");

    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let python_code = "print('Printing from Python...bajja')";
        let _result = py.eval(python_code, None, None).unwrap();
    })
}
