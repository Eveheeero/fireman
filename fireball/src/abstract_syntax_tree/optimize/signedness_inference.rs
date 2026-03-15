//! Infer signed/unsigned integer types from operator usage context.

use crate::{
    abstract_syntax_tree::{
        Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstStatement, AstUnaryOperator, AstValueType, AstVariableId, ProcessedOptimization,
        Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::HashMap;

/// Evidence collected about a variable's signedness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SignEvidence {
    Signed,
    Unsigned,
}

pub(super) fn infer_signedness(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let body;
    let var_map;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
        var_map = function.variables.clone();
    }

    // Phase 1: Collect signedness evidence for each variable.
    let mut evidence: HashMap<AstVariableId, Vec<SignEvidence>> = HashMap::new();

    // Gather evidence from existing variable types.
    {
        let vars = var_map.read().unwrap();
        for (var_id, var) in vars.iter() {
            let ev = match &var.var_type {
                AstValueType::Int8
                | AstValueType::Int16
                | AstValueType::Int32
                | AstValueType::Int64 => Some(SignEvidence::Signed),
                AstValueType::UInt8
                | AstValueType::UInt16
                | AstValueType::UInt32
                | AstValueType::UInt64 => Some(SignEvidence::Unsigned),
                _ => None,
            };
            if let Some(ev) = ev {
                evidence.entry(*var_id).or_default().push(ev);
            }
        }
    }

    // Walk the body and collect evidence from expressions.
    collect_evidence_from_statement_list(&body, &mut evidence);

    // Phase 2: Apply unanimous evidence to variables with generic `Int` type.
    {
        let mut vars = var_map.write().unwrap();
        for (var_id, evidences) in evidence.iter() {
            if evidences.is_empty() {
                continue;
            }
            let Some(var) = vars.get_mut(var_id) else {
                continue;
            };
            // Only refine variables with the generic `Int` type.
            if var.var_type != AstValueType::Int {
                continue;
            }
            let first = evidences[0];
            let unanimous = evidences.iter().all(|e| *e == first);
            if !unanimous {
                continue;
            }
            // Evidence is unanimous -- refine the type.  Since `Int` has no
            // explicit size we keep it size-generic: `Int` -> `Int` (signed,
            // already the default) or `Int` -> `UInt`.
            match first {
                SignEvidence::Unsigned => {
                    var.var_type = AstValueType::UInt;
                }
                SignEvidence::Signed => {
                    // Already `Int` which is the signed generic -- no change needed.
                }
            }
        }
    }

    // Write the body back and record the optimization.
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::SignednessInference);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Evidence collection -- recursive walkers
// ---------------------------------------------------------------------------

fn collect_evidence_from_statement_list(
    stmts: &[WrappedAstStatement],
    evidence: &mut HashMap<AstVariableId, Vec<SignEvidence>>,
) {
    for stmt in stmts.iter() {
        collect_evidence_from_statement(stmt, evidence);
    }
}

fn collect_evidence_from_statement(
    stmt: &WrappedAstStatement,
    evidence: &mut HashMap<AstVariableId, Vec<SignEvidence>>,
) {
    match &stmt.statement {
        AstStatement::Declaration(_var, rhs) => {
            if let Some(rhs) = rhs {
                collect_evidence_from_expression(rhs, evidence);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            collect_evidence_from_expression(lhs, evidence);
            collect_evidence_from_expression(rhs, evidence);
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            collect_evidence_from_expression(cond, evidence);
            collect_evidence_from_statement_list(branch_true, evidence);
            if let Some(branch_false) = branch_false {
                collect_evidence_from_statement_list(branch_false, evidence);
            }
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            collect_evidence_from_expression(cond, evidence);
            collect_evidence_from_statement_list(body, evidence);
        }
        AstStatement::For(init, cond, update, body) => {
            collect_evidence_from_statement(init, evidence);
            collect_evidence_from_expression(cond, evidence);
            collect_evidence_from_statement(update, evidence);
            collect_evidence_from_statement_list(body, evidence);
        }
        AstStatement::Switch(discrim, cases, default) => {
            collect_evidence_from_expression(discrim, evidence);
            for (_lit, case_body) in cases.iter() {
                collect_evidence_from_statement_list(case_body, evidence);
            }
            if let Some(default_body) = default {
                collect_evidence_from_statement_list(default_body, evidence);
            }
        }
        AstStatement::Block(body) => {
            collect_evidence_from_statement_list(body, evidence);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                collect_evidence_from_expression(expr, evidence);
            }
        }
        AstStatement::Call(call) => {
            collect_evidence_from_call(call, evidence);
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

fn collect_evidence_from_call(
    call: &AstCall,
    evidence: &mut HashMap<AstVariableId, Vec<SignEvidence>>,
) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args.iter() {
                collect_evidence_from_expression(arg, evidence);
            }
        }
        AstCall::Builtin(_, args) => match args.as_ref() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(items) => {
                for item in items.iter() {
                    collect_evidence_from_expression(item, evidence);
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
                collect_evidence_from_expression(expr, evidence);
            }
            AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
                collect_evidence_from_expression(expr1, evidence);
                collect_evidence_from_expression(expr2, evidence);
            }
        },
    }
}

fn collect_evidence_from_expression(
    expr: &Wrapped<AstExpression>,
    evidence: &mut HashMap<AstVariableId, Vec<SignEvidence>>,
) {
    match &expr.item {
        // CastSigned / CastUnsigned applied directly to a variable is strong
        // evidence of the variable's signedness.
        AstExpression::UnaryOp(op, arg) => {
            let sign_ev = match op {
                AstUnaryOperator::CastSigned => Some(SignEvidence::Signed),
                AstUnaryOperator::CastUnsigned => Some(SignEvidence::Unsigned),
                _ => None,
            };
            if let Some(ev) = sign_ev {
                collect_variable_ids_with_evidence(arg, ev, evidence);
            }
            collect_evidence_from_expression(arg, evidence);
        }
        AstExpression::BinaryOp(op, left, right) => {
            // NOTE: At the AST level, `AstBinaryOperator` does not distinguish
            // signed vs unsigned comparisons or arithmetic vs logical right
            // shifts.  The IR-level signed comparisons (`SignedLess`, etc.) and
            // shift variants are lowered to generic `Less` / `RightShift` by the
            // time code reaches the AST.  If signed/unsigned operator variants
            // are added to `AstBinaryOperator` in the future, evidence collection
            // should be extended here.
            //
            // For now, no additional evidence is emitted from binary operators
            // beyond recursing into their operands.
            let _ = op;
            collect_evidence_from_expression(left, evidence);
            collect_evidence_from_expression(right, evidence);
        }
        AstExpression::Call(call) => {
            collect_evidence_from_call(call, evidence);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            collect_evidence_from_expression(arg, evidence);
        }
        AstExpression::ArrayAccess(base, idx) => {
            collect_evidence_from_expression(base, evidence);
            collect_evidence_from_expression(idx, evidence);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            collect_evidence_from_expression(cond, evidence);
            collect_evidence_from_expression(true_expr, evidence);
            collect_evidence_from_expression(false_expr, evidence);
        }
        AstExpression::Literal(_)
        | AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => {}
    }
}

/// Record `ev` for every variable referenced (directly) in `expr`.
fn collect_variable_ids_with_evidence(
    expr: &Wrapped<AstExpression>,
    ev: SignEvidence,
    evidence: &mut HashMap<AstVariableId, Vec<SignEvidence>>,
) {
    if let AstExpression::Variable(_, var_id) = &expr.item {
        evidence.entry(*var_id).or_default().push(ev);
    }
}
