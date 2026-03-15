//! Detect and recover loop structures (for, while, do-while).

use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstJumpTarget, AstLiteral, AstStatement, AstUnaryOperator, AstValueOrigin, AstVariableId,
        ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn analyze_loops(
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

    normalize_statement_list(&mut body);
    normalize_infinite_loops(&mut body);
    normalize_rotated_loops(&mut body);
    try_convert_while_to_for(&mut body);
    replace_loop_with_call(&mut body);
    annotate_continue_like_gotos(&mut body);
    convert_loop_gotos_to_break_continue(&mut body);
    try_convert_while_to_dowhile(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::LoopAnalyzation);
    }

    Ok(())
}

fn normalize_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        normalize_statement(stmt);
    }
}

fn normalize_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            normalize_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                normalize_statement_list(branch_false);
            }
        }
        AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
            normalize_statement_list(body);
        }
        AstStatement::For(init, cond, update, body) => {
            normalize_statement(init);
            normalize_statement(update);
            normalize_statement_list(body);
            if is_noop_statement(init.as_ref()) && is_noop_statement(update.as_ref()) {
                stmt.statement = AstStatement::While(cond.clone(), std::mem::take(body));
            }
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                normalize_statement_list(case_body);
            }
            if let Some(default_body) = default {
                normalize_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => {
            normalize_statement_list(body);
        }
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

/// Normalize rotated loops: convert `if(cond) { while(cond) { body; } }` → `while(cond) { body; }`
/// when the condition is side-effect-free (pure).
fn normalize_rotated_loops(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                normalize_rotated_loops(bt);
                if let Some(bf) = bf {
                    normalize_rotated_loops(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                normalize_rotated_loops(body);
            }
            AstStatement::For(_, _, _, body) => normalize_rotated_loops(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    normalize_rotated_loops(case_body);
                }
                if let Some(default_body) = default {
                    normalize_rotated_loops(default_body);
                }
            }
            _ => {}
        }
    }

    // Now look for `if(cond) { while(cond) { body } }` at this level.
    for stmt in stmts.iter_mut() {
        let AstStatement::If(if_cond, branch_true, branch_false) = &mut stmt.statement else {
            continue;
        };
        // Must be if-without-else, and condition must be pure.
        if branch_false.is_some() {
            continue;
        }
        if !super::opt_utils::is_pure_expression(&if_cond.item) {
            continue;
        }
        // Branch body must be exactly one statement: a while with the same condition.
        if branch_true.len() != 1 {
            continue;
        }
        let AstStatement::While(while_cond, _) = &branch_true[0].statement else {
            continue;
        };
        if !super::opt_utils::expr_structurally_equal(&if_cond.item, &while_cond.item) {
            continue;
        }
        // Safe to collapse: replace `if(cond) { while(cond) { body } }` with `while(cond) { body }`.
        let while_stmt = branch_true.remove(0);
        stmt.statement = while_stmt.statement;
    }
}

/// Normalize infinite loops: convert `while(1)` / `while(nonzero_literal)` to `while(true)`.
fn normalize_infinite_loops(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::While(cond, body) => {
                normalize_infinite_loops(body);
                if is_always_true_literal(&cond.item) {
                    cond.item = AstExpression::Literal(AstLiteral::Bool(true));
                }
            }
            AstStatement::If(_, bt, bf) => {
                normalize_infinite_loops(bt);
                if let Some(bf) = bf {
                    normalize_infinite_loops(bf);
                }
            }
            AstStatement::For(_, _, _, body) => normalize_infinite_loops(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    normalize_infinite_loops(case_body);
                }
                if let Some(default_body) = default {
                    normalize_infinite_loops(default_body);
                }
            }
            AstStatement::Block(body) => normalize_infinite_loops(body),
            _ => {}
        }
    }
}

/// Returns true if the expression is a non-zero integer or boolean true literal.
fn is_always_true_literal(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(AstLiteral::Int(n)) => *n != 0,
        AstExpression::Literal(AstLiteral::UInt(n)) => *n != 0,
        AstExpression::Literal(AstLiteral::Bool(true)) => true,
        _ => false,
    }
}

fn is_noop_statement(stmt: &WrappedAstStatement) -> bool {
    matches!(
        &stmt.statement,
        AstStatement::Empty | AstStatement::Comment(_)
    )
}

fn get_assigned_var(stmt: &AstStatement) -> Option<AstVariableId> {
    match stmt {
        AstStatement::Assignment(lhs, _) => {
            if let AstExpression::Variable(_, var_id) = &lhs.item {
                Some(*var_id)
            } else {
                None
            }
        }
        AstStatement::Declaration(var, Some(_)) => Some(var.id),
        _ => None,
    }
}

fn try_convert_while_to_for(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested statement bodies first
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, branch_true, branch_false) => {
                try_convert_while_to_for(branch_true);
                if let Some(branch_false) = branch_false {
                    try_convert_while_to_for(branch_false);
                }
            }
            AstStatement::While(_, body) => {
                try_convert_while_to_for(body);
            }
            AstStatement::For(_, _, _, body) => {
                try_convert_while_to_for(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_lit, case_body) in cases.iter_mut() {
                    try_convert_while_to_for(case_body);
                }
                if let Some(default_body) = default {
                    try_convert_while_to_for(default_body);
                }
            }
            AstStatement::Block(body) => {
                try_convert_while_to_for(body);
            }
            _ => {}
        }
    }

    // Now look for init-before-while patterns at this level
    let mut i = 0;
    while i + 1 < stmts.len() {
        let init_var_id = match get_assigned_var(&stmts[i].statement) {
            Some(id) => id,
            None => {
                i += 1;
                continue;
            }
        };

        let should_convert = if let AstStatement::While(cond, body) = &stmts[i + 1].statement {
            if body.len() >= 2 {
                let last = &body[body.len() - 1];
                if let Some(update_var_id) = get_assigned_var(&last.statement) {
                    if update_var_id == init_var_id {
                        let mut vars = hashbrown::HashSet::new();
                        super::opt_utils::collect_expr_variables(&cond.item, &mut vars);
                        vars.contains(&init_var_id)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if should_convert {
            let init_stmt = stmts.remove(i);
            if let AstStatement::While(cond, mut body) =
                std::mem::replace(&mut stmts[i].statement, AstStatement::Empty)
            {
                let update_stmt = body.pop().unwrap();
                stmts[i].statement =
                    AstStatement::For(Box::new(init_stmt), cond, Box::new(update_stmt), body);
            }
            // Don't increment i; re-check at the same index
        } else {
            i += 1;
        }
    }
}

/// Convert `while(true) { ... if (!cond) break; }` into `do { ... } while(cond);`.
fn try_convert_while_to_dowhile(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, branch_true, branch_false) => {
                try_convert_while_to_dowhile(branch_true);
                if let Some(branch_false) = branch_false {
                    try_convert_while_to_dowhile(branch_false);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => {
                try_convert_while_to_dowhile(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    try_convert_while_to_dowhile(case_body);
                }
                if let Some(default_body) = default {
                    try_convert_while_to_dowhile(default_body);
                }
            }
            _ => {}
        }
    }

    for stmt in stmts.iter_mut() {
        if stmt.comment.is_some() {
            continue;
        }

        let Some((loop_cond, break_index)) = extract_dowhile_rewrite(stmt) else {
            continue;
        };

        let AstStatement::While(_, body) = &mut stmt.statement else {
            continue;
        };
        body.remove(break_index);
        let new_body = std::mem::take(body);
        stmt.statement = AstStatement::DoWhile(loop_cond, new_body);
    }
}

fn extract_dowhile_rewrite(stmt: &WrappedAstStatement) -> Option<(Wrapped<AstExpression>, usize)> {
    let AstStatement::While(cond, body) = &stmt.statement else {
        return None;
    };

    if !matches!(&cond.item, AstExpression::Literal(AstLiteral::Bool(true))) {
        return None;
    }

    let break_index = last_meaningful_statement_index(body)?;
    let break_guard = body.get(break_index)?;
    if break_guard.comment.is_some() {
        return None;
    }

    extract_dowhile_condition_from_break_guard(&break_guard.statement)
        .map(|loop_cond| (loop_cond, break_index))
}

fn last_meaningful_statement_index(stmts: &[WrappedAstStatement]) -> Option<usize> {
    stmts.iter().rposition(|stmt| !is_noop_statement(stmt))
}

fn extract_dowhile_condition_from_break_guard(
    stmt: &AstStatement,
) -> Option<Wrapped<AstExpression>> {
    let AstStatement::If(cond, branch_true, branch_false) = stmt else {
        return None;
    };

    if branch_false.is_some() || branch_true.len() != 1 {
        return None;
    }

    let break_stmt = branch_true.first()?;
    if break_stmt.comment.is_some() || !matches!(&break_stmt.statement, AstStatement::Break) {
        return None;
    }

    match &cond.item {
        AstExpression::UnaryOp(AstUnaryOperator::Not, inner) => Some((**inner).clone()),
        AstExpression::BinaryOp(AstBinaryOperator::Equal, left, right) => {
            if !is_false_literal(&left.item) && !is_false_literal(&right.item) {
                return None;
            }

            Some(Wrapped {
                item: AstExpression::BinaryOp(
                    AstBinaryOperator::NotEqual,
                    left.clone(),
                    right.clone(),
                ),
                origin: cond.origin.clone(),
                comment: cond.comment.clone(),
            })
        }
        _ => None,
    }
}

fn is_false_literal(expr: &AstExpression) -> bool {
    matches!(
        expr,
        AstExpression::Literal(AstLiteral::Int(0))
            | AstExpression::Literal(AstLiteral::UInt(0))
            | AstExpression::Literal(AstLiteral::Bool(false))
    )
}

// ---------------------------------------------------------------------------
// Loop semantic pattern recognition (L473/L475/L477)
// ---------------------------------------------------------------------------

/// Classify a loop body as a known memory-operation pattern.
fn classify_loop_body(body: &[WrappedAstStatement]) -> Option<&'static str> {
    // Filter to non-empty/non-comment statements for pattern matching
    let stmts: Vec<&AstStatement> = body
        .iter()
        .map(|s| &s.statement)
        .filter(|s| !matches!(s, AstStatement::Empty | AstStatement::Comment(_)))
        .collect();

    if stmts.is_empty() {
        return None;
    }

    // --- memcpy pattern: dst[i] = src[i]  or  *dst++ = *src++ ---
    if stmts.len() == 1 {
        if let AstStatement::Assignment(lhs, rhs) = stmts[0] {
            if is_indexed_or_deref(&lhs.item) && is_indexed_or_deref(&rhs.item) {
                return Some("likely memcpy/memmove loop");
            }
        }
    }

    // --- memset pattern: dst[i] = const  or  *dst++ = const ---
    if stmts.len() == 1 {
        if let AstStatement::Assignment(lhs, rhs) = stmts[0] {
            if is_indexed_or_deref(&lhs.item) && is_constant_or_variable(&rhs.item) {
                return Some("likely memset loop");
            }
        }
    }

    // --- memcmp/strcmp pattern: if (a[i] != b[i]) return/goto ---
    for s in &stmts {
        if let AstStatement::If(cond, bt, bf) = s {
            if is_memory_compare_condition(&cond.item) {
                let bt_terminates = bt
                    .first()
                    .map(|s| matches!(s.statement, AstStatement::Return(_) | AstStatement::Goto(_)))
                    .unwrap_or(false);
                let bf_terminates = bf
                    .as_ref()
                    .and_then(|bf| bf.first())
                    .map(|s| matches!(s.statement, AstStatement::Return(_) | AstStatement::Goto(_)))
                    .unwrap_or(false);
                if bt_terminates || bf_terminates {
                    return Some("likely memcmp/strcmp loop");
                }
            }
        }
    }

    // --- strlen pattern: while(*p != 0) or if(*p == 0) break-like ---
    // Detected at the loop condition level: while(arr[i] != 0)
    // Already handled by the caller checking the while condition.
    // Here we check if the body is just an increment (typical strlen body).
    if stmts.len() == 1 {
        if let AstStatement::Assignment(lhs, rhs) = stmts[0] {
            // p++ or i++ pattern
            if let AstExpression::Variable(_, _) = &lhs.item {
                if is_increment_expr(&rhs.item, &lhs.item) {
                    // Body is just an increment — could be strlen if condition checks null
                    return Some("likely strlen/scan loop");
                }
            }
        }
    }

    None
}

fn is_indexed_or_deref(expr: &AstExpression) -> bool {
    matches!(
        expr,
        AstExpression::Deref(_) | AstExpression::ArrayAccess(_, _)
    )
}

fn is_constant_or_variable(expr: &AstExpression) -> bool {
    matches!(
        expr,
        AstExpression::Literal(_) | AstExpression::Variable(_, _)
    )
}

fn is_memory_compare_condition(expr: &AstExpression) -> bool {
    // Check for patterns like: a[i] != b[i], *p != *q, a[i] == b[i]
    if let AstExpression::BinaryOp(op, left, right) = expr {
        if matches!(op, AstBinaryOperator::NotEqual | AstBinaryOperator::Equal) {
            return is_indexed_or_deref(&left.item) && is_indexed_or_deref(&right.item);
        }
    }
    false
}

fn is_increment_expr(rhs: &AstExpression, lhs: &AstExpression) -> bool {
    // Check: rhs == lhs + 1
    if let AstExpression::BinaryOp(AstBinaryOperator::Add, left, right) = rhs {
        if super::opt_utils::expr_structurally_equal(&left.item, lhs) {
            if matches!(
                &right.item,
                AstExpression::Literal(AstLiteral::Int(1) | AstLiteral::UInt(1))
            ) {
                return true;
            }
        }
        if super::opt_utils::expr_structurally_equal(&right.item, lhs) {
            if matches!(
                &left.item,
                AstExpression::Literal(AstLiteral::Int(1) | AstLiteral::UInt(1))
            ) {
                return true;
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Loop-to-call replacement (L474)
// ---------------------------------------------------------------------------

/// Replace simple memset-style loops with an equivalent `AstStatement::Call` to `memset`.
fn replace_loop_with_call(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        // Recurse into nested structures first
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                replace_loop_with_call(bt);
                if let Some(bf) = bf {
                    replace_loop_with_call(bf);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => {
                replace_loop_with_call(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    replace_loop_with_call(case_body);
                }
                if let Some(default_body) = default {
                    replace_loop_with_call(default_body);
                }
            }
            _ => {}
        }

        let loop_body = match &stmt.statement {
            AstStatement::While(_, body) | AstStatement::For(_, _, _, body) => body,
            _ => continue,
        };
        if classify_loop_body(loop_body) != Some("likely memset loop") {
            continue;
        }

        // Extract the memset operands: dst[i] = const_val  or  *dst = const_val
        let meaningful: Vec<&AstStatement> = loop_body
            .iter()
            .map(|s| &s.statement)
            .filter(|s| !matches!(s, AstStatement::Empty | AstStatement::Comment(_)))
            .collect();

        if meaningful.len() != 1 {
            continue;
        }

        let AstStatement::Assignment(lhs, rhs) = meaningful[0] else {
            continue;
        };

        if !is_indexed_or_deref(&lhs.item) || !is_constant_or_variable(&rhs.item) {
            continue;
        }

        // Build: memset(dst, value, count) — use Unknown-name zero for count since
        // precise size recovery requires further analysis.
        let dst_arg = extract_memset_dst(lhs);
        let val_arg = Wrapped {
            item: rhs.item.clone(),
            origin: AstValueOrigin::Unknown,
            comment: None,
        };
        let count_arg = Wrapped {
            item: AstExpression::Unknown,
            origin: AstValueOrigin::Unknown,
            comment: None,
        };

        let call = AstCall::Unknown("memset".into(), vec![dst_arg, val_arg, count_arg]);
        stmt.statement = AstStatement::Call(call);
        stmt.comment = None;
    }
}

/// Extract the base pointer from an indexed/deref destination expression.
fn extract_memset_dst(expr: &Wrapped<AstExpression>) -> Wrapped<AstExpression> {
    match &expr.item {
        AstExpression::ArrayAccess(base, _) | AstExpression::Deref(base) => Wrapped {
            item: base.item.clone(),
            origin: AstValueOrigin::Unknown,
            comment: None,
        },
        _ => Wrapped {
            item: expr.item.clone(),
            origin: AstValueOrigin::Unknown,
            comment: None,
        },
    }
}

// ---------------------------------------------------------------------------
// Continue-like back-edge detection (L826)
// ---------------------------------------------------------------------------

/// Detect gotos inside loops that jump to the first label in the loop body (continue-like).
fn annotate_continue_like_gotos(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                annotate_continue_like_gotos(bt);
                if let Some(bf) = bf {
                    annotate_continue_like_gotos(bf);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => {
                annotate_continue_like_gotos(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_continue_like_gotos(case_body);
                }
                if let Some(default_body) = default {
                    annotate_continue_like_gotos(default_body);
                }
            }
            _ => {}
        }

        let loop_body = match &mut stmt.statement {
            AstStatement::While(_, body) | AstStatement::For(_, _, _, body) => body,
            _ => continue,
        };

        // Find the first label defined at the top of the loop body.
        let first_label = loop_body.iter().find_map(|s| {
            if let AstStatement::Label(name) = &s.statement {
                Some(name.clone())
            } else {
                None
            }
        });

        let Some(first_label) = first_label else {
            continue;
        };

        // Annotate gotos to this label as continue-like back-edges.
        for s in loop_body.iter_mut() {
            mark_gotos_as_continue(s, &first_label);
        }
    }
}

fn convert_loop_gotos_to_break_continue(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        convert_loop_gotos_in_statement(stmt);
    }

    for index in 0..stmts.len() {
        let continue_label = loop_continue_label(&stmts[index].statement);
        let break_label = next_loop_break_label(stmts, index + 1);

        let Some(loop_body) = loop_body_mut(&mut stmts[index].statement) else {
            continue;
        };

        if continue_label.is_none() && break_label.is_none() {
            continue;
        }

        for stmt in loop_body.iter_mut() {
            rewrite_loop_gotos(stmt, continue_label.as_deref(), break_label.as_deref());
        }
    }
}

fn convert_loop_gotos_in_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            convert_loop_gotos_to_break_continue(branch_true);
            if let Some(branch_false) = branch_false {
                convert_loop_gotos_to_break_continue(branch_false);
            }
        }
        AstStatement::While(_, body)
        | AstStatement::DoWhile(_, body)
        | AstStatement::Block(body) => {
            convert_loop_gotos_to_break_continue(body);
        }
        AstStatement::For(init, _, update, body) => {
            convert_loop_gotos_in_statement(init);
            convert_loop_gotos_in_statement(update);
            convert_loop_gotos_to_break_continue(body);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_, case_body) in cases.iter_mut() {
                convert_loop_gotos_to_break_continue(case_body);
            }
            if let Some(default_body) = default {
                convert_loop_gotos_to_break_continue(default_body);
            }
        }
        _ => {}
    }
}

fn loop_continue_label(stmt: &AstStatement) -> Option<String> {
    let loop_body = match stmt {
        AstStatement::While(_, body)
        | AstStatement::DoWhile(_, body)
        | AstStatement::For(_, _, _, body) => body,
        _ => return None,
    };

    loop_body.iter().find_map(|stmt| {
        if let AstStatement::Label(name) = &stmt.statement {
            Some(name.clone())
        } else {
            None
        }
    })
}

fn next_loop_break_label(stmts: &[WrappedAstStatement], start_index: usize) -> Option<String> {
    for stmt in stmts.iter().skip(start_index) {
        match &stmt.statement {
            AstStatement::Empty | AstStatement::Comment(_) => continue,
            AstStatement::Label(name) => return Some(name.clone()),
            _ => return None,
        }
    }

    None
}

fn loop_body_mut(stmt: &mut AstStatement) -> Option<&mut Vec<WrappedAstStatement>> {
    match stmt {
        AstStatement::While(_, body)
        | AstStatement::DoWhile(_, body)
        | AstStatement::For(_, _, _, body) => Some(body),
        _ => None,
    }
}

fn rewrite_loop_gotos(
    stmt: &mut WrappedAstStatement,
    continue_label: Option<&str>,
    break_label: Option<&str>,
) {
    if let AstStatement::Goto(AstJumpTarget::Unknown(name)) = &stmt.statement {
        if continue_label == Some(name.as_str())
            && stmt.comment.as_deref() == Some("continue-like back-edge")
        {
            stmt.statement = AstStatement::Continue;
            stmt.comment = None;
            return;
        }

        if break_label == Some(name.as_str()) {
            stmt.statement = AstStatement::Break;
            return;
        }
    }

    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            for stmt in branch_true.iter_mut() {
                rewrite_loop_gotos(stmt, continue_label, break_label);
            }
            if let Some(branch_false) = branch_false {
                for stmt in branch_false.iter_mut() {
                    rewrite_loop_gotos(stmt, continue_label, break_label);
                }
            }
        }
        AstStatement::Block(body) => {
            for stmt in body.iter_mut() {
                rewrite_loop_gotos(stmt, continue_label, break_label);
            }
        }
        AstStatement::Switch(_, cases, default) => {
            for (_, case_body) in cases.iter_mut() {
                for stmt in case_body.iter_mut() {
                    rewrite_loop_gotos(stmt, continue_label, break_label);
                }
            }
            if let Some(default_body) = default {
                for stmt in default_body.iter_mut() {
                    rewrite_loop_gotos(stmt, continue_label, break_label);
                }
            }
        }
        AstStatement::While(_, _) | AstStatement::DoWhile(_, _) | AstStatement::For(_, _, _, _) => {
        }
        _ => {}
    }
}

fn mark_gotos_as_continue(stmt: &mut WrappedAstStatement, label: &str) {
    if let AstStatement::Goto(AstJumpTarget::Unknown(name)) = &stmt.statement {
        if name == label && stmt.comment.is_none() {
            stmt.comment = Some("continue-like back-edge".to_string());
        }
    }
    // Recurse into branches but not nested loops (their back-edges are their own).
    match &mut stmt.statement {
        AstStatement::If(_, bt, bf) => {
            for s in bt {
                mark_gotos_as_continue(s, label);
            }
            if let Some(bf) = bf {
                for s in bf {
                    mark_gotos_as_continue(s, label);
                }
            }
        }
        AstStatement::Block(body) => {
            for s in body {
                mark_gotos_as_continue(s, label);
            }
        }
        AstStatement::Switch(_, cases, default) => {
            for (_, case_body) in cases {
                for s in case_body {
                    mark_gotos_as_continue(s, label);
                }
            }
            if let Some(default_body) = default {
                for s in default_body {
                    mark_gotos_as_continue(s, label);
                }
            }
        }
        _ => {}
    }
}
