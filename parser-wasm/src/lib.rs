use parser::parse_pycn;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = "parsePycn")]
pub fn parse_pycn_wasm(code: &str) -> String {
    parse_pycn(code)
}

#[wasm_bindgen(js_name = "parsePycnFile")]
pub fn parse_pycn_wasm_file(path: &str) -> String {
    let code = std::fs::read_to_string(path).expect("Failed to read file");
    parse_pycn_wasm(&code)
}