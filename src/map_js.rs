use super::map;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rlew_expand(src: &JsValue, len: usize, rlew_tag: u16) -> JsValue {
	let source : Vec<u8> = src.into_serde().unwrap(); 
	let result = map::rlew_expand(&source, len, rlew_tag);
	JsValue::from_serde(&result).unwrap()	
}