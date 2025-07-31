use crate::{ast::AstNode, lexer::Token};

// 简单递归下降parser
pub fn parse(tokens: &[(Token, String)]) -> AstNode {
    let mut pos = 0;
    // 收集所有类名
    let mut class_names = std::collections::HashSet::new();
    let mut scan_pos = 0;
    while scan_pos < tokens.len() {
        if let Some((Token::Class, _)) = tokens.get(scan_pos) {
            if let Some((Token::Identifier, cname)) = tokens.get(scan_pos + 1) {
                class_names.insert(cname.clone());
            }
        }
        scan_pos += 1;
    }

    fn parse_block(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Vec<AstNode> {
        let mut body = Vec::new();
        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Indent) {
            *pos += 1;
            while *pos < tokens.len() && tokens.get(*pos).map(|t| &t.0) != Some(&Token::Dedent) {
                if let Some(stmt) = parse_stmt(tokens, pos, class_names) {
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
            if let Some(stmt) = parse_stmt(tokens, pos, class_names) {
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

    fn parse_stmt(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        // 导入 xxx [作为 yyy]
        if let Some((Token::Import, _)) = tokens.get(*pos) {
            *pos += 1;
            let module = if let Some((Token::Identifier, name)) = tokens.get(*pos) {
                name.clone()
            } else {
                return None;
            };
            *pos += 1;
            let alias = if let Some((Token::As, _)) = tokens.get(*pos) {
                *pos += 1;
                if let Some((Token::Identifier, alias_name)) = tokens.get(*pos) {
                    let a = Some(alias_name.clone());
                    *pos += 1;
                    a
                } else {
                    return None;
                }
            } else {
                None
            };
            return Some(AstNode::Import { module, alias });
        }
        // 从 xxx 导入 yyy [作为 zzz]
        if let Some((Token::From, _)) = tokens.get(*pos) {
            *pos += 1;
            let module = if let Some((Token::Identifier, name)) = tokens.get(*pos) {
                name.clone()
            } else {
                return None;
            };
            *pos += 1;
            if tokens.get(*pos).map(|t| &t.0) != Some(&Token::Import) {
                return None;
            }
            *pos += 1;
            let mut names = Vec::new();
            loop {
                if let Some((Token::Identifier, name)) = tokens.get(*pos) {
                    let mut alias = None;
                    *pos += 1;
                    if let Some((Token::As, _)) = tokens.get(*pos) {
                        *pos += 1;
                        if let Some((Token::Identifier, alias_name)) = tokens.get(*pos) {
                            alias = Some(alias_name.clone());
                            *pos += 1;
                        } else {
                            return None;
                        }
                    }
                    names.push((name.clone(), alias));
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                        *pos += 1;
                        continue;
                    }
                }
                break;
            }
            return Some(AstNode::ImportFrom { module, names });
        }
        // 递归elif/else辅助函数，移到外部
        fn parse_elif_else(
            tokens: &[(Token, String)],
            pos: &mut usize,
            class_names: &std::collections::HashSet<String>,
        ) -> Vec<AstNode> {
            if let Some((Token::Elif, _)) = tokens.get(*pos) {
                *pos += 1;
                let elif_cond = if let Some(c) = parse_expr(tokens, pos, class_names) {
                    c
                } else {
                    return vec![];
                };
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                    *pos += 1;
                }
                while let Some((Token::Newline, _)) = tokens.get(*pos) {
                    *pos += 1;
                }
                let elif_body = parse_block(tokens, pos, class_names);
                let elif_orelse = parse_elif_else(tokens, pos, class_names);
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
                return parse_block(tokens, pos, class_names);
            } else {
                return Vec::new();
            }
        }
        match tokens.get(*pos) {
            // 类定义
            Some((Token::Class, _)) => {
                *pos += 1;
                let name = tokens.get(*pos).and_then(|t| {
                    if let Token::Identifier = t.0 {
                        Some(t.1.clone())
                    } else {
                        None
                    }
                })?;
                *pos += 1;
                
                // 解析继承的基类
                let mut bases = Vec::new();
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::LParen) {
                    *pos += 1;
                    while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RParen) {
                        match tokens.get(*pos) {
                            Some((Token::Identifier, base_name)) => {
                                bases.push(base_name.clone());
                                *pos += 1;
                            }
                            Some((Token::BuiltInFunc(func_name), _)) => {
                                bases.push(func_name.clone());
                                *pos += 1;
                            }
                            _ => break,
                        }
                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                            *pos += 1;
                        }
                    }
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RParen) {
                        *pos += 1;
                    }
                }
                
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                    *pos += 1;
                }
                while let Some((Token::Newline, _)) = tokens.get(*pos) {
                    *pos += 1;
                }
                let body = parse_block(tokens, pos, class_names);
                Some(AstNode::Class { name, bases, body })
            }
            // 函数定义
            Some((Token::Def, _)) => {
                *pos += 1;
                let name = tokens.get(*pos).and_then(|t| {
                    match &t.0 {
                        Token::Identifier => Some(t.1.clone()),
                        Token::BuiltInFunc(func_name) => Some(func_name.clone()),
                        _ => None
                    }
                })?;
                *pos += 1;
                if tokens.get(*pos).map(|t| &t.0) != Some(&Token::LParen) {
                    return None;
                }
                *pos += 1;
                let mut params = Vec::new();
                while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RParen) {
                    match tokens.get(*pos) {
                        Some((Token::Identifier, pname)) => {
                            params.push(pname.clone());
                            *pos += 1;
                        }
                        Some((Token::BuiltInFunc(func_name), _)) => {
                            // 在函数参数上下文中，内置函数名应该被当作普通参数名
                            params.push(func_name.clone());
                            *pos += 1;
                        }
                        _ => break,
                    }
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                        *pos += 1;
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
                let body = parse_block(tokens, pos, class_names);
                Some(AstNode::Def { name, params, body })
            }
            // if/elif/else
            Some((Token::If, _)) => {
                *pos += 1;
                let cond = parse_expr(tokens, pos, class_names)?;
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                    *pos += 1;
                }
                while let Some((Token::Newline, _)) = tokens.get(*pos) {
                    *pos += 1;
                }
                let body = parse_block(tokens, pos, class_names);
                // 支持链式 elif/else，保证 orelse 只为 else 或下一个 if
                let orelse = parse_elif_else(tokens, pos, class_names);
                Some(AstNode::If {
                    cond: Box::new(cond),
                    body,
                    orelse,
                })
            }
            // for/迭代语法，支持多个变量
            Some((Token::For, _)) => {
                *pos += 1;
                // 解析变量列表（支持 for 索引值,水果 in enumerate(水果)）
                let mut vars = Vec::new();
                while let Some((Token::Identifier, vname)) = tokens.get(*pos) {
                    vars.push(vname.clone());
                    *pos += 1;
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                        *pos += 1;
                    } else {
                        break;
                    }
                }
                // 检查“在”关键字
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::In) {
                    *pos += 1;
                } else {
                    return None;
                }
                let iter = parse_expr(tokens, pos, class_names)?;
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                    *pos += 1;
                }
                while let Some((Token::Newline, _)) = tokens.get(*pos) {
                    *pos += 1;
                }
                let body = parse_block(tokens, pos, class_names);
                // 多变量 for 支持
                if vars.len() == 1 {
                    Some(AstNode::For {
                        var: vars[0].clone(),
                        iter: Box::new(iter),
                        body,
                    })
                } else {
                    // 多变量 for，变量用逗号连接
                    Some(AstNode::For {
                        var: vars.join(", "),
                        iter: Box::new(iter),
                        body,
                    })
                }
            }
            // return
            Some((Token::Return, _)) => {
                *pos += 1;
                // 支持 return/return expr
                let value = if let Some((Token::Newline, _)) = tokens.get(*pos) {
                    None
                } else {
                    Some(Box::new(parse_expr(tokens, pos, class_names)?))
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
            // 赋值 (支持 obj.attr = value 和 name = value 和多变量赋值)
            _ => {
                // 先尝试解析表达式，然后检查是否是赋值
                let start_pos = *pos;
                
                // 尝试解析多变量赋值（例如：甲，乙 赋值为 乙，甲）
                let mut vars = Vec::new();
                let mut temp_pos = *pos;
                
                // 收集逗号分隔的标识符
                while let Some((Token::Identifier, name)) = tokens.get(temp_pos) {
                    vars.push(name.clone());
                    temp_pos += 1;
                    
                    if tokens.get(temp_pos).map(|t| &t.0) == Some(&Token::Comma) {
                        temp_pos += 1;
                        continue;
                    } else {
                        break;
                    }
                }
                
                // 如果找到多个变量且下一个token是赋值符号，则处理多变量赋值
                if vars.len() > 1 && tokens.get(temp_pos).map(|t| &t.0) == Some(&Token::Equal) {
                    *pos = temp_pos + 1; // 跳过等号
                    
                    // 解析右边的值列表
                    let mut values = Vec::new();
                    
                    // 先解析第一个值
                    if let Some(first_val) = parse_expr(tokens, pos, class_names) {
                        values.push(first_val);
                        
                        // 解析后续的逗号分隔的值
                        while tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                            *pos += 1;
                            if let Some(val) = parse_expr(tokens, pos, class_names) {
                                values.push(val);
                            } else {
                                break;
                            }
                        }
                    }
                    
                    return Some(AstNode::MultiAssign { names: vars, values });
                }
                
                // 否则按原来的逻辑处理
                if let Some(expr) = parse_expr(tokens, pos, class_names) {
                    // 检查是否是赋值操作
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Equal) {
                        *pos += 1;
                        let value = parse_expr(tokens, pos, class_names)?;
                        
                        match expr {
                            // 简单变量赋值
                            AstNode::Identifier(name) => {
                                Some(AstNode::Assign {
                                    name,
                                    value: Box::new(value),
                                })
                            }
                            // 属性赋值
                            AstNode::Attribute { value: object, attr } => {
                                Some(AstNode::AttributeAssign {
                                    object,
                                    attr,
                                    value: Box::new(value),
                                })
                            }
                            // 其他情况，恢复原来的表达式
                            _ => {
                                *pos = start_pos;
                                Some(expr)
                            }
                        }
                    } else {
                        // 不是赋值，就是普通表达式
                        Some(expr)
                    }
                } else {
                    None
                }
            }
        }
    }

    // 优先级递归下降表达式解析
    // ====== 递归表达式相关函数全部移到顶层，带class_names参数 =====
    fn parse_expr(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        parse_or(tokens, pos, class_names)
    }
    fn parse_or(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        let mut node = parse_and(tokens, pos, class_names)?;
        while let Some((Token::Or, _)) = tokens.get(*pos) {
            *pos += 1;
            let right = parse_and(tokens, pos, class_names)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: "or".to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_and(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        let mut node = parse_cmp(tokens, pos, class_names)?;
        while let Some((Token::And, _)) = tokens.get(*pos) {
            *pos += 1;
            let right = parse_cmp(tokens, pos, class_names)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: "and".to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_cmp(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        let mut node = parse_bitwise_or(tokens, pos, class_names)?;
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
            let right = parse_bitwise_or(tokens, pos, class_names)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_bitwise_or(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        let mut node = parse_bitwise_xor(tokens, pos, class_names)?;
        while let Some((Token::BitwiseOr, _)) = tokens.get(*pos) {
            *pos += 1;
            let right = parse_bitwise_xor(tokens, pos, class_names)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: "|".to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_bitwise_xor(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        let mut node = parse_bitwise_and(tokens, pos, class_names)?;
        while let Some((Token::BitwiseXor, _)) = tokens.get(*pos) {
            *pos += 1;
            let right = parse_bitwise_and(tokens, pos, class_names)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: "^".to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_bitwise_and(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        let mut node = parse_shift(tokens, pos, class_names)?;
        while let Some((Token::BitwiseAnd, _)) = tokens.get(*pos) {
            *pos += 1;
            let right = parse_shift(tokens, pos, class_names)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: "&".to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_shift(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        let mut node = parse_add(tokens, pos, class_names)?;
        while let Some((tok, _)) = tokens.get(*pos) {
            let op = match tok {
                Token::LeftShift => "<<",
                Token::RightShift => ">>",
                _ => break,
            };
            *pos += 1;
            let right = parse_add(tokens, pos, class_names)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_add(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        let mut node = parse_mul(tokens, pos, class_names)?;
        while let Some((tok, _)) = tokens.get(*pos) {
            let op = match tok {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => break,
            };
            *pos += 1;
            let right = parse_mul(tokens, pos, class_names)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_mul(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        let mut node = parse_unary(tokens, pos, class_names)?;
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
            let right = parse_unary(tokens, pos, class_names)?;
            node = AstNode::BinaryOp {
                left: Box::new(node),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_unary(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        if let Some((tok, _)) = tokens.get(*pos) {
            let op = match tok {
                Token::Not => "not",
                Token::BitwiseNot => "~",
                _ => return parse_atom(tokens, pos, class_names),
            };
            *pos += 1;
            let expr = parse_unary(tokens, pos, class_names)?;
            Some(AstNode::UnaryOp {
                op: op.to_string(),
                expr: Box::new(expr),
            })
        } else {
            parse_atom(tokens, pos, class_names)
        }
    }
    fn parse_atom(
        tokens: &[(Token, String)],
        pos: &mut usize,
        class_names: &std::collections::HashSet<String>,
    ) -> Option<AstNode> {
        fn parse_postfix(
            mut node: AstNode,
            tokens: &[(Token, String)],
            pos: &mut usize,
            class_names: &std::collections::HashSet<String>,
        ) -> AstNode {
            loop {
                match tokens.get(*pos) {
                    Some((Token::Dot, _)) => {
                        *pos += 1;
                        if let Some((Token::Identifier, attr)) = tokens.get(*pos) {
                            node = AstNode::Attribute {
                                value: Box::new(node),
                                attr: attr.clone(),
                            };
                            *pos += 1;
                        } else {
                            break;
                        }
                    }
                    Some((Token::LBracket, _)) => {
                        *pos += 1;
                        // 检查是否为切片语法 (start:end 或 start:end:step)
                        let mut start = None;
                        let mut end = None;
                        let mut step = None;
                        let mut is_slice = false;

                        // 解析第一个表达式（可能是start或者普通index）
                        if tokens.get(*pos).map(|t| &t.0) != Some(&Token::Colon) {
                            if let Some(expr) = parse_expr(tokens, pos, class_names) {
                                start = Some(expr);
                            }
                        }

                        // 检查是否有冒号
                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                            is_slice = true;
                            *pos += 1; // 跳过冒号
                            
                            // 解析end部分（可选）
                            if tokens.get(*pos).map(|t| &t.0) != Some(&Token::Colon) 
                                && tokens.get(*pos).map(|t| &t.0) != Some(&Token::RBracket) {
                                if let Some(expr) = parse_expr(tokens, pos, class_names) {
                                    end = Some(expr);
                                }
                            }

                            // 检查是否有第二个冒号（step）
                            if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                                *pos += 1; // 跳过第二个冒号
                                if tokens.get(*pos).map(|t| &t.0) != Some(&Token::RBracket) {
                                    if let Some(expr) = parse_expr(tokens, pos, class_names) {
                                        step = Some(expr);
                                    }
                                }
                            }
                        }

                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RBracket) {
                            *pos += 1;
                        }

                        if is_slice {
                            node = AstNode::Slice {
                                value: Box::new(node),
                                start: start.map(Box::new),
                                end: end.map(Box::new),
                                step: step.map(Box::new),
                            };
                        } else if let Some(index) = start {
                            node = AstNode::Index {
                                value: Box::new(node),
                                index: Box::new(index),
                            };
                        }
                    }
                    Some((Token::LParen, _)) => {
                        *pos += 1;
                        let mut args = Vec::new();
                        while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RParen) {
                            match parse_expr(tokens, pos, class_names) {
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
                        if let AstNode::Identifier(ref name) = node {
                            if class_names.contains(name) {
                                node = AstNode::Instance {
                                    class: name.clone(),
                                    args,
                                };
                                continue;
                            }
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
            Some((Token::Lambda, _)) => {
                *pos += 1;
                // 解析参数列表
                let mut params = Vec::new();
                while let Some((Token::Identifier, param)) = tokens.get(*pos) {
                    params.push(param.clone());
                    *pos += 1;
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                        *pos += 1;
                    } else {
                        break;
                    }
                }
                // 期望冒号
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                    *pos += 1;
                } else {
                    return None;
                }
                // 解析lambda体（表达式）
                let body = parse_expr(tokens, pos, class_names)?;
                Some(AstNode::Lambda {
                    params,
                    body: Box::new(body),
                })
            }
            Some((Token::BuiltInFunc(name), _)) => {
                let node = AstNode::Identifier(name.clone());
                *pos += 1;
                Some(parse_postfix(node, tokens, pos, class_names))
            }
            Some((Token::Identifier, name)) => {
                let node = AstNode::Identifier(name.clone());
                *pos += 1;
                Some(parse_postfix(node, tokens, pos, class_names))
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
            Some((Token::LBracket, _)) => {
                *pos += 1;
                let mut items = Vec::new();
                while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RBracket) {
                    match parse_expr(tokens, pos, class_names) {
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
            Some((Token::LBrace, _)) => {
                *pos += 1;
                let mut pairs = Vec::new();
                let mut is_dict = false;
                while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RBrace) {
                    let key = match parse_expr(tokens, pos, class_names) {
                        Some(k) => k,
                        None => break,
                    };
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Colon) {
                        is_dict = true;
                        *pos += 1;
                        let value = match parse_expr(tokens, pos, class_names) {
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
            Some((Token::LParen, _)) => {
                *pos += 1;
                let expr = parse_expr(tokens, pos, class_names);
                if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                    let mut items = vec![];
                    if let Some(e) = expr {
                        items.push(e);
                    }
                    while tokens.get(*pos).map(|t| &t.0) != Some(&Token::RParen) {
                        if tokens.get(*pos).map(|t| &t.0) == Some(&Token::Comma) {
                            *pos += 1;
                        }
                        match parse_expr(tokens, pos, class_names) {
                            Some(item) => items.push(item),
                            None => break,
                        }
                    }
                    if tokens.get(*pos).map(|t| &t.0) == Some(&Token::RParen) {
                        *pos += 1;
                    }
                    Some(AstNode::Tuple(items))
                } else {
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
        if let Some(stmt) = parse_stmt(tokens, &mut pos, &class_names) {
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
        AstNode::Instance { class, args } => {
            let args_str = args
                .iter()
                .map(|a| ast_to_python(a, 0))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}{}({})", indent_str(indent), class, args_str)
        }
        AstNode::Class { name, bases, body } => {
            let bases_str = if bases.is_empty() {
                "".to_string()
            } else {
                format!("({})", bases.join(", "))
            };
            let body_str = if body.is_empty() {
                format!("{}pass", indent_str(indent + 1))
            } else {
                body.iter()
                    .map(|s| ast_to_python(s, indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            format!(
                "{}class {}{}:\n{}",
                indent_str(indent),
                name,
                bases_str,
                body_str
            )
        }
        AstNode::Program(stmts) => stmts
            .iter()
            .map(|s| ast_to_python(s, indent))
            .collect::<Vec<_>>()
            .join("\n"),
        AstNode::Def { name, params, body } => {
            let params_str = params.join(", ");
            let body_str = if body.is_empty() {
                format!("{}pass", indent_str(indent + 1))
            } else {
                body.iter()
                    .map(|s| ast_to_python(s, indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            format!(
                "{}def {}({}):\n{}",
                indent_str(indent),
                name,
                params_str,
                body_str
            )
        }
        AstNode::If { cond, body, orelse } => {
            // 条件不加括号
            let cond_str = match &**cond {
                AstNode::Paren(inner) => ast_to_python(inner, 0), // 去掉条件表达式外层括号
                _ => ast_to_python(cond, 0),
            };
            let body_str = if body.is_empty() {
                format!("{}pass", indent_str(indent + 1))
            } else {
                body.iter()
                    .map(|s| ast_to_python(s, indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            let mut result = format!("{}if {}:\n{}", indent_str(indent), cond_str, body_str);
            // 平级 elif/else 输出
            let mut orelse_ref = orelse;
            while !orelse_ref.is_empty() {
                if orelse_ref.len() == 1 {
                    if let AstNode::If {
                        cond: elif_cond,
                        body: elif_body,
                        orelse: elif_orelse,
                    } = &orelse_ref[0]
                    {
                        let elif_cond_str = match &**elif_cond {
                            AstNode::Paren(inner) => ast_to_python(inner, 0),
                            _ => ast_to_python(elif_cond, 0),
                        };
                        let elif_body_str = if elif_body.is_empty() {
                            format!("{}pass", indent_str(indent + 1))
                        } else {
                            elif_body
                                .iter()
                                .map(|s| ast_to_python(s, indent + 1))
                                .collect::<Vec<_>>()
                                .join("\n")
                        };
                        result.push_str(&format!(
                            "\n{}elif {}:\n{}",
                            indent_str(indent),
                            elif_cond_str,
                            elif_body_str
                        ));
                        orelse_ref = elif_orelse;
                        continue;
                    }
                }
                // else 分支
                let else_str = orelse_ref
                    .iter()
                    .map(|s| ast_to_python(s, indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n");
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
                body.iter()
                    .map(|s| ast_to_python(s, indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            format!(
                "{}for {} in {}:\n{}",
                indent_str(indent),
                var,
                iter_str,
                body_str
            )
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
            format!(
                "{}{} = {}",
                indent_str(indent),
                name,
                ast_to_python(value, 0)
            )
        }
        AstNode::MultiAssign { names, values } => {
            let names_str = names.join(", ");
            let values_str = values
                .iter()
                .map(|v| ast_to_python(v, 0))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "{}{} = {}",
                indent_str(indent),
                names_str,
                values_str
            )
        }
        AstNode::AttributeAssign { object, attr, value } => {
            format!(
                "{}{}.{} = {}",
                indent_str(indent),
                ast_to_python(object, 0),
                attr,
                ast_to_python(value, 0)
            )
        }
        AstNode::BinaryOp { left, op, right } => {
            if let AstNode::Call { func, args } = &**left {
                let func_name = ast_to_python(func, 0);
                if func_name == "print" && args.len() == 1 {
                    let str_expr = ast_to_python(&args[0], 0);
                    let right_str = ast_to_python(right, 0);
                    return format!("print({} % {})", str_expr, right_str);
                }
            }
            format!(
                "{} {} {}",
                ast_to_python(left, 0),
                op,
                ast_to_python(right, 0)
            )
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
        AstNode::Bool(b) => {
            if *b {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        AstNode::None => "None".to_string(),
        AstNode::List(items) => {
            let items_str = items
                .iter()
                .map(|i| ast_to_python(i, 0))
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", items_str)
        }
        AstNode::Tuple(items) => {
            let items_str = items
                .iter()
                .map(|i| ast_to_python(i, 0))
                .collect::<Vec<_>>()
                .join(", ");
            if items.len() == 1 {
                format!("({},)", items_str)
            } else {
                format!("({})", items_str)
            }
        }
        AstNode::Set(items) => {
            let items_str = items
                .iter()
                .map(|i| ast_to_python(i, 0))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{}}}", items_str)
        }
        AstNode::Dict(pairs) => {
            let pairs_str = pairs
                .iter()
                .map(|(k, v)| format!("{}: {}", ast_to_python(k, 0), ast_to_python(v, 0)))
                .collect::<Vec<_>>()
                .join(", ");
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
                            return format!(
                                "{}print({} % {})",
                                indent_str(indent),
                                left_str,
                                right_str
                            );
                        }
                    }
                }
            }
            // 其它情况，普通函数调用
            let args_str = args
                .iter()
                .map(|a| ast_to_python(a, 0))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}{}({})", indent_str(indent), func_name, args_str)
        }
        AstNode::Index { value, index } => {
            format!("{}[{}]", ast_to_python(value, 0), ast_to_python(index, 0))
        }
        AstNode::Slice { value, start, end, step } => {
            let start_str = start.as_ref().map(|s| ast_to_python(s, 0)).unwrap_or_default();
            let end_str = end.as_ref().map(|e| ast_to_python(e, 0)).unwrap_or_default();
            
            if let Some(step_node) = step {
                let step_str = ast_to_python(step_node, 0);
                format!("{}[{}:{}:{}]", ast_to_python(value, 0), start_str, end_str, step_str)
            } else {
                format!("{}[{}:{}]", ast_to_python(value, 0), start_str, end_str)
            }
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
        AstNode::Import { module, alias } => {
            if let Some(a) = alias {
                format!("import {} as {}", module, a)
            } else {
                format!("import {}", module)
            }
        }
        AstNode::ImportFrom { module, names } => {
            let names_str = names
                .iter()
                .map(|(n, a)| {
                    if let Some(alias) = a {
                        format!("{} as {}", n, alias)
                    } else {
                        n.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("from {} import {}", module, names_str)
        }
        AstNode::Lambda { params, body } => {
            let params_str = params.join(", ");
            format!("lambda {}: {}", params_str, ast_to_python(body, 0))
        }
    }
}
