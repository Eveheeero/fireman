use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstStatement, AstVariableId, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::{HashMap, HashSet};

/// Approximate live range for a variable within the top-level statement list.
struct VarRange {
    first: usize,
    last: usize,
}

/// Merge non-interfering variables that share the same declared type.
///
/// Two variables can be coalesced when their approximate live ranges
/// (first-use .. last-use at the top-level statement list) do not overlap.
/// Only variables that appear in `Declaration` statements are considered,
/// because the declaration carries the canonical `AstValueType`.
pub(super) fn coalesce_variables(
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

    coalesce_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::VariableCoalescing);
    }

    Ok(())
}

fn coalesce_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Step 1: Collect declared variable types from the top-level statement list.
    let declared_types = collect_declared_types(stmts);

    // Step 2: Compute approximate live ranges.
    let ranges = compute_var_ranges(stmts);

    // Step 3: Build a list of declared variable IDs sorted for deterministic iteration.
    let mut var_ids: Vec<AstVariableId> = declared_types.keys().copied().collect();
    var_ids.sort();

    // Step 4: Greedily merge non-interfering, same-type variables.
    //
    // `rename_map` tracks var_b -> var_a replacements. When a variable has
    // already been merged away its range is no longer relevant; the surviving
    // variable's range is extended to cover both.
    let mut rename_map: HashMap<AstVariableId, AstVariableId> = HashMap::new();
    let mut effective_ranges: HashMap<AstVariableId, VarRange> = ranges;

    for i in 0..var_ids.len() {
        let var_a = var_ids[i];
        if rename_map.contains_key(&var_a) {
            continue; // already merged into another variable
        }
        let Some(type_a) = declared_types.get(&var_a) else {
            continue;
        };

        for j in (i + 1)..var_ids.len() {
            let var_b = var_ids[j];
            if rename_map.contains_key(&var_b) {
                continue;
            }
            let Some(type_b) = declared_types.get(&var_b) else {
                continue;
            };

            // Types must match.
            if type_a != type_b {
                continue;
            }

            // Ranges must not overlap.
            let range_a = match effective_ranges.get(&var_a) {
                Some(r) => r,
                None => continue,
            };
            let range_b = match effective_ranges.get(&var_b) {
                Some(r) => r,
                None => continue,
            };

            if ranges_overlap(range_a, range_b) {
                continue;
            }

            // Merge var_b into var_a: extend var_a's range to cover var_b.
            let new_first = range_a.first.min(range_b.first);
            let new_last = range_a.last.max(range_b.last);
            effective_ranges.insert(
                var_a,
                VarRange {
                    first: new_first,
                    last: new_last,
                },
            );
            effective_ranges.remove(&var_b);
            rename_map.insert(var_b, var_a);
        }
    }

    if rename_map.is_empty() {
        return;
    }

    // Step 5: Record which top-level statement indices are declarations for
    // merged-away variables *before* renaming changes their IDs.
    let merged_away: HashSet<AstVariableId> = rename_map.keys().copied().collect();
    let removal_indices: HashSet<usize> = stmts
        .iter()
        .enumerate()
        .filter_map(|(idx, stmt)| {
            if let AstStatement::Declaration(var, _) = &stmt.statement {
                if merged_away.contains(&var.id) {
                    return Some(idx);
                }
            }
            None
        })
        .collect();

    // Step 6: Apply variable renames throughout the statement list.
    apply_renames(stmts, &rename_map);

    // Step 7: Remove the declarations that originally belonged to merged-away
    // variables. We identified them by index before renaming so the surviving
    // variable's own declaration (and its initializer) is preserved.
    let mut idx = 0;
    stmts.retain(|_| {
        let keep = !removal_indices.contains(&idx);
        idx += 1;
        keep
    });
}

// ---------------------------------------------------------------------------
// Live range computation
// ---------------------------------------------------------------------------

fn compute_var_ranges(stmts: &[WrappedAstStatement]) -> HashMap<AstVariableId, VarRange> {
    let mut ranges: HashMap<AstVariableId, VarRange> = HashMap::new();
    for (idx, stmt) in stmts.iter().enumerate() {
        let mut vars = HashSet::new();
        collect_all_variables_in_statement(&stmt.statement, &mut vars);
        for var_id in vars {
            ranges
                .entry(var_id)
                .and_modify(|r| r.last = idx)
                .or_insert(VarRange {
                    first: idx,
                    last: idx,
                });
        }
    }
    ranges
}

fn ranges_overlap(a: &VarRange, b: &VarRange) -> bool {
    a.first <= b.last && b.first <= a.last
}

// ---------------------------------------------------------------------------
// Collecting declared types
// ---------------------------------------------------------------------------

fn collect_declared_types(
    stmts: &[WrappedAstStatement],
) -> HashMap<AstVariableId, crate::abstract_syntax_tree::AstValueType> {
    let mut types = HashMap::new();
    for stmt in stmts {
        if let AstStatement::Declaration(var, _) = &stmt.statement {
            types.entry(var.id).or_insert_with(|| var.var_type.clone());
        }
    }
    types
}

// ---------------------------------------------------------------------------
// Variable collection (walks all expressions in a statement)
// ---------------------------------------------------------------------------

fn collect_all_variables_in_statement(stmt: &AstStatement, out: &mut HashSet<AstVariableId>) {
    match stmt {
        AstStatement::Declaration(var, rhs) => {
            out.insert(var.id);
            if let Some(rhs) = rhs {
                super::opt_utils::collect_expr_variables(&rhs.item, out);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            super::opt_utils::collect_expr_variables(&lhs.item, out);
            super::opt_utils::collect_expr_variables(&rhs.item, out);
        }
        AstStatement::If(cond, bt, bf) => {
            super::opt_utils::collect_expr_variables(&cond.item, out);
            collect_all_variables_in_list(bt, out);
            if let Some(bf) = bf {
                collect_all_variables_in_list(bf, out);
            }
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            super::opt_utils::collect_expr_variables(&cond.item, out);
            collect_all_variables_in_list(body, out);
        }
        AstStatement::For(init, cond, update, body) => {
            collect_all_variables_in_statement(&init.statement, out);
            super::opt_utils::collect_expr_variables(&cond.item, out);
            collect_all_variables_in_statement(&update.statement, out);
            collect_all_variables_in_list(body, out);
        }
        AstStatement::Switch(discrim, cases, default) => {
            super::opt_utils::collect_expr_variables(&discrim.item, out);
            for (_, case_body) in cases {
                collect_all_variables_in_list(case_body, out);
            }
            if let Some(default_body) = default {
                collect_all_variables_in_list(default_body, out);
            }
        }
        AstStatement::Block(body) => {
            collect_all_variables_in_list(body, out);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                super::opt_utils::collect_expr_variables(&expr.item, out);
            }
        }
        AstStatement::Call(call) => {
            collect_all_variables_in_call(call, out);
        }
        AstStatement::Goto(_)
        | AstStatement::Label(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => {}
    }
}

fn collect_all_variables_in_list(stmts: &[WrappedAstStatement], out: &mut HashSet<AstVariableId>) {
    for stmt in stmts {
        collect_all_variables_in_statement(&stmt.statement, out);
    }
}

fn collect_all_variables_in_call(call: &AstCall, out: &mut HashSet<AstVariableId>) {
    match call {
        AstCall::Variable { var_id, args, .. } => {
            out.insert(*var_id);
            for arg in args {
                super::opt_utils::collect_expr_variables(&arg.item, out);
            }
        }
        AstCall::Function { args, .. } | AstCall::Unknown(_, args) => {
            for arg in args {
                super::opt_utils::collect_expr_variables(&arg.item, out);
            }
        }
        AstCall::Builtin(_, args) => match args.as_ref() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(items) => {
                for item in items {
                    super::opt_utils::collect_expr_variables(&item.item, out);
                }
            }
            AstBuiltinFunctionArgument::ByteSizeOf(e)
            | AstBuiltinFunctionArgument::BitSizeOf(e)
            | AstBuiltinFunctionArgument::OperandExists(e)
            | AstBuiltinFunctionArgument::SignedMax(e)
            | AstBuiltinFunctionArgument::SignedMin(e)
            | AstBuiltinFunctionArgument::UnsignedMax(e)
            | AstBuiltinFunctionArgument::UnsignedMin(e)
            | AstBuiltinFunctionArgument::BitOnes(e)
            | AstBuiltinFunctionArgument::BitZeros(e) => {
                super::opt_utils::collect_expr_variables(&e.item, out);
            }
            AstBuiltinFunctionArgument::Sized(e1, e2) => {
                super::opt_utils::collect_expr_variables(&e1.item, out);
                super::opt_utils::collect_expr_variables(&e2.item, out);
            }
        },
    }
}

// ---------------------------------------------------------------------------
// Rename application (replace var_b with var_a everywhere)
// ---------------------------------------------------------------------------

fn apply_renames(
    stmts: &mut Vec<WrappedAstStatement>,
    rename_map: &HashMap<AstVariableId, AstVariableId>,
) {
    for stmt in stmts.iter_mut() {
        rename_in_statement(stmt, rename_map);
    }
}

fn rename_in_statement(
    stmt: &mut WrappedAstStatement,
    rename_map: &HashMap<AstVariableId, AstVariableId>,
) {
    match &mut stmt.statement {
        AstStatement::Declaration(var, rhs) => {
            if let Some(&new_id) = rename_map.get(&var.id) {
                var.id = new_id;
            }
            if let Some(rhs) = rhs {
                rename_in_expression(rhs, rename_map);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            rename_in_expression(lhs, rename_map);
            rename_in_expression(rhs, rename_map);
        }
        AstStatement::If(cond, bt, bf) => {
            rename_in_expression(cond, rename_map);
            rename_in_list(bt, rename_map);
            if let Some(bf) = bf {
                rename_in_list(bf, rename_map);
            }
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            rename_in_expression(cond, rename_map);
            rename_in_list(body, rename_map);
        }
        AstStatement::For(init, cond, update, body) => {
            rename_in_statement(init, rename_map);
            rename_in_expression(cond, rename_map);
            rename_in_statement(update, rename_map);
            rename_in_list(body, rename_map);
        }
        AstStatement::Switch(discrim, cases, default) => {
            rename_in_expression(discrim, rename_map);
            for (_, case_body) in cases.iter_mut() {
                rename_in_list(case_body, rename_map);
            }
            if let Some(default_body) = default {
                rename_in_list(default_body, rename_map);
            }
        }
        AstStatement::Block(body) => {
            rename_in_list(body, rename_map);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                rename_in_expression(expr, rename_map);
            }
        }
        AstStatement::Call(call) => {
            rename_in_call(call, rename_map);
        }
        AstStatement::Goto(_)
        | AstStatement::Label(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => {}
    }
}

fn rename_in_list(
    stmts: &mut Vec<WrappedAstStatement>,
    rename_map: &HashMap<AstVariableId, AstVariableId>,
) {
    for stmt in stmts.iter_mut() {
        rename_in_statement(stmt, rename_map);
    }
}

fn rename_in_expression(
    expr: &mut Wrapped<AstExpression>,
    rename_map: &HashMap<AstVariableId, AstVariableId>,
) {
    match &mut expr.item {
        AstExpression::Variable(_, var_id) => {
            if let Some(&new_id) = rename_map.get(var_id) {
                *var_id = new_id;
            }
        }
        AstExpression::UnaryOp(_, arg) => {
            rename_in_expression(arg, rename_map);
        }
        AstExpression::BinaryOp(_, left, right) => {
            rename_in_expression(left, rename_map);
            rename_in_expression(right, rename_map);
        }
        AstExpression::Call(call) => {
            rename_in_call(call, rename_map);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            rename_in_expression(arg, rename_map);
        }
        AstExpression::ArrayAccess(base, idx) => {
            rename_in_expression(base, rename_map);
            rename_in_expression(idx, rename_map);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            rename_in_expression(cond, rename_map);
            rename_in_expression(true_expr, rename_map);
            rename_in_expression(false_expr, rename_map);
        }
        AstExpression::Literal(_)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => {}
    }
}

fn rename_in_call(call: &mut AstCall, rename_map: &HashMap<AstVariableId, AstVariableId>) {
    match call {
        AstCall::Variable { var_id, args, .. } => {
            if let Some(&new_id) = rename_map.get(var_id) {
                *var_id = new_id;
            }
            for arg in args.iter_mut() {
                rename_in_expression(arg, rename_map);
            }
        }
        AstCall::Function { args, .. } | AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                rename_in_expression(arg, rename_map);
            }
        }
        AstCall::Builtin(_, args) => match args.as_mut() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(items) => {
                for item in items.iter_mut() {
                    rename_in_expression(item, rename_map);
                }
            }
            AstBuiltinFunctionArgument::ByteSizeOf(e)
            | AstBuiltinFunctionArgument::BitSizeOf(e)
            | AstBuiltinFunctionArgument::OperandExists(e)
            | AstBuiltinFunctionArgument::SignedMax(e)
            | AstBuiltinFunctionArgument::SignedMin(e)
            | AstBuiltinFunctionArgument::UnsignedMax(e)
            | AstBuiltinFunctionArgument::UnsignedMin(e)
            | AstBuiltinFunctionArgument::BitOnes(e)
            | AstBuiltinFunctionArgument::BitZeros(e) => {
                rename_in_expression(e, rename_map);
            }
            AstBuiltinFunctionArgument::Sized(e1, e2) => {
                rename_in_expression(e1, rename_map);
                rename_in_expression(e2, rename_map);
            }
        },
    }
}
