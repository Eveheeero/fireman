//! Rotation recovery and strength reduction reversal extracted from
//! bit_trick_recognition.rs.
//!
//! These transformations rewrite low-level shift/or and shift/add patterns back
//! into higher-level operations:
//!   - `(x >> n) | (x << (W-n))` -> `__builtin_rotate_right(x, n)`
//!   - `(x << n) | (x >> (W-n))` -> `__builtin_rotate_left(x, n)`
//!   - `(x << N) + x`            -> `x * (2^N + 1)`
//!   - `(x << N) - x`            -> `x * (2^N - 1)`
//!   - `(x << N) + (x << M)`     -> `x * (2^N + 2^M)`

use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstLiteral, AstStatement, AstValueOrigin, ProcessedOptimization, Wrapped,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

/// Walk the AST for the given function and apply rotation recovery and
/// strength-reduction reversal to every expression.
pub(crate) fn recognize_rotation_and_strength_reduction(
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

    recognize_in_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::BitTrickRecognition);
    }

    Ok(())
}

fn recognize_in_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        recognize_in_statement(stmt);
    }
}

fn recognize_in_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::Declaration(_lhs, rhs) => {
            if let Some(rhs) = rhs {
                recognize_in_expression(rhs);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            recognize_in_expression(lhs);
            recognize_in_expression(rhs);
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            recognize_in_expression(cond);
            recognize_in_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                recognize_in_statement_list(branch_false);
            }
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            recognize_in_expression(cond);
            recognize_in_statement_list(body);
        }
        AstStatement::For(init, cond, update, body) => {
            recognize_in_statement(init);
            recognize_in_expression(cond);
            recognize_in_statement(update);
            recognize_in_statement_list(body);
        }
        AstStatement::Switch(discrim, cases, default) => {
            recognize_in_expression(discrim);
            for (_lit, case_body) in cases.iter_mut() {
                recognize_in_statement_list(case_body);
            }
            if let Some(default_body) = default {
                recognize_in_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => {
            recognize_in_statement_list(body);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                recognize_in_expression(expr);
            }
        }
        AstStatement::Call(call) => {
            recognize_in_call(call);
        }
        AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Label(_)
        | AstStatement::Comment(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => {}
    }
}

fn recognize_in_call(call: &mut AstCall) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                recognize_in_expression(arg);
            }
        }
        AstCall::Builtin(_, _) => {}
    }
}

fn recognize_in_expression(expr: &mut Wrapped<AstExpression>) {
    // Recurse into sub-expressions first (bottom-up)
    match &mut expr.item {
        AstExpression::UnaryOp(_, arg) => {
            recognize_in_expression(arg);
        }
        AstExpression::BinaryOp(_, left, right) => {
            recognize_in_expression(left);
            recognize_in_expression(right);
        }
        AstExpression::Call(call) => {
            recognize_in_call(call);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            recognize_in_expression(arg);
        }
        AstExpression::ArrayAccess(base, idx) => {
            recognize_in_expression(base);
            recognize_in_expression(idx);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            recognize_in_expression(cond);
            recognize_in_expression(true_expr);
            recognize_in_expression(false_expr);
        }
        AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_) => {}
    }

    // After recursing, try to recognize rotation at this node
    if let Some(replacement) = try_recognize_rotation(expr) {
        *expr = replacement;
    }

    // Strength-reduction reversal: (x << N) + x -> x * (2^N + 1), etc.
    if let Some(replacement) = try_reverse_strength_reduction(expr) {
        *expr = replacement;
    }
}

/// Reverse strength reduction: convert shift+add/sub back to multiplication.
///
/// Patterns:
///   `(x << N) + x`  -> `x * (2^N + 1)`
///   `(x << N) - x`  -> `x * (2^N - 1)`
///   `(x << N) + (x << M)` -> `x * (2^N + 2^M)` (N > M)
pub(crate) fn try_reverse_strength_reduction(
    expr: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    use crate::abstract_syntax_tree::optimize::opt_utils::expr_structurally_equal;

    let (op, left, right) = match &expr.item {
        AstExpression::BinaryOp(op @ (AstBinaryOperator::Add | AstBinaryOperator::Sub), l, r) => {
            (op, l, r)
        }
        _ => return None,
    };

    // Pattern 1: (x << N) +/- x
    if let AstExpression::BinaryOp(AstBinaryOperator::LeftShift, shifted_x, shift_amt) = &left.item
    {
        let n = extract_literal_u64(&shift_amt.item)?;
        if n == 0 || n >= 64 {
            return None;
        }
        if expr_structurally_equal(&shifted_x.item, &right.item) {
            let multiplier = match op {
                AstBinaryOperator::Add => (1u64 << n).checked_add(1)?,
                AstBinaryOperator::Sub => (1u64 << n).checked_sub(1)?,
                _ => return None,
            };
            if multiplier <= 1 {
                return None;
            }
            return Some(build_mul(expr, shifted_x, multiplier));
        }
    }

    // Pattern 1b: x + (x << N)  (commutative add only)
    if matches!(op, AstBinaryOperator::Add) {
        if let AstExpression::BinaryOp(AstBinaryOperator::LeftShift, shifted_x, shift_amt) =
            &right.item
        {
            let n = extract_literal_u64(&shift_amt.item)?;
            if n > 0 && n < 64 && expr_structurally_equal(&shifted_x.item, &left.item) {
                let multiplier = (1u64 << n).checked_add(1)?;
                return Some(build_mul(expr, shifted_x, multiplier));
            }
        }
    }

    // Pattern 2: (x << N) + (x << M) -> x * (2^N + 2^M)
    if matches!(op, AstBinaryOperator::Add) {
        if let (
            AstExpression::BinaryOp(AstBinaryOperator::LeftShift, x1, n_expr),
            AstExpression::BinaryOp(AstBinaryOperator::LeftShift, x2, m_expr),
        ) = (&left.item, &right.item)
        {
            if expr_structurally_equal(&x1.item, &x2.item) {
                let n = extract_literal_u64(&n_expr.item)?;
                let m = extract_literal_u64(&m_expr.item)?;
                if n < 64 && m < 64 && n != m {
                    let multiplier = (1u64 << n).checked_add(1u64 << m)?;
                    return Some(build_mul(expr, x1, multiplier));
                }
            }
        }
    }

    None
}

pub(crate) fn build_mul(
    source: &Wrapped<AstExpression>,
    x: &Wrapped<AstExpression>,
    multiplier: u64,
) -> Wrapped<AstExpression> {
    let x_clone = Box::new(Wrapped {
        item: x.item.clone(),
        origin: x.origin.clone(),
        comment: None,
    });
    let mul_lit = Box::new(Wrapped {
        item: AstExpression::Literal(to_literal_u64(multiplier)),
        origin: AstValueOrigin::Unknown,
        comment: None,
    });
    Wrapped {
        item: AstExpression::BinaryOp(AstBinaryOperator::Mul, x_clone, mul_lit),
        origin: source.origin.clone(),
        comment: source.comment.clone(),
    }
}

/// Try to recognize rotate-right or rotate-left patterns.
///
/// Rotate right: `(x >> n) | (x << (W - n))` where W is 32 or 64
/// Rotate left:  `(x << n) | (x >> (W - n))` where W is 32 or 64
///
/// Both operand orderings of the outer BitOr are handled.
pub(crate) fn try_recognize_rotation(
    expr: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    let AstExpression::BinaryOp(AstBinaryOperator::BitOr, left, right) = &expr.item else {
        return None;
    };

    // Try both orderings: (left, right) and (right, left)
    try_match_rotation(expr, left, right).or_else(|| try_match_rotation(expr, right, left))
}

/// Attempt to match `(shift_a, shift_b)` as a rotation pair.
///
/// Checks for:
///   shift_a = x >> n,  shift_b = x << (W - n)  =>  rotate_right(x, n)
///   shift_a = x << n,  shift_b = x >> (W - n)  =>  rotate_left(x, n)
pub(crate) fn try_match_rotation(
    source: &Wrapped<AstExpression>,
    shift_a: &Wrapped<AstExpression>,
    shift_b: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    use crate::abstract_syntax_tree::optimize::opt_utils::expr_structurally_equal;

    let AstExpression::BinaryOp(op_a, x1, n_expr) = &shift_a.item else {
        return None;
    };
    let AstExpression::BinaryOp(op_b, x2, complement_expr) = &shift_b.item else {
        return None;
    };

    // Determine rotation direction based on shift operators
    let builtin_name = match (op_a, op_b) {
        (AstBinaryOperator::RightShift, AstBinaryOperator::LeftShift) => "__builtin_rotate_right",
        (AstBinaryOperator::LeftShift, AstBinaryOperator::RightShift) => "__builtin_rotate_left",
        _ => return None,
    };

    // Both shifts must operate on the same expression
    if !expr_structurally_equal(&x1.item, &x2.item) {
        return None;
    }

    // Extract the shift amounts as integer literals
    let n = extract_literal_u64(&n_expr.item)?;
    let complement = extract_literal_u64(&complement_expr.item)?;

    // Validate that n + complement == W where W is 32 or 64
    let width = n.checked_add(complement)?;
    if width != 32 && width != 64 {
        return None;
    }

    // Both shift amounts must be in range (0, W) exclusive to be a valid rotation
    if n == 0 || complement == 0 {
        return None;
    }

    // Build the replacement: __builtin_rotate_{right,left}(x, n)
    let x_arg = Wrapped {
        item: x1.item.clone(),
        origin: x1.origin.clone(),
        comment: None,
    };
    let n_arg = Wrapped {
        item: AstExpression::Literal(to_literal_u64(n)),
        origin: n_expr.origin.clone(),
        comment: None,
    };

    let call = AstCall::Unknown(builtin_name.into(), vec![x_arg, n_arg]);
    Some(Wrapped {
        item: AstExpression::Call(call),
        origin: source.origin.clone(),
        comment: source.comment.clone(),
    })
}

/// Extract a u64 value from a literal expression (supports both Int and UInt).
pub(crate) fn extract_literal_u64(expr: &AstExpression) -> Option<u64> {
    match expr {
        AstExpression::Literal(AstLiteral::Int(v)) => u64::try_from(*v).ok(),
        AstExpression::Literal(AstLiteral::UInt(v)) => Some(*v),
        _ => None,
    }
}

/// Convert a u64 back to the canonical literal form used for shift amounts.
pub(crate) fn to_literal_u64(v: u64) -> AstLiteral {
    if let Ok(signed) = i64::try_from(v) {
        AstLiteral::Int(signed)
    } else {
        AstLiteral::UInt(v)
    }
}
