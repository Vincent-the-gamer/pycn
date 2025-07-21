use std::ffi::{c_char, CStr};

use pycn::{run_pycn, run_pycn_file};

#[unsafe(no_mangle)]
pub extern "C" fn run_my_pycn(code: *const c_char) {
    unsafe {
        let code = CStr::from_ptr(code).to_str().expect("Parse code to &str error!");
        run_pycn(code);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn run_my_pycn_file(file_path: *const c_char) {
    unsafe {
        let path = CStr::from_ptr(file_path).to_str().expect("Parse code to &str error!");
        run_pycn_file(path);
    }
}