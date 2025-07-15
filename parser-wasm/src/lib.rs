use parser::parse_pycn;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = "parsePycn")]
pub fn parse_pycn_wasm(code: &str) -> String {
    parse_pycn(code)
}