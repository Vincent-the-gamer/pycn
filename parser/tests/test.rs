use parser::{chinese_to_digits::chinese_to_digits};

#[test]
fn cn_to_digits() {
    let chinese_num: String = "零点一".to_string();
    let result = chinese_to_digits(chinese_num);
    println!("{}", result);
}