pub mod cli;

use pyo3::ffi::c_str;
use std::{collections::HashMap, ffi::{CString}, fs::read_to_string};

use pyo3::prelude::*;

fn replace_keywords(code: &str) -> String {
    let replace_map = HashMap::from([
        // 关键词替换
        ("定义", "def"),
        ("打印", "print"),
        ("空", "None"),
        ("真", "True"),
        ("假", "False"),
        ("如果", "if"),
        ("要不然", "elif"),
        ("否则", "else"),
        ("返回", "return"),
        ("迭代", "for"),
        ("在", "in"),
        ("范围", "range"),
        ("整数", "int"),
        ("小数", "float"),
        ("字符串", "str"),
        ("长度", "len"),
        ("索引迭代", "enumerate"),
        ("解析", "eval"),
        ("字典", "dict"),
        ("类", "class"),
        ("导入", "import"),
        ("是", "is"),
        ("不是", "not"),
        ("拉姆达", "lambda"),
        ("尝试", "try"),
        ("异常的情况", "except"),
        ("从", "from"),
        ("全局的", "global"),
        ("过", "pass"),
        ("举起", "raise"),
        ("异常", "Exception"),
        ("异步的", "async"),
        ("和", "and"),
        ("或", "or"),

        // 符号替换
        ("（", "("),
        ("）", ")"),
        ("《", "<"),
        ("》", ">"),
        ("，", ","),
        ("：", ":"),
        ("【", "["),
        ("】", "]"),
        ("“", "\""),
        ("”", "\""),
        ("‘", "'"),
        ("’", "'"),
        ("！", "!"),
        ("？", "?"),
        ("、", "\\"),
    ]);

    let mut code = code.to_owned();
    for (key, value) in replace_map {
        code = code.replace(key, value);
    }

    code
}

pub fn run_pycn(code: &str) {
   let code = replace_keywords(code);
   let c_code = CString::new(code).unwrap();
   
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