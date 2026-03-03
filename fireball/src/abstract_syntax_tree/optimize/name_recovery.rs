use crate::{
    abstract_syntax_tree::{
        ArcAstVariableMap, Ast, AstBinaryOperator, AstCall, AstExpression, AstFunctionId,
        AstFunctionVersion, AstLiteral, AstStatement, AstVariableId, ProcessedOptimization,
        Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::{HashMap, HashSet};

/// A naming hint collected from usage context.
#[derive(Debug, Clone)]
struct NameHint {
    base: &'static str,
    /// Ordering tie-breaker when multiple hints exist for the same variable.
    /// Lower priority wins.
    priority: u8,
}

pub(super) fn recover_names(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let body;
    let variables: ArcAstVariableMap;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
        variables = function.variables.clone();
    }

    // Collect name hints from usage patterns.
    let mut hints: HashMap<AstVariableId, NameHint> = HashMap::new();
    let mut loop_counter_index: usize = 0;

    collect_hints_from_statement_list(&body, &mut hints, &mut loop_counter_index, ast);

    // Apply hints to unnamed variables, resolving conflicts.
    apply_hints(&variables, &hints);

    // Put the body back and record the optimization.
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::NameRecovery);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Hint collection
// ---------------------------------------------------------------------------

const LOOP_COUNTER_NAMES: &[&str] = &["i", "j", "k"];

fn collect_hints_from_statement_list(
    stmts: &[WrappedAstStatement],
    hints: &mut HashMap<AstVariableId, NameHint>,
    loop_counter_index: &mut usize,
    ast: &Ast,
) {
    for stmt in stmts.iter() {
        collect_hints_from_statement(stmt, hints, loop_counter_index, ast);
    }
}

fn collect_hints_from_statement(
    stmt: &WrappedAstStatement,
    hints: &mut HashMap<AstVariableId, NameHint>,
    loop_counter_index: &mut usize,
    ast: &Ast,
) {
    match &stmt.statement {
        AstStatement::For(init, cond, update, body) => {
            // The variable assigned in init or update is likely a loop counter.
            if let Some(var_id) = extract_assigned_variable(&init.statement) {
                let name = if *loop_counter_index < LOOP_COUNTER_NAMES.len() {
                    LOOP_COUNTER_NAMES[*loop_counter_index]
                } else {
                    "i" // fallback; conflict resolution will append suffix
                };
                *loop_counter_index += 1;
                insert_hint(hints, var_id, name, 0);
            }
            if let Some(var_id) = extract_assigned_variable(&update.statement) {
                // Only hint the update variable if it differs from init (avoid
                // double-hinting the same counter).
                if !hints.contains_key(&var_id) {
                    let name = if *loop_counter_index < LOOP_COUNTER_NAMES.len() {
                        LOOP_COUNTER_NAMES[*loop_counter_index]
                    } else {
                        "i"
                    };
                    *loop_counter_index += 1;
                    insert_hint(hints, var_id, name, 0);
                }
            }

            collect_hints_from_expression(cond, hints, ast);
            collect_hints_from_statement(init, hints, loop_counter_index, ast);
            collect_hints_from_statement(update, hints, loop_counter_index, ast);
            collect_hints_from_statement_list(body, hints, loop_counter_index, ast);
        }
        AstStatement::Declaration(var, rhs) => {
            if let Some(rhs) = rhs {
                // Check for assignment from a function call whose name contains
                // len/length/size/count.
                check_call_assignment_hint(rhs, Some(var.id), hints, ast);
                collect_hints_from_expression(rhs, hints, ast);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            // If lhs is a variable and rhs is a call with a size-like name, hint
            // the variable.
            let lhs_var = extract_variable_id_from_expression(&lhs.item);
            check_call_assignment_hint(rhs, lhs_var, hints, ast);

            collect_hints_from_expression(lhs, hints, ast);
            collect_hints_from_expression(rhs, hints, ast);
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            // Check for variable compared against zero in condition -> "ret"/"err".
            check_zero_comparison_hint(cond, hints);

            collect_hints_from_expression(cond, hints, ast);
            collect_hints_from_statement_list(branch_true, hints, loop_counter_index, ast);
            if let Some(branch_false) = branch_false {
                collect_hints_from_statement_list(branch_false, hints, loop_counter_index, ast);
            }
        }
        AstStatement::While(cond, body) => {
            check_zero_comparison_hint(cond, hints);
            collect_hints_from_expression(cond, hints, ast);
            collect_hints_from_statement_list(body, hints, loop_counter_index, ast);
        }
        AstStatement::Switch(discrim, cases, default) => {
            collect_hints_from_expression(discrim, hints, ast);
            for (_lit, case_body) in cases.iter() {
                collect_hints_from_statement_list(case_body, hints, loop_counter_index, ast);
            }
            if let Some(default_body) = default {
                collect_hints_from_statement_list(default_body, hints, loop_counter_index, ast);
            }
        }
        AstStatement::Block(body) => {
            collect_hints_from_statement_list(body, hints, loop_counter_index, ast);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                collect_hints_from_expression(expr, hints, ast);
            }
        }
        AstStatement::Call(call) => {
            collect_hints_from_call(call, hints, ast);
        }
        AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Label(_)
        | AstStatement::Comment(_)
        | AstStatement::Empty => {}
    }
}

fn collect_hints_from_expression(
    expr: &Wrapped<AstExpression>,
    hints: &mut HashMap<AstVariableId, NameHint>,
    ast: &Ast,
) {
    match &expr.item {
        AstExpression::ArrayAccess(_base, idx) => {
            // If the index is a plain variable, hint it as "idx".
            if let Some(var_id) = extract_variable_id_from_expression(&idx.item) {
                insert_hint(hints, var_id, "idx", 2);
            }
            collect_hints_from_expression(_base, hints, ast);
            collect_hints_from_expression(idx, hints, ast);
        }
        AstExpression::UnaryOp(_, arg) => {
            collect_hints_from_expression(arg, hints, ast);
        }
        AstExpression::BinaryOp(_, left, right) => {
            collect_hints_from_expression(left, hints, ast);
            collect_hints_from_expression(right, hints, ast);
        }
        AstExpression::Call(call) => {
            collect_hints_from_call(call, hints, ast);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            collect_hints_from_expression(arg, hints, ast);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            collect_hints_from_expression(cond, hints, ast);
            collect_hints_from_expression(true_expr, hints, ast);
            collect_hints_from_expression(false_expr, hints, ast);
        }
        AstExpression::Literal(_)
        | AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => {}
    }
}

fn collect_hints_from_call(
    call: &AstCall,
    hints: &mut HashMap<AstVariableId, NameHint>,
    ast: &Ast,
) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args.iter() {
                collect_hints_from_expression(arg, hints, ast);
            }
        }
        AstCall::Builtin(_, _) => {}
    }
}

// ---------------------------------------------------------------------------
// Specific hint patterns
// ---------------------------------------------------------------------------

/// Extract the variable id being assigned in a statement (Declaration or
/// Assignment to a variable).
fn extract_assigned_variable(stmt: &AstStatement) -> Option<AstVariableId> {
    match stmt {
        AstStatement::Declaration(var, _) => Some(var.id),
        AstStatement::Assignment(lhs, _) => extract_variable_id_from_expression(&lhs.item),
        _ => None,
    }
}

/// Extract a variable id from a simple variable expression.
fn extract_variable_id_from_expression(expr: &AstExpression) -> Option<AstVariableId> {
    match expr {
        AstExpression::Variable(_, var_id) => Some(*var_id),
        _ => None,
    }
}

/// If `rhs` is a function call whose name contains "len"/"length"/"size"/"count",
/// hint the assigned variable accordingly.
fn check_call_assignment_hint(
    rhs: &Wrapped<AstExpression>,
    lhs_var: Option<AstVariableId>,
    hints: &mut HashMap<AstVariableId, NameHint>,
    ast: &Ast,
) {
    let var_id = match lhs_var {
        Some(id) => id,
        None => return,
    };

    let call_name = match &rhs.item {
        AstExpression::Call(call) => resolve_call_name(call, ast),
        _ => None,
    };

    if let Some(name) = call_name {
        let lower = name.to_lowercase();
        if lower.contains("length") || lower.contains("len") {
            insert_hint(hints, var_id, "len", 1);
        } else if lower.contains("size") {
            insert_hint(hints, var_id, "size", 1);
        } else if lower.contains("count") {
            insert_hint(hints, var_id, "count", 1);
        }
    }
}

/// Resolve a human-readable name for a call target.
fn resolve_call_name(call: &AstCall, ast: &Ast) -> Option<String> {
    match call {
        AstCall::Function { target, .. } => {
            // Try the function's own name first.
            let functions = ast.functions.read().unwrap();
            if let Some(versions) = functions.get(target) {
                // Look at any version to get the name.
                for (_ver, func) in versions.raw().iter() {
                    if let Some(name) = &func.name {
                        return Some(name.clone());
                    }
                }
            }
            // Fall back to pre_defined_symbols.
            ast.pre_defined_symbols
                .get(&target.address)
                .cloned()
                .or_else(|| Some(target.get_default_name()))
        }
        AstCall::Unknown(name, _) => Some(name.clone()),
        AstCall::Variable { .. } | AstCall::Builtin(_, _) => None,
    }
}

/// If an expression compares a variable against zero (e.g. `var == 0`,
/// `var != 0`, `var < 0`), hint the variable as "ret" or "err".
fn check_zero_comparison_hint(
    expr: &Wrapped<AstExpression>,
    hints: &mut HashMap<AstVariableId, NameHint>,
) {
    if let AstExpression::BinaryOp(op, left, right) = &expr.item {
        let is_comparison = matches!(
            op,
            AstBinaryOperator::Equal
                | AstBinaryOperator::NotEqual
                | AstBinaryOperator::Less
                | AstBinaryOperator::LessEqual
                | AstBinaryOperator::Greater
                | AstBinaryOperator::GreaterEqual
        );
        if !is_comparison {
            return;
        }

        let (var_id, is_negative_check) = match (&left.item, &right.item) {
            (AstExpression::Variable(_, id), AstExpression::Literal(lit))
                if is_zero_literal(lit) =>
            {
                let neg = matches!(op, AstBinaryOperator::Less);
                (Some(*id), neg)
            }
            (AstExpression::Literal(lit), AstExpression::Variable(_, id))
                if is_zero_literal(lit) =>
            {
                let neg = matches!(op, AstBinaryOperator::Greater);
                (Some(*id), neg)
            }
            _ => (None, false),
        };

        if let Some(var_id) = var_id {
            let hint = if is_negative_check { "err" } else { "ret" };
            insert_hint(hints, var_id, hint, 3);
        }
    }
}

fn is_zero_literal(lit: &AstLiteral) -> bool {
    matches!(lit, AstLiteral::Int(0) | AstLiteral::UInt(0))
}

// ---------------------------------------------------------------------------
// Hint insertion and application
// ---------------------------------------------------------------------------

/// Insert a hint, keeping only the one with the lowest (best) priority.
fn insert_hint(
    hints: &mut HashMap<AstVariableId, NameHint>,
    var_id: AstVariableId,
    base: &'static str,
    priority: u8,
) {
    hints
        .entry(var_id)
        .and_modify(|existing| {
            if priority < existing.priority {
                existing.base = base;
                existing.priority = priority;
            }
        })
        .or_insert(NameHint { base, priority });
}

/// Apply collected hints to the variable map, skipping variables that already
/// have names and resolving conflicts by appending `_2`, `_3`, etc.
fn apply_hints(variables: &ArcAstVariableMap, hints: &HashMap<AstVariableId, NameHint>) {
    let mut var_map = variables.write().unwrap();

    // Gather names that are already taken (from variables that have explicit names).
    let mut used_names: HashSet<String> = HashSet::new();
    for var in var_map.values() {
        if let Some(ref name) = var.name {
            used_names.insert(name.clone());
        }
    }

    // Sort hints by variable id for deterministic output.
    let mut sorted_hints: Vec<_> = hints.iter().collect();
    sorted_hints.sort_by_key(|(id, _)| **id);

    for (var_id, hint) in sorted_hints {
        let var = match var_map.get_mut(var_id) {
            Some(v) => v,
            None => continue,
        };

        // Skip variables that already have a name.
        if var.name.is_some() {
            continue;
        }

        let final_name = pick_unique_name(hint.base, &used_names);
        used_names.insert(final_name.clone());
        var.name = Some(final_name);
    }
}

/// Pick a unique name by trying `base`, then `base_2`, `base_3`, etc.
fn pick_unique_name(base: &str, used: &HashSet<String>) -> String {
    let candidate = base.to_string();
    if !used.contains(&candidate) {
        return candidate;
    }
    let mut suffix = 2u32;
    loop {
        let candidate = format!("{}_{}", base, suffix);
        if !used.contains(&candidate) {
            return candidate;
        }
        suffix += 1;
    }
}
