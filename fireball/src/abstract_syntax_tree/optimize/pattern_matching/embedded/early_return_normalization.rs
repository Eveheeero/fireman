//! Guard clause normalization for early returns.
//!
//! Converts if-else with a return in the true branch into a guard clause:
//!   if(cond) { return X; } else { body... }  →  if(cond) { return X; } body...

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn normalize_early_returns(
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

    normalize_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::EarlyReturnNormalization);
    }

    Ok(())
}

fn normalize_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                normalize_in_list(bt);
                if let Some(bf) = bf {
                    normalize_in_list(bf);
                }
            }
            AstStatement::While(_, body) => normalize_in_list(body),
            AstStatement::For(_, _, _, body) => normalize_in_list(body),
            AstStatement::Block(body) => normalize_in_list(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    normalize_in_list(case_body);
                }
                if let Some(default_body) = default {
                    normalize_in_list(default_body);
                }
            }
            _ => {}
        }
    }

    // Apply the transform iteratively: each pass may expose new opportunities.
    loop {
        let mut changed = false;
        let mut i = 0;
        while i < stmts.len() {
            if try_normalize_early_return(stmts, i) {
                changed = true;
                // Don't increment — re-check at same index in case it chains.
            } else {
                i += 1;
            }
        }
        if !changed {
            break;
        }
    }
}

/// Transform: `if (cond) { return X; } else { body... }` → `if (cond) { return X; } body...`
///
/// Safety guards:
/// - Then-branch must be exactly one `Return` statement
/// - Must have an else branch
/// - Else-branch must NOT contain `Label` or `Goto` (preserves jump targets)
/// - Else-branch must NOT contain `Declaration` (avoids scope/shadowing)
fn try_normalize_early_return(stmts: &mut Vec<WrappedAstStatement>, idx: usize) -> bool {
    let AstStatement::If(_, branch_true, Some(branch_false)) = &stmts[idx].statement else {
        return false;
    };

    // Then-branch must be exactly one Return.
    if branch_true.len() != 1 {
        return false;
    }
    if !matches!(&branch_true[0].statement, AstStatement::Return(_)) {
        return false;
    }

    // Else-branch must not contain Label, Goto, or Declaration.
    if branch_contains_unsafe_stmts(branch_false) {
        return false;
    }

    // Safe to transform: extract the else body and splice it after the if.
    let AstStatement::If(cond, branch_true, Some(branch_false)) =
        std::mem::replace(&mut stmts[idx].statement, AstStatement::Empty)
    else {
        unreachable!();
    };
    let origin = stmts[idx].origin.clone();
    let comment = stmts[idx].comment.clone();

    // Rebuild the if without else.
    stmts[idx] = WrappedAstStatement {
        statement: AstStatement::If(cond, branch_true, None),
        origin,
        comment,
    };

    // Insert the former else-body statements after the if.
    let insert_pos = idx + 1;
    for (j, else_stmt) in branch_false.into_iter().enumerate() {
        stmts.insert(insert_pos + j, else_stmt);
    }

    true
}

/// Check if a statement list contains Label, Goto, or Declaration at any depth.
fn branch_contains_unsafe_stmts(stmts: &[WrappedAstStatement]) -> bool {
    for stmt in stmts {
        match &stmt.statement {
            AstStatement::Label(_) | AstStatement::Goto(_) | AstStatement::Declaration(_, _) => {
                return true;
            }
            AstStatement::If(_, bt, bf) => {
                if branch_contains_unsafe_stmts(bt) {
                    return true;
                }
                if let Some(bf) = bf {
                    if branch_contains_unsafe_stmts(bf) {
                        return true;
                    }
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                if branch_contains_unsafe_stmts(body) {
                    return true;
                }
            }
            AstStatement::For(init, _, update, body) => {
                if branch_contains_unsafe_stmts(&[*init.clone()])
                    || branch_contains_unsafe_stmts(&[*update.clone()])
                    || branch_contains_unsafe_stmts(body)
                {
                    return true;
                }
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    if branch_contains_unsafe_stmts(case_body) {
                        return true;
                    }
                }
                if let Some(default_body) = default {
                    if branch_contains_unsafe_stmts(default_body) {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}
