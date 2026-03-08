use crate::{
    abstract_syntax_tree::{
        Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn reverse_if_conversion(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let mut functions = ast.functions.write().unwrap();
    let function = functions
        .get_mut(&function_id)
        .and_then(|x| x.get_mut(&function_version))
        .unwrap();

    reverse_if_conversion_in_statements(&mut function.body);
    function
        .processed_optimizations
        .push(ProcessedOptimization::IfConversionReversal);

    Ok(())
}

fn reverse_if_conversion_in_statements(statements: &mut Vec<WrappedAstStatement>) {
    for statement in statements.iter_mut() {
        reverse_if_conversion_in_statement(statement);
    }
}

fn reverse_if_conversion_in_statement(statement: &mut WrappedAstStatement) {
    match &mut statement.statement {
        AstStatement::Assignment(lhs, rhs) => {
            if let Some(rewritten) = expand_nested_ternary_assignment(lhs, rhs) {
                statement.statement = rewritten;
            }
        }
        AstStatement::If(_, branch_true, branch_false) => {
            reverse_if_conversion_in_statements(branch_true);
            if let Some(branch_false) = branch_false {
                reverse_if_conversion_in_statements(branch_false);
            }
        }
        AstStatement::While(_, body)
        | AstStatement::DoWhile(_, body)
        | AstStatement::Block(body) => reverse_if_conversion_in_statements(body),
        AstStatement::For(init, _, update, body) => {
            reverse_if_conversion_in_statement(init);
            reverse_if_conversion_in_statement(update);
            reverse_if_conversion_in_statements(body);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_, body) in cases.iter_mut() {
                reverse_if_conversion_in_statements(body);
            }
            if let Some(default) = default {
                reverse_if_conversion_in_statements(default);
            }
        }
        AstStatement::Declaration(_, _)
        | AstStatement::Return(_)
        | AstStatement::Call(_)
        | AstStatement::Label(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Empty
        | AstStatement::Break
        | AstStatement::Continue => {}
    }
}

fn expand_nested_ternary_assignment(
    lhs: &Wrapped<AstExpression>,
    rhs: &Wrapped<AstExpression>,
) -> Option<AstStatement> {
    let AstExpression::Ternary(condition, true_expr, false_expr) = &rhs.item else {
        return None;
    };

    let has_nested_ternary = matches!(true_expr.item, AstExpression::Ternary(_, _, _))
        || matches!(false_expr.item, AstExpression::Ternary(_, _, _));
    if !has_nested_ternary {
        return None;
    }

    Some(build_ternary_assignment_if(
        lhs, condition, true_expr, false_expr,
    ))
}

fn build_assignment_branch(
    lhs: &Wrapped<AstExpression>,
    rhs: &Wrapped<AstExpression>,
) -> WrappedAstStatement {
    let statement = match &rhs.item {
        AstExpression::Ternary(condition, true_expr, false_expr) => {
            build_ternary_assignment_if(lhs, condition, true_expr, false_expr)
        }
        _ => AstStatement::Assignment(lhs.clone(), rhs.clone()),
    };

    WrappedAstStatement {
        statement,
        origin: crate::abstract_syntax_tree::AstStatementOrigin::Unknown,
        comment: None,
    }
}

fn build_ternary_assignment_if(
    lhs: &Wrapped<AstExpression>,
    condition: &Box<Wrapped<AstExpression>>,
    true_expr: &Box<Wrapped<AstExpression>>,
    false_expr: &Box<Wrapped<AstExpression>>,
) -> AstStatement {
    AstStatement::If(
        condition.as_ref().clone(),
        vec![build_assignment_branch(lhs, true_expr)],
        Some(vec![build_assignment_branch(lhs, false_expr)]),
    )
}
