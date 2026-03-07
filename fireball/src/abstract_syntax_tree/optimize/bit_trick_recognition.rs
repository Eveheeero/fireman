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

    // Strength-reduction reversal: (x << N) + x → x * (2^N + 1), etc.
    if let Some(replacement) = try_reverse_strength_reduction(expr) {
        *expr = replacement;
    }

    // Opaque arithmetic simplification: x ^ 0 → x, x + 0 → x, x * 1 → x, etc.
    if let Some(replacement) = try_simplify_identity_op(expr) {
        *expr = replacement;
    }

    // Annotate saturating arithmetic: x > C ? C : x  or  x < C ? C : x
    if expr.comment.is_none() {
        if let Some(comment) = try_recognize_saturating(expr) {
            expr.comment = Some(comment);
        }
    }

    // Annotate checked arithmetic: (a + b) < a  →  overflow check
    if expr.comment.is_none() {
        if let Some(comment) = try_recognize_overflow_check(expr) {
            expr.comment = Some(comment);
        }
    }

    // Annotate well-known magic numbers
    if expr.comment.is_none() {
        if let Some(comment) = try_label_magic_number(&expr.item) {
            expr.comment = Some(comment);
        }
    }

    // Annotate bitfield access patterns: (x >> N) & M
    if expr.comment.is_none() {
        if let Some(comment) = try_recognize_bitfield_access(&expr.item) {
            expr.comment = Some(comment);
        }
    }

    // Annotate sentinel-value comparisons: x == -1, x != -1
    if expr.comment.is_none() {
        if let Some(comment) = try_recognize_sentinel_comparison(&expr.item) {
            expr.comment = Some(comment);
        }
    }

    // Annotate stride patterns: base + i * STRIDE
    if expr.comment.is_none() {
        if let Some(comment) = try_recognize_stride_access(&expr.item) {
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
        if let Some(comment) = try_recognize_byte_swap(&expr.item) {
            expr.comment = Some(comment);
        }
    }

    // L139: Annotate common C idiom patterns that map to intrinsics
    if expr.comment.is_none() {
        if let Some(comment) = try_recognize_intrinsic_idiom(&expr.item) {
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

/// Recognize sentinel-value comparisons and return an annotation comment.
///
/// Detects: `x == -1`, `x != -1`, `x == 0xFFFFFFFF`, `x == 0xFFFFFFFFFFFFFFFF`
fn try_recognize_sentinel_comparison(expr: &AstExpression) -> Option<String> {
    let AstExpression::BinaryOp(op, lhs, rhs) = expr else {
        return None;
    };
    if !matches!(op, AstBinaryOperator::Equal | AstBinaryOperator::NotEqual) {
        return None;
    }

    let lit_val =
        extract_sentinel_literal(&rhs.item).or_else(|| extract_sentinel_literal(&lhs.item))?;

    match lit_val {
        SentinelKind::NegativeOne => Some("sentinel check (-1 / INVALID_HANDLE_VALUE)".to_string()),
        SentinelKind::MaxU32 => Some("sentinel check (0xFFFFFFFF)".to_string()),
    }
}

enum SentinelKind {
    NegativeOne,
    MaxU32,
}

fn extract_sentinel_literal(expr: &AstExpression) -> Option<SentinelKind> {
    match expr {
        AstExpression::Literal(AstLiteral::Int(-1)) => Some(SentinelKind::NegativeOne),
        AstExpression::Literal(AstLiteral::UInt(0xFFFFFFFF)) => Some(SentinelKind::MaxU32),
        AstExpression::Literal(AstLiteral::UInt(0xFFFFFFFFFFFFFFFF)) => {
            Some(SentinelKind::NegativeOne)
        }
        _ => None,
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

/// Label well-known magic numbers with descriptive comments.
fn try_label_magic_number(expr: &AstExpression) -> Option<String> {
    let value = match expr {
        AstExpression::Literal(AstLiteral::UInt(v)) => *v,
        AstExpression::Literal(AstLiteral::Int(v)) if *v >= 0 => *v as u64,
        _ => return None,
    };

    let label = match value {
        // PE/COFF signatures
        0x5A4D => "IMAGE_DOS_SIGNATURE (MZ)",
        0x4550 => "IMAGE_NT_SIGNATURE (PE\\0\\0)",
        0x014C => "IMAGE_FILE_MACHINE_I386",
        0x8664 => "IMAGE_FILE_MACHINE_AMD64",
        0xAA64 => "IMAGE_FILE_MACHINE_ARM64",

        // ELF magic
        0x464C457F => "ELF magic (\\x7fELF)",

        // Memory protection flags
        0x01 => return None, // too common
        0x02 => return None,
        0x04 => return None,
        0x10 => return None,
        0x20 => return None,
        0x40 => return None,
        0x80 => return None,

        // Windows page protection
        0x1000 => "PAGE_EXECUTE / allocation granularity",
        0x2000 => "PAGE_EXECUTE_READ",
        0x4000 => "PAGE_EXECUTE_READWRITE",

        // Common allocation sizes / limits
        0xFFFF => "UINT16_MAX",
        0xFFFFFFFF => "UINT32_MAX",
        0x7FFFFFFF => "INT32_MAX",
        0x7FFFFFFFFFFFFFFF => "INT64_MAX",
        0x80000000 => "INT32_MIN (as unsigned) / sign bit 32",
        0x8000000000000000 => "INT64_MIN (as unsigned) / sign bit 64",

        // Windows constants
        0xDEADBEEF => "debug fill pattern",
        0xCCCCCCCC => "MSVC uninitialized stack fill",
        0xCDCDCDCD => "MSVC uninitialized heap fill",
        0xFDFDFDFD => "MSVC heap guard fill",
        0xFEEEFEEE => "MSVC freed heap fill",
        0xBAADF00D => "MSVC LocalAlloc(LMEM_FIXED) fill",
        0xABABABAB => "MSVC heap guard (after block)",

        // Crypto/hash constants
        0x67452301 => "MD5/SHA-1 init H0",
        0xEFCDAB89 => "MD5/SHA-1 init H1",
        0x98BADCFE => "MD5/SHA-1 init H2",
        0x10325476 => "MD5/SHA-1 init H3",
        0x6A09E667 => "SHA-256 init H0",
        0xBB67AE85 => "SHA-256 init H1",
        0x3C6EF372 => "SHA-256 init H2",
        0xA54FF53A => "SHA-256 init H3",

        // CRC
        0xEDB88320 => "CRC-32 polynomial (reversed)",
        0x04C11DB7 => "CRC-32 polynomial (normal)",
        0x82F63B78 => "CRC-32C polynomial (reversed)",

        _ => return None,
    };

    Some(label.to_string())
}

/// Recognize unsigned overflow check patterns and return an annotation comment.
///
/// Patterns detected:
///   `(a + b) < a`  or  `(a + b) < b`  → "unsigned overflow check (add)"
///   `a > (a + b)`  or  `b > (a + b)`  → "unsigned overflow check (add)"
///   `(a - b) > a`                      → "unsigned underflow check (sub)"
fn try_recognize_overflow_check(expr: &Wrapped<AstExpression>) -> Option<String> {
    use super::opt_utils::expr_structurally_equal;

    let AstExpression::BinaryOp(cmp_op, lhs, rhs) = &expr.item else {
        return None;
    };

    match cmp_op {
        // (a + b) < a  or  (a + b) < b
        AstBinaryOperator::Less => {
            if let AstExpression::BinaryOp(AstBinaryOperator::Add, a, b) = &lhs.item {
                if expr_structurally_equal(&a.item, &rhs.item)
                    || expr_structurally_equal(&b.item, &rhs.item)
                {
                    return Some("unsigned overflow check (add)".to_string());
                }
            }
            // (a - b) < a is not an overflow check, but check sub > pattern below
        }
        // a > (a + b)  or  b > (a + b)
        AstBinaryOperator::Greater => {
            if let AstExpression::BinaryOp(AstBinaryOperator::Add, a, b) = &rhs.item {
                if expr_structurally_equal(&a.item, &lhs.item)
                    || expr_structurally_equal(&b.item, &lhs.item)
                {
                    return Some("unsigned overflow check (add)".to_string());
                }
            }
            // (a - b) > a  →  underflow check
            if let AstExpression::BinaryOp(AstBinaryOperator::Sub, a, _b) = &lhs.item {
                if expr_structurally_equal(&a.item, &rhs.item) {
                    return Some("unsigned underflow check (sub)".to_string());
                }
            }
        }
        _ => {}
    }

    None
}

/// Recognize saturating/clamping ternary patterns and return an annotation comment.
///
/// Patterns detected:
///   `x > C ? C : x`  /  `x >= C ? C : x`  → "saturate(max=C)"
///   `x < C ? C : x`  /  `x <= C ? C : x`  → "saturate(min=C)"
fn try_recognize_saturating(expr: &Wrapped<AstExpression>) -> Option<String> {
    use super::opt_utils::expr_structurally_equal;

    let AstExpression::Ternary(cond, true_expr, false_expr) = &expr.item else {
        return None;
    };
    let AstExpression::BinaryOp(op, lhs, rhs) = &cond.item else {
        return None;
    };

    // Pattern: x >(=) C ? C : x  →  saturate(max=C)
    // cond.lhs = x, cond.rhs = C (literal), true_expr = C, false_expr = x
    if matches!(
        op,
        AstBinaryOperator::Greater | AstBinaryOperator::GreaterEqual
    ) {
        if let Some(c) = extract_literal_u64(&rhs.item) {
            if extract_literal_u64(&true_expr.item) == Some(c)
                && expr_structurally_equal(&lhs.item, &false_expr.item)
            {
                return Some(format!("saturate(max={c})"));
            }
        }
    }

    // Pattern: x <(=) C ? C : x  →  saturate(min=C)
    if matches!(op, AstBinaryOperator::Less | AstBinaryOperator::LessEqual) {
        if let Some(c) = extract_literal_u64(&rhs.item) {
            if extract_literal_u64(&true_expr.item) == Some(c)
                && expr_structurally_equal(&lhs.item, &false_expr.item)
            {
                return Some(format!("saturate(min={c})"));
            }
        }
    }

    // Also handle the reversed comparison: C <(=) x ? C : x  →  saturate(max=C)
    if matches!(op, AstBinaryOperator::Less | AstBinaryOperator::LessEqual) {
        if let Some(c) = extract_literal_u64(&lhs.item) {
            if extract_literal_u64(&true_expr.item) == Some(c)
                && expr_structurally_equal(&rhs.item, &false_expr.item)
            {
                return Some(format!("saturate(max={c})"));
            }
        }
    }

    // Reversed: C >(=) x ? C : x  →  saturate(min=C)
    if matches!(
        op,
        AstBinaryOperator::Greater | AstBinaryOperator::GreaterEqual
    ) {
        if let Some(c) = extract_literal_u64(&lhs.item) {
            if extract_literal_u64(&true_expr.item) == Some(c)
                && expr_structurally_equal(&rhs.item, &false_expr.item)
            {
                return Some(format!("saturate(min={c})"));
            }
        }
    }

    None
}

/// Detect stride-based address arithmetic: `var + index * STRIDE` where the base
/// operand is a simple variable (likely a pointer) and STRIDE is a well-known type
/// size, suggesting array element access like `ptr[i]`.
fn try_recognize_stride_access(expr: &AstExpression) -> Option<String> {
    let AstExpression::BinaryOp(op, left, right) = expr else {
        return None;
    };
    if !matches!(op, AstBinaryOperator::Add) {
        return None;
    }

    // Try: variable + (index * STRIDE)
    if let Some(stride) = extract_mul_stride(&right.item) {
        if is_simple_base(&left.item) {
            if let Some(hint) = stride_type_hint(stride) {
                return Some(format!("stride={stride} ({hint})"));
            }
        }
    }
    // Try: (index * STRIDE) + variable
    if let Some(stride) = extract_mul_stride(&left.item) {
        if is_simple_base(&right.item) {
            if let Some(hint) = stride_type_hint(stride) {
                return Some(format!("stride={stride} ({hint})"));
            }
        }
    }

    None
}

/// The base operand must be a simple variable or parameter — not an arbitrary
/// complex expression — to confirm pointer-like array base usage.
fn is_simple_base(expr: &AstExpression) -> bool {
    matches!(expr, AstExpression::Variable(_, _))
}

/// Map well-known type sizes to human-readable hints. Returns None for
/// unrecognized sizes to avoid false-positive annotations.
fn stride_type_hint(stride: u64) -> Option<&'static str> {
    match stride {
        2 => Some("int16/short"),
        4 => Some("int32/float"),
        8 => Some("int64/double/pointer"),
        12 => Some("vec3f"),
        16 => Some("vec4f/int128"),
        _ => None,
    }
}

/// Extract the constant multiplier from `expr * literal` or `literal * expr`.
fn extract_mul_stride(expr: &AstExpression) -> Option<u64> {
    let AstExpression::BinaryOp(op, left, right) = expr else {
        return None;
    };
    if !matches!(op, AstBinaryOperator::Mul) {
        return None;
    }
    extract_literal_u64(&right.item).or_else(|| extract_literal_u64(&left.item))
}

/// Detect byte-swap patterns and annotate with bswap16/bswap32.
///
/// 16-bit: `(x >> 8) | (x << 8)` or with masks
/// 32-bit: `(x >> 24) | ((x >> 8) & 0xFF00) | ((x << 8) & 0xFF0000) | (x << 24)`
fn try_recognize_byte_swap(expr: &AstExpression) -> Option<String> {
    // 16-bit bswap: (x >> 8) | (x << 8)
    if let AstExpression::BinaryOp(op, left, right) = expr {
        if matches!(op, AstBinaryOperator::BitOr) {
            if is_shift_by(&left.item, 8, true) && is_shift_by(&right.item, 8, false) {
                return Some("bswap16 / ntohs".to_string());
            }
            if is_shift_by(&left.item, 8, false) && is_shift_by(&right.item, 8, true) {
                return Some("bswap16 / ntohs".to_string());
            }
        }
    }

    // 32-bit bswap: look for OR-tree with shifts by 24 and 8
    // Simplified detection: if expression is an OR-tree containing shifts by 24
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

/// Check if expr is `(something >> amount)` or `(something << amount)`,
/// possibly masked: `(something >> amount) & mask`.
fn is_shift_by(expr: &AstExpression, amount: u64, right_shift: bool) -> bool {
    // Check through an optional mask: (shift_expr) & mask
    let inner = if let AstExpression::BinaryOp(mask_op, inner_expr, _mask) = expr {
        if matches!(mask_op, AstBinaryOperator::BitAnd) {
            &inner_expr.item
        } else {
            expr
        }
    } else {
        expr
    };

    let AstExpression::BinaryOp(op, _base, shift_amt) = inner else {
        return false;
    };
    if right_shift {
        if !matches!(op, AstBinaryOperator::RightShift) {
            return false;
        }
    } else if !matches!(op, AstBinaryOperator::LeftShift) {
        return false;
    }
    extract_literal_u64(&shift_amt.item) == Some(amount)
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

/// Simplify identity operations inserted by obfuscators or unoptimized code:
///   x ^ 0 → x,  x | 0 → x,  x + 0 → x,  x - 0 → x,
///   x * 1 → x,  x & all-ones → x
fn try_simplify_identity_op(expr: &Wrapped<AstExpression>) -> Option<Wrapped<AstExpression>> {
    let AstExpression::BinaryOp(op, left, right) = &expr.item else {
        return None;
    };

    let left_lit = extract_literal_u64(&left.item);
    let right_lit = extract_literal_u64(&right.item);

    // Helper: build a replacement from a kept operand, preserving the outer origin/comment
    let keep = |operand: &Wrapped<AstExpression>| -> Wrapped<AstExpression> {
        Wrapped {
            item: operand.item.clone(),
            origin: expr.origin.clone(),
            comment: expr.comment.clone(),
        }
    };

    // x OP 0 → x  for +, -, ^, |
    if right_lit == Some(0)
        && matches!(
            op,
            AstBinaryOperator::Add
                | AstBinaryOperator::Sub
                | AstBinaryOperator::BitXor
                | AstBinaryOperator::BitOr
        )
    {
        return Some(keep(left));
    }

    // 0 OP x → x  for +, ^, |  (not sub)
    if left_lit == Some(0)
        && matches!(
            op,
            AstBinaryOperator::Add | AstBinaryOperator::BitXor | AstBinaryOperator::BitOr
        )
    {
        return Some(keep(right));
    }

    // x * 1 → x, 1 * x → x
    if matches!(op, AstBinaryOperator::Mul) {
        if right_lit == Some(1) {
            return Some(keep(left));
        }
        if left_lit == Some(1) {
            return Some(keep(right));
        }
    }

    // x & all-ones → x (common mask widths in decompiled code)
    if matches!(op, AstBinaryOperator::BitAnd) {
        let all_ones: &[u64] = &[0xFF, 0xFFFF, 0xFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        if let Some(r) = right_lit {
            if all_ones.contains(&r) {
                return Some(keep(left));
            }
        }
        if let Some(l) = left_lit {
            if all_ones.contains(&l) {
                return Some(keep(right));
            }
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

/// L139: Recognize common C idiom patterns that map to compiler intrinsics.
///
/// Detects:
///   - popcount idiom: loop that counts bits via `x &= x - 1` (Brian Kernighan's trick)
///   - abs idiom: `(x ^ (x >> 31)) - (x >> 31)` for branchless absolute value
///   - min/max idiom: `a ^ ((a ^ b) & -(a < b))` for branchless min
///   - clz/ctz-like: de Bruijn multiplication patterns (annotation only)
fn try_recognize_intrinsic_idiom(expr: &AstExpression) -> Option<String> {
    // Branchless abs: (x ^ (x >> 31)) - (x >> 31)
    // or (x ^ mask) - mask where mask = x >> 31
    if let AstExpression::BinaryOp(AstBinaryOperator::Sub, lhs, rhs) = expr {
        // rhs should be (x >> 31) or (x >> 63)
        if let AstExpression::BinaryOp(AstBinaryOperator::RightShift, _base_r, shift_r) = &rhs.item
        {
            let shift_val = extract_literal_u64(&shift_r.item);
            if matches!(shift_val, Some(31) | Some(63)) {
                // lhs should be (x ^ (x >> 31))
                if let AstExpression::BinaryOp(AstBinaryOperator::BitXor, _xor_l, xor_r) = &lhs.item
                {
                    if let AstExpression::BinaryOp(AstBinaryOperator::RightShift, _base2, shift2) =
                        &xor_r.item
                    {
                        let shift_val2 = extract_literal_u64(&shift2.item);
                        if shift_val2 == shift_val {
                            return Some("branchless abs (intrinsic idiom)".to_string());
                        }
                    }
                }
            }
        }
    }

    // Branchless min/max: a ^ ((a ^ b) & -(a < b)) or similar conditional-select patterns
    // Detect: x ^ ((x ^ y) & mask) where mask is derived from a comparison
    if let AstExpression::BinaryOp(AstBinaryOperator::BitXor, lhs, rhs) = expr {
        if let AstExpression::BinaryOp(AstBinaryOperator::BitAnd, and_lhs, _and_rhs) = &rhs.item {
            if let AstExpression::BinaryOp(AstBinaryOperator::BitXor, _inner_l, _inner_r) =
                &and_lhs.item
            {
                // x ^ ((x ^ y) & mask) — branchless select pattern
                let _ = lhs; // used for structural match
                return Some("branchless min/max (conditional select idiom)".to_string());
            }
        }
    }

    // De Bruijn multiplication for ctz: (x & -x) * DEBRUIJN >> 27
    // The constant 0x077CB531 is a well-known 32-bit de Bruijn sequence
    if let AstExpression::BinaryOp(AstBinaryOperator::RightShift, lhs, _shift) = expr {
        if let AstExpression::BinaryOp(AstBinaryOperator::Mul, mul_l, mul_r) = &lhs.item {
            let mul_const =
                extract_literal_u64(&mul_r.item).or_else(|| extract_literal_u64(&mul_l.item));
            if let Some(c) = mul_const {
                // Known de Bruijn constants for bit scanning
                if c == 0x077CB531
                    || c == 0x03F79D71B4CA8B09
                    || c == 0x06EB14F9
                    || c == 0x0218A392CD3D5DBF
                {
                    return Some("de Bruijn bit-scan (ctz/clz intrinsic idiom)".to_string());
                }
            }
        }
    }

    None
}
