use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> String {
    String::from(format!("the required output is {}", a + b))
}
