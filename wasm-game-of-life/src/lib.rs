mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // alert("Hello, wasm-game-of-life!");
    alert(format!("Hello {} !", name).as_str());
}
