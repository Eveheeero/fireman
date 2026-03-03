use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstLiteral, AstStatement, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn recognize_bit_tricks(
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
        AstStatement::While(cond, body) => {
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

    // After recursing, try to recognize bit tricks at this node
    if let Some(replacement) = try_recognize_rotation(expr) {
        *expr = replacement;
    }
    // TODO: Recognize absolute value pattern: (x ^ (x >> 31)) - (x >> 31) -> abs(x)
    // TODO: Recognize bswap32 pattern: byte-swap via shifts and masks -> __builtin_bswap32(x)
    // TODO: Recognize bswap16 pattern: ((x >> 8) & 0xFF) | ((x << 8) & 0xFF00) -> __builtin_bswap16(x)
}

/// Try to recognize rotate-right or rotate-left patterns.
///
/// Rotate right: `(x >> n) | (x << (W - n))` where W is 32 or 64
/// Rotate left:  `(x << n) | (x >> (W - n))` where W is 32 or 64
///
/// Both operand orderings of the outer BitOr are handled.
fn try_recognize_rotation(expr: &Wrapped<AstExpression>) -> Option<Wrapped<AstExpression>> {
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
fn try_match_rotation(
    source: &Wrapped<AstExpression>,
    shift_a: &Wrapped<AstExpression>,
    shift_b: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    use super::opt_utils::expr_structurally_equal;

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
fn extract_literal_u64(expr: &AstExpression) -> Option<u64> {
    match expr {
        AstExpression::Literal(AstLiteral::Int(v)) => u64::try_from(*v).ok(),
        AstExpression::Literal(AstLiteral::UInt(v)) => Some(*v),
        _ => None,
    }
}

/// Convert a u64 back to the canonical literal form used for shift amounts.
fn to_literal_u64(v: u64) -> AstLiteral {
    if let Ok(signed) = i64::try_from(v) {
        AstLiteral::Int(signed)
    } else {
        AstLiteral::UInt(v)
    }
}
