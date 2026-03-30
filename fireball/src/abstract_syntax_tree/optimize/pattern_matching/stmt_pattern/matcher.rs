use super::{
    node_name::NodeName,
    types::{Captured, Captures, PatTree},
};
use crate::abstract_syntax_tree::{
    ArcAstVariableMap, AstBinaryOperator, AstExpression, AstLiteral, AstStatement,
    AstUnaryOperator, AstValueType, AstVariable, AstVariableId, Wrapped, WrappedAstStatement,
};

// ---------------------------------------------------------------------------
// Structural matcher — iterative with explicit work stack
// ---------------------------------------------------------------------------

/// Work items for the iterative matcher.
///
/// The matcher uses a LIFO work stack and a `result` accumulator. Items are
/// pushed in *reverse* evaluation order so the first-to-evaluate is on top.
///
/// `Guard(n)` implements short-circuit AND: if `result` is false when a Guard
/// is popped, the next `n` items are skipped (they would have been the
/// remaining operands of the `&&` chain).
#[derive(Clone, Copy)]
enum Work<'a> {
    /// Match a pattern tree against a statement (dispatches like `match_stmt_inner`).
    MatchStmt(&'a PatTree, &'a AstStatement),
    /// Match a pattern tree against a `Wrapped<AstExpression>` (dispatches like `match_wrapped_expr`).
    MatchWrappedExpr(&'a PatTree, &'a Wrapped<AstExpression>),
    /// Match a pattern tree against a `Box<Wrapped<AstExpression>>`.
    MatchBoxedWrappedExpr(&'a PatTree, &'a Box<Wrapped<AstExpression>>),
    /// Match a pattern tree against a statement list.
    MatchStmtList(&'a PatTree, &'a Vec<WrappedAstStatement>),
    /// Match an optional statement list.
    MatchOptStmtList(&'a PatTree, &'a Option<Vec<WrappedAstStatement>>),
    /// Match an optional wrapped expression.
    MatchOptWrappedExpr(&'a PatTree, &'a Option<Wrapped<AstExpression>>),
    /// Short-circuit guard: if `result` is false, skip the next `n` work items.
    Guard(usize),
    /// Set the result to a literal value (AND-ed with current result).
    SetResult(bool),
}

pub fn match_statement(pat: &PatTree, stmt: &AstStatement) -> Option<Captures> {
    let mut caps = Captures::new();
    if match_iterative(Work::MatchStmt(pat, stmt), &mut caps) {
        Some(caps)
    } else {
        None
    }
}

pub(super) fn match_wrapped_expr(
    pat: &PatTree,
    expr: &Wrapped<AstExpression>,
    caps: &mut Captures,
) -> bool {
    match_iterative(Work::MatchWrappedExpr(pat, expr), caps)
}

pub(super) fn match_boxed_wrapped_expr(
    pat: &PatTree,
    expr: &Box<Wrapped<AstExpression>>,
    caps: &mut Captures,
) -> bool {
    match_iterative(Work::MatchBoxedWrappedExpr(pat, expr), caps)
}

// ---------------------------------------------------------------------------
// Core iterative engine
// ---------------------------------------------------------------------------

fn match_iterative(initial: Work<'_>, caps: &mut Captures) -> bool {
    let mut stack: Vec<Work<'_>> = vec![initial];
    let mut result = true;

    while let Some(work) = stack.pop() {
        match work {
            Work::Guard(n) => {
                if !result {
                    // Short-circuit: skip the next n items
                    let skip = n.min(stack.len());
                    stack.truncate(stack.len() - skip);
                }
                continue;
            }
            Work::SetResult(val) => {
                result = val;
                continue;
            }
            _ => {}
        }

        // If result is already false, we keep draining until a Guard handles it.
        // But since every AND chain is preceded by a Guard, and standalone items
        // just set result, we always evaluate the current item.
        // Actually — each work item is either the start of a new AND-chain
        // (with Guards above it on the stack) or a standalone match.
        // We always evaluate.

        match work {
            Work::MatchStmt(pat, stmt) => {
                expand_match_stmt(pat, stmt, &mut stack, &mut result, caps);
            }
            Work::MatchWrappedExpr(pat, expr) => {
                expand_match_wrapped_expr(pat, expr, &mut stack, &mut result, caps);
            }
            Work::MatchBoxedWrappedExpr(pat, expr) => {
                expand_match_boxed_wrapped_expr(pat, expr, &mut stack, &mut result, caps);
            }
            Work::MatchStmtList(pat, stmts) => {
                expand_match_stmt_list(pat, stmts, &mut stack, &mut result, caps);
            }
            Work::MatchOptStmtList(pat, opt) => {
                expand_match_opt_stmt_list(pat, opt, &mut stack, &mut result, caps);
            }
            Work::MatchOptWrappedExpr(pat, opt) => {
                expand_match_opt_wrapped_expr(pat, opt, &mut stack, &mut result, caps);
            }
            Work::Guard(_) | Work::SetResult(_) => unreachable!(),
        }
    }

    result
}

// ---------------------------------------------------------------------------
// Helpers to push AND-chains onto the work stack
// ---------------------------------------------------------------------------

/// Push an AND-chain of work items onto the stack for left-to-right evaluation
/// with short-circuit semantics. If any item sets `result` to false, subsequent
/// items in the chain are skipped via `Guard` entries.
///
/// For `[A, B, C]` the stack ends up (top-first): `A, Guard(3), B, Guard(1), C`.
fn push_and_chain<'a>(stack: &mut Vec<Work<'a>>, items: &[Work<'a>]) {
    let n = items.len();
    if n == 0 {
        return;
    }
    // Push in reverse. Between each pair, insert a Guard that skips all remaining.
    // items[0] && items[1] && ... && items[n-1]
    // Stack (top first): items[0], Guard(remaining), items[1], Guard(remaining-1), ...
    // In reverse push order:
    for i in (0..n).rev() {
        stack.push(items[i].clone());
        if i > 0 {
            // After evaluating items[i-1] (which will be pushed next),
            // if it fails, skip items[i] plus all guards/items after it.
            // Number of stack entries for items[i..n-1] = (n-1-i)*2 + 1
            // (each item + guard pair, except last item has no guard after it)
            let entries_after = (n - 1 - i) * 2 + 1;
            stack.push(Work::Guard(entries_after));
        }
    }
}

// ---------------------------------------------------------------------------
// Expansion functions — one per recursive function
// ---------------------------------------------------------------------------

fn expand_match_stmt<'a>(
    pat: &'a PatTree,
    stmt: &'a AstStatement,
    stack: &mut Vec<Work<'a>>,
    result: &mut bool,
    caps: &mut Captures,
) {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Statement(stmt.clone()));
            *result = true;
        }
        PatTree::Wildcard => {
            *result = true;
        }
        PatTree::Node { name, children } => {
            expand_match_stmt_node(*name, children, stmt, stack, result, caps);
        }
        _ => {
            *result = false;
        }
    }
}

fn expand_match_stmt_node<'a>(
    name: NodeName,
    children: &'a [PatTree],
    stmt: &'a AstStatement,
    stack: &mut Vec<Work<'a>>,
    result: &mut bool,
    caps: &mut Captures,
) {
    match (name, stmt) {
        (NodeName::Assignment, AstStatement::Assignment(lhs, rhs)) if children.len() == 2 => {
            push_and_chain(
                stack,
                &[
                    Work::MatchWrappedExpr(&children[0], lhs),
                    Work::MatchWrappedExpr(&children[1], rhs),
                ],
            );
            // The first item in the chain will set result; don't set it here.
            // But we need the chain to start with result=true so the first
            // item actually executes.
            *result = true;
        }
        (NodeName::If, AstStatement::If(cond, branch_true, branch_false))
            if children.len() == 3 =>
        {
            push_and_chain(
                stack,
                &[
                    Work::MatchWrappedExpr(&children[0], cond),
                    Work::MatchStmtList(&children[1], branch_true),
                    Work::MatchOptStmtList(&children[2], branch_false),
                ],
            );
            *result = true;
        }
        (NodeName::While, AstStatement::While(cond, body)) if children.len() == 2 => {
            push_and_chain(
                stack,
                &[
                    Work::MatchWrappedExpr(&children[0], cond),
                    Work::MatchStmtList(&children[1], body),
                ],
            );
            *result = true;
        }
        (NodeName::DoWhile, AstStatement::DoWhile(cond, body)) if children.len() == 2 => {
            push_and_chain(
                stack,
                &[
                    Work::MatchWrappedExpr(&children[0], cond),
                    Work::MatchStmtList(&children[1], body),
                ],
            );
            *result = true;
        }
        (NodeName::For, AstStatement::For(init, cond, update, body)) if children.len() == 4 => {
            push_and_chain(
                stack,
                &[
                    Work::MatchStmt(&children[0], &init.statement),
                    Work::MatchWrappedExpr(&children[1], cond),
                    Work::MatchStmt(&children[2], &update.statement),
                    Work::MatchStmtList(&children[3], body),
                ],
            );
            *result = true;
        }
        (NodeName::Return, AstStatement::Return(opt_expr)) if children.len() == 1 => {
            stack.push(Work::MatchOptWrappedExpr(&children[0], opt_expr));
            *result = true;
        }
        (NodeName::Return, AstStatement::Return(None)) if children.is_empty() => {
            *result = true;
        }
        (NodeName::Block, AstStatement::Block(body)) if children.len() == 1 => {
            stack.push(Work::MatchStmtList(&children[0], body));
            *result = true;
        }
        (NodeName::Label, AstStatement::Label(s)) if children.len() == 1 => {
            *result = match_string_pat(&children[0], s, caps);
        }
        (NodeName::Comment, AstStatement::Comment(s)) if children.len() == 1 => {
            *result = match_string_pat(&children[0], s, caps);
        }
        (NodeName::Assembly, AstStatement::Assembly(s)) if children.len() == 1 => {
            *result = match_string_pat(&children[0], s, caps);
        }
        (NodeName::Declaration, AstStatement::Declaration(var, opt_init))
            if children.len() == 2 =>
        {
            // match_variable_pat is a leaf, evaluate inline; then chain the opt expr
            let var_ok = match_variable_pat(&children[0], var, caps);
            if var_ok {
                stack.push(Work::MatchOptWrappedExpr(&children[1], opt_init));
            }
            *result = var_ok;
        }
        (NodeName::Call, AstStatement::Call(call)) if children.len() == 1 => {
            *result = match &children[0] {
                PatTree::Capture(name) => {
                    caps.insert(name.clone(), Captured::Call(call.clone()));
                    true
                }
                PatTree::Wildcard => true,
                _ => false,
            };
        }
        (
            NodeName::Goto,
            AstStatement::Goto(crate::abstract_syntax_tree::AstJumpTarget::Unknown(s)),
        ) if children.len() == 1 => {
            *result = match_string_pat(&children[0], s, caps);
        }
        (NodeName::Empty, AstStatement::Empty) if children.is_empty() => {
            *result = true;
        }
        (NodeName::Break, AstStatement::Break) if children.is_empty() => {
            *result = true;
        }
        (NodeName::Continue, AstStatement::Continue) if children.is_empty() => {
            *result = true;
        }
        (NodeName::Undefined, AstStatement::Undefined) if children.is_empty() => {
            *result = true;
        }
        _ => {
            *result = false;
        }
    }
}

fn expand_match_wrapped_expr<'a>(
    pat: &'a PatTree,
    expr: &'a Wrapped<AstExpression>,
    stack: &mut Vec<Work<'a>>,
    result: &mut bool,
    caps: &mut Captures,
) {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Expression(expr.clone()));
            *result = true;
        }
        PatTree::Wildcard => {
            *result = true;
        }
        PatTree::Node { name, children } => {
            expand_match_expr_node(*name, children, &expr.item, stack, result, caps);
        }
        _ => {
            *result = false;
        }
    }
}

fn expand_match_boxed_wrapped_expr<'a>(
    pat: &'a PatTree,
    expr: &'a Box<Wrapped<AstExpression>>,
    stack: &mut Vec<Work<'a>>,
    result: &mut bool,
    caps: &mut Captures,
) {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::ExpressionBox(expr.clone()));
            *result = true;
        }
        PatTree::Wildcard => {
            *result = true;
        }
        PatTree::Node { name, children } => {
            expand_match_expr_node(*name, children, &expr.item, stack, result, caps);
        }
        _ => {
            *result = false;
        }
    }
}

fn expand_match_expr_node<'a>(
    name: NodeName,
    children: &'a [PatTree],
    expr: &'a AstExpression,
    stack: &mut Vec<Work<'a>>,
    result: &mut bool,
    caps: &mut Captures,
) {
    match (name, expr) {
        (NodeName::Variable, AstExpression::Variable(map, var_id)) if children.len() == 2 => {
            // Both are leaf matchers, but we chain them for short-circuit
            let map_ok = match_variable_map_pat(&children[0], map, caps);
            if map_ok {
                *result = match_variable_id_pat(&children[1], var_id, caps);
            } else {
                *result = false;
            }
        }
        (NodeName::Literal, AstExpression::Literal(lit)) if children.len() == 1 => {
            *result = match_literal_pat(&children[0], lit, caps);
        }
        (NodeName::UnaryOp, AstExpression::UnaryOp(op, arg)) if children.len() == 2 => {
            let op_ok = match_unary_op_pat(&children[0], op, caps);
            if op_ok {
                stack.push(Work::MatchBoxedWrappedExpr(&children[1], arg));
            }
            *result = op_ok;
        }
        (NodeName::BinaryOp, AstExpression::BinaryOp(op, lhs, rhs)) if children.len() == 3 => {
            let op_ok = match_binary_op_pat(&children[0], op, caps);
            if op_ok {
                push_and_chain(
                    stack,
                    &[
                        Work::MatchBoxedWrappedExpr(&children[1], lhs),
                        Work::MatchBoxedWrappedExpr(&children[2], rhs),
                    ],
                );
            }
            *result = op_ok;
        }
        (NodeName::Ternary, AstExpression::Ternary(cond, t, f)) if children.len() == 3 => {
            push_and_chain(
                stack,
                &[
                    Work::MatchBoxedWrappedExpr(&children[0], cond),
                    Work::MatchBoxedWrappedExpr(&children[1], t),
                    Work::MatchBoxedWrappedExpr(&children[2], f),
                ],
            );
            *result = true;
        }
        (NodeName::Cast, AstExpression::Cast(ty, arg)) if children.len() == 2 => {
            let ty_ok = match_value_type_pat(&children[0], ty, caps);
            if ty_ok {
                stack.push(Work::MatchBoxedWrappedExpr(&children[1], arg));
            }
            *result = ty_ok;
        }
        (NodeName::Deref, AstExpression::Deref(arg)) if children.len() == 1 => {
            stack.push(Work::MatchBoxedWrappedExpr(&children[0], arg));
            *result = true;
        }
        (NodeName::AddressOf, AstExpression::AddressOf(arg)) if children.len() == 1 => {
            stack.push(Work::MatchBoxedWrappedExpr(&children[0], arg));
            *result = true;
        }
        (NodeName::ArrayAccess, AstExpression::ArrayAccess(base, idx)) if children.len() == 2 => {
            push_and_chain(
                stack,
                &[
                    Work::MatchBoxedWrappedExpr(&children[0], base),
                    Work::MatchBoxedWrappedExpr(&children[1], idx),
                ],
            );
            *result = true;
        }
        (NodeName::MemberAccess, AstExpression::MemberAccess(base, _field))
            if children.len() == 2 =>
        {
            // Original: match_boxed_wrapped_expr(base) && pat_is_wildcard_or_capture(field)
            // pat_is_wildcard_or_capture is a leaf that never mutates caps (despite
            // the signature), so we can push the boxed match and evaluate the leaf
            // after via SetResult. But to preserve exact left-to-right order with
            // short-circuit, push the boxed match first, then a Guard, then the leaf.
            // However, the leaf doesn't go on the stack. Instead: push the boxed
            // match, and when it completes (setting result), the Guard+SetResult
            // will handle the second operand.
            //
            // Simplest correct approach: push items so that base is evaluated first,
            // then if it succeeds, check the leaf.
            stack.push(Work::SetResult(pat_is_wildcard_or_capture(
                &children[1],
                caps,
            )));
            stack.push(Work::Guard(1));
            stack.push(Work::MatchBoxedWrappedExpr(&children[0], base));
            *result = true;
        }
        (NodeName::Call, AstExpression::Call(call)) if children.len() == 1 => {
            *result = match &children[0] {
                PatTree::Capture(name) => {
                    caps.insert(name.clone(), Captured::Call(call.clone()));
                    true
                }
                PatTree::Wildcard => true,
                _ => false,
            };
        }
        (NodeName::Unknown, AstExpression::Unknown) if children.is_empty() => {
            *result = true;
        }
        (NodeName::Undefined, AstExpression::Undefined) if children.is_empty() => {
            *result = true;
        }
        _ => {
            *result = false;
        }
    }
}

fn expand_match_stmt_list<'a>(
    pat: &'a PatTree,
    stmts: &'a Vec<WrappedAstStatement>,
    stack: &mut Vec<Work<'a>>,
    result: &mut bool,
    caps: &mut Captures,
) {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::StmtList(stmts.clone()));
            *result = true;
        }
        PatTree::Wildcard => {
            *result = true;
        }
        PatTree::List(pats) => {
            if pats.len() != stmts.len() {
                *result = false;
                return;
            }
            if pats.is_empty() {
                *result = true;
                return;
            }
            // Build AND-chain of MatchStmt for each (pat, stmt) pair
            let items: Vec<Work<'a>> = pats
                .iter()
                .zip(stmts.iter())
                .map(|(p, s)| Work::MatchStmt(p, &s.statement))
                .collect();
            push_and_chain(stack, &items);
            *result = true;
        }
        _ => {
            *result = false;
        }
    }
}

fn expand_match_opt_stmt_list<'a>(
    pat: &'a PatTree,
    opt: &'a Option<Vec<WrappedAstStatement>>,
    stack: &mut Vec<Work<'a>>,
    result: &mut bool,
    caps: &mut Captures,
) {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::OptStmtList(opt.clone()));
            *result = true;
        }
        PatTree::Wildcard => {
            *result = true;
        }
        PatTree::OptionNone => {
            *result = opt.is_none();
        }
        PatTree::OptionSome(inner) => match opt {
            Some(stmts) => {
                stack.push(Work::MatchStmtList(inner, stmts));
                *result = true;
            }
            None => {
                *result = false;
            }
        },
        _ => {
            *result = false;
        }
    }
}

fn expand_match_opt_wrapped_expr<'a>(
    pat: &'a PatTree,
    opt: &'a Option<Wrapped<AstExpression>>,
    stack: &mut Vec<Work<'a>>,
    result: &mut bool,
    caps: &mut Captures,
) {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::OptExpression(opt.clone()));
            *result = true;
        }
        PatTree::Wildcard => {
            *result = true;
        }
        PatTree::OptionNone => {
            *result = opt.is_none();
        }
        PatTree::OptionSome(inner) => match opt {
            Some(expr) => {
                stack.push(Work::MatchWrappedExpr(inner, expr));
                *result = true;
            }
            None => {
                *result = false;
            }
        },
        _ => {
            *result = false;
        }
    }
}

// ---------------------------------------------------------------------------
// Leaf matchers (no recursion into the cycle — kept as plain functions)
// ---------------------------------------------------------------------------

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
