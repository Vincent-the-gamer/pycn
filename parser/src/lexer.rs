use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // 关键字
    #[token("def")]
    #[token("定义")]
    Def,
    #[token("if")]
    #[token("如果")]
    If,
    #[token("else")]
    #[token("否则")]
    Else,
    #[token("elif")]
    #[token("要不然")]
    Elif,
    #[token("for")]
    #[token("迭代")]
    For,
    #[token("while")]
    #[token("当")]
    While,
    #[token("in")]
    #[token("在")]
    In,
    #[token("is")]
    #[token("是")]
    Is,
    #[token("not")]
    #[token("不是")]
    Not,
    #[token("and")]
    #[token("和")]
    And,
    #[token("or")]
    #[token("或")]
    Or,
    #[token("None")]
    #[token("空")]
    None,
    #[token("True")]
    #[token("真")]
    True,
    #[token("False")]
    #[token("假")]
    False,
    #[token("return")]
    #[token("返回")]
    Return,
    #[token("break")]
    #[token("跳出")]
    Break,
    #[token("continue")]
    #[token("继续")]
    Continue,
    #[token("pass")]
    #[token("过")]
    Pass,
    #[token("import")]
    #[token("导入")]
    Import,
    #[token("from")]
    #[token("从")]
    From,
    #[token("as")]
    #[token("作为")]
    As,
    #[token("class")]
    #[token("类")]
    Class,
    #[token("try")]
    #[token("尝试")]
    Try,
    #[token("except")]
    #[token("意外情况")]
    Except,
    #[token("finally")]
    #[token("最终")]
    Finally,
    #[token("raise")]
    #[token("举起")]
    Raise,
    #[token("assert")]
    #[token("断言")]
    Assert,
    #[token("del")]
    #[token("删除")]
    Del,
    #[token("global")]
    #[token("全局的")]
    Global,
    #[token("nonlocal")]
    #[token("非局部")]
    Nonlocal,
    #[token("lambda")]
    #[token("拉姆达")]
    Lambda,
    #[token("yield")]
    #[token("产出")]
    Yield,
    #[token("await")]
    #[token("等待")]
    Await,
    #[token("async")]
    #[token("异步的")]
    Async,
    #[token("with")]
    #[token("带上")]
    With,
    #[token("match")]
    #[token("匹配")]
    Match,
    #[token("case")]
    #[token("情况")]
    Case,
    #[token("print")]
    #[token("打印")]
    Print,

    // 内置函数
    BuiltInFunc(String),

    // 标识符
    #[regex(r"[a-zA-Z_\u4e00-\u9fa5][a-zA-Z0-9_\u4e00-\u9fa5]*")]
    Identifier,

    // 数字
    #[regex(r"[0-9]+\.[0-9]+")]
    Float,
    #[regex(r"[0-9]+")]
    Integer,
    // 字符串：支持format string格式化字符串，支持英文单双引号和中文单双引号
    #[regex(r#"f'([^'\\]|\\.)*'"#)]
    #[regex(r#"f\"([^\"\\]|\\.)*\""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"“([^“”\\]|\\.)*”"#)]
    #[regex(r#"‘([^‘’\\]|\\.)*’"#)]
    String,
    // 运算符和分隔符
    #[token("**")]
    Pow,
    #[token("//")]
    FloorDiv,
    #[token("%")]
    Mod,
    #[token("=")]
    Equal,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token(".")]
    Dot,
    #[token("(")]
    #[token("（")]
    LParen,
    #[token(")")]
    #[token("）")]
    RParen,
    #[token(":")]
    #[token("：")]
    Colon,
    #[token(",")]
    #[token("，")]
    Comma,
    #[token("==")]
    DoubleEqual,
    #[token("!=")]
    #[token("！=")]
    NotEqual,
    #[token("<")]
    #[token("《")]
    Less,
    #[token("<=")]
    #[token("《=")]
    LessEqual,
    #[token(">")]
    #[token("》")]
    Greater,
    #[token(">=")]
    #[token("》=")]
    GreaterEqual,
    #[token("[")]
    #[token("【")]
    LBracket,
    #[token("]")]
    #[token("】")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("\n")]
    Newline,
    #[regex(r"#.*")]
    Comment,
    Indent,
    Dedent,
    #[regex(r"[ \t\r]+", logos::skip)]
    Error,
}

pub fn lex(input: &str) -> Vec<(Token, String)> {
    use Token::*;
    let keyword_map: std::collections::HashMap<&str, Token> = [
        ("定义", Def), ("def", Def), ("如果", If), ("if", If), ("否则", Else), ("else", Else), ("要不然", Elif), ("elif", Elif),
        ("迭代", For), ("for", For), ("当", While), ("while", While), ("在", In), ("in", In), ("是", Is), ("is", Is),
        ("不是", Not), ("not", Not), ("和", And), ("and", And), ("或", Or), ("or", Or), ("空", None), ("None", None),
        ("真", True), ("True", True), ("假", False), ("False", False), ("返回", Return), ("return", Return),
        ("跳出", Break), ("break", Break), ("继续", Continue), ("continue", Continue), ("过", Pass), ("pass", Pass),
        ("导入", Import), ("import", Import), ("从", From), ("from", From), ("作为", As), ("as", As), ("类", Class), ("class", Class),
        ("尝试", Try), ("try", Try), ("意外情况", Except), ("except", Except), ("最终", Finally), ("finally", Finally),
        ("举起", Raise), ("raise", Raise), ("断言", Assert), ("assert", Assert), ("删除", Del), ("del", Del),
        ("全局的", Global), ("global", Global), ("非局部", Nonlocal), ("nonlocal", Nonlocal), ("拉姆达", Lambda), ("lambda", Lambda),
        ("产出", Yield), ("yield", Yield), ("等待", Await), ("await", Await), ("异步的", Async), ("async", Async),
        ("带上", With), ("with", With), ("匹配", Match), ("match", Match), ("情况", Case), ("case", Case),
        ("打印", Print), ("print", Print)
    ].into_iter().collect();
    let mut tokens = Vec::new();
    let mut indents = vec![0];
    let mut pending_newline = false;
    let mut line_iter = input.lines().peekable();
    while let Some(line) = line_iter.next() {
        let mut chars = line.chars();
        let mut spaces = 0;
        while let Some(c) = chars.next() {
            if c == ' ' {
                spaces += 1;
            } else if c == '\t' {
                spaces += 8; // python官方推荐tab=8
            } else {
                break;
            }
        }
        let trimmed = line.trim_start();
        // 跳过空行和注释行
        if trimmed.is_empty() || trimmed.starts_with('#') {
            pending_newline = true;
            continue;
        }
        let current = *indents.last().unwrap();
        if spaces > current {
            indents.push(spaces);
            tokens.push((Token::Indent, "<INDENT>".to_string()));
        } else if spaces < current {
            while spaces < *indents.last().unwrap() {
                indents.pop();
                tokens.push((Token::Dedent, "<DEDENT>".to_string()));
            }
        }
        // 行内分词
        let mut lexer = Token::lexer(trimmed);
        let mut first_token = true;
        while let Some(res) = lexer.next() {
            if let Ok(mut tok) = res {
                let slice = lexer.slice().to_string();
                if let Token::Identifier = tok {
                    if let Some(kw_token) = keyword_map.get(slice.as_str()) {
                        tok = kw_token.clone();
                    } else {
                        let builtin = match slice.as_str() {
                            "打印" | "print" => Some("print"),
                            "范围" | "range" => Some("range"),
                            "整数" | "int" => Some("int"),
                            "小数" | "float" => Some("float"),
                            "字符串" | "str" => Some("str"),
                            "输入" | "input" => Some("input"),
                            "长度" | "len" => Some("len"),
                            "列表" | "list" => Some("list"),
                            "字典" | "dict" => Some("dict"),
                            "集合" | "set" => Some("set"),
                            "元组" | "tuple" => Some("tuple"),
                            "索引迭代" | "enumerate" => Some("enumerate"),
                            "解析" | "eval" => Some("eval"),
                            "求和" | "sum" => Some("sum"),
                            "最小值" | "min" => Some("min"),
                            "最大值" | "max" => Some("max"),
                            "绝对值" | "abs" => Some("abs"),
                            "全部为真" | "all" => Some("all"),
                            "有一个为真" | "any" => Some("any"),
                            "映射" | "map" => Some("map"),
                            "过滤" | "filter" => Some("filter"),
                            "拉链" | "zip" => Some("zip"),
                            "打开" | "open" => Some("open"),
                            "执行" | "exec" => Some("exec"),
                            "类型" | "type" => Some("type"),
                            "实例判断" | "isinstance" => Some("isinstance"),
                            "子类判断" | "issubclass" => Some("issubclass"),
                            "目录" | "dir" => Some("dir"),
                            "变量" | "vars" => Some("vars"),
                            "本地变量" | "locals" => Some("locals"),
                            "全局变量" | "globals" => Some("globals"),
                            "帮助" | "help" => Some("help"),
                            "编号" | "id" => Some("id"),
                            "表达" | "repr" => Some("repr"),
                            "排序" | "sorted" => Some("sorted"),
                            "反转" | "reversed" => Some("reversed"),
                            "下一个" | "next" => Some("next"),
                            "迭代器" | "iter" => Some("iter"),
                            "父类" | "super" => Some("super"),
                            "对象" | "object" => Some("object"),
                            "类方法" | "classmethod" => Some("classmethod"),
                            "静态方法" | "staticmethod" => Some("staticmethod"),
                            "属性" | "property" => Some("property"),
                            "异常" | "Exception" => Some("Exception"),
                            _ => Option::None,
                        };
                        if let Some(eng) = builtin {
                            tok = Token::BuiltInFunc(eng.to_string());
                        }
                    }
                }
                if first_token && pending_newline {
                    tokens.push((Token::Newline, "\n".to_string()));
                    pending_newline = false;
                }
                first_token = false;
                tokens.push((tok.clone(), slice));
            }
        }
        tokens.push((Token::Newline, "\n".to_string()));
    }
    // 文件结尾补齐所有 dedent
    while indents.len() > 1 {
        indents.pop();
        tokens.push((Token::Dedent, "<DEDENT>".to_string()));
    }
    tokens
}
