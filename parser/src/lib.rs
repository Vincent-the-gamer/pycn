pub mod lexer;
pub mod ast;
pub mod parser;

use crate::{lexer::lex, parser::{ast_to_python, parse}};

pub fn parse_pycn(code: &str) -> String {
    let tokens = lex(code);
    let ast = parse(&tokens);
    println!("AST: {:#?}", ast);
    let result = ast_to_python(&ast, 0);
    println!("{}", result);
    result
}
