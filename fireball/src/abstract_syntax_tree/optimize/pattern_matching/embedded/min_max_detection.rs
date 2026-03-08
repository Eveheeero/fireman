//! Detect MIN/MAX ternary patterns and annotate assignments with comments.
//!
//! Extracted from auto_comment.rs for parity testing with min-max-detection.fb.

use crate::abstract_syntax_tree::{
    optimize::opt_utils,
    Ast, AstBinaryOperator, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement,
    AstStatementOrigin, WrappedAstStatement,
};
use crate::prelude::DecompileError;

pub(crate) fn annotate_min_max(
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

    annotate_min_max_in_stmts(&mut body);

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

fn annotate_min_max_in_stmts(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                annotate_min_max_in_stmts(bt);
                if let Some(bf) = bf {
                    annotate_min_max_in_stmts(bf);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::Block(body) => {
                annotate_min_max_in_stmts(body);
            }
            AstStatement::For(_, _, _, body) => annotate_min_max_in_stmts(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_min_max_in_stmts(case_body);
                }
                if let Some(default_body) = default {
                    annotate_min_max_in_stmts(default_body);
                }
            }
            _ => {}
        }
    }

    // Detect and insert comments.
    let mut insertions: Vec<(usize, String)> = Vec::new();
    for (i, stmt) in stmts.iter().enumerate() {
        if let AstStatement::Assignment(_, rhs) = &stmt.statement {
            if let Some(label) = detect_min_max(&rhs.item) {
                insertions.push((i, format!("// {label}")));
            }
        }
    }

    for (offset, (idx, comment)) in insertions.into_iter().enumerate() {
        stmts.insert(
            idx + offset,
            WrappedAstStatement {
                statement: AstStatement::Comment(comment),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn detect_min_max(expr: &AstExpression) -> Option<&'static str> {
    let AstExpression::Ternary(cond, true_expr, false_expr) = expr else {
        return None;
    };
    let AstExpression::BinaryOp(op, cond_lhs, cond_rhs) = &cond.item else {
        return None;
    };

    let true_eq_lhs = opt_utils::expr_structurally_equal(&true_expr.item, &cond_lhs.item);
    let false_eq_rhs = opt_utils::expr_structurally_equal(&false_expr.item, &cond_rhs.item);
    let true_eq_rhs = opt_utils::expr_structurally_equal(&true_expr.item, &cond_rhs.item);
    let false_eq_lhs = opt_utils::expr_structurally_equal(&false_expr.item, &cond_lhs.item);

    match op {
        AstBinaryOperator::Less | AstBinaryOperator::LessEqual => {
            if true_eq_lhs && false_eq_rhs {
                Some("MIN")
            } else if true_eq_rhs && false_eq_lhs {
                Some("MAX")
            } else {
                None
            }
        }
        AstBinaryOperator::Greater | AstBinaryOperator::GreaterEqual => {
            if true_eq_lhs && false_eq_rhs {
                Some("MAX")
            } else if true_eq_rhs && false_eq_lhs {
                Some("MIN")
            } else {
                None
            }
        }
        _ => None,
    }
}
