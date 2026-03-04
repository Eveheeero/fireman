use super::{
    opt_utils::expr_structurally_equal,
    pattern_matching::{Blake3StdHasher, hash_statement_list},
};
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

    // Try to convert if-else chains to switch (linear chains first, then binary-search trees)
    for stmt in stmts.iter_mut() {
        try_convert_to_switch(stmt);
    }
    for stmt in stmts.iter_mut() {
        try_convert_binary_search_to_switch(stmt);
    }

    // Cluster switch cases with identical bodies
    for stmt in stmts.iter_mut() {
        cluster_switch_cases(stmt);
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

/// Detect binary-search switch trees: nested if-else using `<`/`<=`/`>`/`>=` to split
/// value ranges, with `==` equality checks in leaf branches. Collect all cases and
/// build a switch statement.
fn try_convert_binary_search_to_switch(stmt: &mut WrappedAstStatement) {
    let AstStatement::If(_, _, _) = &stmt.statement else {
        return;
    };

    let mut cases: Vec<(AstLiteral, Vec<WrappedAstStatement>)> = Vec::new();
    let mut discriminant_expr: Option<AstExpression> = None;
    let mut default_body: Option<Vec<WrappedAstStatement>> = None;

    if !collect_binary_search_cases(
        &stmt.statement,
        &mut discriminant_expr,
        &mut cases,
        &mut default_body,
    ) {
        return;
    }

    if cases.len() < MIN_SWITCH_CASES {
        return;
    }

    let disc = discriminant_expr.unwrap();
    let disc_wrapped = crate::abstract_syntax_tree::Wrapped {
        item: disc,
        origin: match &stmt.statement {
            AstStatement::If(cond, _, _) => cond.origin.clone(),
            _ => unreachable!(),
        },
        comment: None,
    };

    // Sort cases by literal value for consistent output
    cases.sort_by(|(a, _), (b, _)| cmp_literal(a, b));

    stmt.statement = AstStatement::Switch(disc_wrapped, cases, default_body);
}

/// Recursively collect equality cases from a binary-search if-else tree.
/// Returns false if the tree doesn't match the expected pattern.
fn collect_binary_search_cases(
    stmt: &AstStatement,
    discriminant: &mut Option<AstExpression>,
    cases: &mut Vec<(AstLiteral, Vec<WrappedAstStatement>)>,
    default_body: &mut Option<Vec<WrappedAstStatement>>,
) -> bool {
    let AstStatement::If(cond, branch_true, branch_false) = stmt else {
        return false;
    };

    match &cond.item {
        // Equality case: if (x == C) { body } else { ... }
        AstExpression::BinaryOp(AstBinaryOperator::Equal, left, right) => {
            let (var_expr, literal) = match (&left.item, &right.item) {
                (_, AstExpression::Literal(lit)) => (&left.item, lit.clone()),
                (AstExpression::Literal(lit), _) => (&right.item, lit.clone()),
                _ => return false,
            };
            if !check_or_set_discriminant(discriminant, var_expr) {
                return false;
            }
            cases.push((literal, branch_true.clone()));
            // Process else branch
            match branch_false {
                Some(else_stmts) if else_stmts.len() == 1 => {
                    if matches!(&else_stmts[0].statement, AstStatement::If(..)) {
                        collect_binary_search_cases(
                            &else_stmts[0].statement,
                            discriminant,
                            cases,
                            default_body,
                        )
                    } else {
                        *default_body = Some(else_stmts.clone());
                        true
                    }
                }
                Some(else_stmts) => {
                    *default_body = Some(else_stmts.clone());
                    true
                }
                None => true,
            }
        }
        // Range split: if (x < C) or if (x <= C) or if (x > C) or if (x >= C)
        AstExpression::BinaryOp(op, left, right)
            if matches!(
                op,
                AstBinaryOperator::Less
                    | AstBinaryOperator::LessEqual
                    | AstBinaryOperator::Greater
                    | AstBinaryOperator::GreaterEqual
            ) =>
        {
            // One side must be a variable/expression, other must be a literal (threshold)
            let var_expr = if matches!(&right.item, AstExpression::Literal(_)) {
                &left.item
            } else if matches!(&left.item, AstExpression::Literal(_)) {
                &right.item
            } else {
                return false;
            };

            if !check_or_set_discriminant(discriminant, var_expr) {
                return false;
            }

            // Both branches must contain further if-else or be leaf equality checks
            let Some(else_stmts) = branch_false else {
                return false;
            };

            let true_ok = collect_cases_from_branch(branch_true, discriminant, cases, default_body);
            let false_ok =
                collect_cases_from_branch(else_stmts, discriminant, cases, default_body);

            true_ok && false_ok
        }
        _ => false,
    }
}

fn collect_cases_from_branch(
    stmts: &[WrappedAstStatement],
    discriminant: &mut Option<AstExpression>,
    cases: &mut Vec<(AstLiteral, Vec<WrappedAstStatement>)>,
    default_body: &mut Option<Vec<WrappedAstStatement>>,
) -> bool {
    if stmts.len() == 1 {
        if matches!(&stmts[0].statement, AstStatement::If(..)) {
            return collect_binary_search_cases(
                &stmts[0].statement,
                discriminant,
                cases,
                default_body,
            );
        }
    }
    // Non-if branch in a binary search tree — could be default body
    // Only set default if not already set
    if default_body.is_none() {
        *default_body = Some(stmts.to_vec());
        true
    } else {
        // Multiple default candidates — not a clean binary search tree
        false
    }
}

fn check_or_set_discriminant(disc: &mut Option<AstExpression>, var_expr: &AstExpression) -> bool {
    if let Some(existing) = disc.as_ref() {
        expr_structurally_equal(existing, var_expr)
    } else {
        *disc = Some(var_expr.clone());
        true
    }
}

fn cmp_literal(a: &AstLiteral, b: &AstLiteral) -> std::cmp::Ordering {
    match (a, b) {
        (AstLiteral::Int(a), AstLiteral::Int(b)) => a.cmp(b),
        (AstLiteral::UInt(a), AstLiteral::UInt(b)) => a.cmp(b),
        (AstLiteral::Int(a), AstLiteral::UInt(b)) => (*a as i128).cmp(&(*b as i128)),
        (AstLiteral::UInt(a), AstLiteral::Int(b)) => (*a as i128).cmp(&(*b as i128)),
        _ => std::cmp::Ordering::Equal,
    }
}

/// Cluster *adjacent* switch cases that have identical bodies into multi-label cases.
/// For example: `case 1: body; case 2: body;` → `case 1: case 2: body;`
///
/// Only merges contiguous runs to preserve case ordering and fallthrough semantics.
/// Uses full 256-bit blake3 structural hashing (which walks the full AST) for equality —
/// collision probability is negligible (2^-128 for birthday attacks).
fn cluster_switch_cases(stmt: &mut WrappedAstStatement) {
    let AstStatement::Switch(_, cases, _) = &mut stmt.statement else {
        return;
    };
    if cases.len() < 2 {
        return;
    }

    fn body_digest(body: &[WrappedAstStatement]) -> [u8; 32] {
        let mut hasher = Blake3StdHasher::new();
        hash_statement_list(&mut hasher, body);
        hasher.finish_bytes()
    }

    // Compute full 256-bit digests for all case bodies.
    let digests: Vec<[u8; 32]> = cases.iter().map(|(_, body)| body_digest(body)).collect();

    // Merge adjacent cases with identical digests (identical structural content).
    // Walk backwards so index arithmetic stays simple.
    let mut i = cases.len() - 1;
    while i > 0 {
        if digests[i] == digests[i - 1] {
            // Adjacent cases have identical bodies — clear the earlier one's body
            // so it becomes a fallthrough label.
            cases[i - 1].1.clear();
        }
        i -= 1;
    }
}
