//! Magic division recovery extracted from magic_division_recovery.rs.
//!
//! Detects `(x * magic_constant) >> shift_amount` patterns that encode
//! unsigned (or signed) division by a small constant, and rewrites them
//! to `x / divisor`.

use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral,
        AstStatement, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn recover_magic_divisions(
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

    if let Some(replacement) = try_recover_at(expression) {
        *expression = replacement;
    }
}

fn try_recover_at(expression: &Wrapped<AstExpression>) -> Option<Wrapped<AstExpression>> {
    let AstExpression::BinaryOp(AstBinaryOperator::RightShift, left, right) = &expression.item
    else {
        return None;
    };

    let shift = extract_uint_literal(&right.item)?;

    let AstExpression::BinaryOp(AstBinaryOperator::Mul, mul_left, mul_right) = &left.item else {
        return None;
    };

    let (operand, magic, is_signed) = if let Some((mag, signed)) =
        crate::abstract_syntax_tree::optimize::magic_division_recovery::extract_magic_constant(
            &mul_right.item,
        ) {
        (mul_left, mag, signed)
    } else if let Some((mag, signed)) =
        crate::abstract_syntax_tree::optimize::magic_division_recovery::extract_magic_constant(
            &mul_left.item,
        )
    {
        (mul_right, mag, signed)
    } else {
        return None;
    };

    if is_signed {
        let magic_unsigned = magic as u64;
        let divisor =
            crate::abstract_syntax_tree::optimize::magic_division_recovery::try_recover_division(
                magic_unsigned,
                shift,
            )?;
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
        let divisor =
            crate::abstract_syntax_tree::optimize::magic_division_recovery::try_recover_division(
                magic, shift,
            )?;
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

fn extract_uint_literal(expr: &AstExpression) -> Option<u64> {
    match expr {
        AstExpression::Literal(AstLiteral::UInt(v)) => Some(*v),
        AstExpression::Literal(AstLiteral::Int(v)) if *v >= 0 => Some(*v as u64),
        _ => None,
    }
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
