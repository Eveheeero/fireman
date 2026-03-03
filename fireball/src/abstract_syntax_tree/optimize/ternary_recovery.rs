use crate::{
    abstract_syntax_tree::{
        Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn recover_ternary(
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

    recover_ternary_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::TernaryRecovery);
    }

    Ok(())
}

fn recover_ternary_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // First recurse into nested structures
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                recover_ternary_in_list(bt);
                if let Some(bf) = bf {
                    recover_ternary_in_list(bf);
                }
            }
            AstStatement::While(_, body) => recover_ternary_in_list(body),
            AstStatement::For(_, _, _, body) => recover_ternary_in_list(body),
            AstStatement::Block(body) => recover_ternary_in_list(body),
            _ => {}
        }
    }

    // Now try to convert if/else assignments to ternary
    for stmt in stmts.iter_mut() {
        try_convert_to_ternary(stmt);
    }
}

/// Detect pattern:
///   if (cond) { v = expr_a; } else { v = expr_b; }
/// and rewrite to:
///   v = cond ? expr_a : expr_b;
fn try_convert_to_ternary(stmt: &mut WrappedAstStatement) {
    let AstStatement::If(cond, branch_true, Some(branch_false)) = &stmt.statement else {
        return;
    };

    // Both branches must have exactly one statement
    if branch_true.len() != 1 || branch_false.len() != 1 {
        return;
    }

    // Both must be assignments to the same plain variable
    let (true_var, true_rhs) = match &branch_true[0].statement {
        AstStatement::Assignment(lhs, rhs) => {
            if let AstExpression::Variable(_, var_id) = &lhs.item {
                (*var_id, rhs)
            } else {
                return;
            }
        }
        _ => return,
    };

    let (false_var, false_rhs) = match &branch_false[0].statement {
        AstStatement::Assignment(lhs, rhs) => {
            if let AstExpression::Variable(_, var_id) = &lhs.item {
                (*var_id, rhs)
            } else {
                return;
            }
        }
        _ => return,
    };

    if true_var != false_var {
        return;
    }

    // Build the ternary expression
    let target_var_id = true_var;
    let ternary_expr = AstExpression::Ternary(
        Box::new(cond.clone()),
        Box::new(true_rhs.clone()),
        Box::new(false_rhs.clone()),
    );

    // Get the LHS from the true branch (it has the variable map we need)
    let lhs = match &branch_true[0].statement {
        AstStatement::Assignment(lhs, _) => lhs.clone(),
        _ => unreachable!(),
    };

    let _ = target_var_id;
    let ternary_wrapped = Wrapped {
        item: ternary_expr,
        origin: cond.origin.clone(),
        comment: None,
    };

    stmt.statement = AstStatement::Assignment(lhs, ternary_wrapped);
}
