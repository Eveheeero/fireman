//! Bit manipulation pattern recognition (rotation, strength reduction, identity ops).

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

    // After recursing, try to recognize bit tricks at this node
    if let Some(replacement) = try_recognize_rotation(expr) {
        *expr = replacement;
    }

    // Strength-reduction reversal: (x << N) + x → x * (2^N + 1), etc.
    if let Some(replacement) = try_reverse_strength_reduction(expr) {
        *expr = replacement;
    }

    // Note: identity simplification (x+0→x, x*1→x, etc.) is now handled by
    // the identity-simplification.fb pattern, applied before this pass.

    // Annotate bitfield access patterns: (x >> N) & M
    if expr.comment.is_none() {
        if let Some(comment) = try_recognize_bitfield_access(&expr.item) {
            expr.comment = Some(comment);
        }
    }

    // Annotate alignment mask patterns: x & ~(N-1)
    if expr.comment.is_none() {
        if let Some(comment) = try_recognize_alignment_mask(&expr.item) {
            expr.comment = Some(comment);
        }
    }

    // Annotate byte-swap patterns: bswap16/bswap32
    if expr.comment.is_none() {
        if let Some(comment) = try_recognize_bswap32(&expr.item) {
            expr.comment = Some(comment);
        }
    }
}

/// Reverse strength reduction: convert shift+add/sub back to multiplication.
///
/// Patterns:
///   `(x << N) + x`  → `x * (2^N + 1)`
///   `(x << N) - x`  → `x * (2^N - 1)`
///   `(x << N) + (x << M)` → `x * (2^N + 2^M)` (N > M)
fn try_reverse_strength_reduction(expr: &Wrapped<AstExpression>) -> Option<Wrapped<AstExpression>> {
    use super::opt_utils::expr_structurally_equal;

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
            return Some(build_mul(expr, &shifted_x, multiplier));
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
                return Some(build_mul(expr, &shifted_x, multiplier));
            }
        }
    }

    // Pattern 2: (x << N) + (x << M) → x * (2^N + 2^M)
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

fn build_mul(
    source: &Wrapped<AstExpression>,
    x: &Wrapped<AstExpression>,
    multiplier: u64,
) -> Wrapped<AstExpression> {
    use crate::abstract_syntax_tree::{AstValueOrigin, Wrapped};

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

/// Recognize bitfield access patterns and return an annotation comment.
///
/// Patterns detected:
///   `(x >> shift) & mask`  → "bits[shift..shift+width]" where width = popcount(mask)
///   `(x & mask) >> shift`  → same
///   `x & mask`             → "bits[0..width]" when mask is a contiguous bit run
fn try_recognize_bitfield_access(expr: &AstExpression) -> Option<String> {
    // Pattern 1: (x >> shift) & mask
    if let AstExpression::BinaryOp(AstBinaryOperator::BitAnd, lhs, rhs) = expr {
        // Either order: (x >> N) & M or M & (x >> N)
        if let Some(comment) = try_shift_then_mask(lhs, rhs) {
            return Some(comment);
        }
        if let Some(comment) = try_shift_then_mask(rhs, lhs) {
            return Some(comment);
        }
        // Pattern: x & mask where mask is contiguous bits from bit 0
        if let Some(mask) = extract_literal_u64(&rhs.item) {
            if let Some(width) = contiguous_low_bits(mask) {
                if width > 0 && width < 64 {
                    return Some(format!("bits[0..{width}]"));
                }
            }
        }
        if let Some(mask) = extract_literal_u64(&lhs.item) {
            if let Some(width) = contiguous_low_bits(mask) {
                if width > 0 && width < 64 {
                    return Some(format!("bits[0..{width}]"));
                }
            }
        }
    }

    // Pattern 2: (x & mask) >> shift (mask then shift)
    if let AstExpression::BinaryOp(AstBinaryOperator::RightShift, lhs, rhs) = expr {
        if let Some(shift) = extract_literal_u64(&rhs.item) {
            if let AstExpression::BinaryOp(AstBinaryOperator::BitAnd, inner_l, inner_r) = &lhs.item
            {
                if let Some(mask) = extract_literal_u64(&inner_r.item) {
                    let width = mask.count_ones() as u64;
                    if width > 0 && shift < 64 {
                        return Some(format!("bits[{shift}..{}]", shift + width));
                    }
                }
                if let Some(mask) = extract_literal_u64(&inner_l.item) {
                    let width = mask.count_ones() as u64;
                    if width > 0 && shift < 64 {
                        return Some(format!("bits[{shift}..{}]", shift + width));
                    }
                }
            }
        }
    }

    None
}

fn try_shift_then_mask(
    shifted: &Wrapped<AstExpression>,
    mask_expr: &Wrapped<AstExpression>,
) -> Option<String> {
    let mask = extract_literal_u64(&mask_expr.item)?;
    let AstExpression::BinaryOp(AstBinaryOperator::RightShift, _, shift_amt) = &shifted.item else {
        return None;
    };
    let shift = extract_literal_u64(&shift_amt.item)?;
    let width = mask.count_ones() as u64;
    if width > 0 && shift < 64 && shift + width <= 64 {
        Some(format!("bits[{shift}..{}]", shift + width))
    } else {
        None
    }
}

/// Returns Some(width) if the value is a contiguous run of `width` bits from bit 0.
/// e.g., 0xFF → Some(8), 0xFFF → Some(12), 0x3 → Some(2)
fn contiguous_low_bits(mask: u64) -> Option<u32> {
    if mask == 0 {
        return None;
    }
    let width = mask.trailing_ones();
    // Check that there are no other bits set above the contiguous run
    if mask == (1u64.wrapping_shl(width)) - 1 || (width == 64 && mask == u64::MAX) {
        Some(width)
    } else {
        None
    }
}

/// Detect the remaining 32-bit byte-swap OR-tree heuristic.
fn try_recognize_bswap32(expr: &AstExpression) -> Option<String> {
    let shifts = collect_or_shifts(expr);
    if shifts.len() >= 4 {
        let has_24_right = shifts.iter().any(|&(amt, right)| amt == 24 && right);
        let has_24_left = shifts.iter().any(|&(amt, right)| amt == 24 && !right);
        if has_24_right && has_24_left {
            return Some("bswap32 / ntohl".to_string());
        }
    }

    None
}

/// Collect (shift_amount, is_right_shift) from an OR-tree of shifts.
fn collect_or_shifts(expr: &AstExpression) -> Vec<(u64, bool)> {
    let mut result = Vec::new();
    collect_or_shifts_inner(expr, &mut result);
    result
}

fn collect_or_shifts_inner(expr: &AstExpression, out: &mut Vec<(u64, bool)>) {
    match expr {
        AstExpression::BinaryOp(op, left, right) if matches!(op, AstBinaryOperator::BitOr) => {
            collect_or_shifts_inner(&left.item, out);
            collect_or_shifts_inner(&right.item, out);
        }
        _ => {
            // Try to extract shift info, possibly through a mask
            if let Some((amt, is_right)) = extract_shift_info(expr) {
                out.push((amt, is_right));
            }
        }
    }
}

fn extract_shift_info(expr: &AstExpression) -> Option<(u64, bool)> {
    // Direct shift: x >> N or x << N
    if let AstExpression::BinaryOp(op, _base, amt) = expr {
        if matches!(op, AstBinaryOperator::RightShift) {
            return extract_literal_u64(&amt.item).map(|a| (a, true));
        }
        if matches!(op, AstBinaryOperator::LeftShift) {
            return extract_literal_u64(&amt.item).map(|a| (a, false));
        }
        // Masked shift: (x >> N) & M or (x << N) & M
        if matches!(op, AstBinaryOperator::BitAnd) {
            return extract_shift_info(&_base.item);
        }
    }
    None
}

/// Detect alignment masking patterns: `x & ~(N-1)` where N is a power of two.
/// These round down to an alignment boundary.
/// Also detects `(x + (N-1)) & ~(N-1)` for round-up alignment.
fn try_recognize_alignment_mask(expr: &AstExpression) -> Option<String> {
    let AstExpression::BinaryOp(op, left, right) = expr else {
        return None;
    };
    if !matches!(op, AstBinaryOperator::BitAnd) {
        return None;
    }

    // Check if either side is a mask that's the bitwise NOT of (power_of_two - 1)
    // i.e., 0xFFFFFFF0 for align-16, 0xFFFFFFFC for align-4, etc.
    let mask_val = extract_literal_u64(&right.item).or_else(|| extract_literal_u64(&left.item))?;

    // ~(N-1) for power of two N means the mask has all high bits set and low bits clear
    // The complement +1 should be a power of two
    let complement = (!mask_val).wrapping_add(1);
    if complement == 0 || (complement & (complement - 1)) != 0 {
        return None; // not a power of two
    }
    // Filter out trivially small or suspiciously large alignments
    if complement < 2 || complement > 0x1000 {
        return None;
    }

    // Check if the non-mask operand is `x + (N-1)`, indicating round-up alignment
    let non_mask = if extract_literal_u64(&right.item).is_some() {
        &left.item
    } else {
        &right.item
    };
    if let AstExpression::BinaryOp(add_op, _, add_rhs) = non_mask {
        if matches!(add_op, AstBinaryOperator::Add) {
            if extract_literal_u64(&add_rhs.item) == Some(complement - 1) {
                return Some(format!("align up to {complement}"));
            }
        }
    }

    Some(format!("align down to {complement}"))
}
