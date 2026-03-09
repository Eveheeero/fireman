use super::{
    node_name::NodeName,
    types::{Captured, Captures, PatTree},
};
use crate::abstract_syntax_tree::{
    ArcAstVariableMap, AstBinaryOperator, AstCall, AstExpression, AstLiteral, AstStatement,
    AstStatementOrigin, AstUnaryOperator, AstValueOrigin, AstValueType, AstVariableId, Wrapped,
    WrappedAstStatement,
};

// ---------------------------------------------------------------------------
// Emit (construct replacement)
// ---------------------------------------------------------------------------

pub fn construct_statement(pat: &PatTree, caps: &Captures) -> Option<AstStatement> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Statement(s) => Some(s.clone()),
            _ => None,
        },
        PatTree::Node { name, children } => construct_stmt_node(*name, children, caps),
        _ => None,
    }
}

fn construct_stmt_node(
    name: NodeName,
    children: &[PatTree],
    caps: &Captures,
) -> Option<AstStatement> {
    match name {
        NodeName::Assignment if children.len() == 2 => {
            let lhs = construct_wrapped_expr(&children[0], caps)?;
            let rhs = construct_wrapped_expr(&children[1], caps)?;
            Some(AstStatement::Assignment(lhs, rhs))
        }
        NodeName::If if children.len() == 3 => {
            let cond = construct_wrapped_expr(&children[0], caps)?;
            let branch_true = construct_stmt_list(&children[1], caps)?;
            let branch_false = construct_opt_stmt_list(&children[2], caps)?;
            Some(AstStatement::If(cond, branch_true, branch_false))
        }
        NodeName::While if children.len() == 2 => {
            let cond = construct_wrapped_expr(&children[0], caps)?;
            let body = construct_stmt_list(&children[1], caps)?;
            Some(AstStatement::While(cond, body))
        }
        NodeName::DoWhile if children.len() == 2 => {
            let cond = construct_wrapped_expr(&children[0], caps)?;
            let body = construct_stmt_list(&children[1], caps)?;
            Some(AstStatement::DoWhile(cond, body))
        }
        NodeName::Return if children.len() == 1 => {
            let opt = construct_opt_wrapped_expr(&children[0], caps)?;
            Some(AstStatement::Return(opt))
        }
        NodeName::Return if children.is_empty() => Some(AstStatement::Return(None)),
        NodeName::Block if children.len() == 1 => {
            let body = construct_stmt_list(&children[0], caps)?;
            Some(AstStatement::Block(body))
        }
        NodeName::Call if children.len() == 1 => {
            let call = construct_call(&children[0], caps)?;
            Some(AstStatement::Call(call))
        }
        NodeName::Call if children.len() == 2 => {
            let call = construct_call_from_name_and_args(&children[0], &children[1], caps)?;
            Some(AstStatement::Call(call))
        }
        NodeName::Comment if children.len() == 1 => {
            let s = construct_string(&children[0], caps)?;
            Some(AstStatement::Comment(s))
        }
        NodeName::Label if children.len() == 1 => {
            let s = construct_string(&children[0], caps)?;
            Some(AstStatement::Label(s))
        }
        NodeName::Goto if children.len() == 1 => {
            let s = construct_string(&children[0], caps)?;
            Some(AstStatement::Goto(
                crate::abstract_syntax_tree::AstJumpTarget::Unknown(s),
            ))
        }
        NodeName::Empty if children.is_empty() => Some(AstStatement::Empty),
        NodeName::Break if children.is_empty() => Some(AstStatement::Break),
        NodeName::Continue if children.is_empty() => Some(AstStatement::Continue),
        _ => None,
    }
}

fn construct_string(pat: &PatTree, caps: &Captures) -> Option<String> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Literal(AstLiteral::String(s)) => Some(s.clone()),
            _ => None,
        },
        PatTree::StringLiteral(s) => Some(s.clone()),
        PatTree::Node { name, children } if children.is_empty() => {
            // NodeName variant used as string literal (e.g. Label(Unknown))
            Some(name.as_str().to_string())
        }
        _ => None,
    }
}

fn construct_value_type(pat: &PatTree, caps: &Captures) -> Option<AstValueType> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::ValueType(ty) => Some(ty.clone()),
            _ => None,
        },
        PatTree::Node { name, children } if children.is_empty() => match name {
            NodeName::Void => Some(AstValueType::Void),
            NodeName::Unknown => Some(AstValueType::Unknown),
            NodeName::Int => Some(AstValueType::Int),
            NodeName::Int8 => Some(AstValueType::Int8),
            NodeName::Int16 => Some(AstValueType::Int16),
            NodeName::Int32 => Some(AstValueType::Int32),
            NodeName::Int64 => Some(AstValueType::Int64),
            NodeName::UInt => Some(AstValueType::UInt),
            NodeName::UInt8 => Some(AstValueType::UInt8),
            NodeName::UInt16 => Some(AstValueType::UInt16),
            NodeName::UInt32 => Some(AstValueType::UInt32),
            NodeName::UInt64 => Some(AstValueType::UInt64),
            NodeName::Char => Some(AstValueType::Char),
            NodeName::Float => Some(AstValueType::Float),
            NodeName::Double => Some(AstValueType::Double),
            NodeName::Bool => Some(AstValueType::Bool),
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn construct_wrapped_expr(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Wrapped<AstExpression>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Expression(e) => Some(e.clone()),
            Captured::ExpressionBox(e) => Some((**e).clone()),
            _ => None,
        },
        PatTree::Node { name, children } => {
            let expr = construct_expr_node(*name, children, caps)?;
            // Find an origin from any captured expression to use
            let origin = find_any_origin(caps);
            Some(Wrapped {
                item: expr,
                origin,
                comment: None,
            })
        }
        _ => None,
    }
}

fn construct_boxed_wrapped_expr(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Box<Wrapped<AstExpression>>> {
    construct_wrapped_expr(pat, caps).map(Box::new)
}

fn construct_expr_node(
    name: NodeName,
    children: &[PatTree],
    caps: &Captures,
) -> Option<AstExpression> {
    match name {
        NodeName::Variable if children.len() == 2 => {
            let map = construct_variable_map(&children[0], caps)?;
            let var_id = construct_variable_id(&children[1], caps)?;
            Some(AstExpression::Variable(map, var_id))
        }
        NodeName::Literal if children.len() == 1 => {
            let lit = construct_literal(&children[0], caps)?;
            Some(AstExpression::Literal(lit))
        }
        NodeName::UnaryOp if children.len() == 2 => {
            let op = construct_unary_op(&children[0], caps)?;
            let arg = construct_boxed_wrapped_expr(&children[1], caps)?;
            Some(AstExpression::UnaryOp(op, arg))
        }
        NodeName::BinaryOp if children.len() == 3 => {
            let op = construct_binary_op(&children[0], caps)?;
            let lhs = construct_boxed_wrapped_expr(&children[1], caps)?;
            let rhs = construct_boxed_wrapped_expr(&children[2], caps)?;
            Some(AstExpression::BinaryOp(op, lhs, rhs))
        }
        NodeName::Ternary if children.len() == 3 => {
            let cond = construct_boxed_wrapped_expr(&children[0], caps)?;
            let t = construct_boxed_wrapped_expr(&children[1], caps)?;
            let f = construct_boxed_wrapped_expr(&children[2], caps)?;
            Some(AstExpression::Ternary(cond, t, f))
        }
        NodeName::Cast if children.len() == 2 => {
            let ty = construct_value_type(&children[0], caps)?;
            let arg = construct_boxed_wrapped_expr(&children[1], caps)?;
            Some(AstExpression::Cast(ty, arg))
        }
        NodeName::Deref if children.len() == 1 => {
            let arg = construct_boxed_wrapped_expr(&children[0], caps)?;
            Some(AstExpression::Deref(arg))
        }
        NodeName::AddressOf if children.len() == 1 => {
            let arg = construct_boxed_wrapped_expr(&children[0], caps)?;
            Some(AstExpression::AddressOf(arg))
        }
        NodeName::ArrayAccess if children.len() == 2 => {
            let base = construct_boxed_wrapped_expr(&children[0], caps)?;
            let idx = construct_boxed_wrapped_expr(&children[1], caps)?;
            Some(AstExpression::ArrayAccess(base, idx))
        }
        NodeName::Call if children.len() == 1 => {
            let call = construct_call(&children[0], caps)?;
            Some(AstExpression::Call(call))
        }
        NodeName::Call if children.len() == 2 => {
            let call = construct_call_from_name_and_args(&children[0], &children[1], caps)?;
            Some(AstExpression::Call(call))
        }
        NodeName::Unknown if children.is_empty() => Some(AstExpression::Unknown),
        NodeName::Undefined if children.is_empty() => Some(AstExpression::Undefined),
        _ => None,
    }
}

fn construct_call(pat: &PatTree, caps: &Captures) -> Option<AstCall> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Call(c) => Some(c.clone()),
            _ => None,
        },
        _ => None,
    }
}

/// Construct an `AstCall::Unknown(name, args)` from a string name + expression list.
fn construct_call_from_name_and_args(
    name_pat: &PatTree,
    args_pat: &PatTree,
    caps: &Captures,
) -> Option<AstCall> {
    let name = match name_pat {
        PatTree::StringLiteral(s) => s.clone(),
        PatTree::Capture(cap_name) => match caps.get(cap_name)? {
            Captured::Literal(AstLiteral::String(s)) => s.clone(),
            _ => return None,
        },
        _ => return None,
    };
    let args = match args_pat {
        PatTree::List(pats) => {
            let mut result = Vec::new();
            for p in pats {
                result.push(construct_wrapped_expr(p, caps)?);
            }
            result
        }
        PatTree::Capture(cap_name) => match caps.get(cap_name)? {
            Captured::StmtList(_) => return None,
            _ => return None,
        },
        _ => return None,
    };
    Some(AstCall::Unknown(name, args))
}

fn construct_variable_map(pat: &PatTree, caps: &Captures) -> Option<ArcAstVariableMap> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::VariableMap(m) => Some(m.clone()),
            _ => None,
        },
        PatTree::Wildcard => {
            // Fallback: find any VariableMap in captures
            for v in caps.values() {
                if let Captured::VariableMap(m) = v {
                    return Some(m.clone());
                }
            }
            None
        }
        _ => None,
    }
}

fn construct_variable_id(pat: &PatTree, caps: &Captures) -> Option<AstVariableId> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::VariableId(id) => Some(*id),
            _ => None,
        },
        _ => None,
    }
}

fn construct_literal(pat: &PatTree, caps: &Captures) -> Option<AstLiteral> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Literal(l) => Some(l.clone()),
            _ => None,
        },
        PatTree::Node { name, children } => construct_literal_node(*name, children, caps),
        _ => None,
    }
}

fn construct_literal_node(
    name: NodeName,
    children: &[PatTree],
    caps: &Captures,
) -> Option<AstLiteral> {
    match name {
        NodeName::Bool if children.len() == 1 => match &children[0] {
            PatTree::Node { name, children } if children.is_empty() => match name {
                NodeName::True => Some(AstLiteral::Bool(true)),
                NodeName::False => Some(AstLiteral::Bool(false)),
                _ => None,
            },
            PatTree::Capture(cap_name) => match caps.get(cap_name)? {
                Captured::Literal(l) => Some(l.clone()),
                _ => None,
            },
            _ => None,
        },
        NodeName::Int if children.len() == 1 => match &children[0] {
            PatTree::NumberLiteral(v) => Some(AstLiteral::Int(*v)),
            PatTree::Capture(cap_name) => match caps.get(cap_name)? {
                Captured::Literal(l) => Some(l.clone()),
                _ => None,
            },
            _ => None,
        },
        NodeName::UInt if children.len() == 1 => match &children[0] {
            PatTree::NumberLiteral(v) if *v >= 0 => Some(AstLiteral::UInt(*v as u64)),
            PatTree::UIntLiteral(v) => Some(AstLiteral::UInt(*v)),
            PatTree::Capture(cap_name) => match caps.get(cap_name)? {
                Captured::Literal(l) => Some(l.clone()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn construct_unary_op(pat: &PatTree, caps: &Captures) -> Option<AstUnaryOperator> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::UnaryOp(op) => Some(op.clone()),
            _ => None,
        },
        PatTree::Node { name, children } if children.is_empty() => match name {
            NodeName::Negate => Some(AstUnaryOperator::Negate),
            NodeName::Not => Some(AstUnaryOperator::Not),
            NodeName::BitNot => Some(AstUnaryOperator::BitNot),
            NodeName::PreInc => Some(AstUnaryOperator::PreInc),
            NodeName::PreDec => Some(AstUnaryOperator::PreDec),
            NodeName::PostInc => Some(AstUnaryOperator::PostInc),
            NodeName::PostDec => Some(AstUnaryOperator::PostDec),
            NodeName::CastSigned => Some(AstUnaryOperator::CastSigned),
            NodeName::CastUnsigned => Some(AstUnaryOperator::CastUnsigned),
            _ => None,
        },
        _ => None,
    }
}

fn construct_binary_op(pat: &PatTree, caps: &Captures) -> Option<AstBinaryOperator> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::BinaryOp(op) => Some(op.clone()),
            _ => None,
        },
        PatTree::Node { name, children } if children.is_empty() => match name {
            NodeName::Add => Some(AstBinaryOperator::Add),
            NodeName::Sub => Some(AstBinaryOperator::Sub),
            NodeName::Mul => Some(AstBinaryOperator::Mul),
            NodeName::Div => Some(AstBinaryOperator::Div),
            NodeName::Mod => Some(AstBinaryOperator::Mod),
            NodeName::BitAnd => Some(AstBinaryOperator::BitAnd),
            NodeName::BitOr => Some(AstBinaryOperator::BitOr),
            NodeName::BitXor => Some(AstBinaryOperator::BitXor),
            NodeName::LogicAnd => Some(AstBinaryOperator::LogicAnd),
            NodeName::LogicOr => Some(AstBinaryOperator::LogicOr),
            NodeName::Equal => Some(AstBinaryOperator::Equal),
            NodeName::NotEqual => Some(AstBinaryOperator::NotEqual),
            NodeName::Less => Some(AstBinaryOperator::Less),
            NodeName::LessEqual => Some(AstBinaryOperator::LessEqual),
            NodeName::Greater => Some(AstBinaryOperator::Greater),
            NodeName::GreaterEqual => Some(AstBinaryOperator::GreaterEqual),
            NodeName::LeftShift => Some(AstBinaryOperator::LeftShift),
            NodeName::RightShift => Some(AstBinaryOperator::RightShift),
            _ => None,
        },
        _ => None,
    }
}

fn construct_stmt_list(pat: &PatTree, caps: &Captures) -> Option<Vec<WrappedAstStatement>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::StmtList(l) => Some(l.clone()),
            Captured::Statement(stmt) => Some(vec![WrappedAstStatement {
                statement: stmt.clone(),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            }]),
            _ => None,
        },
        PatTree::Node { .. } => {
            let stmt = construct_statement(pat, caps)?;
            Some(vec![WrappedAstStatement {
                statement: stmt,
                origin: AstStatementOrigin::Unknown,
                comment: None,
            }])
        }
        PatTree::List(pats) => {
            let mut result = Vec::new();
            for p in pats {
                let stmt = construct_statement(p, caps)?;
                result.push(WrappedAstStatement {
                    statement: stmt,
                    origin: AstStatementOrigin::Unknown,
                    comment: None,
                });
            }
            Some(result)
        }
        _ => None,
    }
}

fn construct_opt_stmt_list(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Option<Vec<WrappedAstStatement>>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::OptStmtList(l) => Some(l.clone()),
            _ => None,
        },
        PatTree::OptionNone => Some(None),
        PatTree::OptionSome(inner) => {
            let list = construct_stmt_list(inner, caps)?;
            Some(Some(list))
        }
        _ => None,
    }
}

fn construct_opt_wrapped_expr(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Option<Wrapped<AstExpression>>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::OptExpression(e) => Some(e.clone()),
            _ => None,
        },
        PatTree::OptionNone => Some(None),
        PatTree::OptionSome(inner) => {
            let expr = construct_wrapped_expr(inner, caps)?;
            Some(Some(expr))
        }
        _ => None,
    }
}

/// Inject captured values into a rhai `Scope` as debug-string variables.
/// Each capture `$name` becomes a rhai variable `name` with its `Debug` representation.
pub fn inject_captures_into_rhai_scope(caps: &Captures, scope: &mut rhai::Scope<'static>) {
    for (name, captured) in caps {
        let value = match captured {
            Captured::Statement(s) => format!("{s:?}"),
            Captured::Expression(e) => format!("{:?}", e.item),
            Captured::ExpressionBox(e) => format!("{:?}", e.item),
            Captured::VariableId(id) => format!("{id:?}"),
            Captured::VariableMap(_) => "VariableMap(...)".to_string(),
            Captured::Literal(l) => format!("{l:?}"),
            Captured::StmtList(l) => format!("{l:?}"),
            Captured::OptStmtList(l) => format!("{l:?}"),
            Captured::OptExpression(e) => format!("{e:?}"),
            Captured::UnaryOp(op) => format!("{op:?}"),
            Captured::BinaryOp(op) => format!("{op:?}"),
            Captured::Variable(v) => format!("{v:?}"),
            Captured::ValueType(t) => format!("{t:?}"),
            Captured::Call(c) => format!("{c:?}"),
        };
        scope.push(name.clone(), value);
    }
}

fn find_any_origin(caps: &Captures) -> AstValueOrigin {
    for v in caps.values() {
        match v {
            Captured::Expression(e) => return e.origin.clone(),
            Captured::ExpressionBox(e) => return e.origin.clone(),
            _ => {}
        }
    }
    AstValueOrigin::Unknown
}

// ---------------------------------------------------------------------------
// Construct a Vec<WrappedAstStatement> from a capture (for emit_after)
// ---------------------------------------------------------------------------

/// Construct a list of statements from a pattern and captures.
/// Used by `emit_after` to produce the statements to splice after the matched one.
pub fn construct_emit_after_list(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Vec<WrappedAstStatement>> {
    construct_stmt_list(pat, caps)
}
