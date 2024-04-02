use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[export_name = "_start"]
pub fn print() {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        println!("Import numpy...");
        //let np = py.import("numpy").unwrap();
        //let locals = [("np", np)].into_py_dict(py);
        //let locals = [];
        let builtins = PyModule::import(py, "builtins").unwrap();

        let some_code = PyModule::from_code(
            py,
            r#"
def something(input):
    print(f'Printing from something! {input}')
"#,
            "something.py",
            "something",
        )
        .unwrap();
        some_code.getattr("something").unwrap().call1(("bajja",));

        let python_code = r#"print('Printing from Python!')"#;
        println!("Going to eval the following Python code:  {}", python_code);
        //let result = py.eval(python_code, Some(locals), None);
        let result = py.eval(python_code, None, None);
        match result {
            Ok(_) => println!("Python code executed successfully!"),
            Err(e) => println!("Python code execution failed! {:#?}", e),
        }

        let pm = py.import("time").unwrap();
        println!("{:?}", pm.getattr("ctime").unwrap().call0());
    });
}
