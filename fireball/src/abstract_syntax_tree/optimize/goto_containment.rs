use crate::{
    abstract_syntax_tree::{
        Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstJumpTarget, AstStatement,
        AstUnaryOperator, AstValueOrigin, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn contain_gotos(
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

    contain_gotos_in_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::GotoContainment);
    }

    Ok(())
}

fn contain_gotos_in_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    // First, recurse into nested structures.
    for stmt in stmts.iter_mut() {
        contain_gotos_in_statement(stmt);
    }

    // Convert forward goto patterns into structured if-blocks.
    loop {
        let changed = try_convert_goto_to_if(stmts);
        if !changed {
            break;
        }
    }

    // Remove simple forward gotos (goto L; ...dead code... L:).
    remove_simple_forward_gotos(stmts);

    // NOTE: We intentionally do NOT remove unreferenced labels here.
    // Labels may be targets of indirect jumps or code outside the current
    // function's visible AST. Removing them would break control flow.
}

fn contain_gotos_in_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            contain_gotos_in_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                contain_gotos_in_statement_list(branch_false);
            }
        }
        AstStatement::While(_, body) => contain_gotos_in_statement_list(body),
        AstStatement::For(init, _, update, body) => {
            contain_gotos_in_statement(init);
            contain_gotos_in_statement(update);
            contain_gotos_in_statement_list(body);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                contain_gotos_in_statement_list(case_body);
            }
            if let Some(default_body) = default {
                contain_gotos_in_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => contain_gotos_in_statement_list(body),
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

/// Pattern: `if (cond) { goto L; } A; B; C; L:` => `if (!cond) { A; B; C; }`
///
/// Returns `true` if any transformation was applied.
fn try_convert_goto_to_if(stmts: &mut Vec<WrappedAstStatement>) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < stmts.len() {
        // Check if the current statement matches the pattern:
        //   if(cond) { goto L; }  (no else branch, single statement in body)
        let label_name = match &stmts[i].statement {
            AstStatement::If(_, body_true, None) if body_true.len() == 1 => {
                if let AstStatement::Goto(target) = &body_true[0].statement {
                    jump_target_label(target)
                } else {
                    None
                }
            }
            _ => None,
        };

        let Some(label_name) = label_name else {
            i += 1;
            continue;
        };

        // Find the matching label at the same nesting level, after i.
        let Some(label_idx) = find_label_index(stmts, i + 1, &label_name) else {
            i += 1;
            continue;
        };

        // Collect all statements between the if and the label (exclusive).
        let between: Vec<WrappedAstStatement> = stmts.drain((i + 1)..label_idx).collect();

        // Extract the condition from the if-goto, negate it, and replace.
        let cond = match &stmts[i].statement {
            AstStatement::If(cond, _, _) => cond.clone(),
            _ => unreachable!(),
        };
        let negated_cond = negate_condition(cond);
        stmts[i].statement = AstStatement::If(negated_cond, between, None);

        // Remove the label if no other gotos reference it.
        // After the drain, the label is now at index i+1.
        if i + 1 < stmts.len() {
            let should_remove = matches!(
                &stmts[i + 1].statement,
                AstStatement::Label(lbl) if lbl == &label_name
            ) && count_gotos_to_label(stmts, &label_name) == 0;

            if should_remove {
                stmts.remove(i + 1);
            }
        }

        changed = true;
        // Don't increment i; re-check at same position for nested patterns.
    }
    changed
}

/// Remove simple forward goto patterns: `goto L; ... L:` where the goto jumps
/// forward to a label at the same nesting level. The statements between the
/// goto and the label are dead code (already handled by control_flow_cleanup),
/// so this mainly removes the goto itself and the label if unreferenced.
fn remove_simple_forward_gotos(stmts: &mut Vec<WrappedAstStatement>) {
    let mut i = 0;
    while i < stmts.len() {
        if let AstStatement::Goto(target) = &stmts[i].statement {
            if let Some(label_name) = jump_target_label(target) {
                if find_label_index(stmts, i + 1, &label_name).is_some() {
                    // Remove the goto statement.
                    stmts.remove(i);
                    // The label is now at label_idx - 1. We don't remove dead code
                    // between goto and label here because control_flow_cleanup handles
                    // that. We just removed the goto itself.
                    continue;
                }
            }
        }
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Extract the label name from a jump target, if it is a label-based goto.
fn jump_target_label(target: &AstJumpTarget) -> Option<String> {
    match target {
        AstJumpTarget::Unknown(name) => Some(name.clone()),
        AstJumpTarget::Variable { .. }
        | AstJumpTarget::Function { .. }
        | AstJumpTarget::Instruction { .. } => None,
    }
}

/// Find the index of a `Label(label)` statement in `stmts[from..]`.
fn find_label_index(stmts: &[WrappedAstStatement], from: usize, label: &str) -> Option<usize> {
    for idx in from..stmts.len() {
        if let AstStatement::Label(ref lbl) = stmts[idx].statement {
            if lbl == label {
                return Some(idx);
            }
        }
    }
    None
}

/// Negate a condition expression by wrapping it in `UnaryOp(Not, ...)`.
fn negate_condition(cond: Wrapped<AstExpression>) -> Wrapped<AstExpression> {
    Wrapped {
        item: AstExpression::UnaryOp(AstUnaryOperator::Not, Box::new(cond)),
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

/// Count the number of `Goto` statements in the slice that target the given label.
/// Recurses into nested structures.
fn count_gotos_to_label(stmts: &[WrappedAstStatement], label: &str) -> usize {
    let mut count = 0;
    for stmt in stmts {
        count += count_gotos_to_label_in_statement(stmt, label);
    }
    count
}

fn count_gotos_to_label_in_statement(stmt: &WrappedAstStatement, label: &str) -> usize {
    match &stmt.statement {
        AstStatement::Goto(target) => {
            if jump_target_label(target).as_deref() == Some(label) {
                1
            } else {
                0
            }
        }
        AstStatement::If(_, branch_true, branch_false) => {
            let mut count = count_gotos_to_label(branch_true, label);
            if let Some(branch_false) = branch_false {
                count += count_gotos_to_label(branch_false, label);
            }
            count
        }
        AstStatement::While(_, body) => count_gotos_to_label(body, label),
        AstStatement::For(init, _, update, body) => {
            count_gotos_to_label_in_statement(init, label)
                + count_gotos_to_label_in_statement(update, label)
                + count_gotos_to_label(body, label)
        }
        AstStatement::Switch(_, cases, default) => {
            let mut count = 0;
            for (_lit, case_body) in cases {
                count += count_gotos_to_label(case_body, label);
            }
            if let Some(default_body) = default {
                count += count_gotos_to_label(default_body, label);
            }
            count
        }
        AstStatement::Block(body) => count_gotos_to_label(body, label),
        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::Return(_)
        | AstStatement::Call(_)
        | AstStatement::Label(_)
        | AstStatement::Assembly(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Empty => 0,
    }
}

/// Remove labels that have no remaining goto references anywhere in the
/// statement list (including nested structures).
fn remove_unreferenced_labels(stmts: &mut Vec<WrappedAstStatement>) {
    // Collect all labels first, then check references.
    let labels: Vec<String> = stmts
        .iter()
        .filter_map(|s| {
            if let AstStatement::Label(ref lbl) = s.statement {
                Some(lbl.clone())
            } else {
                None
            }
        })
        .collect();

    for label in labels {
        if count_gotos_to_label(stmts, &label) == 0 {
            stmts.retain(|s| !matches!(&s.statement, AstStatement::Label(lbl) if lbl == &label));
        }
    }
}
