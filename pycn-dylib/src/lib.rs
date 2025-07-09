use std::ffi::{c_char, CStr};

use pycn::run_pycn;

#[unsafe(no_mangle)]
pub extern "C" fn run_my_pycn(code: *const c_char) {
    unsafe {
        let code = CStr::from_ptr(code).to_str().expect("Parse code to &str error!");
        run_pycn(code);
    }
}