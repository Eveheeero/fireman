//! Clamp recovery: min(max(x, low), high) → clamp(x, low, high).

use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement,
        ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn recover_clamp(
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

    recover_clamp_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        // Re-use BitTrickRecognition or similar if no specific variant exists, 
        // but since we added variants, let's assume we can add ClampRecovery.
        // For now, using PatternMatching to avoid adding to enum every time.
        function
            .processed_optimizations
            .push(ProcessedOptimization::PatternMatching);
    }

    Ok(())
}

fn recover_clamp_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                recover_clamp_in_list(bt);
                if let Some(bf) = bf {
                    recover_clamp_in_list(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
                recover_clamp_in_list(body)
            }
            AstStatement::For(init, _, update, body) => {
                recover_clamp_in_list(body);
                // Also recurse into init and update if they are blocks (unlikely)
                try_recover_clamp_in_stmt(init);
                try_recover_clamp_in_stmt(update);
            }
            AstStatement::Block(body) => recover_clamp_in_list(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, body) in cases {
                    recover_clamp_in_list(body);
                }
                if let Some(body) = default {
                    recover_clamp_in_list(body);
                }
            }
            _ => {}
        }
        try_recover_clamp_in_stmt(stmt);
    }
}

fn try_recover_clamp_in_stmt(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::Assignment(_, rhs) | AstStatement::Return(Some(rhs)) => {
            try_recover_clamp_in_expr(&mut rhs.item);
        }
        AstStatement::If(cond, _, _) | AstStatement::While(cond, _) | AstStatement::DoWhile(cond, _) => {
            try_recover_clamp_in_expr(&mut cond.item);
        }
        AstStatement::Call(call) => {
            let args = match call {
                AstCall::Unknown(_, args) | AstCall::Builtin(_, args) => args,
                _ => return,
            };
            for arg in args {
                try_recover_clamp_in_expr(&mut arg.item);
            }
        }
        _ => {}
    }
}

fn try_recover_clamp_in_expr(expr: &mut AstExpression) {
    // Traverse down
    match expr {
        AstExpression::BinaryOp(_, lhs, rhs) => {
            try_recover_clamp_in_expr(&mut lhs.item);
            try_recover_clamp_in_expr(&mut rhs.item);
        }
        AstExpression::UnaryOp(_, inner) | AstExpression::Cast(_, inner) | AstExpression::Deref(inner) | AstExpression::AddressOf(inner) => {
            try_recover_clamp_in_expr(&mut inner.item);
        }
        AstExpression::Ternary(c, t, f) => {
            try_recover_clamp_in_expr(&mut c.item);
            try_recover_clamp_in_expr(&mut t.item);
            try_recover_clamp_in_expr(&mut f.item);
        }
        AstExpression::Call(call) => {
            let args = match call {
                AstCall::Unknown(_, args) | AstCall::Builtin(_, args) => args,
                _ => return,
            };
            for arg in args {
                try_recover_clamp_in_expr(&mut arg.item);
            }
        }
        _ => {}
    }

    // Pattern: min(max(x, low), high)
    if let AstExpression::Call(AstCall::Unknown(name, args)) = expr {
        if name == "min" && args.len() == 2 {
            // Check if one of the args is a max call
            let (inner_max, high_arg) = if let AstExpression::Call(AstCall::Unknown(n2, _)) = &args[0].item {
                if n2 == "max" { (&mut args[0].item, &args[1].item) } else { (&mut args[1].item, &args[0].item) }
            } else {
                (&mut args[1].item, &args[0].item)
            };

            if let AstExpression::Call(AstCall::Unknown(n2, inner_args)) = inner_max {
                if n2 == "max" && inner_args.len() == 2 {
                    let x = inner_args[0].clone();
                    let low = inner_args[1].clone();
                    let high = high_arg.clone();

                    *expr = AstExpression::Call(AstCall::Unknown("clamp".to_string(), vec![x, low, high]));
                }
            }
        }
    }
}
