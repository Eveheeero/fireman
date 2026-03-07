use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId,
        AstFunctionVersion, AstLiteral, AstStatement, AstUnaryOperator, AstValue, AstVariableId,
        ProcessedOptimization, Wrapped, WrappedAstStatement,
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
            // Double-unary cancellation: ~~x -> x, --x -> x, !!x -> x
            if let AstExpression::UnaryOp(inner_op, inner_arg) = &arg.item {
                let cancels = matches!(
                    (operator, inner_op),
                    (AstUnaryOperator::BitNot, AstUnaryOperator::BitNot)
                        | (AstUnaryOperator::Negate, AstUnaryOperator::Negate)
                        | (AstUnaryOperator::Not, AstUnaryOperator::Not)
                );
                if cancels {
                    return Some(wrap_with_source(expression, inner_arg.item.clone()));
                }
            }
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
            fold_identity(expression, operator, left, right)
                .or_else(|| fold_reassociate(expression, operator, left, right))
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

fn fold_identity(
    source: &Wrapped<AstExpression>,
    operator: &AstBinaryOperator,
    left: &Wrapped<AstExpression>,
    right: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    use super::opt_utils::{expr_structurally_equal, is_pure_expression};

    // Identity element rules: x op identity = x
    match (operator, &left.item, &right.item) {
        (AstBinaryOperator::Add, _, AstExpression::Literal(AstLiteral::Int(0)))
        | (AstBinaryOperator::Add, _, AstExpression::Literal(AstLiteral::UInt(0)))
        | (AstBinaryOperator::Sub, _, AstExpression::Literal(AstLiteral::Int(0)))
        | (AstBinaryOperator::Sub, _, AstExpression::Literal(AstLiteral::UInt(0)))
        | (AstBinaryOperator::Mul, _, AstExpression::Literal(AstLiteral::Int(1)))
        | (AstBinaryOperator::Mul, _, AstExpression::Literal(AstLiteral::UInt(1)))
        | (AstBinaryOperator::Div, _, AstExpression::Literal(AstLiteral::Int(1)))
        | (AstBinaryOperator::Div, _, AstExpression::Literal(AstLiteral::UInt(1)))
        | (AstBinaryOperator::LogicAnd, _, AstExpression::Literal(AstLiteral::Bool(true)))
        | (AstBinaryOperator::LogicOr, _, AstExpression::Literal(AstLiteral::Bool(false))) => {
            return Some(rewrap_child(source, left));
        }
        (AstBinaryOperator::Add, AstExpression::Literal(AstLiteral::Int(0)), _)
        | (AstBinaryOperator::Add, AstExpression::Literal(AstLiteral::UInt(0)), _)
        | (AstBinaryOperator::Mul, AstExpression::Literal(AstLiteral::Int(1)), _)
        | (AstBinaryOperator::Mul, AstExpression::Literal(AstLiteral::UInt(1)), _)
        | (AstBinaryOperator::LogicAnd, AstExpression::Literal(AstLiteral::Bool(true)), _)
        | (AstBinaryOperator::LogicOr, AstExpression::Literal(AstLiteral::Bool(false)), _) => {
            return Some(rewrap_child(source, right));
        }
        _ => {}
    }

    // Absorbing element rules: x op absorber = absorber (purity guard on discarded operand)
    match (operator, &left.item, &right.item) {
        // x * 0 = 0 (if x is pure)
        (AstBinaryOperator::Mul, _, AstExpression::Literal(AstLiteral::Int(0)))
        | (AstBinaryOperator::Mul, _, AstExpression::Literal(AstLiteral::UInt(0)))
        | (AstBinaryOperator::BitAnd, _, AstExpression::Literal(AstLiteral::Int(0)))
        | (AstBinaryOperator::BitAnd, _, AstExpression::Literal(AstLiteral::UInt(0))) => {
            if is_pure_expression(&left.item) {
                return Some(rewrap_child(source, right));
            }
        }
        // 0 * x = 0 (if x is pure)
        (AstBinaryOperator::Mul, AstExpression::Literal(AstLiteral::Int(0)), _)
        | (AstBinaryOperator::Mul, AstExpression::Literal(AstLiteral::UInt(0)), _)
        | (AstBinaryOperator::BitAnd, AstExpression::Literal(AstLiteral::Int(0)), _)
        | (AstBinaryOperator::BitAnd, AstExpression::Literal(AstLiteral::UInt(0)), _) => {
            if is_pure_expression(&right.item) {
                return Some(rewrap_child(source, left));
            }
        }
        // false && x = false (if x is pure)
        (AstBinaryOperator::LogicAnd, AstExpression::Literal(AstLiteral::Bool(false)), _) => {
            if is_pure_expression(&right.item) {
                return Some(rewrap_child(source, left));
            }
        }
        // true || x = true (if x is pure)
        (AstBinaryOperator::LogicOr, AstExpression::Literal(AstLiteral::Bool(true)), _) => {
            if is_pure_expression(&right.item) {
                return Some(rewrap_child(source, left));
            }
        }
        _ => {}
    }

    // Same-operand simplifications (both operands must be pure to avoid folding f()-f())
    if expr_structurally_equal(&left.item, &right.item)
        && is_pure_expression(&left.item)
        && is_pure_expression(&right.item)
    {
        match operator {
            // x & x = x, x | x = x
            AstBinaryOperator::BitAnd | AstBinaryOperator::BitOr => {
                return Some(rewrap_child(source, left));
            }
            // x ^ x = 0, x - x = 0
            AstBinaryOperator::BitXor | AstBinaryOperator::Sub => {
                return Some(wrap_with_source(
                    source,
                    AstExpression::Literal(AstLiteral::Int(0)),
                ));
            }
            // x == x, x <= x, x >= x -> true
            AstBinaryOperator::Equal
            | AstBinaryOperator::LessEqual
            | AstBinaryOperator::GreaterEqual => {
                return Some(wrap_with_source(
                    source,
                    AstExpression::Literal(AstLiteral::Bool(true)),
                ));
            }
            // x != x, x < x, x > x -> false
            AstBinaryOperator::NotEqual | AstBinaryOperator::Less | AstBinaryOperator::Greater => {
                return Some(wrap_with_source(
                    source,
                    AstExpression::Literal(AstLiteral::Bool(false)),
                ));
            }
            _ => {}
        }
    }

    None
}

/// Commutative constant reassociation:
///   (x op c1) op c2  ->  x op (c1 op c2)
///   c2 op (c1 op x)  ->  (c2 op c1) op x
/// Only for commutative+associative ops: Add, Mul, BitAnd, BitOr, BitXor.
fn fold_reassociate(
    source: &Wrapped<AstExpression>,
    operator: &AstBinaryOperator,
    left: &Wrapped<AstExpression>,
    right: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    let is_reassociable = matches!(
        operator,
        AstBinaryOperator::Add
            | AstBinaryOperator::Mul
            | AstBinaryOperator::BitAnd
            | AstBinaryOperator::BitOr
            | AstBinaryOperator::BitXor
    );
    if !is_reassociable {
        return None;
    }

    // Form: (non_lit op c1) op c2
    if let AstExpression::Literal(c2) = &right.item {
        if let AstExpression::BinaryOp(inner_op, inner_left, inner_right) = &left.item {
            if std::mem::discriminant(operator) == std::mem::discriminant(inner_op) {
                if let AstExpression::Literal(c1) = &inner_right.item {
                    if let Some(folded) = eval_binary(operator, c1, c2) {
                        return Some(wrap_with_source(
                            source,
                            AstExpression::BinaryOp(
                                operator.clone(),
                                inner_left.clone(),
                                Box::new(wrap_with_source(source, AstExpression::Literal(folded))),
                            ),
                        ));
                    }
                }
            }
        }
    }

    // Mirrored form: c2 op (c1 op non_lit)
    if let AstExpression::Literal(c2) = &left.item {
        if let AstExpression::BinaryOp(inner_op, inner_left, inner_right) = &right.item {
            if std::mem::discriminant(operator) == std::mem::discriminant(inner_op) {
                if let AstExpression::Literal(c1) = &inner_left.item {
                    if let Some(folded) = eval_binary(operator, c2, c1) {
                        return Some(wrap_with_source(
                            source,
                            AstExpression::BinaryOp(
                                operator.clone(),
                                Box::new(wrap_with_source(source, AstExpression::Literal(folded))),
                                inner_right.clone(),
                            ),
                        ));
                    }
                }
            }
        }
    }

    None
}

fn eval_unary(operator: &AstUnaryOperator, value: &AstLiteral) -> Option<AstLiteral> {
    match (operator, value) {
        (AstUnaryOperator::Negate, AstLiteral::Int(v)) => v.checked_neg().map(AstLiteral::Int),
        (AstUnaryOperator::Not, AstLiteral::Bool(v)) => Some(AstLiteral::Bool(!v)),
        (AstUnaryOperator::BitNot, AstLiteral::Int(v)) => Some(AstLiteral::Int(!v)),
        (AstUnaryOperator::BitNot, AstLiteral::UInt(v)) => Some(AstLiteral::UInt(!v)),
        (AstUnaryOperator::CastSigned, AstLiteral::Int(v)) => Some(AstLiteral::Int(*v)),
        (AstUnaryOperator::CastSigned, AstLiteral::UInt(v)) => {
            i64::try_from(*v).ok().map(AstLiteral::Int)
        }
        (AstUnaryOperator::CastSigned, AstLiteral::Bool(v)) => {
            Some(AstLiteral::Int(if *v { 1 } else { 0 }))
        }
        (AstUnaryOperator::CastUnsigned, AstLiteral::UInt(v)) => Some(AstLiteral::UInt(*v)),
        (AstUnaryOperator::CastUnsigned, AstLiteral::Int(v)) => {
            u64::try_from(*v).ok().map(AstLiteral::UInt)
        }
        (AstUnaryOperator::CastUnsigned, AstLiteral::Bool(v)) => {
            Some(AstLiteral::UInt(if *v { 1 } else { 0 }))
        }
        _ => None,
    }
}

fn eval_binary(
    operator: &AstBinaryOperator,
    left: &AstLiteral,
    right: &AstLiteral,
) -> Option<AstLiteral> {
    match (left, right) {
        (AstLiteral::Int(a), AstLiteral::Int(b)) => eval_binary_i64(operator, *a, *b),
        (AstLiteral::UInt(a), AstLiteral::UInt(b)) => eval_binary_u64(operator, *a, *b),
        (AstLiteral::Float(a), AstLiteral::Float(b)) => eval_binary_f64(operator, *a, *b),
        (AstLiteral::Bool(a), AstLiteral::Bool(b)) => eval_binary_bool(operator, *a, *b),
        (AstLiteral::Char(a), AstLiteral::Char(b)) => eval_binary_char(operator, *a, *b),
        (AstLiteral::String(a), AstLiteral::String(b)) => eval_binary_str(operator, a, b),
        _ => None,
    }
}

fn eval_binary_i64(operator: &AstBinaryOperator, a: i64, b: i64) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Add => a.checked_add(b).map(AstLiteral::Int),
        AstBinaryOperator::Sub => a.checked_sub(b).map(AstLiteral::Int),
        AstBinaryOperator::Mul => a.checked_mul(b).map(AstLiteral::Int),
        AstBinaryOperator::Div => {
            if b == 0 {
                None
            } else {
                a.checked_div(b).map(AstLiteral::Int)
            }
        }
        AstBinaryOperator::Mod => {
            if b == 0 {
                None
            } else {
                a.checked_rem(b).map(AstLiteral::Int)
            }
        }
        AstBinaryOperator::BitAnd => Some(AstLiteral::Int(a & b)),
        AstBinaryOperator::BitOr => Some(AstLiteral::Int(a | b)),
        AstBinaryOperator::BitXor => Some(AstLiteral::Int(a ^ b)),
        AstBinaryOperator::LeftShift => {
            if b < 0 || b >= 64 {
                None
            } else {
                Some(AstLiteral::Int(a.wrapping_shl(b as u32)))
            }
        }
        AstBinaryOperator::RightShift => {
            if b < 0 || b >= 64 {
                None
            } else {
                Some(AstLiteral::Int(a.wrapping_shr(b as u32)))
            }
        }
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool(a < b)),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool(a <= b)),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool(a > b)),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool(a >= b)),
        AstBinaryOperator::LogicAnd | AstBinaryOperator::LogicOr => None,
    }
}

fn eval_binary_u64(operator: &AstBinaryOperator, a: u64, b: u64) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Add => a.checked_add(b).map(AstLiteral::UInt),
        AstBinaryOperator::Sub => a.checked_sub(b).map(AstLiteral::UInt),
        AstBinaryOperator::Mul => a.checked_mul(b).map(AstLiteral::UInt),
        AstBinaryOperator::Div => {
            if b == 0 {
                None
            } else {
                a.checked_div(b).map(AstLiteral::UInt)
            }
        }
        AstBinaryOperator::Mod => {
            if b == 0 {
                None
            } else {
                a.checked_rem(b).map(AstLiteral::UInt)
            }
        }
        AstBinaryOperator::BitAnd => Some(AstLiteral::UInt(a & b)),
        AstBinaryOperator::BitOr => Some(AstLiteral::UInt(a | b)),
        AstBinaryOperator::BitXor => Some(AstLiteral::UInt(a ^ b)),
        AstBinaryOperator::LeftShift => {
            if b >= 64 {
                None
            } else {
                Some(AstLiteral::UInt(a.wrapping_shl(b as u32)))
            }
        }
        AstBinaryOperator::RightShift => {
            if b >= 64 {
                None
            } else {
                Some(AstLiteral::UInt(a.wrapping_shr(b as u32)))
            }
        }
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool(a < b)),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool(a <= b)),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool(a > b)),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool(a >= b)),
        AstBinaryOperator::LogicAnd | AstBinaryOperator::LogicOr => None,
    }
}

fn eval_binary_f64(operator: &AstBinaryOperator, a: f64, b: f64) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Add => Some(AstLiteral::Float(a + b)),
        AstBinaryOperator::Sub => Some(AstLiteral::Float(a - b)),
        AstBinaryOperator::Mul => Some(AstLiteral::Float(a * b)),
        AstBinaryOperator::Div => Some(AstLiteral::Float(a / b)),
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool(a < b)),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool(a <= b)),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool(a > b)),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool(a >= b)),
        AstBinaryOperator::Mod
        | AstBinaryOperator::BitAnd
        | AstBinaryOperator::BitOr
        | AstBinaryOperator::BitXor
        | AstBinaryOperator::LogicAnd
        | AstBinaryOperator::LogicOr
        | AstBinaryOperator::LeftShift
        | AstBinaryOperator::RightShift => None,
    }
}

fn eval_binary_bool(operator: &AstBinaryOperator, a: bool, b: bool) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::LogicAnd => Some(AstLiteral::Bool(a && b)),
        AstBinaryOperator::LogicOr => Some(AstLiteral::Bool(a || b)),
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool((a as u8) < (b as u8))),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool((a as u8) <= (b as u8))),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool((a as u8) > (b as u8))),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool((a as u8) >= (b as u8))),
        AstBinaryOperator::Add
        | AstBinaryOperator::Sub
        | AstBinaryOperator::Mul
        | AstBinaryOperator::Div
        | AstBinaryOperator::Mod
        | AstBinaryOperator::BitAnd
        | AstBinaryOperator::BitOr
        | AstBinaryOperator::BitXor
        | AstBinaryOperator::LeftShift
        | AstBinaryOperator::RightShift => None,
    }
}

fn eval_binary_char(operator: &AstBinaryOperator, a: char, b: char) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool(a < b)),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool(a <= b)),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool(a > b)),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool(a >= b)),
        AstBinaryOperator::Add
        | AstBinaryOperator::Sub
        | AstBinaryOperator::Mul
        | AstBinaryOperator::Div
        | AstBinaryOperator::Mod
        | AstBinaryOperator::BitAnd
        | AstBinaryOperator::BitOr
        | AstBinaryOperator::BitXor
        | AstBinaryOperator::LogicAnd
        | AstBinaryOperator::LogicOr
        | AstBinaryOperator::LeftShift
        | AstBinaryOperator::RightShift => None,
    }
}

fn eval_binary_str(operator: &AstBinaryOperator, a: &str, b: &str) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Add => Some(AstLiteral::String(format!("{}{}", a, b))),
        AstBinaryOperator::Sub
        | AstBinaryOperator::Mul
        | AstBinaryOperator::Div
        | AstBinaryOperator::Mod
        | AstBinaryOperator::BitAnd
        | AstBinaryOperator::BitOr
        | AstBinaryOperator::BitXor
        | AstBinaryOperator::LogicAnd
        | AstBinaryOperator::LogicOr
        | AstBinaryOperator::Less
        | AstBinaryOperator::LessEqual
        | AstBinaryOperator::Greater
        | AstBinaryOperator::GreaterEqual
        | AstBinaryOperator::LeftShift
        | AstBinaryOperator::RightShift => None,
    }
}

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

fn rewrap_child(
    source: &Wrapped<AstExpression>,
    child: &Wrapped<AstExpression>,
) -> Wrapped<AstExpression> {
    wrap_with_source(source, child.item.clone())
}
