//! Eliminate compiler-introduced temporary variables by forwarding values.

use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstStatement, AstVariableId, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::HashSet;

pub(super) fn eliminate_temporaries(
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

    eliminate_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::TemporaryElimination);
    }

    Ok(())
}

/// Extract a candidate for temporary elimination: a statement that assigns or declares
/// a variable with an initializer. Unlike expression_inlining, we do NOT require the
/// RHS to be pure.
fn extract_candidate(stmt: &AstStatement) -> Option<(AstVariableId, Wrapped<AstExpression>)> {
    match stmt {
        AstStatement::Assignment(lhs, rhs) => {
            if let AstExpression::Variable(_, var_id) = &lhs.item {
                Some((*var_id, rhs.clone()))
            } else {
                None
            }
        }
        AstStatement::Declaration(var, Some(rhs)) => Some((var.id, rhs.clone())),
        _ => None,
    }
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

/// Returns true if the statement is a side-effect barrier that prevents inlining across it.
/// For temporary elimination we block on control-flow structures (If, While, For, Switch, Block)
/// because they may contain nested writes that could clobber RHS dependencies, and on low-level
/// statements (Label, Goto, Assembly, Ir) that disrupt sequential execution. Calls are NOT
/// barriers here because the whole point of this pass is to inline non-pure expressions (like
/// calls) when they are immediately adjacent to their single use site -- and a top-level Call
/// statement is the *consumer*, not an intervening side-effect.
fn is_barrier_for_inline(stmt: &AstStatement) -> bool {
    matches!(
        stmt,
        AstStatement::Label(_)
            | AstStatement::Goto(_)
            | AstStatement::Assembly(_)
            | AstStatement::Ir(_)
            | AstStatement::If(_, _, _)
            | AstStatement::While(_, _)
            | AstStatement::For(_, _, _, _)
            | AstStatement::Switch(_, _, _)
            | AstStatement::Block(_)
    )
}

/// Replace reads of `target` with `replacement` in an expression. Returns true if any
/// replacement was made.
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

/// Recursively process nested structures, then perform temporary elimination on a
/// flat statement list. Only inlines into the IMMEDIATELY next statement (window = 1).
fn eliminate_in_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, bt, bf) => {
            eliminate_in_list(bt);
            if let Some(bf) = bf {
                eliminate_in_list(bf);
            }
        }
        AstStatement::While(_, body) => eliminate_in_list(body),
        AstStatement::For(_, _, _, body) => eliminate_in_list(body),
        AstStatement::Block(body) => eliminate_in_list(body),
        _ => {}
    }
}

fn eliminate_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first
    for stmt in stmts.iter_mut() {
        eliminate_in_statement(stmt);
    }

    let mut removals: Vec<usize> = Vec::new();
    let mut i = 0;
    while i + 1 < stmts.len() {
        let candidate = extract_candidate(&stmts[i].statement);
        if let Some((var_id, rhs)) = candidate {
            // Only inline into the IMMEDIATELY next statement (no window)
            let next = &stmts[i + 1];
            // Don't inline across barriers
            if !is_barrier_for_inline(&next.statement) {
                let read_count = count_reads_in_statement(&next.statement, var_id);
                if read_count == 1 {
                    // Check that next statement doesn't write any variable read by rhs
                    let mut rhs_deps = HashSet::new();
                    super::opt_utils::collect_expr_variables(&rhs.item, &mut rhs_deps);
                    let written = get_written_var(&stmts[i + 1].statement);
                    let deps_ok = written.map_or(true, |w| !rhs_deps.contains(&w));
                    if deps_ok {
                        substitute_in_statement(&mut stmts[i + 1].statement, var_id, &rhs);
                        removals.push(i);
                        i += 2;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }

    for &idx in removals.iter().rev() {
        stmts.remove(idx);
    }
}
