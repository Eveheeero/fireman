use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstStatement, AstVariableId, WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::HashSet;

/// AST-level dead store elimination.
///
/// Forward-scans the statement list and removes assignments to variables that
/// are overwritten before being read. Unlike `collapse_unused_variable` which
/// relies on IR `data_access_count`, this works purely at the AST level and
/// catches dead stores in multi-access variables.
pub(super) fn eliminate_dead_stores(
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
    }

    Ok(())
}

fn eliminate_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first
    for stmt in stmts.iter_mut() {
        eliminate_in_statement(stmt);
    }

    // Backward scan: track which variables are "live" (read before next write).
    // If we see a write to a variable that is not live, it's a dead store.
    //
    // When `all_live` is true we treat every variable as potentially read
    // (e.g. after seeing a call that could observe any local via aliasing).
    // Once set, it stays set for the remainder of the backward scan — we
    // cannot prove that earlier stores are dead if a later call may read them.
    let mut live: HashSet<AstVariableId> = HashSet::new();
    let mut all_live = false;
    let mut removals: Vec<usize> = Vec::new();

    for i in (0..stmts.len()).rev() {
        let stmt = &stmts[i];
        match &stmt.statement {
            AstStatement::Assignment(lhs, rhs) => {
                if let AstExpression::Variable(_, var_id) = &lhs.item {
                    let var_id = *var_id;
                    if !super::opt_utils::is_pure_expression(&rhs.item) {
                        // Side-effecting RHS: can't remove
                        collect_reads_expr(&rhs.item, &mut live);
                    } else if !all_live && !live.contains(&var_id) {
                        // Pure RHS and variable is not live: dead store
                        removals.push(i);
                        continue;
                    } else {
                        // Variable is live: this write satisfies the read
                        live.remove(&var_id);
                        collect_reads_expr(&rhs.item, &mut live);
                    }
                } else {
                    // Complex LHS (deref, array access, etc.): reads in both sides
                    collect_reads_expr(&lhs.item, &mut live);
                    collect_reads_expr(&rhs.item, &mut live);
                }
            }
            AstStatement::Declaration(var, rhs) => {
                let var_id = var.id;
                if let Some(rhs) = rhs {
                    if !super::opt_utils::is_pure_expression(&rhs.item) {
                        collect_reads_expr(&rhs.item, &mut live);
                    } else if !all_live && !live.contains(&var_id) {
                        removals.push(i);
                        continue;
                    } else {
                        live.remove(&var_id);
                        collect_reads_expr(&rhs.item, &mut live);
                    }
                } else if !all_live && !live.contains(&var_id) {
                    removals.push(i);
                    continue;
                }
            }
            AstStatement::Return(expr) => {
                live.clear();
                all_live = false;
                if let Some(expr) = expr {
                    collect_reads_expr(&expr.item, &mut live);
                }
            }
            AstStatement::Call(call) => {
                // A call can observe any variable through aliasing.
                // Mark everything as live from this point backwards.
                collect_reads_call(call, &mut live);
                all_live = true;
            }
            AstStatement::If(cond, bt, bf) => {
                collect_reads_expr(&cond.item, &mut live);
                collect_reads_list(bt, &mut live);
                if let Some(bf) = bf {
                    collect_reads_list(bf, &mut live);
                }
                // Branches may contain calls
                if list_contains_call(bt) || bf.as_ref().is_some_and(|bf| list_contains_call(bf)) {
                    all_live = true;
                }
            }
            AstStatement::While(cond, body)
            | AstStatement::DoWhile(cond, body)
            | AstStatement::For(_, cond, _, body) => {
                collect_reads_expr(&cond.item, &mut live);
                collect_reads_list(body, &mut live);
                if list_contains_call(body) {
                    all_live = true;
                }
            }
            AstStatement::Switch(discrim, cases, default) => {
                collect_reads_expr(&discrim.item, &mut live);
                for (_, case_body) in cases {
                    collect_reads_list(case_body, &mut live);
                    if list_contains_call(case_body) {
                        all_live = true;
                    }
                }
                if let Some(default_body) = default {
                    collect_reads_list(default_body, &mut live);
                    if list_contains_call(default_body) {
                        all_live = true;
                    }
                }
            }
            AstStatement::Block(body) => {
                collect_reads_list(body, &mut live);
                if list_contains_call(body) {
                    all_live = true;
                }
            }
            AstStatement::Goto(_)
            | AstStatement::Label(_)
            | AstStatement::Assembly(_)
            | AstStatement::Ir(_)
            | AstStatement::Undefined
            | AstStatement::Exception(_) => {
                // Control flow disruption: conservatively treat all as live
                all_live = true;
            }
            AstStatement::Comment(_)
            | AstStatement::Break
            | AstStatement::Continue
            | AstStatement::Empty => {}
        }
    }

    // Remove dead stores in reverse order (indices are already from high to low)
    removals.sort_unstable();
    for &idx in removals.iter().rev() {
        stmts.remove(idx);
    }
}

fn eliminate_in_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, bt, bf) => {
            eliminate_in_list(bt);
            if let Some(bf) = bf {
                eliminate_in_list(bf);
            }
        }
        AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => eliminate_in_list(body),
        AstStatement::For(_, _, _, body) => eliminate_in_list(body),
        AstStatement::Block(body) => eliminate_in_list(body),
        AstStatement::Switch(_, cases, default) => {
            for (_, case_body) in cases.iter_mut() {
                eliminate_in_list(case_body);
            }
            if let Some(default_body) = default {
                eliminate_in_list(default_body);
            }
        }
        _ => {}
    }
}

fn collect_reads_expr(expr: &AstExpression, out: &mut HashSet<AstVariableId>) {
    super::opt_utils::collect_expr_variables(expr, out);
}

fn collect_reads_call(call: &AstCall, out: &mut HashSet<AstVariableId>) {
    match call {
        AstCall::Variable { var_id, args, .. } => {
            out.insert(*var_id);
            for arg in args {
                collect_reads_expr(&arg.item, out);
            }
        }
        AstCall::Function { args, .. } | AstCall::Unknown(_, args) => {
            for arg in args {
                collect_reads_expr(&arg.item, out);
            }
        }
        AstCall::Builtin(_, args) => match args.as_ref() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(items) => {
                for item in items {
                    collect_reads_expr(&item.item, out);
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
                collect_reads_expr(&e.item, out);
            }
            AstBuiltinFunctionArgument::Sized(e1, e2) => {
                collect_reads_expr(&e1.item, out);
                collect_reads_expr(&e2.item, out);
            }
        },
    }
}

fn collect_reads_list(stmts: &[WrappedAstStatement], out: &mut HashSet<AstVariableId>) {
    for stmt in stmts {
        collect_reads_stmt(&stmt.statement, out);
    }
}

fn collect_reads_stmt(stmt: &AstStatement, out: &mut HashSet<AstVariableId>) {
    match stmt {
        AstStatement::Assignment(lhs, rhs) => {
            collect_reads_expr(&lhs.item, out);
            collect_reads_expr(&rhs.item, out);
        }
        AstStatement::Declaration(_, rhs) => {
            if let Some(rhs) = rhs {
                collect_reads_expr(&rhs.item, out);
            }
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                collect_reads_expr(&expr.item, out);
            }
        }
        AstStatement::Call(call) => collect_reads_call(call, out),
        AstStatement::If(cond, bt, bf) => {
            collect_reads_expr(&cond.item, out);
            collect_reads_list(bt, out);
            if let Some(bf) = bf {
                collect_reads_list(bf, out);
            }
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            collect_reads_expr(&cond.item, out);
            collect_reads_list(body, out);
        }
        AstStatement::For(init, cond, update, body) => {
            collect_reads_stmt(&init.statement, out);
            collect_reads_expr(&cond.item, out);
            collect_reads_stmt(&update.statement, out);
            collect_reads_list(body, out);
        }
        AstStatement::Switch(discrim, cases, default) => {
            collect_reads_expr(&discrim.item, out);
            for (_, case_body) in cases {
                collect_reads_list(case_body, out);
            }
            if let Some(default_body) = default {
                collect_reads_list(default_body, out);
            }
        }
        AstStatement::Block(body) => collect_reads_list(body, out),
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

fn list_contains_call(stmts: &[WrappedAstStatement]) -> bool {
    stmts.iter().any(|s| stmt_contains_call(&s.statement))
}

fn stmt_contains_call(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::Call(_) => true,
        AstStatement::Assignment(lhs, rhs) => {
            expr_contains_call(&lhs.item) || expr_contains_call(&rhs.item)
        }
        AstStatement::Declaration(_, rhs) => {
            rhs.as_ref().is_some_and(|r| expr_contains_call(&r.item))
        }
        AstStatement::If(cond, bt, bf) => {
            expr_contains_call(&cond.item)
                || list_contains_call(bt)
                || bf.as_ref().is_some_and(|bf| list_contains_call(bf))
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            expr_contains_call(&cond.item) || list_contains_call(body)
        }
        AstStatement::For(init, cond, update, body) => {
            stmt_contains_call(&init.statement)
                || expr_contains_call(&cond.item)
                || stmt_contains_call(&update.statement)
                || list_contains_call(body)
        }
        AstStatement::Switch(discrim, cases, default) => {
            expr_contains_call(&discrim.item)
                || cases.iter().any(|(_, body)| list_contains_call(body))
                || default.as_ref().is_some_and(|d| list_contains_call(d))
        }
        AstStatement::Block(body) => list_contains_call(body),
        AstStatement::Return(expr) => expr.as_ref().is_some_and(|e| expr_contains_call(&e.item)),
        _ => false,
    }
}

fn expr_contains_call(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Call(_) => true,
        AstExpression::UnaryOp(_, arg)
        | AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => expr_contains_call(&arg.item),
        AstExpression::BinaryOp(_, l, r) | AstExpression::ArrayAccess(l, r) => {
            expr_contains_call(&l.item) || expr_contains_call(&r.item)
        }
        AstExpression::Ternary(c, t, f) => {
            expr_contains_call(&c.item)
                || expr_contains_call(&t.item)
                || expr_contains_call(&f.item)
        }
        _ => false,
    }
}
