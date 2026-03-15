//! Infer variable lifetimes and insert scope boundaries.

use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstStatement, AstVariableId, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

/// Narrow variable lifetimes by moving declarations closer to their first use.
///
/// This pass scans for uninitialized declarations (`Declaration(var, None)`)
/// and, when the first reference is a plain assignment to that variable,
/// merges them into a single `Declaration(var, Some(rhs))`. This reduces
/// the distance between declaration and use, improving readability.
pub(super) fn narrow_lifetimes(
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

    narrow_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::LifetimeScoping);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Top-level recursive driver
// ---------------------------------------------------------------------------

/// Process a statement list: recurse into nested structures, then merge
/// declarations with their first assignment.
fn narrow_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first (bottom-up).
    for stmt in stmts.iter_mut() {
        narrow_nested(&mut stmt.statement);
    }

    // Merge uninitialized declarations with their first assignment.
    merge_declaration_with_first_assignment(stmts);
}

/// Recurse into nested blocks inside a single statement.
fn narrow_nested(stmt: &mut AstStatement) {
    match stmt {
        AstStatement::If(_, bt, bf) => {
            narrow_statement_list(bt);
            if let Some(bf) = bf {
                narrow_statement_list(bf);
            }
        }
        AstStatement::While(_, body) => narrow_statement_list(body),
        AstStatement::For(_, _, _, body) => narrow_statement_list(body),
        AstStatement::Block(body) => narrow_statement_list(body),
        AstStatement::Switch(_, cases, default) => {
            for (_, case_body) in cases.iter_mut() {
                narrow_statement_list(case_body);
            }
            if let Some(default_body) = default {
                narrow_statement_list(default_body);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Step 3: merge Declaration(var, None) with the first subsequent assignment
// ---------------------------------------------------------------------------

fn merge_declaration_with_first_assignment(stmts: &mut Vec<WrappedAstStatement>) {
    let mut i = 0;
    while i < stmts.len() {
        if let AstStatement::Declaration(var, None) = &stmts[i].statement {
            let var_id = var.id;
            // Find first use after i
            let mut first_assign = None;
            let has_read_before_assign = false;
            for j in (i + 1)..stmts.len() {
                let reads = count_reads_in_statement(&stmts[j].statement, var_id);
                let writes = get_written_var(&stmts[j].statement);
                if writes == Some(var_id) && reads == 0 && !has_read_before_assign {
                    first_assign = Some(j);
                    break;
                }
                if reads > 0 || writes == Some(var_id) {
                    break;
                }
                // Check for barriers
                if is_barrier(&stmts[j].statement) {
                    break;
                }
            }
            if let Some(assign_idx) = first_assign {
                // Merge: Declaration(var, None) + Assignment(var, rhs) -> Declaration(var, Some(rhs))
                if let AstStatement::Assignment(_, rhs) = &stmts[assign_idx].statement {
                    let rhs = rhs.clone();
                    if let AstStatement::Declaration(_, init) = &mut stmts[i].statement {
                        *init = Some(rhs);
                    }
                    stmts.remove(assign_idx);
                    // Don't increment i — check again in case the merged declaration
                    // can participate in further transformations on the next iteration.
                    continue;
                }
            }
        }
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// Helpers: count reads, get written var, barrier detection
// ---------------------------------------------------------------------------

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

/// Count reads of `target` in a statement (including nested expressions and blocks).
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

/// Get the variable ID being written on the LHS of an assignment, if it is a
/// plain variable. Also returns the declared variable ID for declarations.
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

/// Returns true if the statement is a side-effect barrier that prevents
/// reordering across it (calls, gotos, labels, control flow, etc.).
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
