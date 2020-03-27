extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// #[ ] is called an "attribute" and it modifies the next statement somehow
#[wasm_bindgen]
extern {
	pub fn alert(s: &str);
}
// extern -> denotes a wasm_bindgen that this code will consume

// this method is the external method that will be used by 
#[wasm_bindgen]
pub fn greet(name: &str) {
	alert(&format!("Hello, {}!", name));
}
