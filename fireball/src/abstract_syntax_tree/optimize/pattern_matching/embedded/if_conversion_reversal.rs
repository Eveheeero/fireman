//! If-conversion reversal: expand nested ternary assignments to if-else.
//!
//! Rewrites:
//!   x = c1 ? (c2 ? a : b) : d  →  if(c1) { x = c2 ? a : b } else { x = d }
//!   x = c1 ? a : (c2 ? b : d)  →  if(c1) { x = a } else { x = c2 ? b : d }

use crate::{
    abstract_syntax_tree::{
        Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn reverse_if_conversion(
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

    reverse_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::IfConversionReversal);
    }

    Ok(())
}

fn reverse_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, branch_true, branch_false) => {
                reverse_in_list(branch_true);
                if let Some(branch_false) = branch_false {
                    reverse_in_list(branch_false);
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => reverse_in_list(body),
            AstStatement::DoWhile(_, body) => reverse_in_list(body),
            AstStatement::For(_, _, _, body) => reverse_in_list(body),
            _ => {}
        }
    }

    for stmt in stmts.iter_mut() {
        try_expand_nested_ternary_assignment(stmt);
    }
}

fn try_expand_nested_ternary_assignment(stmt: &mut WrappedAstStatement) {
    let AstStatement::Assignment(lhs, rhs) = &stmt.statement else {
        return;
    };
    if !matches!(lhs.item, AstExpression::Variable(_, _)) {
        return;
    }

    let AstExpression::Ternary(cond, true_expr, false_expr) = &rhs.item else {
        return;
    };
    if !matches!(true_expr.item, AstExpression::Ternary(_, _, _))
        && !matches!(false_expr.item, AstExpression::Ternary(_, _, _))
    {
        return;
    }

    stmt.statement = AstStatement::If(
        cond.as_ref().clone(),
        vec![build_assignment_branch(
            lhs.clone(),
            true_expr.as_ref().clone(),
            stmt,
        )],
        Some(vec![build_assignment_branch(
            lhs.clone(),
            false_expr.as_ref().clone(),
            stmt,
        )]),
    );
}

fn build_assignment_branch(
    lhs: crate::abstract_syntax_tree::Wrapped<AstExpression>,
    rhs: crate::abstract_syntax_tree::Wrapped<AstExpression>,
    template: &WrappedAstStatement,
) -> WrappedAstStatement {
    match rhs.item {
        AstExpression::Ternary(cond, true_expr, false_expr) => WrappedAstStatement {
            statement: AstStatement::If(
                cond.as_ref().clone(),
                vec![build_assignment_branch(
                    lhs.clone(),
                    true_expr.as_ref().clone(),
                    template,
                )],
                Some(vec![build_assignment_branch(
                    lhs,
                    false_expr.as_ref().clone(),
                    template,
                )]),
            ),
            origin: template.origin.clone(),
            comment: None,
        },
        _ => WrappedAstStatement {
            statement: AstStatement::Assignment(lhs, rhs),
            origin: template.origin.clone(),
            comment: None,
        },
    }
}
