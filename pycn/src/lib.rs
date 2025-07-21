pub mod cli;

use parser::{parse_pycn, lexer::lex, parser::parse, parser::ast_to_python};
use pyo3::ffi::c_str;
use std::{ffi::{CString}, fs::{read_to_string}, path::Path};

use pyo3::prelude::*;

pub fn run_pycn(pycn_code: &str) {
   let py_code = parse_pycn(pycn_code);
   let c_code = CString::new(py_code).unwrap();

   Python::with_gil(|py| {
      let _ = PyModule::from_code(py, &c_code, c_str!(""), c_str!(""));
   });
}

pub fn run_pycn_file(path: &str) {
    let code = match read_to_string(path) {
        Ok(c) => c,
        Err(err) => {
            println!("File read error: {}", err);
            return;
        }
    };
    // 递归收集所有 pycn 依赖，构建 HashMap<模块名, py代码>
    let mut module_map = std::collections::HashMap::new();
    let entry_mod = std::path::Path::new(path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap()
        .to_string();
    collect_py_modules(&code, std::path::Path::new(path), &mut module_map);
    // 注入 import hook 并执行入口模块
    run_pycn_with_modules(&entry_mod, &module_map);
}


/// 递归收集 pycn 依赖，构建 HashMap<模块名, py代码>
fn collect_py_modules(code: &str, pycn_path: &Path, modules: &mut std::collections::HashMap<String, String>) {
    let tokens = lex(code);
    let ast = parse(&tokens);
    let mod_name = pycn_path.file_stem().and_then(|n| n.to_str()).unwrap_or("main").to_string();
    let py_code = ast_to_python(&ast, 0);
    modules.insert(mod_name.clone(), py_code);
    // 递归处理 import
    collect_imports_from_ast(&ast, pycn_path.parent().unwrap_or(Path::new(".")), modules);
}

fn collect_imports_from_ast(ast: &parser::ast::AstNode, dir: &Path, modules: &mut std::collections::HashMap<String, String>) {
    use parser::ast::AstNode;
    match ast {
        AstNode::Program(stmts) => {
            for stmt in stmts {
                collect_imports_from_ast(stmt, dir, modules);
            }
        }
        AstNode::Import { module, .. } | AstNode::ImportFrom { module, .. } => {
            let pycn_path = dir.join(format!("{}.pycn", module));
            if pycn_path.exists() && !modules.contains_key(module) {
                if let Ok(code) = std::fs::read_to_string(&pycn_path) {
                    collect_py_modules(&code, &pycn_path, modules);
                }
            }
        }
        _ => {}
    }
}

/// 在 Python 侧注册 import hook，并执行入口模块
fn run_pycn_with_modules(entry_mod: &str, modules: &std::collections::HashMap<String, String>) {
    use pyo3::types::PyDict;
    pyo3::Python::with_gil(|py| {
        let importlib = py.import("importlib").unwrap();
        // 构造 modules dict
        let py_modules = PyDict::new(py);
        for (k, v) in modules.iter() {
            py_modules.set_item(k, v).unwrap();
        }
        // Python 代码：注册 import hook
        let hook_code = r#"
import sys, types
class InMemoryLoader:
    def __init__(self, modules):
        self.modules = modules
    def find_spec(self, fullname, path, target=None):
        if fullname in self.modules:
            import importlib.machinery
            return importlib.machinery.ModuleSpec(fullname, self)
        return None
    def create_module(self, spec):
        return None
    def exec_module(self, module):
        code = self.modules[module.__name__]
        exec(code, module.__dict__)
sys.meta_path.insert(0, InMemoryLoader(__modules__ref__))
"#;
        let locals = PyDict::new(py);
        locals.set_item("__modules__ref__", py_modules).unwrap();
        locals.set_item("importlib", importlib).unwrap();
        let builtins = py.import("builtins").unwrap();
        let exec_func = builtins.getattr("exec").unwrap();
        // 先执行 import hook 注册代码
        exec_func.call1((hook_code, &locals)).unwrap();
        // 构造入口模块 globals，包含 __name__
        let entry_code = modules.get(entry_mod).unwrap();
        let entry_globals = PyDict::new(py);
        entry_globals.set_item("__name__", "__main__").unwrap();
        exec_func.call1((entry_code.as_str(), entry_globals)).unwrap();
    });
}

