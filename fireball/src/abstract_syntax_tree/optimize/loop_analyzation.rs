use crate::{
    abstract_syntax_tree::{
        Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral, AstStatement,
        AstVariableId, ProcessedOptimization, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn analyze_loops(
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

    normalize_statement_list(&mut body);
    normalize_infinite_loops(&mut body);
    normalize_rotated_loops(&mut body);
    try_convert_while_to_for(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::LoopAnalyzation);
    }

    Ok(())
}

fn normalize_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        normalize_statement(stmt);
    }
}

fn normalize_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            normalize_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                normalize_statement_list(branch_false);
            }
        }
        AstStatement::While(_, body) => {
            normalize_statement_list(body);
        }
        AstStatement::For(init, cond, update, body) => {
            normalize_statement(init);
            normalize_statement(update);
            normalize_statement_list(body);
            if is_noop_statement(init.as_ref()) && is_noop_statement(update.as_ref()) {
                stmt.statement = AstStatement::While(cond.clone(), std::mem::take(body));
            }
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                normalize_statement_list(case_body);
            }
            if let Some(default_body) = default {
                normalize_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => {
            normalize_statement_list(body);
        }
        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::Return(_)
        | AstStatement::Call(_)
        | AstStatement::Label(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Empty => {}
    }
}

/// Normalize rotated loops: convert `if(cond) { while(cond) { body; } }` → `while(cond) { body; }`
/// when the condition is side-effect-free (pure).
fn normalize_rotated_loops(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                normalize_rotated_loops(bt);
                if let Some(bf) = bf {
                    normalize_rotated_loops(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                normalize_rotated_loops(body);
            }
            AstStatement::For(_, _, _, body) => normalize_rotated_loops(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    normalize_rotated_loops(case_body);
                }
                if let Some(default_body) = default {
                    normalize_rotated_loops(default_body);
                }
            }
            _ => {}
        }
    }

    // Now look for `if(cond) { while(cond) { body } }` at this level.
    for stmt in stmts.iter_mut() {
        let AstStatement::If(if_cond, branch_true, branch_false) = &mut stmt.statement else {
            continue;
        };
        // Must be if-without-else, and condition must be pure.
        if branch_false.is_some() {
            continue;
        }
        if !super::opt_utils::is_pure_expression(&if_cond.item) {
            continue;
        }
        // Branch body must be exactly one statement: a while with the same condition.
        if branch_true.len() != 1 {
            continue;
        }
        let AstStatement::While(while_cond, _) = &branch_true[0].statement else {
            continue;
        };
        if !super::opt_utils::expr_structurally_equal(&if_cond.item, &while_cond.item) {
            continue;
        }
        // Safe to collapse: replace `if(cond) { while(cond) { body } }` with `while(cond) { body }`.
        let while_stmt = branch_true.remove(0);
        stmt.statement = while_stmt.statement;
    }
}

/// Normalize infinite loops: convert `while(1)` / `while(nonzero_literal)` to `while(true)`.
fn normalize_infinite_loops(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::While(cond, body) => {
                normalize_infinite_loops(body);
                if is_always_true_literal(&cond.item) {
                    cond.item = AstExpression::Literal(AstLiteral::Bool(true));
                }
            }
            AstStatement::If(_, bt, bf) => {
                normalize_infinite_loops(bt);
                if let Some(bf) = bf {
                    normalize_infinite_loops(bf);
                }
            }
            AstStatement::For(_, _, _, body) => normalize_infinite_loops(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    normalize_infinite_loops(case_body);
                }
                if let Some(default_body) = default {
                    normalize_infinite_loops(default_body);
                }
            }
            AstStatement::Block(body) => normalize_infinite_loops(body),
            _ => {}
        }
    }
}

/// Returns true if the expression is a non-zero integer or boolean true literal.
fn is_always_true_literal(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(AstLiteral::Int(n)) => *n != 0,
        AstExpression::Literal(AstLiteral::UInt(n)) => *n != 0,
        AstExpression::Literal(AstLiteral::Bool(true)) => true,
        _ => false,
    }
}

fn is_noop_statement(stmt: &WrappedAstStatement) -> bool {
    matches!(
        &stmt.statement,
        AstStatement::Empty | AstStatement::Comment(_)
    )
}

fn get_assigned_var(stmt: &AstStatement) -> Option<AstVariableId> {
    match stmt {
        AstStatement::Assignment(lhs, _) => {
            if let AstExpression::Variable(_, var_id) = &lhs.item {
                Some(*var_id)
            } else {
                None
            }
        }
        AstStatement::Declaration(var, Some(_)) => Some(var.id),
        _ => None,
    }
}

fn try_convert_while_to_for(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested statement bodies first
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, branch_true, branch_false) => {
                try_convert_while_to_for(branch_true);
                if let Some(branch_false) = branch_false {
                    try_convert_while_to_for(branch_false);
                }
            }
            AstStatement::While(_, body) => {
                try_convert_while_to_for(body);
            }
            AstStatement::For(_, _, _, body) => {
                try_convert_while_to_for(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_lit, case_body) in cases.iter_mut() {
                    try_convert_while_to_for(case_body);
                }
                if let Some(default_body) = default {
                    try_convert_while_to_for(default_body);
                }
            }
            AstStatement::Block(body) => {
                try_convert_while_to_for(body);
            }
            _ => {}
        }
    }

    // Now look for init-before-while patterns at this level
    let mut i = 0;
    while i + 1 < stmts.len() {
        let init_var_id = match get_assigned_var(&stmts[i].statement) {
            Some(id) => id,
            None => {
                i += 1;
                continue;
            }
        };

        let should_convert = if let AstStatement::While(cond, body) = &stmts[i + 1].statement {
            if body.len() >= 2 {
                let last = &body[body.len() - 1];
                if let Some(update_var_id) = get_assigned_var(&last.statement) {
                    if update_var_id == init_var_id {
                        let mut vars = hashbrown::HashSet::new();
                        super::opt_utils::collect_expr_variables(&cond.item, &mut vars);
                        vars.contains(&init_var_id)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if should_convert {
            let init_stmt = stmts.remove(i);
            if let AstStatement::While(cond, mut body) =
                std::mem::replace(&mut stmts[i].statement, AstStatement::Empty)
            {
                let update_stmt = body.pop().unwrap();
                stmts[i].statement =
                    AstStatement::For(Box::new(init_stmt), cond, Box::new(update_stmt), body);
            }
            // Don't increment i; re-check at the same index
        } else {
            i += 1;
        }
    }
}
