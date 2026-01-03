use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_from_wasm() -> String {
    core::hello()
}
