use pyo3::prelude::*;

#[export_name = "_start"]
pub fn print() {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let python_code = "print('Printing from Python!')";
        println!("Going to eval the following Python code:  {}", python_code);
        let result = py.eval(python_code, None, None);
        match result {
            Ok(_) => println!("Python code executed successfully!"),
            Err(e) => println!("Python code execution failed! {:#?}", e),
        }
    });
}
