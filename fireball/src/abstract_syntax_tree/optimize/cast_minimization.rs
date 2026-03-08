use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstLiteral, AstStatement, AstUnaryOperator, AstValueType, ProcessedOptimization, Wrapped,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn minimize_casts(
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

    minimize_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::CastMinimization);
    }

    Ok(())
}

fn minimize_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        minimize_statement(stmt);
    }
}

fn minimize_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::Declaration(_lhs, rhs) => {
            if let Some(rhs) = rhs {
                minimize_expression(rhs);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            minimize_expression(lhs);
            minimize_expression(rhs);
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            minimize_expression(cond);
            minimize_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                minimize_statement_list(branch_false);
            }
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            minimize_expression(cond);
            minimize_statement_list(body);
        }
        AstStatement::For(init, cond, update, body) => {
            minimize_statement(init);
            minimize_expression(cond);
            minimize_statement(update);
            minimize_statement_list(body);
        }
        AstStatement::Switch(discrim, cases, default) => {
            minimize_expression(discrim);
            for (_lit, case_body) in cases.iter_mut() {
                minimize_statement_list(case_body);
            }
            if let Some(default_body) = default {
                minimize_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => {
            minimize_statement_list(body);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                minimize_expression(expr);
            }
        }
        AstStatement::Call(call) => {
            minimize_call(call);
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

fn minimize_call(call: &mut AstCall) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                minimize_expression(arg);
            }
        }
        AstCall::Builtin(_, args) => match args.as_mut() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(items) => {
                for item in items.iter_mut() {
                    minimize_expression(item);
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
                minimize_expression(expr);
            }
            AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
                minimize_expression(expr1);
                minimize_expression(expr2);
            }
        },
    }
}

fn minimize_expression(expr: &mut Wrapped<AstExpression>) {
    // Recurse into children first (bottom-up minimization).
    match &mut expr.item {
        AstExpression::UnaryOp(_, arg) => {
            minimize_expression(arg);
        }
        AstExpression::BinaryOp(_, left, right) => {
            minimize_expression(left);
            minimize_expression(right);
        }
        AstExpression::Call(call) => {
            minimize_call(call);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            minimize_expression(arg);
        }
        AstExpression::ArrayAccess(base, idx) => {
            minimize_expression(base);
            minimize_expression(idx);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            minimize_expression(cond);
            minimize_expression(true_expr);
            minimize_expression(false_expr);
        }
        AstExpression::Literal(_)
        | AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => {}
    }

    // Apply cast minimization rules to the current node.
    minimize_current(expr);
}

fn minimize_current(expr: &mut Wrapped<AstExpression>) {
    while simplify_structural_cast(expr) {}

    // Rule 2: Identity cast on literal -- Cast(Int32, Literal(Int(n))) where n fits in i32.
    // The cast is redundant if the literal already fits in the target type.
    if let AstExpression::Cast(target_ty, inner) = &expr.item {
        if let AstExpression::Literal(AstLiteral::Int(n)) = &inner.item {
            let fits = match target_ty {
                AstValueType::Int32 => *n >= i64::from(i32::MIN) && *n <= i64::from(i32::MAX),
                AstValueType::Int16 => *n >= i64::from(i16::MIN) && *n <= i64::from(i16::MAX),
                AstValueType::Int8 => *n >= i64::from(i8::MIN) && *n <= i64::from(i8::MAX),
                AstValueType::Int64 | AstValueType::Int => true,
                _ => false,
            };
            if fits {
                expr.item = inner.item.clone();
                return;
            }
        }
    }
}

fn simplify_structural_cast(expr: &mut Wrapped<AstExpression>) -> bool {
    match &expr.item {
        AstExpression::Cast(target_ty, inner) => {
            let AstExpression::Cast(_, source) = &inner.item else {
                return false;
            };

            expr.item = AstExpression::Cast(target_ty.clone(), Box::new((**source).clone()));
            true
        }
        AstExpression::UnaryOp(operator, inner)
            if matches!(
                operator,
                AstUnaryOperator::CastSigned | AstUnaryOperator::CastUnsigned
            ) =>
        {
            let AstExpression::UnaryOp(inner_operator, source) = &inner.item else {
                return false;
            };

            if !matches!(
                inner_operator,
                AstUnaryOperator::CastSigned | AstUnaryOperator::CastUnsigned
            ) {
                return false;
            }

            expr.item = AstExpression::UnaryOp(operator.clone(), Box::new((**source).clone()));
            true
        }
        _ => false,
    }
}
