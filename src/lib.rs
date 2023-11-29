mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut returned_text: String = String::from("Hello, ");
    returned_text.push_str(name);
    alert(&returned_text);
}
