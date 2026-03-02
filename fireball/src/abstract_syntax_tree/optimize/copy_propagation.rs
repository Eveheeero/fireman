use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstStatement, AstVariableId, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::HashMap;

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
            // Conservative: clear all before entering loop
            env.clear();
            substitute_expression(cond, env);
            propagate_statement_list(body, &mut HashMap::new());
            // After loop, nothing is safe
        }
        AstStatement::For(init, cond, update, body) => {
            propagate_statement(init, env);
            // Conservative: clear all before loop body
            env.clear();
            substitute_expression(cond, env);
            propagate_statement_list(body, &mut HashMap::new());
            propagate_statement(&mut *update, &mut HashMap::new());
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
        AstStatement::Comment(_) | AstStatement::Empty => {}
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
