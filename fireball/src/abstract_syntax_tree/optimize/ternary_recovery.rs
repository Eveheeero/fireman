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
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, branch_true, branch_false) => {
                recover_ternary_in_list(branch_true);
                if let Some(branch_false) = branch_false {
                    recover_ternary_in_list(branch_false);
                }
            }
            AstStatement::While(_, body) => recover_ternary_in_list(body),
            AstStatement::For(_, _, _, body) => recover_ternary_in_list(body),
            AstStatement::Block(body) => recover_ternary_in_list(body),
            _ => {}
        }
    }

    for stmt in stmts.iter_mut() {
        try_convert_to_ternary(stmt);
    }
}

fn try_convert_to_ternary(stmt: &mut WrappedAstStatement) {
    let AstStatement::If(cond, branch_true, Some(branch_false)) = &stmt.statement else {
        return;
    };

    if branch_true.len() != 1 || branch_false.len() != 1 {
        return;
    }

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

    let ternary_wrapped = Wrapped {
        item: AstExpression::Ternary(
            Box::new(cond.clone()),
            Box::new(true_rhs.clone()),
            Box::new(false_rhs.clone()),
        ),
        origin: cond.origin.clone(),
        comment: None,
    };

    let lhs = match &branch_true[0].statement {
        AstStatement::Assignment(lhs, _) => lhs.clone(),
        _ => unreachable!(),
    };

    stmt.statement = AstStatement::Assignment(lhs, ternary_wrapped);
}
