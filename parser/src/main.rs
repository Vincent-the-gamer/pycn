use parser::{lexer::lex, parser::parse};

fn main() {
    let code = r#"
    定义 是否是质数（被判断的数）：
        如果 被判断的数 《 2：
            返回 假
        
        迭代 数1 在 范围（2，整数（被判断的数 ** 0.5）+ 1）：
            如果 被判断的数 % 数1 == 0：
                返回 假
        返回 真

    定义 主函数（）：
        迭代 数字 在 范围（1，100）：
            如果 是否是质数（数字）：
                打印（数字）

        布尔值1 = 1 》 2 和 1 《 3
        布尔值2 = 1 》 2 或 1 《 3
        
        打印（布尔值1）
        打印（布尔值2）

    主函数（）
    "#;
    let tokens = lex(code);
    for (i, (tok, s)) in tokens.iter().enumerate() {
        println!("{:03}: {:?} => {}", i, tok, s);
    }
    let ast = parse(&tokens);
    println!("{:#?}", ast);
}
