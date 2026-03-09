//! Simplify control flow: dead branches, tail-call merge, branch inversion.

use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral, AstStatement,
        AstStatementOrigin, AstUnaryOperator, AstValueOrigin, ProcessedOptimization, Wrapped,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerminationOutcome {
    NoTerminate,
    ReturnToCaller,
    NoReturn,
}

pub(super) fn cleanup_control_flow(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let noreturn_targets = collect_noreturn_targets(ast);
    let mut body;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
    }

    cleanup_statement_list(&mut body, &noreturn_targets);
    prune_constant_condition_branches(&mut body);
    factor_common_tails(&mut body);
    invert_negated_branches(&mut body);
    merge_consecutive_same_condition_ifs(&mut body);
    annotate_goto_cleanup_patterns(&mut body);
    annotate_register_spill_patterns(&mut body);
    annotate_likely_unrolled_loops(&mut body);

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

fn cleanup_statement_list(
    stmts: &mut Vec<WrappedAstStatement>,
    noreturn_targets: &HashSet<AstFunctionId>,
) {
    for stmt in stmts.iter_mut() {
        cleanup_statement(stmt, noreturn_targets);
    }

    if let Some((index, _outcome)) = first_terminal_index(stmts, noreturn_targets) {
        stmts.truncate(index + 1);
    }

    // Tail-call detection: merge trailing Call + Return(None) into Return(Some(Call(...))).
    merge_trailing_call_return(stmts);

    // Thunk/wrapper annotation: mark trivial forwarding bodies with a comment.
    if detect_thunk_functions(stmts) {
        stmts.insert(
            0,
            WrappedAstStatement {
                statement: AstStatement::Comment("// thunk".to_string()),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

/// Merge a trailing `Call(c); Return(None)` pair into `Return(Some(Call(c)))`,
/// making the tail-call explicit in the AST.
fn merge_trailing_call_return(stmts: &mut Vec<WrappedAstStatement>) {
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

/// Detect whether the function body is a trivial thunk/wrapper that only forwards
/// a call and returns. Returns `true` if the body (ignoring comments and empties) is:
/// - A single `Return(Some(Call(...)))`, or
/// - A single `Call(...)` followed by `Return(None)`.
fn detect_thunk_functions(stmts: &[WrappedAstStatement]) -> bool {
    let meaningful: Vec<&WrappedAstStatement> = stmts
        .iter()
        .filter(|s| !matches!(&s.statement, AstStatement::Comment(_) | AstStatement::Empty))
        .collect();

    match meaningful.as_slice() {
        [single] => matches!(
            &single.statement,
            AstStatement::Return(Some(expr)) if matches!(&expr.item, AstExpression::Call(_))
        ),
        [first, second] => {
            matches!(&first.statement, AstStatement::Call(_))
                && matches!(&second.statement, AstStatement::Return(None))
        }
        _ => false,
    }
}

fn meaningful_statement_count(stmts: &[WrappedAstStatement]) -> usize {
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

fn cleanup_statement(stmt: &mut WrappedAstStatement, noreturn_targets: &HashSet<AstFunctionId>) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            cleanup_statement_list(branch_true, noreturn_targets);
            if let Some(branch_false) = branch_false {
                cleanup_statement_list(branch_false, noreturn_targets);
            }
        }
        AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
            cleanup_statement_list(body, noreturn_targets)
        }
        AstStatement::For(init, _, update, body) => {
            cleanup_statement(init, noreturn_targets);
            cleanup_statement(update, noreturn_targets);
            cleanup_statement_list(body, noreturn_targets);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                cleanup_statement_list(case_body, noreturn_targets);
            }
            if let Some(default_body) = default {
                cleanup_statement_list(default_body, noreturn_targets);
            }
            // Annotate non-terminal switch cases with "fallthrough".
            annotate_switch_fallthrough(cases, noreturn_targets);
        }
        AstStatement::Block(body) => cleanup_statement_list(body, noreturn_targets),
        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::Return(_)
        | AstStatement::Call(_)
        | AstStatement::Label(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => {}
    }
}

/// Annotate switch cases that don't end with a terminal statement (return, break, etc.)
/// with a "fallthrough" comment on the last meaningful statement.
fn annotate_switch_fallthrough(
    cases: &mut [(AstLiteral, Vec<WrappedAstStatement>)],
    noreturn_targets: &HashSet<AstFunctionId>,
) {
    // Skip the last case — fallthrough from the last case is irrelevant.
    if cases.len() <= 1 {
        return;
    }
    for i in 0..cases.len() - 1 {
        let case_body = &mut cases[i].1;
        if case_body.is_empty() {
            continue;
        }
        let outcome = statement_list_outcome(case_body, noreturn_targets);
        if outcome != TerminationOutcome::NoTerminate {
            continue;
        }
        // Find the last meaningful (non-comment, non-empty) statement and annotate it.
        if let Some(last) = case_body
            .iter_mut()
            .rev()
            .find(|s| !matches!(&s.statement, AstStatement::Comment(_) | AstStatement::Empty))
        {
            if last.comment.is_none() {
                last.comment = Some("fallthrough".to_string());
            }
        }
    }
}

fn first_terminal_index(
    stmts: &[WrappedAstStatement],
    noreturn_targets: &HashSet<AstFunctionId>,
) -> Option<(usize, TerminationOutcome)> {
    for (index, stmt) in stmts.iter().enumerate() {
        let outcome = statement_outcome(&stmt.statement, noreturn_targets);
        if outcome == TerminationOutcome::NoTerminate {
            continue;
        }

        let has_label_after = stmts
            .iter()
            .skip(index + 1)
            .any(|next| matches!(&next.statement, AstStatement::Label(_)));
        if !has_label_after {
            return Some((index, outcome));
        }
    }
    None
}

fn statement_outcome(
    statement: &AstStatement,
    noreturn_targets: &HashSet<AstFunctionId>,
) -> TerminationOutcome {
    match statement {
        AstStatement::Return(_) => TerminationOutcome::ReturnToCaller,
        AstStatement::Undefined | AstStatement::Exception(_) => TerminationOutcome::NoReturn,
        AstStatement::Call(call) => {
            if call_is_noreturn(call, noreturn_targets) {
                TerminationOutcome::NoReturn
            } else {
                TerminationOutcome::NoTerminate
            }
        }
        AstStatement::Block(body) => statement_list_outcome(body, noreturn_targets),
        AstStatement::If(_, branch_true, Some(branch_false)) => combine_branch_outcomes(
            statement_list_outcome(branch_true, noreturn_targets),
            statement_list_outcome(branch_false, noreturn_targets),
        ),
        AstStatement::Switch(_, cases, default) => {
            // Switch terminates only if every case AND default all terminate
            if let Some(default_body) = default {
                let mut all_terminate = true;
                let mut combined = statement_list_outcome(default_body, noreturn_targets);
                if combined == TerminationOutcome::NoTerminate {
                    all_terminate = false;
                }
                for (_lit, case_body) in cases.iter() {
                    let case_outcome = statement_list_outcome(case_body, noreturn_targets);
                    if case_outcome == TerminationOutcome::NoTerminate {
                        all_terminate = false;
                        break;
                    }
                    combined = combine_branch_outcomes(combined, case_outcome);
                }
                if all_terminate {
                    combined
                } else {
                    TerminationOutcome::NoTerminate
                }
            } else {
                TerminationOutcome::NoTerminate
            }
        }
        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::If(_, _, None)
        | AstStatement::While(_, _)
        | AstStatement::DoWhile(_, _)
        | AstStatement::For(_, _, _, _)
        | AstStatement::Label(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Empty => TerminationOutcome::NoTerminate,
        // Break/Continue transfer control within a loop/switch but do not
        // cause a function-level return or noreturn, so for this analysis
        // (which tracks function-level termination) they are non-terminating.
        AstStatement::Break | AstStatement::Continue => TerminationOutcome::NoTerminate,
    }
}

fn statement_list_outcome(
    stmts: &[WrappedAstStatement],
    noreturn_targets: &HashSet<AstFunctionId>,
) -> TerminationOutcome {
    first_terminal_index(stmts, noreturn_targets)
        .map(|(_, outcome)| outcome)
        .unwrap_or(TerminationOutcome::NoTerminate)
}

fn combine_branch_outcomes(
    true_outcome: TerminationOutcome,
    false_outcome: TerminationOutcome,
) -> TerminationOutcome {
    if true_outcome == false_outcome {
        true_outcome
    } else {
        TerminationOutcome::NoTerminate
    }
}

fn collect_noreturn_targets(ast: &Ast) -> HashSet<AstFunctionId> {
    let mut active_ids: Vec<_> = ast.function_versions.keys().copied().collect();
    active_ids.sort_unstable();

    let functions = ast.functions.read().unwrap();
    let mut noreturn = HashSet::new();
    for function_id in active_ids.iter().copied() {
        let Some(function_version) = ast.function_versions.get(&function_id) else {
            continue;
        };
        let Some(function) = functions
            .get(&function_id)
            .and_then(|version_map| version_map.get(function_version))
        else {
            continue;
        };
        if function.name.as_deref().is_some_and(is_known_noreturn_name) {
            noreturn.insert(function_id);
        }
    }

    loop {
        let mut changed = false;
        for function_id in active_ids.iter().copied() {
            if noreturn.contains(&function_id) {
                continue;
            }
            let Some(function_version) = ast.function_versions.get(&function_id) else {
                continue;
            };
            let Some(function) = functions
                .get(&function_id)
                .and_then(|version_map| version_map.get(function_version))
            else {
                continue;
            };
            if statement_list_outcome(&function.body, &noreturn) == TerminationOutcome::NoReturn {
                noreturn.insert(function_id);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    noreturn
}

fn call_is_noreturn(call: &AstCall, noreturn_targets: &HashSet<AstFunctionId>) -> bool {
    match call {
        AstCall::Function { target, .. } => noreturn_targets.contains(target),
        AstCall::Unknown(name, _) => is_known_noreturn_name(name),
        AstCall::Variable { .. } | AstCall::Builtin(_, _) => false,
    }
}

fn is_known_noreturn_name(name: &str) -> bool {
    let normalized = name.to_ascii_lowercase();
    if normalized.contains("exitprocess")
        || normalized.contains("terminateprocess")
        || normalized.contains("__stack_chk_fail")
    {
        return true;
    }

    normalized
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(is_noreturn_token)
}

fn is_noreturn_token(token: &str) -> bool {
    matches!(
        token,
        "exit"
            | "_exit"
            | "quick_exit"
            | "abort"
            | "panic"
            | "terminate"
            | "fatal"
            | "__assert_fail"
    )
}

/// Prune branches with constant integer conditions that the constant folder missed.
/// `if(0) { A } else { B }` → B (or empty if no else).
/// `if(nonzero_int) { A } else { B }` → A.
fn constant_condition_truth(expr: &AstExpression) -> Option<bool> {
    match expr {
        AstExpression::Literal(AstLiteral::Int(value)) => Some(*value != 0),
        AstExpression::Literal(AstLiteral::UInt(value)) => Some(*value != 0),
        AstExpression::Literal(AstLiteral::Bool(value)) => Some(*value),
        AstExpression::UnaryOp(AstUnaryOperator::Not, inner) => {
            constant_condition_truth(&inner.item).map(|value| !value)
        }
        _ => None,
    }
}

fn prune_constant_condition_branches(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(cond, bt, bf) => {
                prune_constant_condition_branches(bt);
                if let Some(bf) = bf {
                    prune_constant_condition_branches(bf);
                }
                let const_truth = constant_condition_truth(&cond.item);
                if let Some(is_true) = const_truth {
                    if is_true {
                        let body = std::mem::take(bt);
                        stmt.statement = AstStatement::Block(body);
                    } else if let Some(else_body) = bf.take() {
                        stmt.statement = AstStatement::Block(else_body);
                    } else {
                        stmt.statement = AstStatement::Empty;
                    }
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                prune_constant_condition_branches(body);
            }
            AstStatement::For(_, _, _, body) => prune_constant_condition_branches(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    prune_constant_condition_branches(case_body);
                }
                if let Some(default_body) = default {
                    prune_constant_condition_branches(default_body);
                }
            }
            _ => {}
        }
    }
}

/// Merge consecutive if-statements that test the same pure condition:
/// `if(cond) { A } if(cond) { B }` → `if(cond) { A; B }`
///
/// Only merges when the condition is side-effect-free (pure) and the first if has no else branch.
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
                (AstStatement::If(cond1, _, None), AstStatement::If(cond2, _, _)) => {
                    conditions_are_equivalent(cond1, cond2)
                }
                _ => false,
            }
        };

        if should_merge {
            let removed = stmts.remove(i + 1);
            if let AstStatement::If(_, mut body2, else2) = removed.statement {
                if let AstStatement::If(_, body1, else1) = &mut stmts[i].statement {
                    body1.append(&mut body2);
                    // If the second if had an else branch, adopt it.
                    if else2.is_some() && else1.is_none() {
                        *else1 = else2;
                    }
                }
            }
            // Don't increment — check for more consecutive matches.
        } else {
            i += 1;
        }
    }
}

fn conditions_are_equivalent(
    cond1: &Wrapped<AstExpression>,
    cond2: &Wrapped<AstExpression>,
) -> bool {
    if !super::opt_utils::is_pure_expression(&cond1.item)
        || !super::opt_utils::is_pure_expression(&cond2.item)
    {
        return false;
    }

    if super::opt_utils::expr_structurally_equal(&cond1.item, &cond2.item) {
        return true;
    }

    let mut normalized_cond1 = cond1.clone();
    let mut normalized_cond2 = cond2.clone();
    super::operator_canonicalization::canonicalize_condition_expression(&mut normalized_cond1);
    super::operator_canonicalization::canonicalize_condition_expression(&mut normalized_cond2);
    super::opt_utils::expr_structurally_equal(&normalized_cond1.item, &normalized_cond2.item)
}

/// Invert `if(!cond) { A } else { B }` → `if(cond) { B } else { A }` when doing
/// so keeps the larger branch on the positive path and improves readability.
fn invert_negated_branches(stmts: &mut Vec<WrappedAstStatement>) {
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

/// Factor identical trailing statements out of if/else branches.
/// `if(c) { A; T; } else { B; T; }` → `if(c) { A; } else { B; } T;`
///
/// Recurses into nested structures first, then rewrites at each list level.
fn factor_common_tails(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                factor_common_tails(bt);
                if let Some(bf) = bf {
                    factor_common_tails(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                factor_common_tails(body);
            }
            AstStatement::For(_, _, _, body) => factor_common_tails(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    factor_common_tails(case_body);
                }
                if let Some(default_body) = default {
                    factor_common_tails(default_body);
                }
            }
            _ => {}
        }
    }

    // Now look for if/else with common tails at this level and splice them out.
    let mut insertions: Vec<(usize, Vec<WrappedAstStatement>)> = Vec::new();
    for (idx, stmt) in stmts.iter_mut().enumerate() {
        let AstStatement::If(_, bt, Some(bf)) = &mut stmt.statement else {
            continue;
        };
        let common = count_common_tail(bt, bf);
        if common == 0 {
            continue;
        }
        let tail: Vec<WrappedAstStatement> = bt.drain(bt.len() - common..).collect();
        bf.truncate(bf.len() - common);
        insertions.push((idx, tail));
    }

    // Insert extracted tails after their respective if/else (in reverse to keep indices stable).
    for (idx, tail) in insertions.into_iter().rev() {
        let insert_at = idx + 1;
        for (j, s) in tail.into_iter().enumerate() {
            stmts.insert(insert_at + j, s);
        }
    }
}

/// Detect likely unrolled loops: loops whose body contains N consecutive structurally
/// identical statement groups (N >= 2). Annotates with "// likely unrolled x{N}".
fn annotate_likely_unrolled_loops(stmts: &mut Vec<WrappedAstStatement>) {
    use super::pattern_matching::{Blake3StdHasher, hash_statement_list};

    for stmt in stmts.iter_mut() {
        // Recurse into nested structures
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                annotate_likely_unrolled_loops(bt);
                if let Some(bf) = bf {
                    annotate_likely_unrolled_loops(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                annotate_likely_unrolled_loops(body);
            }
            AstStatement::For(_, _, _, body) => {
                annotate_likely_unrolled_loops(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_likely_unrolled_loops(case_body);
                }
                if let Some(default_body) = default {
                    annotate_likely_unrolled_loops(default_body);
                }
            }
            _ => {}
        }

        // Check loop bodies for repeated statement patterns
        let body = match &mut stmt.statement {
            AstStatement::While(_, body) | AstStatement::For(_, _, _, body) => body,
            _ => continue,
        };

        if body.len() < 4 {
            continue;
        }

        // Try group sizes from 1 up to half the body length.
        // For each group size, hash consecutive non-overlapping groups and
        // count the longest run of identical hashes.
        let mut best_repeat = 1usize;
        let max_group = body.len() / 2;
        for group_size in 1..=max_group {
            let n_groups = body.len() / group_size;
            if n_groups < 2 {
                continue;
            }

            // Hash each group of `group_size` consecutive statements
            let group_hashes: Vec<[u8; 32]> = (0..n_groups)
                .map(|g| {
                    let start = g * group_size;
                    let mut h = Blake3StdHasher::new();
                    hash_statement_list(&mut h, &body[start..start + group_size]);
                    h.finish_bytes()
                })
                .collect();

            let mut run = 1;
            for i in 1..group_hashes.len() {
                if group_hashes[i] == group_hashes[i - 1] {
                    run += 1;
                } else {
                    run = 1;
                }
                if run > best_repeat {
                    best_repeat = run;
                }
            }
        }

        if best_repeat >= 2 && stmt.comment.is_none() {
            stmt.comment = Some(format!("likely unrolled x{best_repeat}"));
        }
    }
}

/// Annotate goto-cleanup patterns: detect `if (error) { goto cleanup; }` sequences
/// where multiple error checks jump to the same label, followed by resource-release
/// statements before a return. This is the classic C "goto fail" error-handling idiom.
fn annotate_goto_cleanup_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                annotate_goto_cleanup_patterns(bt);
                if let Some(bf) = bf {
                    annotate_goto_cleanup_patterns(bf);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => {
                annotate_goto_cleanup_patterns(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_goto_cleanup_patterns(case_body);
                }
                if let Some(default_body) = default {
                    annotate_goto_cleanup_patterns(default_body);
                }
            }
            _ => {}
        }
    }

    // Count how many gotos target each label in this scope.
    let mut goto_counts: hashbrown::HashMap<String, usize> = hashbrown::HashMap::new();
    for stmt in stmts.iter() {
        count_gotos_recursive(&stmt.statement, &mut goto_counts);
    }

    // A cleanup label is one targeted by 2+ gotos from error checks.
    let cleanup_labels: HashSet<String> = goto_counts
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .map(|(label, _)| label)
        .collect();

    if cleanup_labels.is_empty() {
        return;
    }

    // Annotate: label definitions that are cleanup targets, and gotos that jump to them.
    for stmt in stmts.iter_mut() {
        match &stmt.statement {
            AstStatement::Label(label) if cleanup_labels.contains(label) => {
                if stmt.comment.is_none() {
                    stmt.comment = Some("error cleanup".to_string());
                }
            }
            _ => {}
        }
    }

    // Annotate if-goto patterns: if (cond) { goto cleanup; }
    for stmt in stmts.iter_mut() {
        if let AstStatement::If(_, bt, None) = &stmt.statement {
            if bt.len() == 1 {
                if let AstStatement::Goto(target) = &bt[0].statement {
                    if let Some(label) = jump_target_label_name(target) {
                        if cleanup_labels.contains(&label) && stmt.comment.is_none() {
                            stmt.comment = Some("error check → cleanup".to_string());
                        }
                    }
                }
            }
        }
    }
}

fn jump_target_label_name(target: &crate::abstract_syntax_tree::AstJumpTarget) -> Option<String> {
    match target {
        crate::abstract_syntax_tree::AstJumpTarget::Unknown(name) => Some(name.clone()),
        _ => None,
    }
}

fn count_gotos_recursive(stmt: &AstStatement, counts: &mut hashbrown::HashMap<String, usize>) {
    match stmt {
        AstStatement::Goto(target) => {
            if let Some(label) = jump_target_label_name(target) {
                *counts.entry(label).or_insert(0) += 1;
            }
        }
        AstStatement::If(_, bt, bf) => {
            for s in bt {
                count_gotos_recursive(&s.statement, counts);
            }
            if let Some(bf) = bf {
                for s in bf {
                    count_gotos_recursive(&s.statement, counts);
                }
            }
        }
        AstStatement::While(_, body)
        | AstStatement::For(_, _, _, body)
        | AstStatement::Block(body) => {
            for s in body {
                count_gotos_recursive(&s.statement, counts);
            }
        }
        AstStatement::Switch(_, cases, default) => {
            for (_, case_body) in cases {
                for s in case_body {
                    count_gotos_recursive(&s.statement, counts);
                }
            }
            if let Some(default_body) = default {
                for s in default_body {
                    count_gotos_recursive(&s.statement, counts);
                }
            }
        }
        _ => {}
    }
}

/// Count how many trailing statements are structurally identical between two lists.
/// Uses full 256-bit blake3 structural hashing for comparison.
fn count_common_tail(a: &[WrappedAstStatement], b: &[WrappedAstStatement]) -> usize {
    use super::pattern_matching::{Blake3StdHasher, hash_statement_list};

    let mut count = 0;
    let min_len = a.len().min(b.len());
    // Don't factor out ALL statements — leave at least 1 in each branch.
    let max_factor = if min_len > 0 { min_len - 1 } else { 0 };

    for i in 0..max_factor {
        let ai = a.len() - 1 - i;
        let bi = b.len() - 1 - i;
        let a_slice = &a[ai..=ai];
        let b_slice = &b[bi..=bi];

        let mut ha = Blake3StdHasher::new();
        hash_statement_list(&mut ha, a_slice);
        let da = ha.finish_bytes();

        let mut hb = Blake3StdHasher::new();
        hash_statement_list(&mut hb, b_slice);
        let db = hb.finish_bytes();

        if da != db {
            break;
        }
        count += 1;
    }
    count
}

// ---------------------------------------------------------------------------
// Register spill/reload detection (L252)
// ---------------------------------------------------------------------------

/// Detect save/call/restore sequences: `temp = var; call(); var = temp;`
fn annotate_register_spill_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                annotate_register_spill_patterns(bt);
                if let Some(bf) = bf {
                    annotate_register_spill_patterns(bf);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => {
                annotate_register_spill_patterns(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_register_spill_patterns(case_body);
                }
                if let Some(default_body) = default {
                    annotate_register_spill_patterns(default_body);
                }
            }
            _ => {}
        }
    }

    // Look for: temp = var; call(...); var = temp; at this level.
    let len = stmts.len();
    if len < 3 {
        return;
    }
    for i in 0..len - 2 {
        // Statement i: temp = var (save)
        let (save_target, save_source) = match &stmts[i].statement {
            AstStatement::Assignment(lhs, rhs) => match (&lhs.item, &rhs.item) {
                (AstExpression::Variable(_, t), AstExpression::Variable(_, s)) if t != s => {
                    (*t, *s)
                }
                _ => continue,
            },
            _ => continue,
        };

        // Statement i+1: call(...)
        if !matches!(&stmts[i + 1].statement, AstStatement::Call(_)) {
            continue;
        }

        // Statement i+2: var = temp (restore)
        let (restore_target, restore_source) = match &stmts[i + 2].statement {
            AstStatement::Assignment(lhs, rhs) => match (&lhs.item, &rhs.item) {
                (AstExpression::Variable(_, t), AstExpression::Variable(_, s)) => (*t, *s),
                _ => continue,
            },
            _ => continue,
        };

        // Check: save_source == restore_target (original var) and save_target == restore_source (temp)
        if save_source == restore_target && save_target == restore_source {
            if stmts[i].comment.is_none() {
                stmts[i].comment = Some("likely register spill (save before call)".to_string());
            }
            if stmts[i + 2].comment.is_none() {
                stmts[i + 2].comment =
                    Some("likely register reload (restore after call)".to_string());
            }
        }
    }
}
