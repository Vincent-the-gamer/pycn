use crate::{ast::AstNode, lexer::Token};

// 简单递归下降parser
pub fn parse(tokens: &[(Token, String)]) -> AstNode {
    let mut pos = 0;

    fn parse_block(tokens: &[(Token, String)], pos: &mut usize) -> Vec<AstNode> {
        let mut body = Vec::new();
        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Indent) {
            *pos += 1;
            while *pos < tokens.len() && tokens.get(*pos).map(|t| &t.0) != Some(&Token::Dedent) {
                if let Some(stmt) = parse_stmt(tokens, pos) {
                    body.push(stmt);
                } else {
                    *pos += 1;
                }
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Newline) {
                    *pos += 1;
                }
            }
            if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Dedent) {
                *pos += 1;
            }
        } else {
            if let Some(stmt) = parse_stmt(tokens, pos) {
                body.push(stmt);
            }
            while *pos < tokens.len()
                && tokens.get(*pos).map(|t| &t.0) != Some(&Token::Newline)
                && tokens.get(*pos).map(|t| &t.0) != Some(&Token::Dedent)
            {
                *pos += 1;
            }
            if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Newline) {
                *pos += 1;
            }
        }
        body
    }

    fn parse_stmt(tokens: &[(Token, String)], pos: &mut usize) -> Option<AstNode> {
        match tokens.get(*pos) {
            // 函数定义
            Some((Token::Def, _)) => {
                *pos += 1;
                let name = tokens.get(*pos).and_then(|t| {
                    if let Token::Identifier = t.0 {
                        Some(t.1.clone())
                    } else {
                        None
                    }
                })?;
                *pos += 1;
                if tokens.get(*pos).map(|t| &t.0) != Some(&Token::LParen) {
                    return None;
                }
                *pos += 1;
                let mut params = Vec::new();
                while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RParen) {
                    if let Some((Token::Identifier, pname)) = tokens.get(*pos) {
                        params.push(pname.clone());
                        *pos += 1;
                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                            *pos += 1;
                        }
                    } else {
                        break;
                    }
                }
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RParen) {
                    *pos += 1;
                }
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                    *pos += 1;
                }
                while let Some((Token::Newline, _)) = tokens.get(*pos) {
                    *pos += 1;
                }
                let body = parse_block(tokens, pos);
                Some(AstNode::Def { name, params, body })
            }
            // if/elif/else
            Some((Token::If, _)) => {
                *pos += 1;
                let cond = parse_expr(tokens, pos)?;
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                    *pos += 1;
                }
                while let Some((Token::Newline, _)) = tokens.get(*pos) {
                    *pos += 1;
                }
                let body = parse_block(tokens, pos);
                // 支持链式 elif/else，保证 orelse 只为 else 或下一个 if
                fn parse_elif_else(tokens: &[(Token, String)], pos: &mut usize) -> Vec<AstNode> {
                    if let Some((Token::Elif, _)) = tokens.get(*pos) {
                        *pos += 1;
                        let elif_cond = if let Some(c) = parse_expr(tokens, pos) { c } else { return vec![] };
                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                            *pos += 1;
                        }
                        while let Some((Token::Newline, _)) = tokens.get(*pos) {
                            *pos += 1;
                        }
                        let elif_body = parse_block(tokens, pos);
                        let elif_orelse = parse_elif_else(tokens, pos);
                        return vec![AstNode::If {
                            cond: Box::new(elif_cond),
                            body: elif_body,
                            orelse: elif_orelse,
                        }];
                    } else if let Some((Token::Else, _)) = tokens.get(*pos) {
                        *pos += 1;
                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                            *pos += 1;
                        }
                        while let Some((Token::Newline, _)) = tokens.get(*pos) {
                            *pos += 1;
                        }
                        return parse_block(tokens, pos);
                    } else {
                        return Vec::new();
                    }
                }
                let orelse = parse_elif_else(tokens, pos);
                Some(AstNode::If {
                    cond: Box::new(cond),
                    body,
                    orelse,
                })
            }
            // for
            Some((Token::For, _)) => {
                *pos += 1;
                let var = match tokens.get(*pos) {
                    Some((Token::Identifier, name)) => name.clone(),
                    _ => return None,
                };
                *pos += 1;
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::In) {
                    *pos += 1;
                }
                let iter = parse_expr(tokens, pos)?;
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                    *pos += 1;
                }
                while let Some((Token::Newline, _)) = tokens.get(*pos) {
                    *pos += 1;
                }
                let body = parse_block(tokens, pos);
                Some(AstNode::For {
                    var,
                    iter: Box::new(iter),
                    body,
                })
            }
            // return
            Some((Token::Return, _)) => {
                *pos += 1;
                // 支持 return/return expr
                let value = if let Some((Token::Newline, _)) = tokens.get(*pos) {
                    None
                } else {
                    Some(Box::new(parse_expr(tokens, pos)?))
                };
                Some(AstNode::Return(value))
            }
            // break/continue/pass
            Some((Token::Break, _)) => {
                *pos += 1;
                Some(AstNode::Break)
            }
            Some((Token::Continue, _)) => {
                *pos += 1;
                Some(AstNode::Continue)
            }
            Some((Token::Pass, _)) => {
                *pos += 1;
                Some(AstNode::Pass)
            }
            // 赋值
            Some((Token::Identifier, name))
                if tokens.get(*pos + 1).map(|t| &t.0) == Some(&Token::Equal) =>
            {
                let name = name.clone();
                *pos += 2;
                let value = parse_expr(tokens, pos)?;
                Some(AstNode::Assign {
                    name,
                    value: Box::new(value),
                })
            }
            // 表达式语句
            _ => parse_expr(tokens, pos),
        }
    }

    // 优先级递归下降表达式解析
    fn parse_expr(tokens: &[(Token, String)], pos: &mut usize) -> Option<AstNode> {
        parse_or(tokens, pos)
    }
    fn parse_or(tokens: &[(Token, String)], pos: &mut usize) -> Option<AstNode> {
        let mut node = parse_and(tokens, pos)?;
        while let Some((Token::Or, _)) = tokens.get(*pos) {
            *pos += 1;
            let right = parse_and(tokens, pos)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: "or".to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_and(tokens: &[(Token, String)], pos: &mut usize) -> Option<AstNode> {
        let mut node = parse_cmp(tokens, pos)?;
        while let Some((Token::And, _)) = tokens.get(*pos) {
            *pos += 1;
            let right = parse_cmp(tokens, pos)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: "and".to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_cmp(tokens: &[(Token, String)], pos: &mut usize) -> Option<AstNode> {
        let mut node = parse_add(tokens, pos)?;
        while let Some((tok, _)) = tokens.get(*pos) {
            let op = match tok {
                Token::DoubleEqual => "==",
                Token::NotEqual => "!=",
                Token::Less => "<",
                Token::LessEqual => "<=",
                Token::Greater => ">",
                Token::GreaterEqual => ">=",
                _ => break,
            };
            *pos += 1;
            let right = parse_add(tokens, pos)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_add(tokens: &[(Token, String)], pos: &mut usize) -> Option<AstNode> {
        let mut node = parse_mul(tokens, pos)?;
        while let Some((tok, _)) = tokens.get(*pos) {
            let op = match tok {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => break,
            };
            *pos += 1;
            let right = parse_mul(tokens, pos)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_mul(tokens: &[(Token, String)], pos: &mut usize) -> Option<AstNode> {
        let mut node = parse_unary(tokens, pos)?;
        while let Some((tok, _)) = tokens.get(*pos) {
            let op = match tok {
                Token::Star => "*",
                Token::Slash => "/",
                Token::Mod => "%",
                Token::FloorDiv => "//",
                Token::Pow => "**",
                _ => break,
            };
            *pos += 1;
            let right = parse_unary(tokens, pos)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_unary(tokens: &[(Token, String)], pos: &mut usize) -> Option<AstNode> {
        if let Some((Token::Not, _)) = tokens.get(*pos) {
            *pos += 1;
            let expr = parse_unary(tokens, pos)?;
            Some(AstNode::UnaryOp {
                op: "not".to_string(),
                expr: Box::new(expr),
            })
        } else {
            parse_atom(tokens, pos)
        }
    }
    fn parse_atom(tokens: &[(Token, String)], pos: &mut usize) -> Option<AstNode> {
        // 支持属性、索引、嵌套调用
        fn parse_postfix(mut node: AstNode, tokens: &[(Token, String)], pos: &mut usize) -> AstNode {
            loop {
                match tokens.get(*pos) {
                    // 暂不支持 Token::Dot，直接跳过属性链式调用
                    // 你可以在 lexer.rs 增加 Dot 词法支持后再解开此分支
                    // Some((Token::Dot, _)) => { ... }
                    Some((Token::LBracket, _)) => {
                        *pos += 1;
                        let index = match parse_expr(tokens, pos) {
                            Some(idx) => idx,
                            None => break,
                        };
                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RBracket) {
                            *pos += 1;
                        }
                        node = AstNode::Index {
                            value: Box::new(node),
                            index: Box::new(index),
                        };
                    }
                    Some((Token::LParen, _)) => {
                        *pos += 1;
                        let mut args = Vec::new();
                        while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RParen) {
                            match parse_expr(tokens, pos) {
                                Some(arg) => args.push(arg),
                                None => break,
                            }
                            if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                                *pos += 1;
                            }
                        }
                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RParen) {
                            *pos += 1;
                        }
                        node = AstNode::Call {
                            func: Box::new(node),
                            args,
                        };
                    }
                    _ => break,
                }
            }
            node
        }
        match tokens.get(*pos) {
            Some((Token::BuiltInFunc(name), _)) => {
                let node = AstNode::Identifier(name.clone());
                *pos += 1;
                Some(parse_postfix(node, tokens, pos))
            }
            Some((Token::Print, _)) => {
                let node = AstNode::Identifier("print".to_string());
                *pos += 1;
                Some(parse_postfix(node, tokens, pos))
            }
            Some((Token::Identifier, name)) => {
                let node = AstNode::Identifier(name.clone());
                *pos += 1;
                Some(parse_postfix(node, tokens, pos))
            }
            Some((Token::Integer, n)) => {
                *pos += 1;
                Some(AstNode::Integer(n.parse().ok()?))
            }
            Some((Token::Float, n)) => {
                *pos += 1;
                Some(AstNode::Float(n.parse().ok()?))
            }
            Some((Token::String, s)) => {
                *pos += 1;
                Some(AstNode::String(s.clone()))
            }
            Some((Token::True, _)) => {
                *pos += 1;
                Some(AstNode::Bool(true))
            }
            Some((Token::False, _)) => {
                *pos += 1;
                Some(AstNode::Bool(false))
            }
            Some((Token::None, _)) => {
                *pos += 1;
                Some(AstNode::None)
            }
            // 列表
            Some((Token::LBracket, _)) => {
                *pos += 1;
                let mut items = Vec::new();
                while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RBracket) {
                    match parse_expr(tokens, pos) {
                        Some(item) => items.push(item),
                        None => break,
                    }
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                        *pos += 1;
                    }
                }
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RBracket) {
                    *pos += 1;
                }
                Some(AstNode::List(items))
            }
            // 字典/集合
            Some((Token::LBrace, _)) => {
                *pos += 1;
                let mut pairs = Vec::new();
                let mut is_dict = false;
                while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RBrace) {
                    let key = match parse_expr(tokens, pos) {
                        Some(k) => k,
                        None => break,
                    };
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                        is_dict = true;
                        *pos += 1;
                        let value = match parse_expr(tokens, pos) {
                            Some(v) => v,
                            None => break,
                        };
                        pairs.push((key, value));
                    } else {
                        pairs.push((key, AstNode::None));
                    }
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                        *pos += 1;
                    }
                }
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RBrace) {
                    *pos += 1;
                }
                if is_dict {
                    Some(AstNode::Dict(pairs))
                } else {
                    Some(AstNode::Set(pairs.into_iter().map(|(k, _)| k).collect()))
                }
            }
            // 括号表达式/元组
            Some((Token::LParen, _)) => {
                *pos += 1;
                let expr = parse_expr(tokens, pos);
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                    // 逗号，说明是元组
                    let mut items = vec![];
                    if let Some(e) = expr { items.push(e); }
                    while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RParen) {
                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                            *pos += 1;
                        }
                        match parse_expr(tokens, pos) {
                            Some(item) => items.push(item),
                            None => break,
                        }
                    }
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RParen) {
                        *pos += 1;
                    }
                    Some(AstNode::Tuple(items))
                } else {
                    // 单一表达式，且无逗号，保留括号
                    let inner = expr?;
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RParen) {
                        *pos += 1;
                    }
                    Some(AstNode::Paren(Box::new(inner)))
                }
            }
            _ => None,
        }
    }

    let mut stmts = Vec::new();
    while pos < tokens.len() {
        if let Some(stmt) = parse_stmt(tokens, &mut pos) {
            stmts.push(stmt);
        } else {
            pos += 1;
        }
    }
    AstNode::Program(stmts)
}


// AstNode 转 Python 源代码
pub fn ast_to_python(node: &AstNode, indent: usize) -> String {
    let indent_str = |n| "    ".repeat(n);
    match node {
        AstNode::Program(stmts) => stmts.iter().map(|s| ast_to_python(s, indent)).collect::<Vec<_>>().join("\n"),
        AstNode::Def { name, params, body } => {
            let params_str = params.join(", ");
            let body_str = if body.is_empty() {
                format!("{}pass", indent_str(indent + 1))
            } else {
                body.iter().map(|s| ast_to_python(s, indent + 1)).collect::<Vec<_>>().join("\n")
            };
            format!("{}def {}({}):\n{}", indent_str(indent), name, params_str, body_str)
        }
        AstNode::If { cond, body, orelse } => {
            // 条件不加括号
            let cond_str = match &**cond {
                AstNode::Paren(inner) => ast_to_python(inner, 0), // 去掉条件表达式外层括号
                _ => ast_to_python(cond, 0)
            };
            let body_str = if body.is_empty() {
                format!("{}pass", indent_str(indent + 1))
            } else {
                body.iter().map(|s| ast_to_python(s, indent + 1)).collect::<Vec<_>>().join("\n")
            };
            let mut result = format!("{}if {}:\n{}", indent_str(indent), cond_str, body_str);
            // 平级 elif/else 输出
            let mut orelse_ref = orelse;
            while !orelse_ref.is_empty() {
                if orelse_ref.len() == 1 {
                    if let AstNode::If { cond: elif_cond, body: elif_body, orelse: elif_orelse } = &orelse_ref[0] {
                        let elif_cond_str = match &**elif_cond {
                            AstNode::Paren(inner) => ast_to_python(inner, 0),
                            _ => ast_to_python(elif_cond, 0)
                        };
                        let elif_body_str = if elif_body.is_empty() {
                            format!("{}pass", indent_str(indent + 1))
                        } else {
                            elif_body.iter().map(|s| ast_to_python(s, indent + 1)).collect::<Vec<_>>().join("\n")
                        };
                        result.push_str(&format!("\n{}elif {}:\n{}", indent_str(indent), elif_cond_str, elif_body_str));
                        orelse_ref = elif_orelse;
                        continue;
                    }
                }
                // else 分支
                let else_str = orelse_ref.iter().map(|s| ast_to_python(s, indent + 1)).collect::<Vec<_>>().join("\n");
                result.push_str(&format!("\n{}else:\n{}", indent_str(indent), else_str));
                break;
            }
            result
        }
        AstNode::For { var, iter, body } => {
            let iter_str = ast_to_python(iter, 0);
            let body_str = if body.is_empty() {
                format!("{}pass", indent_str(indent + 1))
            } else {
                body.iter().map(|s| ast_to_python(s, indent + 1)).collect::<Vec<_>>().join("\n")
            };
            format!("{}for {} in {}:\n{}", indent_str(indent), var, iter_str, body_str)
        }
        AstNode::Return(val) => {
            if let Some(expr) = val {
                format!("{}return {}", indent_str(indent), ast_to_python(expr, 0))
            } else {
                format!("{}return", indent_str(indent))
            }
        }
        AstNode::Break => format!("{}break", indent_str(indent)),
        AstNode::Continue => format!("{}continue", indent_str(indent)),
        AstNode::Pass => format!("{}pass", indent_str(indent)),
        AstNode::Assign { name, value } => {
            format!("{}{} = {}", indent_str(indent), name, ast_to_python(value, 0))
        }
        AstNode::BinaryOp { left, op, right } => {
            // print('xxx') % 变量 这种结构，输出 print('xxx' % 变量)
            if let AstNode::Call { func, args } = &**left {
                let func_name = ast_to_python(func, 0);
                if func_name == "print" && args.len() == 1 {
                    let str_expr = ast_to_python(&args[0], 0);
                    let right_str = ast_to_python(right, 0);
                    return format!("print({} % {})", str_expr, right_str);
                }
            }
            format!("{} {} {}", ast_to_python(left, 0), op, ast_to_python(right, 0))
        }
        AstNode::Paren(inner) => {
            format!("({})", ast_to_python(inner, 0))
        }
        AstNode::UnaryOp { op, expr } => {
            format!("({} {})", op, ast_to_python(expr, 0))
        }
        AstNode::Identifier(name) => name.clone(),
        AstNode::Integer(n) => n.to_string(),
        AstNode::Float(f) => f.to_string(),
        AstNode::String(s) => s.to_string().replace('“', "\"").replace('”', "\""),
        AstNode::Bool(b) => if *b { "True".to_string() } else { "False".to_string() },
        AstNode::None => "None".to_string(),
        AstNode::List(items) => {
            let items_str = items.iter().map(|i| ast_to_python(i, 0)).collect::<Vec<_>>().join(", ");
            format!("[{}]", items_str)
        }
        AstNode::Tuple(items) => {
            let items_str = items.iter().map(|i| ast_to_python(i, 0)).collect::<Vec<_>>().join(", ");
            if items.len() == 1 {
                format!("({},)", items_str)
            } else {
                format!("({})", items_str)
            }
        }
        AstNode::Set(items) => {
            let items_str = items.iter().map(|i| ast_to_python(i, 0)).collect::<Vec<_>>().join(", ");
            format!("{{{}}}", items_str)
        }
        AstNode::Dict(pairs) => {
            let pairs_str = pairs.iter().map(|(k, v)| format!("{}: {}", ast_to_python(k, 0), ast_to_python(v, 0))).collect::<Vec<_>>().join(", ");
            format!("{{{}}}", pairs_str)
        }
        AstNode::Call { func, args } => {
            let func_name = ast_to_python(func, 0);
            if func_name == "print" && !args.is_empty() {
                if args.len() == 1 {
                    if let AstNode::BinaryOp { left, op, right } = &args[0] {
                        if op == "%" {
                            let left_str = ast_to_python(left, 0);
                            let right_str = ast_to_python(right, 0);
                            return format!("{}print({} % {})", indent_str(indent), left_str, right_str);
                        }
                    }
                }
            }
            // 其它情况，普通函数调用
            let args_str = args.iter().map(|a| ast_to_python(a, 0)).collect::<Vec<_>>().join(", ");
            format!("{}{}({})", indent_str(indent), func_name, args_str)
        }
        AstNode::Index { value, index } => {
            format!("{}[{}]", ast_to_python(value, 0), ast_to_python(index, 0))
        }
        AstNode::Range { start, end, step } => {
            let start_str = ast_to_python(start.as_ref(), 0);
            let end_str = ast_to_python(end.as_ref(), 0);
            if let Some(n) = step {
                let step_str = ast_to_python(n.as_ref(), 0);
                format!("{}:{}:{}", start_str, end_str, step_str)
            } else {
                format!("{}:{}", start_str, end_str)
            }
        }
        AstNode::Attribute { value, attr } => {
            format!("{}.{}", ast_to_python(value, 0), attr)
        }
    }
}