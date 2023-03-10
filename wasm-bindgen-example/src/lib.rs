use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let val = document.create_element("p")?;
    val.set_inner_html("wasm-bindgen example!");
    body.append_child(&val)?;
    Ok(())
}

#[wasm_bindgen]
pub fn get_message(s: &str) -> String {
    format!("message: {}", s)
}
