use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstStatement, AstVariableId, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::{HashMap, HashSet};

pub(super) fn propagate_copies(
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

    let mut env: HashMap<AstVariableId, AstVariableId> = HashMap::new();
    propagate_statement_list(&mut body, &mut env);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::CopyPropagation);
    }

    Ok(())
}

/// Resolve a variable through the copy chain to its ultimate source.
fn resolve(env: &HashMap<AstVariableId, AstVariableId>, mut id: AstVariableId) -> AstVariableId {
    let mut visited = 0u8;
    while let Some(&src) = env.get(&id) {
        id = src;
        visited += 1;
        if visited > 32 {
            break;
        }
    }
    id
}

fn propagate_statement_list(
    stmts: &mut Vec<WrappedAstStatement>,
    env: &mut HashMap<AstVariableId, AstVariableId>,
) {
    for stmt in stmts.iter_mut() {
        propagate_statement(stmt, env);
    }
}

fn propagate_statement(
    stmt: &mut WrappedAstStatement,
    env: &mut HashMap<AstVariableId, AstVariableId>,
) {
    match &mut stmt.statement {
        AstStatement::Assignment(lhs, rhs) => {
            // First, apply substitutions to the RHS (reads)
            substitute_expression(rhs, env);
            // Then apply to LHS (for complex lhs like deref)
            substitute_expression(lhs, env);

            // Check if this is a copy: v_dst = v_src
            if let (AstExpression::Variable(_, dst_id), AstExpression::Variable(_, src_id)) =
                (&lhs.item, &rhs.item)
            {
                let dst = *dst_id;
                let src = resolve(env, *src_id);
                // Invalidate any mappings whose source is dst (it's being overwritten)
                invalidate_source(env, dst);
                // Record the new copy
                env.insert(dst, src);
            } else if let AstExpression::Variable(_, dst_id) = &lhs.item {
                // dst is being assigned a non-variable expression; invalidate
                let dst = *dst_id;
                invalidate_source(env, dst);
                env.remove(&dst);
            }
        }
        AstStatement::Declaration(lhs, rhs) => {
            if let Some(rhs) = rhs {
                substitute_expression(rhs, env);
            }
            // Declaration overwrites; invalidate
            invalidate_source(env, lhs.id);
            env.remove(&lhs.id);
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            substitute_expression(cond, env);
            let env_before = env.clone();
            let mut env_true = env_before.clone();
            propagate_statement_list(branch_true, &mut env_true);
            if let Some(branch_false) = branch_false {
                let mut env_false = env_before;
                propagate_statement_list(branch_false, &mut env_false);
                *env = intersect_envs(&env_true, &env_false);
            } else {
                *env = intersect_envs(env, &env_true);
            }
        }
        AstStatement::While(cond, body) => {
            // Pre-scan loop body for written variables; only invalidate those
            let mut written = HashSet::new();
            collect_written_vars_list(body, &mut written);
            invalidate_written(env, &written);
            substitute_expression(cond, env);
            let mut env_body = env.clone();
            propagate_statement_list(body, &mut env_body);
            // After loop, invalidate written vars again (loop may or may not execute)
            invalidate_written(env, &written);
        }
        AstStatement::DoWhile(cond, body) => {
            // do-while: body executes before condition.
            let mut written = HashSet::new();
            collect_written_vars_list(body, &mut written);
            invalidate_written(env, &written);
            let mut env_body = env.clone();
            propagate_statement_list(body, &mut env_body);
            substitute_expression(cond, &mut env_body);
            invalidate_written(env, &written);
        }
        AstStatement::For(init, cond, update, body) => {
            propagate_statement(init, env);
            // Pre-scan loop body + update for written variables; only invalidate those
            let mut written = HashSet::new();
            collect_written_vars_list(body, &mut written);
            collect_written_vars_stmt(&update.statement, &mut written);
            invalidate_written(env, &written);
            substitute_expression(cond, env);
            let mut env_body = env.clone();
            propagate_statement_list(body, &mut env_body);
            propagate_statement(&mut *update, &mut env_body);
            // After loop, invalidate written vars
            invalidate_written(env, &written);
        }
        AstStatement::Switch(discrim, cases, default) => {
            substitute_expression(discrim, env);
            let env_before = env.clone();
            let mut branch_envs: Vec<HashMap<AstVariableId, AstVariableId>> = Vec::new();
            for (_lit, case_body) in cases.iter_mut() {
                let mut env_case = env_before.clone();
                propagate_statement_list(case_body, &mut env_case);
                branch_envs.push(env_case);
            }
            if let Some(default_body) = default {
                let mut env_default = env_before.clone();
                propagate_statement_list(default_body, &mut env_default);
                branch_envs.push(env_default);
            }
            if branch_envs.is_empty() {
                *env = env_before;
            } else {
                let mut result = branch_envs[0].clone();
                for other in &branch_envs[1..] {
                    result = intersect_envs(&result, other);
                }
                *env = result;
            }
        }
        AstStatement::Block(body) => {
            propagate_statement_list(body, env);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                substitute_expression(expr, env);
            }
            env.clear();
        }
        AstStatement::Call(call) => {
            substitute_call(call, env);
            // Calls may modify anything
            env.clear();
        }
        AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_) => {
            env.clear();
        }
        AstStatement::Label(_) => {
            // Join point: cannot know which path reached here
            env.clear();
        }
        AstStatement::Comment(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => {}
    }
}

fn substitute_expression(
    expr: &mut Wrapped<AstExpression>,
    env: &HashMap<AstVariableId, AstVariableId>,
) {
    match &mut expr.item {
        AstExpression::Variable(vars, var_id) => {
            let resolved = resolve(env, *var_id);
            if resolved != *var_id {
                *var_id = resolved;
                // Update the variable map reference — the vars Arc stays the same
                // since all variables are in the same function scope
                let _ = vars;
            }
        }
        AstExpression::UnaryOp(_, arg) => {
            substitute_expression(arg, env);
        }
        AstExpression::BinaryOp(_, left, right) => {
            substitute_expression(left, env);
            substitute_expression(right, env);
        }
        AstExpression::Call(call) => {
            substitute_call(call, env);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            substitute_expression(arg, env);
        }
        AstExpression::ArrayAccess(base, idx) => {
            substitute_expression(base, env);
            substitute_expression(idx, env);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            substitute_expression(cond, env);
            substitute_expression(true_expr, env);
            substitute_expression(false_expr, env);
        }
        AstExpression::Literal(_)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => {}
    }
}

fn substitute_call(call: &mut AstCall, env: &HashMap<AstVariableId, AstVariableId>) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                substitute_expression(arg, env);
            }
        }
        AstCall::Builtin(_, args) => match args.as_mut() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(items) => {
                for item in items.iter_mut() {
                    substitute_expression(item, env);
                }
            }
            AstBuiltinFunctionArgument::ByteSizeOf(expr)
            | AstBuiltinFunctionArgument::BitSizeOf(expr)
            | AstBuiltinFunctionArgument::OperandExists(expr)
            | AstBuiltinFunctionArgument::SignedMax(expr)
            | AstBuiltinFunctionArgument::SignedMin(expr)
            | AstBuiltinFunctionArgument::UnsignedMax(expr)
            | AstBuiltinFunctionArgument::UnsignedMin(expr)
            | AstBuiltinFunctionArgument::BitOnes(expr)
            | AstBuiltinFunctionArgument::BitZeros(expr) => {
                substitute_expression(expr, env);
            }
            AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
                substitute_expression(expr1, env);
                substitute_expression(expr2, env);
            }
        },
    }
}

/// Remove all env entries whose source (value) is the given variable.
fn invalidate_source(env: &mut HashMap<AstVariableId, AstVariableId>, source: AstVariableId) {
    env.retain(|_dst, src| *src != source);
}

/// Collect all variable IDs that are written (assigned/declared) in a statement list.
fn collect_written_vars_list(stmts: &[WrappedAstStatement], out: &mut HashSet<AstVariableId>) {
    for stmt in stmts {
        collect_written_vars_stmt(&stmt.statement, out);
    }
}

fn collect_written_vars_stmt(stmt: &AstStatement, out: &mut HashSet<AstVariableId>) {
    match stmt {
        AstStatement::Assignment(lhs, _) => {
            if let AstExpression::Variable(_, var_id) = &lhs.item {
                out.insert(*var_id);
            }
        }
        AstStatement::Declaration(var, _) => {
            out.insert(var.id);
        }
        AstStatement::If(_, bt, bf) => {
            collect_written_vars_list(bt, out);
            if let Some(bf) = bf {
                collect_written_vars_list(bf, out);
            }
        }
        AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
            collect_written_vars_list(body, out);
        }
        AstStatement::For(init, _, update, body) => {
            collect_written_vars_stmt(&init.statement, out);
            collect_written_vars_stmt(&update.statement, out);
            collect_written_vars_list(body, out);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases {
                collect_written_vars_list(case_body, out);
            }
            if let Some(default_body) = default {
                collect_written_vars_list(default_body, out);
            }
        }
        AstStatement::Block(body) => {
            collect_written_vars_list(body, out);
        }
        _ => {}
    }
}

/// Invalidate env entries where the dst or src is in the written set.
fn invalidate_written(
    env: &mut HashMap<AstVariableId, AstVariableId>,
    written: &HashSet<AstVariableId>,
) {
    env.retain(|dst, src| !written.contains(dst) && !written.contains(src));
}

fn intersect_envs(
    lhs: &HashMap<AstVariableId, AstVariableId>,
    rhs: &HashMap<AstVariableId, AstVariableId>,
) -> HashMap<AstVariableId, AstVariableId> {
    lhs.iter()
        .filter_map(|(dst, lhs_src)| {
            rhs.get(dst)
                .filter(|rhs_src| *rhs_src == lhs_src)
                .map(|_| (*dst, *lhs_src))
        })
        .collect()
}
