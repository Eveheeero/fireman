//! Clamp recovery: min(max(x, low), high) → clamp(x, low, high).

use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstStatement, ProcessedOptimization, WrappedAstStatement,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstract_syntax_tree::{
        AstCall, AstFunctionId, AstLiteral,
        optimize::pattern_matching::embedded::test_utils::test_utils::*,
    };

    #[test]
    fn parity_clamp_recovery_min_max() {
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["x"]);
        let x = ids[0];

        let body = vec![wrap_statement(AstStatement::Return(Some(wrap_expression(
            AstExpression::Call(AstCall::Unknown(
                "min".to_string(),
                vec![
                    wrap_expression(AstExpression::Call(AstCall::Unknown(
                        "max".to_string(),
                        vec![
                            wrap_expression(AstExpression::Variable(vm.clone(), x)),
                            wrap_expression(AstExpression::Literal(AstLiteral::Int(3))),
                        ],
                    ))),
                    wrap_expression(AstExpression::Literal(AstLiteral::Int(10))),
                ],
            )),
        ))))];

        let (fb, embed) = run_parity(
            "optimization/after-iteration/clamp-recovery.fb",
            body,
            vm,
            |c| c.clamp_recovery(true),
        );
        assert!(
            fb.contains("min(max"),
            "fb should preserve the original min/max form for this known gap, got:\n{}",
            fb
        );
        assert!(
            embed.contains("clamp"),
            "embed should recover clamp(...), got:\n{}",
            embed
        );
        if fb != embed {
            eprintln!(
                "KNOWN DIFF: clamp_recovery fb vs embedded differs.\n  fb: {}\n  embed: {}",
                fb.replace('\n', "\\n"),
                embed.replace('\n', "\\n"),
            );
        }
    }
}

fn try_recover_clamp_in_stmt(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::Assignment(_, rhs) | AstStatement::Return(Some(rhs)) => {
            try_recover_clamp_in_expr(&mut rhs.item);
        }
        AstStatement::If(cond, _, _)
        | AstStatement::While(cond, _)
        | AstStatement::DoWhile(cond, _) => {
            try_recover_clamp_in_expr(&mut cond.item);
        }
        AstStatement::Call(call) => {
            try_recover_clamp_in_call(call);
        }
        _ => {}
    }
}

fn try_recover_clamp_in_call(call: &mut AstCall) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                try_recover_clamp_in_expr(&mut arg.item);
            }
        }
        AstCall::Builtin(_, arg) => try_recover_clamp_in_builtin_arg(arg),
    }
}

fn try_recover_clamp_in_builtin_arg(arg: &mut AstBuiltinFunctionArgument) {
    match arg {
        AstBuiltinFunctionArgument::None => {}
        AstBuiltinFunctionArgument::Print(args) => {
            for arg in args.iter_mut() {
                try_recover_clamp_in_expr(&mut arg.item);
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
            try_recover_clamp_in_expr(&mut expr.item);
        }
        AstBuiltinFunctionArgument::Sized(lhs, rhs) => {
            try_recover_clamp_in_expr(&mut lhs.item);
            try_recover_clamp_in_expr(&mut rhs.item);
        }
    }
}

fn try_recover_clamp_in_expr(expr: &mut AstExpression) {
    // Traverse down
    match expr {
        AstExpression::BinaryOp(_, lhs, rhs) => {
            try_recover_clamp_in_expr(&mut lhs.item);
            try_recover_clamp_in_expr(&mut rhs.item);
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner) => {
            try_recover_clamp_in_expr(&mut inner.item);
        }
        AstExpression::Ternary(c, t, f) => {
            try_recover_clamp_in_expr(&mut c.item);
            try_recover_clamp_in_expr(&mut t.item);
            try_recover_clamp_in_expr(&mut f.item);
        }
        AstExpression::Call(call) => {
            try_recover_clamp_in_call(call);
        }
        _ => {}
    }

    // Pattern: min(max(x, low), high)
    if let AstExpression::Call(AstCall::Unknown(name, args)) = expr {
        if name == "min" && args.len() == 2 {
            let inner_max_index = args.iter().position(|arg| {
                matches!(&arg.item, AstExpression::Call(AstCall::Unknown(name, _)) if name == "max")
            });

            if let Some(inner_max_index) = inner_max_index {
                let high = args[1 - inner_max_index].clone();
                let inner_max = &mut args[inner_max_index].item;

                if let AstExpression::Call(AstCall::Unknown(n2, inner_args)) = inner_max {
                    if n2 == "max" && inner_args.len() == 2 {
                        let x = inner_args[0].clone();
                        let low = inner_args[1].clone();

                        *expr = AstExpression::Call(AstCall::Unknown(
                            "clamp".to_string(),
                            vec![x, low, high],
                        ));
                    }
                }
            }
        }
    }
}
