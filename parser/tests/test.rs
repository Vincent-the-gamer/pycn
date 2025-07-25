use parser::{parse_pycn};

#[test]
fn test() {
    let code = include_str!("../../examples/函数.pycn");
    parse_pycn(code);
}