use js_sys::Number;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module ="/src/package.js")]
extern "C" {

    #[wasm_bindgen(js_name = "isEven")]
    pub fn is_even(number: Number) -> bool;
}