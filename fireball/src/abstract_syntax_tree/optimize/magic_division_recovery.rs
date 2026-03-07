use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral,
        AstStatement, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn recover_magic_divisions(
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

    recover_in_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::MagicDivisionRecovery);
    }

    Ok(())
}

fn recover_in_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        recover_in_statement(stmt);
    }
}

fn recover_in_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::Declaration(_, rhs) => {
            if let Some(rhs) = rhs {
                recover_in_expression(rhs);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            recover_in_expression(lhs);
            recover_in_expression(rhs);
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            recover_in_expression(cond);
            recover_in_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                recover_in_statement_list(branch_false);
            }
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            recover_in_expression(cond);
            recover_in_statement_list(body);
        }
        AstStatement::For(init, cond, update, body) => {
            recover_in_statement(init);
            recover_in_expression(cond);
            recover_in_statement(update);
            recover_in_statement_list(body);
        }
        AstStatement::Switch(discrim, cases, default) => {
            recover_in_expression(discrim);
            for (_, case_body) in cases.iter_mut() {
                recover_in_statement_list(case_body);
            }
            if let Some(default_body) = default {
                recover_in_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => {
            recover_in_statement_list(body);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                recover_in_expression(expr);
            }
        }
        AstStatement::Call(call) => {
            recover_in_call(call);
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

fn recover_in_call(call: &mut crate::abstract_syntax_tree::AstCall) {
    match call {
        crate::abstract_syntax_tree::AstCall::Variable { args, .. }
        | crate::abstract_syntax_tree::AstCall::Function { args, .. }
        | crate::abstract_syntax_tree::AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                recover_in_expression(arg);
            }
        }
        crate::abstract_syntax_tree::AstCall::Builtin(_, args) => {
            use crate::abstract_syntax_tree::AstBuiltinFunctionArgument;
            match args.as_mut() {
                AstBuiltinFunctionArgument::None => {}
                AstBuiltinFunctionArgument::Print(items) => {
                    for item in items.iter_mut() {
                        recover_in_expression(item);
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
                    recover_in_expression(expr);
                }
                AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
                    recover_in_expression(expr1);
                    recover_in_expression(expr2);
                }
            }
        }
    }
}

fn recover_in_expression(expression: &mut Wrapped<AstExpression>) {
    // Recurse into sub-expressions first (bottom-up)
    match &mut expression.item {
        AstExpression::UnaryOp(_, arg) => {
            recover_in_expression(arg);
        }
        AstExpression::BinaryOp(_, left, right) => {
            recover_in_expression(left);
            recover_in_expression(right);
        }
        AstExpression::Call(call) => {
            recover_in_call(call);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            recover_in_expression(arg);
        }
        AstExpression::ArrayAccess(base, idx) => {
            recover_in_expression(base);
            recover_in_expression(idx);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            recover_in_expression(cond);
            recover_in_expression(true_expr);
            recover_in_expression(false_expr);
        }
        AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_) => {}
    }

    // Try to replace the current expression with a recovered division
    if let Some(replacement) = try_recover_at(expression) {
        *expression = replacement;
    }
}

/// Try to recover a magic division pattern at the current expression node.
///
/// Detects:
///   (x * magic_constant) >> shift_amount
/// where the multiply-then-shift encodes unsigned division by a small constant.
///
/// Also detects signed variants where the magic constant is negative (represented
/// as `AstLiteral::Int` with a negative value).
fn try_recover_at(expression: &Wrapped<AstExpression>) -> Option<Wrapped<AstExpression>> {
    let AstExpression::BinaryOp(AstBinaryOperator::RightShift, left, right) = &expression.item
    else {
        return None;
    };

    // The shift amount must be a literal
    let shift = extract_uint_literal(&right.item)?;

    // The left operand must be a multiplication by a literal constant
    let AstExpression::BinaryOp(AstBinaryOperator::Mul, mul_left, mul_right) = &left.item else {
        return None;
    };

    // Try both orderings: (x * magic) and (magic * x)
    let (operand, magic, is_signed) =
        if let Some((mag, signed)) = extract_magic_constant(&mul_right.item) {
            (mul_left, mag, signed)
        } else if let Some((mag, signed)) = extract_magic_constant(&mul_left.item) {
            (mul_right, mag, signed)
        } else {
            return None;
        };

    if is_signed {
        // For signed division, the magic constant is negative in two's complement.
        // Reinterpret as unsigned for the recovery algorithm.
        let magic_unsigned = magic as u64;
        let divisor = try_recover_division(magic_unsigned, shift)?;
        let div_expr = AstExpression::BinaryOp(
            AstBinaryOperator::Div,
            Box::new(operand.as_ref().clone()),
            Box::new(wrap_with_source(
                expression,
                AstExpression::Literal(AstLiteral::Int(divisor as i64)),
            )),
        );
        Some(wrap_with_source(expression, div_expr))
    } else {
        let divisor = try_recover_division(magic, shift)?;
        let div_expr = AstExpression::BinaryOp(
            AstBinaryOperator::Div,
            Box::new(operand.as_ref().clone()),
            Box::new(wrap_with_source(
                expression,
                AstExpression::Literal(AstLiteral::UInt(divisor)),
            )),
        );
        Some(wrap_with_source(expression, div_expr))
    }
}

/// Extract an unsigned integer literal value from an expression.
fn extract_uint_literal(expr: &AstExpression) -> Option<u64> {
    match expr {
        AstExpression::Literal(AstLiteral::UInt(v)) => Some(*v),
        AstExpression::Literal(AstLiteral::Int(v)) if *v >= 0 => Some(*v as u64),
        _ => None,
    }
}

/// Extract the magic constant from an expression.
/// Returns (value_as_u64, is_signed).
fn extract_magic_constant(expr: &AstExpression) -> Option<(u64, bool)> {
    match expr {
        AstExpression::Literal(AstLiteral::UInt(v)) => Some((*v, false)),
        AstExpression::Literal(AstLiteral::Int(v)) => {
            if *v < 0 {
                // Negative magic constant indicates signed division.
                // Reinterpret as unsigned bits.
                Some((*v as u64, true))
            } else {
                Some((*v as u64, false))
            }
        }
        _ => None,
    }
}

/// For 64-bit unsigned division by constant d, the compiler generates:
///   (x * magic) >> (64 + shift)   or   mulhi(x, magic) >> shift
/// where magic is approximately 2^(64+shift) / d.
///
/// Try small divisors 2..=1024 and verify the relationship holds exactly.
fn try_recover_division(magic: u64, shift: u64) -> Option<u64> {
    let magic128 = magic as u128;

    for d in 2u64..=1024 {
        let d128 = d as u128;

        // Check with total shift = shift + 64 (mulhi already drops low 64 bits)
        let expected_shift_total = shift + 64;
        if expected_shift_total <= 127 {
            let power = 1u128 << expected_shift_total;
            // Exact: magic * d == 2^(64+shift)
            if magic128 * d128 == power {
                return Some(d);
            }
            // Round-up variant: magic * d + d - 1 == power - 1
            // i.e. magic = ceil(2^(64+shift) / d)
            if magic128 * d128 + d128 - 1 == power - 1 {
                return Some(d);
            }
        }

        // Also try with just shift (no +64), for patterns where the full
        // 128-bit multiply result is used rather than mulhi.
        if shift > 0 && shift < 128 {
            let power_small = 1u128 << shift;
            if magic128 * d128 == power_small {
                return Some(d);
            }
        }
    }

    None
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
