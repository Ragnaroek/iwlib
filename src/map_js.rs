extern crate console_error_panic_hook;
extern crate web_sys;

use std::io::{Cursor};
use js_sys::{ArrayBuffer, Uint8Array};

use super::map;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Buffer;

    #[wasm_bindgen(method, getter)]
    fn buffer(this: &Buffer) -> ArrayBuffer;

    #[wasm_bindgen(method, getter, js_name = byteOffset)]
    fn byte_offset(this: &Buffer) -> u32;

    #[wasm_bindgen(method, getter)]
    fn length(this: &Buffer) -> u32;
}

#[wasm_bindgen]
pub fn load_map(map_data_js: &JsValue, map_headers_js: &JsValue, map_offsets_js: &JsValue, mapnum: usize) -> JsValue {
	console_error_panic_hook::set_once();
	let map_data : Vec<u8> = map_data_js.into_serde().unwrap();
	let map_headers : Vec<map::MapType> = map_headers_js.into_serde().unwrap();
	let map_offsets : map::MapFileType = map_offsets_js.into_serde().unwrap();
	let result = map::load_map(&mut Cursor::new(map_data), &map_headers, &map_offsets, mapnum).unwrap();
	JsValue::from_serde(&result).unwrap()

}

#[wasm_bindgen]
pub fn load_map_offsets(buffer: &Buffer) -> JsValue {
	console_error_panic_hook::set_once();

    let bytes: Vec<u8> = Uint8Array::new_with_byte_offset_and_length(
        &buffer.buffer(),
        buffer.byte_offset(),
        buffer.length(),
    ).to_vec();

	let result = map::load_map_offsets(&bytes).unwrap();
	JsValue::from_serde(&result).unwrap()
}