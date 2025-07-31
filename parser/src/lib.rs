pub mod lexer;
pub mod ast;
pub mod parser;
pub mod chinese_to_digits;

use crate::{lexer::lex, parser::{ast_to_python, parse}};

pub fn parse_pycn(code: &str) -> String {
    let tokens = lex(code);
    let ast = parse(&tokens);
    let result = ast_to_python(&ast, 0);
    // println!("{}", result);
    result
}
