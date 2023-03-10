use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn print(msg: String) {
    let js_string: JsValue = JsValue::from_str(&msg);
    console::log_2(&"print mesg: ".into(), &js_string);
}

fn main() {
    print("bajja".to_string());
}
