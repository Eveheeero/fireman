use super::opt_utils::expr_structurally_equal;
use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral,
        AstStatement, ProcessedOptimization, WrappedAstStatement,
    },
    prelude::DecompileError,
};

/// Minimum number of cases required to form a switch.
const MIN_SWITCH_CASES: usize = 3;

pub(super) fn reconstruct_switches(
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

    reconstruct_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::SwitchReconstruction);
    }

    Ok(())
}

fn reconstruct_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                reconstruct_in_list(bt);
                if let Some(bf) = bf {
                    reconstruct_in_list(bf);
                }
            }
            AstStatement::While(_, body) => reconstruct_in_list(body),
            AstStatement::For(_, _, _, body) => reconstruct_in_list(body),
            AstStatement::Block(body) => reconstruct_in_list(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    reconstruct_in_list(case_body);
                }
                if let Some(default_body) = default {
                    reconstruct_in_list(default_body);
                }
            }
            _ => {}
        }
    }

    // Try to convert if-else chains to switch
    for stmt in stmts.iter_mut() {
        try_convert_to_switch(stmt);
    }
}

/// Detect chains of `if (x == c1) { ... } else if (x == c2) { ... } else if (x == c3) { ... } else { ... }`
/// and convert to `switch (x) { case c1: ...; case c2: ...; case c3: ...; default: ...; }`
fn try_convert_to_switch(stmt: &mut WrappedAstStatement) {
    let mut cases: Vec<(AstLiteral, Vec<WrappedAstStatement>)> = Vec::new();
    let mut discriminant_expr = None;
    let mut default_body: Option<Vec<WrappedAstStatement>> = None;

    // Walk the if-else chain, collecting cases and the trailing default.
    // `pending_else` tracks the else body we're about to descend into. If we
    // break out of the loop due to a pattern mismatch, `pending_else` becomes
    // the default, preserving the remaining branches.
    let mut current = &stmt.statement;
    let mut pending_else: Option<&Vec<WrappedAstStatement>> = None;
    loop {
        let AstStatement::If(cond, branch_true, branch_false) = current else {
            // `current` is not an If — the pending_else (which contained this
            // non-If statement) is the default body.
            if let Some(else_stmts) = pending_else {
                default_body = Some(else_stmts.clone());
            }
            break;
        };

        // Condition must be `x == literal`
        let matched = match &cond.item {
            AstExpression::BinaryOp(AstBinaryOperator::Equal, left, right) => {
                match (&left.item, &right.item) {
                    (_, AstExpression::Literal(lit)) => Some((&left.item, lit.clone())),
                    (AstExpression::Literal(lit), _) => Some((&right.item, lit.clone())),
                    _ => None,
                }
            }
            _ => None,
        };

        let Some((var_expr, literal)) = matched else {
            // Pattern mismatch: the remaining else (which contains this If and
            // its subtree) becomes the default.
            if let Some(else_stmts) = pending_else {
                default_body = Some(else_stmts.clone());
            }
            break;
        };

        // All cases must compare the same expression
        if let Some(ref disc) = discriminant_expr {
            if !expr_structurally_equal(disc, var_expr) {
                // Discriminant mismatch: remaining chain becomes default
                if let Some(else_stmts) = pending_else {
                    default_body = Some(else_stmts.clone());
                }
                break;
            }
        } else {
            discriminant_expr = Some(var_expr.clone());
        }

        cases.push((literal, branch_true.clone()));

        // Follow else chain
        match branch_false {
            Some(else_stmts) if else_stmts.len() == 1 => {
                if matches!(&else_stmts[0].statement, AstStatement::If(..)) {
                    pending_else = Some(else_stmts);
                    current = &else_stmts[0].statement;
                } else {
                    // Single non-if statement in else: this is the default body
                    default_body = Some(else_stmts.clone());
                    break;
                }
            }
            Some(else_stmts) => {
                // Multi-statement else: this is the default body
                default_body = Some(else_stmts.clone());
                break;
            }
            None => {
                // No else: no default
                break;
            }
        }
    }

    if cases.len() < MIN_SWITCH_CASES {
        return;
    }

    // Build the switch discriminant
    let disc = discriminant_expr.unwrap();
    let disc_wrapped = crate::abstract_syntax_tree::Wrapped {
        item: disc,
        origin: match &stmt.statement {
            AstStatement::If(cond, _, _) => cond.origin.clone(),
            _ => unreachable!(),
        },
        comment: None,
    };

    stmt.statement = AstStatement::Switch(disc_wrapped, cases, default_body);
}
