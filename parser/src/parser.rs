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
                // 处理 elif/else
                let mut orelse = Vec::new();
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Elif) {
                    orelse.push(parse_stmt(tokens, pos)?);
                } else if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Else) {
                    *pos += 1;
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                        *pos += 1;
                    }
                    while let Some((Token::Newline, _)) = tokens.get(*pos) {
                        *pos += 1;
                    }
                    orelse = parse_block(tokens, pos);
                }
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
        while let Some((tok, opstr)) = tokens.get(*pos) {
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
            // 元组
            Some((Token::LParen, _)) => {
                *pos += 1;
                let mut items = Vec::new();
                while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RParen) {
                    match parse_expr(tokens, pos) {
                        Some(item) => items.push(item),
                        None => break,
                    }
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                        *pos += 1;
                    }
                }
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RParen) {
                    *pos += 1;
                }
                if items.len() == 1 {
                    Some(items.remove(0))
                } else {
                    Some(AstNode::Tuple(items))
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
