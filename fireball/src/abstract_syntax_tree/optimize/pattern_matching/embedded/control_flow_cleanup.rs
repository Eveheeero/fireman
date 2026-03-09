//! Tail-call merge, branch inversion, and same-condition-if merging
//! extracted from control_flow_cleanup.rs.
//!
//! These transformations improve readability of decompiled control flow:
//!   - Merge trailing `Call(c); Return(None)` into `Return(Some(Call(c)))`
//!   - Invert `if(!cond) { A } else { B }` to `if(cond) { B } else { A }`
//!     when the else branch is larger
//!   - Merge `if(cond) { A } if(cond) { B }` into `if(cond) { A; B }`

use crate::{
    abstract_syntax_tree::{
        Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement, AstUnaryOperator,
        AstValueOrigin, ProcessedOptimization, Wrapped, WrappedAstStatement, optimize::opt_utils,
    },
    prelude::DecompileError,
};

/// Extract the function body, apply tail-call merging and branch inversion,
/// then put the body back.
pub(crate) fn cleanup_tail_calls_and_branches(
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

    merge_trailing_call_return(&mut body);
    invert_negated_branches(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::ControlFlowCleanup);
    }

    Ok(())
}

/// Merge a trailing `Call(c); Return(None)` pair into `Return(Some(Call(c)))`,
/// making the tail-call explicit in the AST.
pub(crate) fn merge_trailing_call_return(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                merge_trailing_call_return(bt);
                if let Some(bf) = bf {
                    merge_trailing_call_return(bf);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::Block(body) => {
                merge_trailing_call_return(body);
            }
            AstStatement::For(_, _, _, body) => {
                merge_trailing_call_return(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    merge_trailing_call_return(case_body);
                }
                if let Some(default_body) = default {
                    merge_trailing_call_return(default_body);
                }
            }
            _ => {}
        }
    }

    // Find the indices of the last two meaningful (non-comment, non-empty) statements.
    let meaningful: Vec<usize> = stmts
        .iter()
        .enumerate()
        .filter(|(_, s)| !matches!(&s.statement, AstStatement::Comment(_) | AstStatement::Empty))
        .map(|(i, _)| i)
        .collect();

    if meaningful.len() < 2 {
        return;
    }

    let call_idx = meaningful[meaningful.len() - 2];
    let ret_idx = meaningful[meaningful.len() - 1];

    let is_call = matches!(&stmts[call_idx].statement, AstStatement::Call(_));
    let is_return_none = matches!(&stmts[ret_idx].statement, AstStatement::Return(None));

    if is_call && is_return_none {
        // Remove the Return(None) first (higher index), then the Call.
        stmts.remove(ret_idx);
        let removed_call = stmts.remove(call_idx);

        if let AstStatement::Call(call) = removed_call.statement {
            let return_stmt = WrappedAstStatement {
                statement: AstStatement::Return(Some(Wrapped {
                    item: AstExpression::Call(call),
                    origin: AstValueOrigin::Unknown,
                    comment: None,
                })),
                origin: removed_call.origin,
                comment: removed_call.comment,
            };
            stmts.insert(call_idx, return_stmt);
        }
    }
}

/// Invert `if(!cond) { A } else { B }` -> `if(cond) { B } else { A }` when doing
/// so keeps the larger branch on the positive path and improves readability.
pub(crate) fn invert_negated_branches(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(cond, bt, bf) => {
                invert_negated_branches(bt);
                if let Some(bf) = bf {
                    invert_negated_branches(bf);
                }
                // Only invert when both branches exist and condition is `!expr`.
                if let Some(bf) = bf {
                    if let AstExpression::UnaryOp(AstUnaryOperator::Not, inner) = &cond.item {
                        let true_branch_size = meaningful_statement_count(bt);
                        let false_branch_size = meaningful_statement_count(bf);

                        if false_branch_size > true_branch_size {
                            // Unwrap the negation only when the positive path stays dominant.
                            cond.item = inner.item.clone();
                            std::mem::swap(bt, bf);
                        }
                    }
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                invert_negated_branches(body);
            }
            AstStatement::For(_, _, _, body) => invert_negated_branches(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    invert_negated_branches(case_body);
                }
                if let Some(default_body) = default {
                    invert_negated_branches(default_body);
                }
            }
            _ => {}
        }
    }
}

pub(crate) fn meaningful_statement_count(stmts: &[WrappedAstStatement]) -> usize {
    stmts
        .iter()
        .filter(|stmt| {
            !matches!(
                &stmt.statement,
                AstStatement::Comment(_) | AstStatement::Empty
            )
        })
        .count()
}

/// Merge consecutive `if(cond) { A } if(cond) { B }` into `if(cond) { A; B }`
/// when the condition is pure and the first if has no else branch.
pub(crate) fn merge_same_condition_ifs(
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

    merge_consecutive_same_condition_ifs(&mut body);

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

fn merge_consecutive_same_condition_ifs(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                merge_consecutive_same_condition_ifs(bt);
                if let Some(bf) = bf {
                    merge_consecutive_same_condition_ifs(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                merge_consecutive_same_condition_ifs(body);
            }
            AstStatement::For(_, _, _, body) => merge_consecutive_same_condition_ifs(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    merge_consecutive_same_condition_ifs(case_body);
                }
                if let Some(default_body) = default {
                    merge_consecutive_same_condition_ifs(default_body);
                }
            }
            _ => {}
        }
    }

    // Merge at this level.
    let mut i = 0;
    while i + 1 < stmts.len() {
        let should_merge = {
            let (first, rest) = stmts.split_at(i + 1);
            let first_stmt = &first[i].statement;
            let next_stmt = &rest[0].statement;
            match (first_stmt, next_stmt) {
                (AstStatement::If(cond1, _, None), AstStatement::If(cond2, _, None)) => {
                    opt_utils::is_pure_expression(&cond1.item)
                        && opt_utils::is_pure_expression(&cond2.item)
                        && opt_utils::expr_structurally_equal(&cond1.item, &cond2.item)
                }
                _ => false,
            }
        };

        if should_merge {
            let removed = stmts.remove(i + 1);
            if let AstStatement::If(_, mut body2, _) = removed.statement {
                if let AstStatement::If(_, body1, _) = &mut stmts[i].statement {
                    body1.append(&mut body2);
                }
            }
            // Don't increment — check for more consecutive matches.
        } else {
            i += 1;
        }
    }
}
