use logos::Logos;
use crate::chinese_to_digits::chinese_to_digits;

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
    // 中文数字（包括：零一二三四五六七八九十百千万等）
    #[regex(r"[点零一二三四五六七八九十百千万亿壹贰叁肆伍陆柒捌玖拾佰仟]+", priority = 3)]
    ChineseNumber,
    // 字符串：支持format string格式化字符串，支持英文单双引号和中文单双引号
    #[regex(r#"f'([^'\\]|\\.)*'"#)]
    #[regex(r#"f\"([^\"\\]|\\.)*\""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"“([^“”\\]|\\.)*”"#)] // 前引号在前
    #[regex(r#"”([^“”\\]|\\.)*“"#)] // 后引号在前
    #[regex(r#"“([^“”\\]|\\.)*“"#)] // 两个前引号
    #[regex(r#"”([^“”\\]|\\.)*”"#)] // 两个后引号
    #[regex(r#"‘([^‘’\\]|\\.)*’"#)]
    String,
    // 运算符和分隔符
    #[token("**")]
    #[token("取幂")]
    Pow,
    #[token("//")]
    #[token("地板除")]
    FloorDiv,
    #[token("%")]
    #[token("取余")]
    #[token("取模")]
    Mod,
    #[token("=")]
    #[token("赋值为")]
    Equal,
    #[token("+")]
    #[token("加")]
    Plus,
    #[token("-")]
    #[token("减")]
    Minus,
    #[token("*")]
    #[token("乘")]
    Star,
    #[token("/")]
    #[token("除以")]
    Slash,
    #[token("&")]
    #[token("按位与")]
    BitwiseAnd,
    #[token("|")]
    #[token("按位或")]
    BitwiseOr,
    #[token("^")]
    #[token("按位异或")]
    BitwiseXor,
    #[token("~")]
    #[token("按位取反")]
    BitwiseNot,
    #[token("<<")]
    #[token("左移")]
    LeftShift,
    #[token(">>")]
    #[token("右移")]
    RightShift,
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
    #[token("等于")]
    DoubleEqual,
    #[token("!=")]
    #[token("！=")]
    #[token("不等于")]
    NotEqual,
    #[token("<")]
    #[token("《")]
    #[token("小于")]
    Less,
    #[token("<=")]
    #[token("《=")]
    #[token("小于等于")]
    LessEqual,
    #[token(">")]
    #[token("》")]
    #[token("大于")]
    Greater,
    #[token(">=")]
    #[token("》=")]
    #[token("大于等于")]
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
        ("带上", With), ("with", With), ("匹配", Match), ("match", Match), ("情况", Case), ("case", Case)
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
                            // 普通内置函数
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
                            "排序" | "sorted" => Some("sorted"),
                            "父类" | "super" => Some("super"),
                            "对象" | "object" => Some("object"),
                            "类方法" | "classmethod" => Some("classmethod"),
                            "静态方法" | "staticmethod" => Some("staticmethod"),
                            "属性" | "property" => Some("property"),
                            "异常" | "Exception" => Some("Exception"),
                            // Python魔法方法中英文映射
                            "魔法初始化" | "__init__" => Some("__init__"),
                            "魔法新建" | "__new__" => Some("__new__"),
                            "魔法销毁" | "__del__" => Some("__del__"),
                            "魔法字符串表示" | "__str__" => Some("__str__"),
                            "魔法表达式" | "__repr__" => Some("__repr__"),
                            "魔法字节表示" | "__bytes__" => Some("__bytes__"),
                            "魔法格式化" | "__format__" => Some("__format__"),
                            "魔法等于" | "__eq__" => Some("__eq__"),
                            "魔法不等于" | "__ne__" => Some("__ne__"),
                            "魔法小于" | "__lt__" => Some("__lt__"),
                            "魔法小于等于" | "__le__" => Some("__le__"),
                            "魔法大于" | "__gt__" => Some("__gt__"),
                            "魔法大于等于" | "__ge__" => Some("__ge__"),
                            "魔法哈希" | "__hash__" => Some("__hash__"),
                            "魔法布尔" | "__bool__" => Some("__bool__"),
                            "魔法属性获取" | "__getattr__" => Some("__getattr__"),
                            "魔法属性设置" | "__setattr__" => Some("__setattr__"),
                            "魔法属性删除" | "__delattr__" => Some("__delattr__"),
                            "魔法属性获取项" | "__getitem__" => Some("__getitem__"),
                            "魔法属性设置项" | "__setitem__" => Some("__setitem__"),
                            "魔法属性删除项" | "__delitem__" => Some("__delitem__"),
                            "魔法长度" | "__len__" => Some("__len__"),
                            "魔法可迭代" | "__iter__" => Some("__iter__"),
                            "魔法迭代下一个" | "__next__" => Some("__next__"),
                            "魔法反转" | "__reversed__" => Some("__reversed__"),
                            "魔法包含" | "__contains__" => Some("__contains__"),
                            "魔法加" | "__add__" => Some("__add__"),
                            "魔法减" | "__sub__" => Some("__sub__"),
                            "魔法乘" | "__mul__" => Some("__mul__"),
                            "魔法除" | "__truediv__" => Some("__truediv__"),
                            "魔法地板除" | "__floordiv__" => Some("__floordiv__"),
                            "魔法取余" | "__mod__" => Some("__mod__"),
                            "魔法取幂" | "__pow__" => Some("__pow__"),
                            "魔法左移" | "__lshift__" => Some("__lshift__"),
                            "魔法右移" | "__rshift__" => Some("__rshift__"),
                            "魔法按位与" | "__and__" => Some("__and__"),
                            "魔法按位或" | "__or__" => Some("__or__"),
                            "魔法按位异或" | "__xor__" => Some("__xor__"),
                            "魔法取反" | "__invert__" => Some("__invert__"),
                            "魔法可调用" | "__call__" => Some("__call__"),
                            "魔法进入" | "__enter__" => Some("__enter__"),
                            "魔法退出" | "__exit__" => Some("__exit__"),
                            "魔法拷贝" | "__copy__" => Some("__copy__"),
                            "魔法深拷贝" | "__deepcopy__" => Some("__deepcopy__"),
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
                
                // 处理中文数字转换
                if let Token::ChineseNumber = tok {
                    let converted = chinese_to_digits(slice.clone());
                    // 检查转换后的结果是否包含小数点
                    if converted.contains('.') {
                        tokens.push((Token::Float, converted));
                    } else {
                        tokens.push((Token::Integer, converted));
                    }
                } else {
                    tokens.push((tok.clone(), slice));
                }
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
