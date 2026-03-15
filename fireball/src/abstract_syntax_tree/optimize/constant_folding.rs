//! Evaluate constant expressions and fold identity/absorbing operations.

use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstLiteral, AstStatement, AstValue, AstVariableId, ProcessedOptimization, Wrapped,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::HashMap;

pub(super) fn fold_constants(
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

    let mut const_env: HashMap<AstVariableId, AstLiteral> = HashMap::new();
    fold_statement_list(&mut body, &mut const_env);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::ConstantFolding);
    }

    Ok(())
}

fn fold_statement_list(
    stmts: &mut Vec<WrappedAstStatement>,
    const_env: &mut HashMap<AstVariableId, AstLiteral>,
) {
    for stmt in stmts.iter_mut() {
        fold_statement(stmt, const_env);
    }
}

fn fold_statement(
    stmt: &mut WrappedAstStatement,
    const_env: &mut HashMap<AstVariableId, AstLiteral>,
) {
    match &mut stmt.statement {
        AstStatement::Declaration(lhs, rhs) => {
            if let Some(rhs) = rhs {
                fold_expression(rhs, const_env, true);
                if let AstExpression::Literal(literal) = &rhs.item {
                    const_env.insert(lhs.id, literal.clone());
                } else {
                    const_env.remove(&lhs.id);
                }
            } else {
                const_env.remove(&lhs.id);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            fold_expression(lhs, const_env, false);
            fold_expression(rhs, const_env, true);
            if let AstExpression::Variable(_, var_id) = &lhs.item {
                if let AstExpression::Literal(literal) = &rhs.item {
                    const_env.insert(*var_id, literal.clone());
                } else {
                    const_env.remove(var_id);
                }
            }
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            fold_expression(cond, const_env, true);

            // Dead branch elimination: if condition is a constant bool (either a literal
            // or a variable with a boolean const_value), replace with the surviving branch.
            let const_bool = match &cond.item {
                AstExpression::Literal(AstLiteral::Bool(b)) => Some(*b),
                AstExpression::Variable(vars, var_id) => {
                    let vars = vars.read().unwrap();
                    vars.get(var_id)
                        .and_then(|var| var.const_value.as_ref())
                        .and_then(|cv| match &cv.item {
                            AstValue::Bool(b) => Some(*b),
                            _ => None,
                        })
                }
                _ => None,
            };
            if let Some(constant) = const_bool {
                if constant {
                    // if (true) { body } ... → Block(body)
                    let mut env_true = const_env.clone();
                    fold_statement_list(branch_true, &mut env_true);
                    let body = std::mem::take(branch_true);
                    stmt.statement = AstStatement::Block(body);
                    *const_env = env_true;
                } else {
                    // if (false) { ... } else { else_body } → Block(else_body)
                    // if (false) { ... } → Empty
                    if let Some(branch_false) = branch_false {
                        let mut env_false = const_env.clone();
                        fold_statement_list(branch_false, &mut env_false);
                        let body = std::mem::take(branch_false);
                        stmt.statement = AstStatement::Block(body);
                        *const_env = env_false;
                    } else {
                        stmt.statement = AstStatement::Empty;
                    }
                }
                return;
            }

            let env_before = const_env.clone();
            let mut env_true = env_before.clone();
            fold_statement_list(branch_true, &mut env_true);
            if let Some(branch_false) = branch_false {
                let mut env_false = env_before;
                fold_statement_list(branch_false, &mut env_false);
                *const_env = intersect_envs(&env_true, &env_false);
            } else {
                *const_env = intersect_envs(const_env, &env_true);
            }
        }
        AstStatement::While(cond, body) => {
            fold_expression(cond, const_env, true);
            let mut env_loop = const_env.clone();
            fold_statement_list(body, &mut env_loop);
        }
        AstStatement::DoWhile(cond, body) => {
            // do-while evaluates body before condition, so fold body first.
            let mut env_loop = const_env.clone();
            fold_statement_list(body, &mut env_loop);
            fold_expression(cond, &mut env_loop, true);
        }
        AstStatement::For(init, cond, update, body) => {
            fold_statement(init, const_env);
            fold_expression(cond, const_env, true);
            let mut env_loop = const_env.clone();
            fold_statement_list(body, &mut env_loop);
            fold_statement(update, &mut env_loop);
        }
        AstStatement::Switch(discrim, cases, default) => {
            fold_expression(discrim, const_env, true);
            let env_before = const_env.clone();
            let mut branch_envs: Vec<HashMap<AstVariableId, AstLiteral>> = Vec::new();
            for (_lit, case_body) in cases.iter_mut() {
                let mut env_case = env_before.clone();
                fold_statement_list(case_body, &mut env_case);
                branch_envs.push(env_case);
            }
            if let Some(default_body) = default {
                let mut env_default = env_before.clone();
                fold_statement_list(default_body, &mut env_default);
                branch_envs.push(env_default);
            }
            if branch_envs.is_empty() {
                *const_env = env_before;
            } else {
                let mut result = branch_envs[0].clone();
                for other in &branch_envs[1..] {
                    result = intersect_envs(&result, other);
                }
                *const_env = result;
            }
        }
        AstStatement::Block(body) => {
            let mut env_block = const_env.clone();
            fold_statement_list(body, &mut env_block);
            *const_env = env_block;
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                fold_expression(expr, const_env, true);
            }
            const_env.clear();
        }
        AstStatement::Call(call) => {
            fold_call(call, const_env);
            const_env.clear();
        }
        AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_) => {
            const_env.clear();
        }
        AstStatement::Label(_)
        | AstStatement::Comment(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => {}
    }
}

fn fold_call(call: &mut AstCall, const_env: &HashMap<AstVariableId, AstLiteral>) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                fold_expression(arg, const_env, true);
            }
        }
        AstCall::Builtin(_, args) => match args.as_mut() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(items) => {
                for item in items.iter_mut() {
                    fold_expression(item, const_env, true);
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
                fold_expression(expr, const_env, true);
            }
            AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
                fold_expression(expr1, const_env, true);
                fold_expression(expr2, const_env, true);
            }
        },
    }
}

fn fold_expression(
    expression: &mut Wrapped<AstExpression>,
    const_env: &HashMap<AstVariableId, AstLiteral>,
    replace_root_variable: bool,
) {
    match &mut expression.item {
        AstExpression::UnaryOp(_, arg) => {
            fold_expression(arg, const_env, true);
        }
        AstExpression::BinaryOp(_, left, right) => {
            fold_expression(left, const_env, true);
            fold_expression(right, const_env, true);
        }
        AstExpression::Call(call) => {
            fold_call(call, const_env);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            fold_expression(arg, const_env, true);
        }
        AstExpression::ArrayAccess(base, idx) => {
            fold_expression(base, const_env, true);
            fold_expression(idx, const_env, true);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            fold_expression(cond, const_env, true);
            fold_expression(true_expr, const_env, true);
            fold_expression(false_expr, const_env, true);
        }
        AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_) => {}
    }

    if let Some(folded) = fold_current(expression, const_env, replace_root_variable) {
        *expression = folded;
    }
}

fn fold_current(
    expression: &Wrapped<AstExpression>,
    const_env: &HashMap<AstVariableId, AstLiteral>,
    replace_root_variable: bool,
) -> Option<Wrapped<AstExpression>> {
    match &expression.item {
        AstExpression::Variable(_, var_id) if replace_root_variable => const_env
            .get(var_id)
            .cloned()
            .map(|literal| wrap_with_source(expression, AstExpression::Literal(literal))),
        AstExpression::UnaryOp(operator, arg) => {
            // Double-unary cancellation (~~x, --x, !!x) is now handled by
            // identity-simplification.fb and operator-canonicalization.fb.
            if let AstExpression::Literal(literal) = &arg.item {
                return eval_unary(operator, literal)
                    .map(|literal| wrap_with_source(expression, AstExpression::Literal(literal)));
            }
            None
        }
        AstExpression::BinaryOp(operator, left, right) => {
            if let (AstExpression::Literal(lhs), AstExpression::Literal(rhs)) =
                (&left.item, &right.item)
            {
                if let Some(literal) = eval_binary(operator, lhs, rhs) {
                    return Some(wrap_with_source(
                        expression,
                        AstExpression::Literal(literal),
                    ));
                }
            }
            // Identity/absorbing/same-operand rules are in identity-simplification.fb.
            // Reassociation is in constant-reassociation.fb.
            None
        }
        AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_)
        | AstExpression::Call(_)
        | AstExpression::Cast(_, _)
        | AstExpression::Deref(_)
        | AstExpression::AddressOf(_)
        | AstExpression::ArrayAccess(_, _)
        | AstExpression::MemberAccess(_, _)
        | AstExpression::Ternary(_, _, _) => None,
    }
}

// eval_unary and eval_binary are now shared in opt_utils.
// Reassociation is now in constant-reassociation.fb.
use super::opt_utils::{eval_binary, eval_unary};

fn intersect_envs(
    lhs: &HashMap<AstVariableId, AstLiteral>,
    rhs: &HashMap<AstVariableId, AstLiteral>,
) -> HashMap<AstVariableId, AstLiteral> {
    lhs.iter()
        .filter_map(|(var_id, lhs_literal)| {
            rhs.get(var_id)
                .filter(|rhs_literal| *rhs_literal == lhs_literal)
                .map(|_| (*var_id, lhs_literal.clone()))
        })
        .collect()
}

fn wrap_with_source(
    source: &Wrapped<AstExpression>,
    item: AstExpression,
) -> Wrapped<AstExpression> {
    Wrapped {
        item,
        origin: source.origin.clone(),
        comment: source.comment.clone(),
    }
}
