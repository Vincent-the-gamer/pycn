pub mod cli;

use parser::parse_pycn;
use pyo3::ffi::c_str;
use std::{ffi::{CString}, fs::read_to_string};

use pyo3::prelude::*;

pub fn run_pycn(pycn_code: &str) {
   let py_code = parse_pycn(pycn_code);
   let c_code = CString::new(py_code).unwrap();
   
   Python::with_gil(|py| {
      let _ = PyModule::from_code(py, &c_code, c_str!(""), c_str!(""));
   });
}

pub fn run_pycn_file(path: &str) {
    let code = read_to_string(path);
    match code {
        Ok(code) => run_pycn(&code),
        Err(err) => println!("File read error: {}", err)
    };
}