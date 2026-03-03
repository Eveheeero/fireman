use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement,
        AstStatementOrigin, AstValueOrigin, ProcessedOptimization, Wrapped, WrappedAstStatement,
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

    // Flatten Block([...]) nodes: splice their contents into the parent list.
    let mut i = 0;
    while i < stmts.len() {
        if matches!(&stmts[i].statement, AstStatement::Block(_)) {
            let removed = stmts.remove(i);
            if let AstStatement::Block(inner) = removed.statement {
                let count = inner.len();
                for (j, s) in inner.into_iter().enumerate() {
                    stmts.insert(i + j, s);
                }
                i += count;
            }
        } else {
            i += 1;
        }
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

fn cleanup_statement(stmt: &mut WrappedAstStatement, noreturn_targets: &HashSet<AstFunctionId>) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            cleanup_statement_list(branch_true, noreturn_targets);
            if let Some(branch_false) = branch_false {
                cleanup_statement_list(branch_false, noreturn_targets);
            }
        }
        AstStatement::While(_, body) => cleanup_statement_list(body, noreturn_targets),
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
        | AstStatement::Empty => {}
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
        | AstStatement::For(_, _, _, _)
        | AstStatement::Label(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Empty => TerminationOutcome::NoTerminate,
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
