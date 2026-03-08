//! Annotate sentinel-value comparisons (x == -1, x != 0xFFFFFFFF, etc.)
//! with expression-level comments.
//!
//! Extracted from bit_trick_recognition.rs for parity testing with sentinel-comparison.fb.

use crate::abstract_syntax_tree::{
    Ast, AstBinaryOperator, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral,
    AstStatement, Wrapped, WrappedAstStatement,
};
use crate::prelude::DecompileError;

pub(crate) fn annotate_sentinel_comparisons(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let mut body;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
    }

    annotate_in_stmts(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
    }

    Ok(())
}

fn annotate_in_stmts(stmts: &mut [WrappedAstStatement]) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(cond, bt, bf) => {
                annotate_in_expr(cond);
                annotate_in_stmts(bt);
                if let Some(bf) = bf {
                    annotate_in_stmts(bf);
                }
            }
            AstStatement::Assignment(lhs, rhs) => {
                annotate_in_expr(lhs);
                annotate_in_expr(rhs);
            }
            AstStatement::Declaration(_, Some(rhs)) => {
                annotate_in_expr(rhs);
            }
            AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
                annotate_in_expr(cond);
                annotate_in_stmts(body);
            }
            AstStatement::For(_, cond, _, body) => {
                annotate_in_expr(cond);
                annotate_in_stmts(body);
            }
            AstStatement::Switch(discrim, cases, default) => {
                annotate_in_expr(discrim);
                for (_, case_body) in cases.iter_mut() {
                    annotate_in_stmts(case_body);
                }
                if let Some(default_body) = default {
                    annotate_in_stmts(default_body);
                }
            }
            AstStatement::Block(body) => annotate_in_stmts(body),
            AstStatement::Return(Some(expr)) => annotate_in_expr(expr),
            _ => {}
        }
    }
}

fn annotate_in_expr(expr: &mut Wrapped<AstExpression>) {
    // Recurse first.
    match &mut expr.item {
        AstExpression::UnaryOp(_, arg) => annotate_in_expr(arg),
        AstExpression::BinaryOp(_, left, right) => {
            annotate_in_expr(left);
            annotate_in_expr(right);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => annotate_in_expr(arg),
        AstExpression::ArrayAccess(base, idx) => {
            annotate_in_expr(base);
            annotate_in_expr(idx);
        }
        AstExpression::Ternary(cond, t, f) => {
            annotate_in_expr(cond);
            annotate_in_expr(t);
            annotate_in_expr(f);
        }
        _ => {}
    }

    if expr.comment.is_some() {
        return;
    }

    if let Some(comment) = detect_sentinel(&expr.item) {
        expr.comment = Some(comment.to_string());
    }
}

fn detect_sentinel(expr: &AstExpression) -> Option<&'static str> {
    let AstExpression::BinaryOp(op, lhs, rhs) = expr else {
        return None;
    };
    if !matches!(op, AstBinaryOperator::Equal | AstBinaryOperator::NotEqual) {
        return None;
    }

    extract_sentinel_kind(&rhs.item)
        .or_else(|| extract_sentinel_kind(&lhs.item))
}

fn extract_sentinel_kind(expr: &AstExpression) -> Option<&'static str> {
    match expr {
        AstExpression::Literal(AstLiteral::Int(-1)) => {
            Some("sentinel check (-1 / INVALID_HANDLE_VALUE)")
        }
        AstExpression::Literal(AstLiteral::UInt(0xFFFFFFFF)) => {
            Some("sentinel check (0xFFFFFFFF)")
        }
        AstExpression::Literal(AstLiteral::UInt(0xFFFFFFFFFFFFFFFF)) => {
            Some("sentinel check (-1 / INVALID_HANDLE_VALUE)")
        }
        _ => None,
    }
}
