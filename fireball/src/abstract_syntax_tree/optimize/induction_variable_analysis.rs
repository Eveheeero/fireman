use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral,
        AstStatement, AstVariableId, ProcessedOptimization, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn analyze_induction_variables(
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

    analyze_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::InductionVariableAnalysis);
    }

    Ok(())
}

fn analyze_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        analyze_statement(stmt);
    }
}

fn analyze_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            analyze_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                analyze_statement_list(branch_false);
            }
        }
        AstStatement::While(_, body) => {
            analyze_statement_list(body);
        }
        AstStatement::For(init, cond, update, body) => {
            analyze_statement_list(body);

            // Only rewrite `i != N` when we can prove the direction from the
            // init value and bound.  This avoids mis-compiling loops where the
            // bound is a runtime variable on the "wrong" side of the start
            // (e.g. `for (i = 0; i != runtime; ++i)` where runtime could be
            // negative or wrapped).

            if let Some(update_var) = get_update_var(&update.statement) {
                // Determine the replacement operator (if any) before mutating,
                // so that borrows of `cond.item` are released first.
                let new_op =
                    if let AstExpression::BinaryOp(AstBinaryOperator::NotEqual, left, right) =
                        &cond.item
                    {
                        let cond_var_matches = matches!(
                            &left.item,
                            AstExpression::Variable(_, vid) if *vid == update_var
                        );
                        if cond_var_matches {
                            let init_val = get_init_literal(&init.statement, update_var);
                            let bound_val = get_literal_i128(&right.item);
                            match (init_val, bound_val) {
                                (Some(start), Some(bound))
                                    if is_increment_by_one(&update.statement) && start < bound =>
                                {
                                    Some(AstBinaryOperator::Less)
                                }
                                (Some(start), Some(bound))
                                    if is_decrement_by_one(&update.statement) && start > bound =>
                                {
                                    Some(AstBinaryOperator::Greater)
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                // Apply the rewrite now that the borrow is released.
                if let Some(op) = new_op {
                    if let AstExpression::BinaryOp(_, left, right) = &cond.item {
                        let left = left.clone();
                        let right = right.clone();
                        cond.item = AstExpression::BinaryOp(op, left, right);
                    }
                }
            }
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                analyze_statement_list(case_body);
            }
            if let Some(default_body) = default {
                analyze_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => {
            analyze_statement_list(body);
        }
        _ => {}
    }
}

/// Extract a constant integer value from an `AstExpression::Literal`, widened
/// to `i128` so signed and unsigned literals can be compared uniformly.
fn get_literal_i128(expr: &AstExpression) -> Option<i128> {
    match expr {
        AstExpression::Literal(AstLiteral::Int(v)) => Some(*v as i128),
        AstExpression::Literal(AstLiteral::UInt(v)) => Some(*v as i128),
        _ => None,
    }
}

/// If `init_stmt` initializes `var_id` to a literal value, return that value
/// as an `i128`.  Handles both `Declaration(var, Some(lit))` and
/// `Assignment(Variable(var), Literal(lit))`.
fn get_init_literal(init_stmt: &AstStatement, var_id: AstVariableId) -> Option<i128> {
    match init_stmt {
        AstStatement::Declaration(var, Some(expr)) if var.id == var_id => {
            get_literal_i128(&expr.item)
        }
        AstStatement::Assignment(lhs, rhs) => {
            if matches!(&lhs.item, AstExpression::Variable(_, vid) if *vid == var_id) {
                get_literal_i128(&rhs.item)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Extract the variable being updated in an assignment statement of the form
/// `var = var op expr`.
fn get_update_var(stmt: &AstStatement) -> Option<AstVariableId> {
    if let AstStatement::Assignment(lhs, rhs) = stmt {
        if let AstExpression::Variable(_, lhs_var) = &lhs.item {
            if let AstExpression::BinaryOp(
                AstBinaryOperator::Add | AstBinaryOperator::Sub,
                op_left,
                _,
            ) = &rhs.item
            {
                if let AstExpression::Variable(_, rhs_var) = &op_left.item {
                    if lhs_var == rhs_var {
                        return Some(*lhs_var);
                    }
                }
            }
        }
    }
    None
}

/// Check if the statement is `var = var + 1`.
fn is_increment_by_one(stmt: &AstStatement) -> bool {
    if let AstStatement::Assignment(lhs, rhs) = stmt {
        if let AstExpression::Variable(_, lhs_var) = &lhs.item {
            if let AstExpression::BinaryOp(AstBinaryOperator::Add, op_left, op_right) = &rhs.item {
                if let AstExpression::Variable(_, rhs_var) = &op_left.item {
                    if lhs_var == rhs_var {
                        return matches!(
                            &op_right.item,
                            AstExpression::Literal(AstLiteral::Int(1))
                                | AstExpression::Literal(AstLiteral::UInt(1))
                        );
                    }
                }
            }
        }
    }
    false
}

/// Check if the statement is `var = var - 1`.
fn is_decrement_by_one(stmt: &AstStatement) -> bool {
    if let AstStatement::Assignment(lhs, rhs) = stmt {
        if let AstExpression::Variable(_, lhs_var) = &lhs.item {
            if let AstExpression::BinaryOp(AstBinaryOperator::Sub, op_left, op_right) = &rhs.item {
                if let AstExpression::Variable(_, rhs_var) = &op_left.item {
                    if lhs_var == rhs_var {
                        return matches!(
                            &op_right.item,
                            AstExpression::Literal(AstLiteral::Int(1))
                                | AstExpression::Literal(AstLiteral::UInt(1))
                        );
                    }
                }
            }
        }
    }
    false
}
