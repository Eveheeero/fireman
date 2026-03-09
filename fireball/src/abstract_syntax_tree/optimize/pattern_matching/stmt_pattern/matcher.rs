use super::{
    node_name::NodeName,
    types::{Captured, Captures, PatTree},
};
use crate::abstract_syntax_tree::{
    ArcAstVariableMap, AstBinaryOperator, AstExpression, AstLiteral, AstStatement,
    AstUnaryOperator, AstValueType, AstVariable, AstVariableId, Wrapped, WrappedAstStatement,
};

// ---------------------------------------------------------------------------
// Structural matcher
// ---------------------------------------------------------------------------

pub fn match_statement(pat: &PatTree, stmt: &AstStatement) -> Option<Captures> {
    let mut caps = Captures::new();
    if match_stmt_inner(pat, stmt, &mut caps) {
        Some(caps)
    } else {
        None
    }
}

fn match_stmt_inner(pat: &PatTree, stmt: &AstStatement, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Statement(stmt.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_stmt_node(*name, children, stmt, caps),
        _ => false,
    }
}

fn match_stmt_node(
    name: NodeName,
    children: &[PatTree],
    stmt: &AstStatement,
    caps: &mut Captures,
) -> bool {
    match (name, stmt) {
        (NodeName::Assignment, AstStatement::Assignment(lhs, rhs)) if children.len() == 2 => {
            match_wrapped_expr(&children[0], lhs, caps)
                && match_wrapped_expr(&children[1], rhs, caps)
        }
        (NodeName::If, AstStatement::If(cond, branch_true, branch_false))
            if children.len() == 3 =>
        {
            match_wrapped_expr(&children[0], cond, caps)
                && match_stmt_list(&children[1], branch_true, caps)
                && match_opt_stmt_list(&children[2], branch_false, caps)
        }
        (NodeName::While, AstStatement::While(cond, body)) if children.len() == 2 => {
            match_wrapped_expr(&children[0], cond, caps)
                && match_stmt_list(&children[1], body, caps)
        }
        (NodeName::DoWhile, AstStatement::DoWhile(cond, body)) if children.len() == 2 => {
            match_wrapped_expr(&children[0], cond, caps)
                && match_stmt_list(&children[1], body, caps)
        }
        (NodeName::For, AstStatement::For(init, cond, update, body)) if children.len() == 4 => {
            match_stmt_inner(&children[0], &init.statement, caps)
                && match_wrapped_expr(&children[1], cond, caps)
                && match_stmt_inner(&children[2], &update.statement, caps)
                && match_stmt_list(&children[3], body, caps)
        }
        (NodeName::Return, AstStatement::Return(opt_expr)) if children.len() == 1 => {
            match_opt_wrapped_expr(&children[0], opt_expr, caps)
        }
        (NodeName::Return, AstStatement::Return(None)) if children.is_empty() => true,
        (NodeName::Block, AstStatement::Block(body)) if children.len() == 1 => {
            match_stmt_list(&children[0], body, caps)
        }
        (NodeName::Label, AstStatement::Label(s)) if children.len() == 1 => {
            match_string_pat(&children[0], s, caps)
        }
        (NodeName::Comment, AstStatement::Comment(s)) if children.len() == 1 => {
            match_string_pat(&children[0], s, caps)
        }
        (NodeName::Assembly, AstStatement::Assembly(s)) if children.len() == 1 => {
            match_string_pat(&children[0], s, caps)
        }
        (NodeName::Declaration, AstStatement::Declaration(var, opt_init))
            if children.len() == 2 =>
        {
            match_variable_pat(&children[0], var, caps)
                && match_opt_wrapped_expr(&children[1], opt_init, caps)
        }
        (NodeName::Call, AstStatement::Call(call)) if children.len() == 1 => match &children[0] {
            PatTree::Capture(name) => {
                caps.insert(name.clone(), Captured::Call(call.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        (
            NodeName::Goto,
            AstStatement::Goto(crate::abstract_syntax_tree::AstJumpTarget::Unknown(s)),
        ) if children.len() == 1 => match_string_pat(&children[0], s, caps),
        (NodeName::Empty, AstStatement::Empty) if children.is_empty() => true,
        (NodeName::Break, AstStatement::Break) if children.is_empty() => true,
        (NodeName::Continue, AstStatement::Continue) if children.is_empty() => true,
        (NodeName::Undefined, AstStatement::Undefined) if children.is_empty() => true,
        _ => false,
    }
}

pub(super) fn match_wrapped_expr(
    pat: &PatTree,
    expr: &Wrapped<AstExpression>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Expression(expr.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_expr_node(*name, children, &expr.item, caps),
        _ => false,
    }
}

pub(super) fn match_boxed_wrapped_expr(
    pat: &PatTree,
    expr: &Box<Wrapped<AstExpression>>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::ExpressionBox(expr.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_expr_node(*name, children, &expr.item, caps),
        _ => false,
    }
}

fn match_expr_node(
    name: NodeName,
    children: &[PatTree],
    expr: &AstExpression,
    caps: &mut Captures,
) -> bool {
    match (name, expr) {
        (NodeName::Variable, AstExpression::Variable(map, var_id)) if children.len() == 2 => {
            match_variable_map_pat(&children[0], map, caps)
                && match_variable_id_pat(&children[1], var_id, caps)
        }
        (NodeName::Literal, AstExpression::Literal(lit)) if children.len() == 1 => {
            match_literal_pat(&children[0], lit, caps)
        }
        (NodeName::UnaryOp, AstExpression::UnaryOp(op, arg)) if children.len() == 2 => {
            match_unary_op_pat(&children[0], op, caps)
                && match_boxed_wrapped_expr(&children[1], arg, caps)
        }
        (NodeName::BinaryOp, AstExpression::BinaryOp(op, lhs, rhs)) if children.len() == 3 => {
            match_binary_op_pat(&children[0], op, caps)
                && match_boxed_wrapped_expr(&children[1], lhs, caps)
                && match_boxed_wrapped_expr(&children[2], rhs, caps)
        }
        (NodeName::Ternary, AstExpression::Ternary(cond, t, f)) if children.len() == 3 => {
            match_boxed_wrapped_expr(&children[0], cond, caps)
                && match_boxed_wrapped_expr(&children[1], t, caps)
                && match_boxed_wrapped_expr(&children[2], f, caps)
        }
        (NodeName::Cast, AstExpression::Cast(ty, arg)) if children.len() == 2 => {
            match_value_type_pat(&children[0], ty, caps)
                && match_boxed_wrapped_expr(&children[1], arg, caps)
        }
        (NodeName::Deref, AstExpression::Deref(arg)) if children.len() == 1 => {
            match_boxed_wrapped_expr(&children[0], arg, caps)
        }
        (NodeName::AddressOf, AstExpression::AddressOf(arg)) if children.len() == 1 => {
            match_boxed_wrapped_expr(&children[0], arg, caps)
        }
        (NodeName::ArrayAccess, AstExpression::ArrayAccess(base, idx)) if children.len() == 2 => {
            match_boxed_wrapped_expr(&children[0], base, caps)
                && match_boxed_wrapped_expr(&children[1], idx, caps)
        }
        (NodeName::MemberAccess, AstExpression::MemberAccess(base, _field))
            if children.len() == 2 =>
        {
            match_boxed_wrapped_expr(&children[0], base, caps)
                && pat_is_wildcard_or_capture(&children[1], caps)
        }
        (NodeName::Call, AstExpression::Call(call)) if children.len() == 1 => match &children[0] {
            PatTree::Capture(name) => {
                caps.insert(name.clone(), Captured::Call(call.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        (NodeName::Unknown, AstExpression::Unknown) if children.is_empty() => true,
        (NodeName::Undefined, AstExpression::Undefined) if children.is_empty() => true,
        _ => false,
    }
}

fn pat_is_wildcard_or_capture(pat: &PatTree, _caps: &mut Captures) -> bool {
    matches!(pat, PatTree::Wildcard | PatTree::Capture(_))
}

fn match_variable_map_pat(pat: &PatTree, map: &ArcAstVariableMap, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::VariableMap(map.clone()));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_variable_id_pat(pat: &PatTree, var_id: &AstVariableId, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::VariableId(*var_id));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_value_type_pat(pat: &PatTree, ty: &AstValueType, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::ValueType(ty.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } if children.is_empty() => match (name, ty) {
            (NodeName::Void, AstValueType::Void) => true,
            (NodeName::Unknown, AstValueType::Unknown) => true,
            (NodeName::Int, AstValueType::Int) => true,
            (NodeName::Int8, AstValueType::Int8) => true,
            (NodeName::Int16, AstValueType::Int16) => true,
            (NodeName::Int32, AstValueType::Int32) => true,
            (NodeName::Int64, AstValueType::Int64) => true,
            (NodeName::UInt, AstValueType::UInt) => true,
            (NodeName::UInt8, AstValueType::UInt8) => true,
            (NodeName::UInt16, AstValueType::UInt16) => true,
            (NodeName::UInt32, AstValueType::UInt32) => true,
            (NodeName::UInt64, AstValueType::UInt64) => true,
            (NodeName::Char, AstValueType::Char) => true,
            (NodeName::Float, AstValueType::Float) => true,
            (NodeName::Double, AstValueType::Double) => true,
            (NodeName::Bool, AstValueType::Bool) => true,
            _ => false,
        },
        _ => false,
    }
}

fn match_literal_pat(pat: &PatTree, lit: &AstLiteral, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Literal(lit.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_literal_node(*name, children, lit, caps),
        _ => false,
    }
}

fn match_literal_node(
    name: NodeName,
    children: &[PatTree],
    lit: &AstLiteral,
    caps: &mut Captures,
) -> bool {
    match (name, lit) {
        (NodeName::Bool, AstLiteral::Bool(b)) if children.len() == 1 => match &children[0] {
            PatTree::Node { name, children } if children.is_empty() => match (name, b) {
                (NodeName::True, true) | (NodeName::False, false) => true,
                _ => false,
            },
            PatTree::Capture(cap_name) => {
                caps.insert(cap_name.clone(), Captured::Literal(lit.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        (NodeName::Int, AstLiteral::Int(n)) if children.len() == 1 => match &children[0] {
            PatTree::NumberLiteral(v) => *v == *n,
            PatTree::Capture(cap_name) => {
                caps.insert(cap_name.clone(), Captured::Literal(lit.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        (NodeName::UInt, AstLiteral::UInt(n)) if children.len() == 1 => match &children[0] {
            PatTree::NumberLiteral(v) if *v >= 0 => *v as u64 == *n,
            PatTree::UIntLiteral(v) => *v == *n,
            PatTree::Capture(cap_name) => {
                caps.insert(cap_name.clone(), Captured::Literal(lit.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        _ => false,
    }
}

fn match_unary_op_pat(pat: &PatTree, op: &AstUnaryOperator, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::UnaryOp(op.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } if children.is_empty() => match (name, op) {
            (NodeName::Negate, AstUnaryOperator::Negate) => true,
            (NodeName::Not, AstUnaryOperator::Not) => true,
            (NodeName::BitNot, AstUnaryOperator::BitNot) => true,
            (NodeName::PreInc, AstUnaryOperator::PreInc) => true,
            (NodeName::PreDec, AstUnaryOperator::PreDec) => true,
            (NodeName::PostInc, AstUnaryOperator::PostInc) => true,
            (NodeName::PostDec, AstUnaryOperator::PostDec) => true,
            (NodeName::CastSigned, AstUnaryOperator::CastSigned) => true,
            (NodeName::CastUnsigned, AstUnaryOperator::CastUnsigned) => true,
            _ => false,
        },
        _ => false,
    }
}

fn match_binary_op_pat(pat: &PatTree, op: &AstBinaryOperator, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::BinaryOp(op.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } if children.is_empty() => match (name, op) {
            (NodeName::Add, AstBinaryOperator::Add) => true,
            (NodeName::Sub, AstBinaryOperator::Sub) => true,
            (NodeName::Mul, AstBinaryOperator::Mul) => true,
            (NodeName::Div, AstBinaryOperator::Div) => true,
            (NodeName::Mod, AstBinaryOperator::Mod) => true,
            (NodeName::BitAnd, AstBinaryOperator::BitAnd) => true,
            (NodeName::BitOr, AstBinaryOperator::BitOr) => true,
            (NodeName::BitXor, AstBinaryOperator::BitXor) => true,
            (NodeName::LogicAnd, AstBinaryOperator::LogicAnd) => true,
            (NodeName::LogicOr, AstBinaryOperator::LogicOr) => true,
            (NodeName::Equal, AstBinaryOperator::Equal) => true,
            (NodeName::NotEqual, AstBinaryOperator::NotEqual) => true,
            (NodeName::Less, AstBinaryOperator::Less) => true,
            (NodeName::LessEqual, AstBinaryOperator::LessEqual) => true,
            (NodeName::Greater, AstBinaryOperator::Greater) => true,
            (NodeName::GreaterEqual, AstBinaryOperator::GreaterEqual) => true,
            (NodeName::LeftShift, AstBinaryOperator::LeftShift) => true,
            (NodeName::RightShift, AstBinaryOperator::RightShift) => true,
            _ => false,
        },
        _ => false,
    }
}

fn match_variable_pat(pat: &PatTree, var: &AstVariable, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Variable(var.clone()));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_string_pat(pat: &PatTree, s: &str, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Wildcard => true,
        PatTree::Capture(name) => {
            caps.insert(
                name.clone(),
                Captured::Literal(AstLiteral::String(s.to_string())),
            );
            true
        }
        PatTree::StringLiteral(lit) => lit == s,
        PatTree::Node { name, children } if children.is_empty() => {
            // NodeName variant used as string literal (e.g. Label(Unknown))
            name.as_str() == s
        }
        _ => false,
    }
}

fn match_stmt_list(pat: &PatTree, stmts: &Vec<WrappedAstStatement>, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::StmtList(stmts.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::List(pats) => {
            if pats.len() != stmts.len() {
                return false;
            }
            for (p, s) in pats.iter().zip(stmts.iter()) {
                if !match_stmt_inner(p, &s.statement, caps) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

fn match_opt_stmt_list(
    pat: &PatTree,
    opt: &Option<Vec<WrappedAstStatement>>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::OptStmtList(opt.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::OptionNone => opt.is_none(),
        PatTree::OptionSome(inner) => match opt {
            Some(stmts) => match_stmt_list(inner, stmts, caps),
            None => false,
        },
        _ => false,
    }
}

fn match_opt_wrapped_expr(
    pat: &PatTree,
    opt: &Option<Wrapped<AstExpression>>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::OptExpression(opt.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::OptionNone => opt.is_none(),
        PatTree::OptionSome(inner) => match opt {
            Some(expr) => match_wrapped_expr(inner, expr, caps),
            None => false,
        },
        _ => false,
    }
}
