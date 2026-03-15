//! Inline single-use variable assignments into their use sites.

use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstStatement, AstVariableId, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::HashSet;

pub(super) fn inline_expressions(
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

    inline_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::ExpressionInlining);
    }

    Ok(())
}

/// Check if an expression is safe to inline (delegates to opt_utils::is_pure_expression).
fn is_safe_to_inline(expr: &AstExpression) -> bool {
    super::opt_utils::is_pure_expression(expr)
}

/// Collect all variable IDs referenced (read) in an expression (delegates to opt_utils).
fn collect_expr_variables(expr: &AstExpression, out: &mut HashSet<AstVariableId>) {
    super::opt_utils::collect_expr_variables(expr, out);
}

fn call_args(call: &AstCall) -> Vec<&Wrapped<AstExpression>> {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => args.iter().collect(),
        AstCall::Builtin(_, args) => match args.as_ref() {
            AstBuiltinFunctionArgument::None => vec![],
            AstBuiltinFunctionArgument::Print(items) => items.iter().collect(),
            AstBuiltinFunctionArgument::ByteSizeOf(e)
            | AstBuiltinFunctionArgument::BitSizeOf(e)
            | AstBuiltinFunctionArgument::OperandExists(e)
            | AstBuiltinFunctionArgument::SignedMax(e)
            | AstBuiltinFunctionArgument::SignedMin(e)
            | AstBuiltinFunctionArgument::UnsignedMax(e)
            | AstBuiltinFunctionArgument::UnsignedMin(e)
            | AstBuiltinFunctionArgument::BitOnes(e)
            | AstBuiltinFunctionArgument::BitZeros(e) => vec![e],
            AstBuiltinFunctionArgument::Sized(e1, e2) => vec![e1, e2],
        },
    }
}

/// Count how many times `target` appears as a read in an expression.
fn count_reads_in_expr(expr: &AstExpression, target: AstVariableId) -> usize {
    match expr {
        AstExpression::Variable(_, var_id) => {
            if *var_id == target {
                1
            } else {
                0
            }
        }
        AstExpression::UnaryOp(_, arg) => count_reads_in_expr(&arg.item, target),
        AstExpression::BinaryOp(_, left, right) => {
            count_reads_in_expr(&left.item, target) + count_reads_in_expr(&right.item, target)
        }
        AstExpression::Cast(_, arg) => count_reads_in_expr(&arg.item, target),
        AstExpression::Call(call) => call_args(call)
            .iter()
            .map(|a| count_reads_in_expr(&a.item, target))
            .sum(),
        AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => count_reads_in_expr(&arg.item, target),
        AstExpression::ArrayAccess(base, idx) => {
            count_reads_in_expr(&base.item, target) + count_reads_in_expr(&idx.item, target)
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            count_reads_in_expr(&cond.item, target)
                + count_reads_in_expr(&true_expr.item, target)
                + count_reads_in_expr(&false_expr.item, target)
        }
        AstExpression::Literal(_)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => 0,
    }
}

/// Count reads of `target` in a statement (including nested expressions).
fn count_reads_in_statement(stmt: &AstStatement, target: AstVariableId) -> usize {
    match stmt {
        AstStatement::Assignment(lhs, rhs) => {
            // For assignment LHS, only count reads in complex expressions (deref etc.)
            // A plain variable LHS is a write, not a read.
            let lhs_reads = match &lhs.item {
                AstExpression::Variable(_, _) => 0,
                _ => count_reads_in_expr(&lhs.item, target),
            };
            lhs_reads + count_reads_in_expr(&rhs.item, target)
        }
        AstStatement::Declaration(_, rhs) => rhs
            .as_ref()
            .map(|r| count_reads_in_expr(&r.item, target))
            .unwrap_or(0),
        AstStatement::If(cond, bt, bf) => {
            count_reads_in_expr(&cond.item, target)
                + bt.iter()
                    .map(|s| count_reads_in_statement(&s.statement, target))
                    .sum::<usize>()
                + bf.as_ref()
                    .map(|fb| {
                        fb.iter()
                            .map(|s| count_reads_in_statement(&s.statement, target))
                            .sum::<usize>()
                    })
                    .unwrap_or(0)
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            count_reads_in_expr(&cond.item, target)
                + body
                    .iter()
                    .map(|s| count_reads_in_statement(&s.statement, target))
                    .sum::<usize>()
        }
        AstStatement::For(init, cond, update, body) => {
            count_reads_in_statement(&init.statement, target)
                + count_reads_in_expr(&cond.item, target)
                + count_reads_in_statement(&update.statement, target)
                + body
                    .iter()
                    .map(|s| count_reads_in_statement(&s.statement, target))
                    .sum::<usize>()
        }
        AstStatement::Switch(discrim, cases, default) => {
            count_reads_in_expr(&discrim.item, target)
                + cases
                    .iter()
                    .flat_map(|(_, body)| body.iter())
                    .map(|s| count_reads_in_statement(&s.statement, target))
                    .sum::<usize>()
                + default
                    .as_ref()
                    .map(|d| {
                        d.iter()
                            .map(|s| count_reads_in_statement(&s.statement, target))
                            .sum::<usize>()
                    })
                    .unwrap_or(0)
        }
        AstStatement::Block(body) => body
            .iter()
            .map(|s| count_reads_in_statement(&s.statement, target))
            .sum(),
        AstStatement::Return(expr) => expr
            .as_ref()
            .map(|e| count_reads_in_expr(&e.item, target))
            .unwrap_or(0),
        AstStatement::Call(call) => call_args(call)
            .iter()
            .map(|a| count_reads_in_expr(&a.item, target))
            .sum(),
        AstStatement::Goto(_)
        | AstStatement::Label(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => 0,
    }
}

/// Get the variable ID being written on the LHS of an assignment, if it's a plain variable.
fn get_written_var(stmt: &AstStatement) -> Option<AstVariableId> {
    match stmt {
        AstStatement::Assignment(lhs, _) => {
            if let AstExpression::Variable(_, var_id) = &lhs.item {
                Some(*var_id)
            } else {
                None
            }
        }
        AstStatement::Declaration(var, _) => Some(var.id),
        _ => None,
    }
}

/// Returns true if the statement is a side-effect barrier (call, goto, jump, etc.)
fn is_barrier(stmt: &AstStatement) -> bool {
    matches!(
        stmt,
        AstStatement::Call(_)
            | AstStatement::Goto(_)
            | AstStatement::Label(_)
            | AstStatement::Assembly(_)
            | AstStatement::Ir(_)
            | AstStatement::Undefined
            | AstStatement::Exception(_)
            | AstStatement::If(_, _, _)
            | AstStatement::While(_, _)
            | AstStatement::For(_, _, _, _)
            | AstStatement::Switch(_, _, _)
    )
}

/// Replace reads of `target` with `replacement` in an expression. Returns true if any replacement
/// was made.
fn substitute_in_expr(
    expr: &mut Wrapped<AstExpression>,
    target: AstVariableId,
    replacement: &Wrapped<AstExpression>,
) -> bool {
    match &expr.item {
        AstExpression::Variable(_, var_id) if *var_id == target => {
            *expr = replacement.clone();
            return true;
        }
        _ => {}
    }

    match &mut expr.item {
        AstExpression::UnaryOp(_, arg) => substitute_in_expr(arg, target, replacement),
        AstExpression::BinaryOp(_, left, right) => {
            let l = substitute_in_expr(left, target, replacement);
            let r = substitute_in_expr(right, target, replacement);
            l || r
        }
        AstExpression::Cast(_, arg) => substitute_in_expr(arg, target, replacement),
        AstExpression::Call(call) => substitute_in_call(call, target, replacement),
        AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => substitute_in_expr(arg, target, replacement),
        AstExpression::ArrayAccess(base, idx) => {
            let b = substitute_in_expr(base, target, replacement);
            let i = substitute_in_expr(idx, target, replacement);
            b || i
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            let c = substitute_in_expr(cond, target, replacement);
            let t = substitute_in_expr(true_expr, target, replacement);
            let f = substitute_in_expr(false_expr, target, replacement);
            c || t || f
        }
        AstExpression::Variable(_, _)
        | AstExpression::Literal(_)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => false,
    }
}

fn substitute_in_call(
    call: &mut AstCall,
    target: AstVariableId,
    replacement: &Wrapped<AstExpression>,
) -> bool {
    let mut changed = false;
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                changed |= substitute_in_expr(arg, target, replacement);
            }
        }
        AstCall::Builtin(_, args) => match args.as_mut() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(items) => {
                for item in items.iter_mut() {
                    changed |= substitute_in_expr(item, target, replacement);
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
                changed |= substitute_in_expr(e, target, replacement);
            }
            AstBuiltinFunctionArgument::Sized(e1, e2) => {
                changed |= substitute_in_expr(e1, target, replacement);
                changed |= substitute_in_expr(e2, target, replacement);
            }
        },
    }
    changed
}

/// Replace all reads of `target` in a statement with `replacement`.
fn substitute_in_statement(
    stmt: &mut AstStatement,
    target: AstVariableId,
    replacement: &Wrapped<AstExpression>,
) {
    match stmt {
        AstStatement::Assignment(lhs, rhs) => {
            // Only substitute in LHS if it's not a plain variable write
            if !matches!(&lhs.item, AstExpression::Variable(_, id) if *id == target) {
                substitute_in_expr(lhs, target, replacement);
            }
            substitute_in_expr(rhs, target, replacement);
        }
        AstStatement::Declaration(_, rhs) => {
            if let Some(rhs) = rhs {
                substitute_in_expr(rhs, target, replacement);
            }
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                substitute_in_expr(expr, target, replacement);
            }
        }
        AstStatement::Call(call) => {
            substitute_in_call(call, target, replacement);
        }
        AstStatement::Block(body) => {
            for s in body.iter_mut() {
                substitute_in_statement(&mut s.statement, target, replacement);
            }
        }
        _ => {}
    }
}

/// Maximum number of statements to scan forward when looking for an inlining target.
const INLINE_WINDOW: usize = 4;

/// Check if we can inline into an If statement's condition only (not branches).
/// Returns true if var_id is read exactly once in the condition and zero times in branches,
/// the rhs is pure, and no deps are written before the condition is evaluated.
fn can_inline_into_if_condition(
    cond: &Wrapped<AstExpression>,
    bt: &[WrappedAstStatement],
    bf: &Option<Vec<WrappedAstStatement>>,
    var_id: AstVariableId,
) -> bool {
    let cond_reads = count_reads_in_expr(&cond.item, var_id);
    if cond_reads != 1 {
        return false;
    }
    let branch_reads: usize = bt
        .iter()
        .map(|s| count_reads_in_statement(&s.statement, var_id))
        .sum();
    if branch_reads != 0 {
        return false;
    }
    if let Some(bf) = bf {
        let else_reads: usize = bf
            .iter()
            .map(|s| count_reads_in_statement(&s.statement, var_id))
            .sum();
        if else_reads != 0 {
            return false;
        }
    }
    true
}

fn inline_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Process nested structures first
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                inline_statement_list(bt);
                if let Some(bf) = bf {
                    inline_statement_list(bf);
                }
            }
            AstStatement::While(_, body) => inline_statement_list(body),
            AstStatement::For(_, _, _, body) => inline_statement_list(body),
            AstStatement::Block(body) => inline_statement_list(body),
            _ => {}
        }
    }

    // Scan candidates and try to inline into subsequent statements within a window
    let mut removals: Vec<usize> = Vec::new();
    let mut i = 0;
    while i + 1 < stmts.len() {
        // Extract candidate: Assignment(Variable(_, var_id), rhs) or Declaration(var, Some(rhs))
        let candidate = match &stmts[i].statement {
            AstStatement::Assignment(lhs, rhs) => {
                if let AstExpression::Variable(_, var_id) = &lhs.item {
                    if is_safe_to_inline(&rhs.item) {
                        Some((*var_id, rhs.clone()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            AstStatement::Declaration(var, Some(rhs)) => {
                if is_safe_to_inline(&rhs.item) {
                    Some((var.id, rhs.clone()))
                } else {
                    None
                }
            }
            _ => None,
        };

        let Some((var_id, rhs_expr)) = candidate else {
            i += 1;
            continue;
        };

        let mut expr_deps = HashSet::new();
        collect_expr_variables(&rhs_expr.item, &mut expr_deps);

        let window_end = (i + 1 + INLINE_WINDOW).min(stmts.len());
        let mut inlined = false;

        for j in (i + 1)..window_end {
            // Check if stmts[j] writes var_id (def killed)
            if let Some(written) = get_written_var(&stmts[j].statement) {
                if written == var_id {
                    break;
                }
                // Check if stmts[j] writes any dep of rhs_expr (deps modified)
                if expr_deps.contains(&written) {
                    break;
                }
            }

            // Allow inlining into If conditions: if the stmt is an If and the var
            // is read exactly once in the condition (not in branches), treat it as
            // a valid target for inlining into the condition expression.
            if let AstStatement::If(cond, bt, bf) = &stmts[j].statement {
                if is_safe_to_inline(&rhs_expr.item)
                    && can_inline_into_if_condition(cond, bt, bf, var_id)
                {
                    // No deps written before condition evaluation (condition is first)
                    let mut deps_ok = true;
                    for dep in &expr_deps {
                        if get_written_var(&stmts[j].statement) == Some(*dep) {
                            deps_ok = false;
                            break;
                        }
                    }
                    if deps_ok {
                        if let AstStatement::If(cond, _, _) = &mut stmts[j].statement {
                            substitute_in_expr(cond, var_id, &rhs_expr);
                        }
                        removals.push(i);
                        inlined = true;
                        break;
                    }
                }
                // If is normally a barrier, stop scanning
                break;
            }

            // Standard barrier check (excluding If which we handle above)
            if is_barrier(&stmts[j].statement) {
                break;
            }

            let read_count = count_reads_in_statement(&stmts[j].statement, var_id);
            if read_count == 1 {
                // Single read: inline
                substitute_in_statement(&mut stmts[j].statement, var_id, &rhs_expr);
                removals.push(i);
                inlined = true;
                break;
            }
            if read_count > 0 {
                // Multiple reads: can't inline, stop
                break;
            }
            // read_count == 0: var not used here, continue scanning
        }

        if inlined {
            i += 2;
        } else {
            i += 1;
        }
    }

    // Remove inlined assignments/declarations in reverse order
    for &idx in removals.iter().rev() {
        stmts.remove(idx);
    }
}
