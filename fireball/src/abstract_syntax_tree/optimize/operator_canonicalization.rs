use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId,
        AstFunctionVersion, AstStatement, AstUnaryOperator, ProcessedOptimization, Wrapped,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn canonicalize_operators(
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

    canonicalize_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::OperatorCanonicalization);
    }

    Ok(())
}

fn canonicalize_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        canonicalize_statement(stmt);
    }
}

fn canonicalize_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::Declaration(_lhs, rhs) => {
            if let Some(rhs) = rhs {
                canonicalize_expression(rhs);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            canonicalize_expression(lhs);
            canonicalize_expression(rhs);
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            canonicalize_expression(cond);
            canonicalize_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                canonicalize_statement_list(branch_false);
            }
        }
        AstStatement::While(cond, body) => {
            canonicalize_expression(cond);
            canonicalize_statement_list(body);
        }
        AstStatement::For(init, cond, update, body) => {
            canonicalize_statement(init);
            canonicalize_expression(cond);
            canonicalize_statement(update);
            canonicalize_statement_list(body);
        }
        AstStatement::Switch(discrim, cases, default) => {
            canonicalize_expression(discrim);
            for (_lit, case_body) in cases.iter_mut() {
                canonicalize_statement_list(case_body);
            }
            if let Some(default_body) = default {
                canonicalize_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => {
            canonicalize_statement_list(body);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                canonicalize_expression(expr);
            }
        }
        AstStatement::Call(call) => {
            canonicalize_call(call);
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

fn canonicalize_call(call: &mut AstCall) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                canonicalize_expression(arg);
            }
        }
        AstCall::Builtin(_, args) => match args.as_mut() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(items) => {
                for item in items.iter_mut() {
                    canonicalize_expression(item);
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
                canonicalize_expression(expr);
            }
            AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
                canonicalize_expression(expr1);
                canonicalize_expression(expr2);
            }
        },
    }
}

fn canonicalize_expression(expr: &mut Wrapped<AstExpression>) {
    // Recurse into children first (bottom-up canonicalization).
    match &mut expr.item {
        AstExpression::UnaryOp(_, arg) => {
            canonicalize_expression(arg);
        }
        AstExpression::BinaryOp(_, left, right) => {
            canonicalize_expression(left);
            canonicalize_expression(right);
        }
        AstExpression::Call(call) => {
            canonicalize_call(call);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            canonicalize_expression(arg);
        }
        AstExpression::ArrayAccess(base, idx) => {
            canonicalize_expression(base);
            canonicalize_expression(idx);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            canonicalize_expression(cond);
            canonicalize_expression(true_expr);
            canonicalize_expression(false_expr);
        }
        AstExpression::Literal(_)
        | AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => {}
    }

    // Apply canonicalization rules to the current node.
    canonicalize_current(expr);
}

fn canonicalize_current(expr: &mut Wrapped<AstExpression>) {
    // Rule 3: Double negation elimination -- Not(Not(x)) -> x
    if let AstExpression::UnaryOp(AstUnaryOperator::Not, inner) = &expr.item {
        if let AstExpression::UnaryOp(AstUnaryOperator::Not, innermost) = &inner.item {
            expr.item = innermost.item.clone();
            return;
        }
    }

    // Rule 4: Comparison negation simplification -- Not(cmp(a,b)) -> inverse_cmp(a,b)
    if let AstExpression::UnaryOp(AstUnaryOperator::Not, inner) = &expr.item {
        if let AstExpression::BinaryOp(op, left, right) = &inner.item {
            let inverted = invert_comparison(op);
            if let Some(new_op) = inverted {
                expr.item = AstExpression::BinaryOp(new_op, left.clone(), right.clone());
                return;
            }
        }
    }

    // Rules 1 & 2: Commutative literal normalization and comparison flipping.
    if let AstExpression::BinaryOp(op, left, right) = &expr.item {
        if is_literal(&left.item) && !is_literal(&right.item) {
            if is_commutative(op) {
                // Rule 1: Swap operands for commutative ops so literal is on the right.
                expr.item = AstExpression::BinaryOp(op.clone(), right.clone(), left.clone());
            } else if let Some(flipped_op) = flip_comparison(op) {
                // Rule 2: Flip comparison direction when literal is on the left.
                expr.item = AstExpression::BinaryOp(flipped_op, right.clone(), left.clone());
            }
        }
    }
}

/// Returns true if the expression is a literal value.
fn is_literal(expr: &AstExpression) -> bool {
    matches!(expr, AstExpression::Literal(_))
}

/// Returns true if the binary operator is commutative (operands can be swapped
/// without changing semantics).
fn is_commutative(op: &AstBinaryOperator) -> bool {
    matches!(
        op,
        AstBinaryOperator::Add
            | AstBinaryOperator::Mul
            | AstBinaryOperator::BitAnd
            | AstBinaryOperator::BitOr
            | AstBinaryOperator::BitXor
            | AstBinaryOperator::Equal
            | AstBinaryOperator::NotEqual
    )
}

/// For non-commutative comparison operators, returns the operator with flipped
/// direction: `a < b` becomes `b > a`, etc.
fn flip_comparison(op: &AstBinaryOperator) -> Option<AstBinaryOperator> {
    match op {
        AstBinaryOperator::Less => Some(AstBinaryOperator::Greater),
        AstBinaryOperator::LessEqual => Some(AstBinaryOperator::GreaterEqual),
        AstBinaryOperator::Greater => Some(AstBinaryOperator::Less),
        AstBinaryOperator::GreaterEqual => Some(AstBinaryOperator::LessEqual),
        _ => None,
    }
}

/// Returns the logical inverse of a comparison operator for
/// `Not(cmp(a, b))` -> `inv_cmp(a, b)` simplification.
fn invert_comparison(op: &AstBinaryOperator) -> Option<AstBinaryOperator> {
    match op {
        AstBinaryOperator::Equal => Some(AstBinaryOperator::NotEqual),
        AstBinaryOperator::NotEqual => Some(AstBinaryOperator::Equal),
        AstBinaryOperator::Less => Some(AstBinaryOperator::GreaterEqual),
        AstBinaryOperator::LessEqual => Some(AstBinaryOperator::Greater),
        AstBinaryOperator::Greater => Some(AstBinaryOperator::LessEqual),
        AstBinaryOperator::GreaterEqual => Some(AstBinaryOperator::Less),
        _ => None,
    }
}
