use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn return_some_vec() -> Vec<String> {
    ["a", "b", "c"].map(String::from).to_vec()
}

#[wasm_bindgen]
extern "C" {
    fn name() -> String;
}

#[wasm_bindgen]
pub fn greet() -> String {
    format!("Hello, {}", name())
}
