mod convert;

use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstDescriptor, AstExpression, AstFunctionId, AstFunctionVersion,
        AstLiteral, AstStatement, AstStatementOrigin, AstUnaryOperator, AstValue, AstValueOrigin,
        AstValueType, AstVariable, AstVariableId, PrintWithConfig, ProcessedOptimization, Wrapped,
        WrappedAstStatement,
        optimize::ir_analyzation::convert::{
            convert_expr, convert_stmt, resolve_constant, wdn, ws,
        },
    },
    ir::{
        analyze::{DataType, variables::resolve_operand},
        data::IrData,
        statements::{IrStatement, IrStatementSpecial},
    },
    prelude::{DecompileError, *},
    utils::Aos,
};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

/// Generate Ast function body with given ir function
pub(super) fn analyze_ir_function(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let ir_function;
    let mut body;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();

        // if analyzed, pass
        if function
            .processed_optimizations
            .contains(&ProcessedOptimization::IrAnalyzation)
        {
            return Ok(());
        }

        body = std::mem::take(&mut function.body);
        ir_function = function.ir.clone();
    }

    let mut locals = HashMap::new();
    let mut var_map: HashMap<Aos<IrData>, AstVariableId> = HashMap::new();
    for var in ir_function.get_variables().iter() {
        let var_id = ast.new_variable_id(&function_id);
        let mut c_type = match var.data_type {
            DataType::Unknown => AstValueType::Unknown,
            DataType::Bool => AstValueType::Bool,
            DataType::Int => AstValueType::Int,
            DataType::Float => AstValueType::Double,
            DataType::StringPointer => AstValueType::Pointer(Box::new(AstValueType::Char)),
            DataType::Char => AstValueType::Char,
            DataType::Address => AstValueType::Pointer(Box::new(AstValueType::Void)),
        };
        let mut const_value: Option<Wrapped<AstValue>> = None;
        let mut accesses_by_position: Vec<_> = var.get_data_accesses().iter().collect();
        accesses_by_position.sort_unstable_by_key(|(position, _)| position.to_u64());
        for (position, accesses) in accesses_by_position {
            let instruction_arg_size = ir_function.get_instructions()[position.ir_index() as usize]
                .inner
                .arguments
                .len() as u8;
            let instruction_byte_size = ir_function.get_instructions()
                [position.ir_index() as usize]
                .inner
                .bytes
                .as_ref()
                .map(|x| x.len() as u8)
                .unwrap_or(0);
            let position = &ir_function.get_ir()[position.ir_index() as usize].address;
            for da in accesses.iter() {
                var_map.insert(da.location().clone(), var_id);
                // Resolve constant value
                if let Some(c) = resolve_constant(
                    position,
                    instruction_arg_size,
                    instruction_byte_size,
                    &da.location(),
                    &da.location(),
                )? {
                    trace!(
                        "Constant value found in {}: {}",
                        position,
                        c.to_string_with_config(None)
                    );
                    if c_type == AstValueType::Unknown {
                        c_type = match &c.item {
                            AstValue::Void => AstValueType::Void,
                            AstValue::Unknown => AstValueType::Unknown,
                            AstValue::Undefined => AstValueType::Unknown,
                            AstValue::Max => AstValueType::Int,
                            AstValue::Min => AstValueType::Int,
                            AstValue::Num(_) => AstValueType::Int,
                            AstValue::Char(_) => AstValueType::Char,
                            AstValue::Double(_) => AstValueType::Double,
                            AstValue::Bool(_) => AstValueType::Bool,
                            AstValue::Pointer(_) | AstValue::Array(_) => {
                                AstValueType::Pointer(Box::new(AstValueType::Void))
                            }
                        };
                        debug!(
                            "Constant value found in {}({}) but datatype not set. init datatype to {}",
                            position,
                            c.to_string_with_config(None),
                            c_type.to_string_with_config(None)
                        );
                    }
                    if const_value.is_some() && const_value.as_ref().unwrap() != &c {
                        warn!(
                            "Constant value mismatch in position {}: {} != {}",
                            position,
                            const_value.as_ref().unwrap().to_string_with_config(None),
                            c.to_string_with_config(None)
                        );
                        debug_assert!(
                            false,
                            "Constant value mismatch in position {}: {} != {}",
                            position,
                            const_value.unwrap().to_string_with_config(None),
                            c.to_string_with_config(None)
                        );
                    }
                    const_value = Some(c);
                }
            }
        }
        locals.insert(
            var_id,
            AstVariable {
                name: None,
                id: var_id,
                var_type: c_type,
                const_value,
                data_access_ir: Some(var.get_data_accesses().clone()),
            },
        );
    }
    ast.functions
        .write()
        .unwrap()
        .get_mut(&function_id)
        .and_then(|x| x.get_mut(&function_version))
        .unwrap()
        .variables = Arc::new(RwLock::new(locals));

    let map = ir_function.get_instructions().as_ref();

    // Phase 1: Pre-scan IR statements to identify CalcFlagsAutomatically → Condition pairs.
    // Between them, only TypeSpecified, Assertion, and flag-writing Assignments (e.g. of=0
    // from `test`) are allowed — any other statement breaks the association.
    let mut condition_calc_map: HashMap<usize, (usize, Aos<IrData>)> = HashMap::new();
    {
        let mut last_calc: Option<(usize, Aos<IrData>, Vec<Aos<IrData>>)> = None;
        for (idx, item) in body.iter().enumerate() {
            let AstStatement::Ir(stmt) = &item.statement else {
                continue;
            };
            match stmt.as_ref() {
                IrStatement::Special(IrStatementSpecial::CalcFlagsAutomatically {
                    operation,
                    flags,
                    ..
                }) => {
                    // Resolve operand references using this instruction's args.
                    let AstStatementOrigin::Ir(pos) = &item.origin else {
                        continue;
                    };
                    let ir_idx = usize::try_from(pos.descriptor().ir_index())
                        .expect("does your architecture smaller than 32bit?");
                    let inst = &map[ir_idx];
                    let resolved_op = resolve_operand(operation, &inst.inner.arguments);
                    last_calc = Some((idx, resolved_op, flags.clone()));
                }
                IrStatement::Condition { condition, .. } => {
                    if let Some((calc_idx, ref calc_op, ref calc_flags)) = last_calc {
                        if condition_references_flags(condition, calc_flags) {
                            condition_calc_map.insert(idx, (calc_idx, calc_op.clone()));
                        }
                    }
                    last_calc = None;
                }
                // TypeSpecified and Assertion are inert metadata — safe to skip.
                IrStatement::Special(IrStatementSpecial::TypeSpecified { .. })
                | IrStatement::Special(IrStatementSpecial::Assertion { .. }) => {}
                // Assignments that write to a flag register (e.g. of=0 from `test`)
                // are safe separators; anything else breaks the association.
                IrStatement::Assignment { to, .. } => {
                    if !is_flag_register(to) {
                        last_calc = None;
                    }
                }
                _ => {
                    last_calc = None;
                }
            }
        }
    }

    // Collect CalcFlagsAutomatically indices that were successfully recovered,
    // so we can elide them after the conversion loop.
    let mut calc_to_elide: std::collections::HashSet<usize> = std::collections::HashSet::new();

    // Phase 2: Convert IR statements to AST, with condition recovery for matched pairs.
    for (idx, item) in body.iter_mut().enumerate() {
        let AstStatement::Ir(stmt) = &item.statement else {
            continue;
        };
        let AstStatementOrigin::Ir(stmt_position) = &item.origin else {
            continue;
        };

        let instruction = &map[usize::try_from(stmt_position.descriptor().ir_index())
            .expect("does your architecture smaller than 32bit?")];
        let instruction_args = &instruction.inner.arguments;

        // Try condition recovery for matched pairs.
        if let Some((calc_idx, calc_op)) = condition_calc_map.remove(&idx) {
            if let IrStatement::Condition {
                condition,
                true_branch,
                false_branch,
            } = stmt.as_ref()
            {
                if let Some(mut recovered) = try_recover_condition_from_ir(
                    ast,
                    function_id,
                    function_version,
                    condition,
                    true_branch,
                    false_branch,
                    &calc_op,
                    stmt_position,
                    &var_map,
                    instruction_args,
                )? {
                    recovered.comment = item.comment.clone();
                    *item = recovered;
                    // Only mark CalcFlagsAutomatically for elision after successful recovery.
                    calc_to_elide.insert(calc_idx);
                    continue;
                }
            }
        }

        /* analyze and turn into ast */
        let mut converted = convert_stmt(
            ast,
            function_id,
            function_version,
            stmt,
            stmt_position,
            None,
            &var_map,
            instruction_args,
        )?;
        converted.comment = item.comment.clone();
        *item = converted;
    }

    // Phase 3: Elide CalcFlagsAutomatically blocks whose semantics were folded
    // into recovered conditions.
    for idx in calc_to_elide {
        body[idx].statement = AstStatement::Empty;
    }

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::IrAnalyzation);
    }
    Ok(())
}

/// Check if an IR data expression is a flag register (of, sf, zf, af, cf, pf).
fn is_flag_register(data: &Aos<IrData>) -> bool {
    if let IrData::Register(reg) = data.as_ref() {
        matches!(reg.name(), "of" | "sf" | "zf" | "af" | "cf" | "pf")
    } else {
        false
    }
}

/// Check if a Condition's IR expression tree references any flag register from
/// the given CalcFlagsAutomatically affected registers list.
fn condition_references_flags(condition: &Aos<IrData>, calc_flags: &[Aos<IrData>]) -> bool {
    match condition.as_ref() {
        IrData::Register(_) => calc_flags.iter().any(|f| f == condition),
        IrData::Operation(op) => match op {
            crate::ir::data::IrDataOperation::Unary { arg, .. } => {
                condition_references_flags(arg, calc_flags)
            }
            crate::ir::data::IrDataOperation::Binary { arg1, arg2, .. } => {
                condition_references_flags(arg1, calc_flags)
                    || condition_references_flags(arg2, calc_flags)
            }
        },
        _ => false,
    }
}

/// Attempt to recover a high-level comparison from an IR-level Condition paired
/// with a CalcFlagsAutomatically operation.
///
/// Returns `Ok(Some(recovered_if))` on success, `Ok(None)` if the pattern is
/// unrecognized (fall through to normal conversion), or `Err` on conversion failure.
fn try_recover_condition_from_ir(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    condition: &Aos<IrData>,
    true_branch: &[IrStatement],
    false_branch: &[IrStatement],
    calc_operation: &Aos<IrData>,
    stmt_position: &AstDescriptor,
    var_map: &HashMap<Aos<IrData>, AstVariableId>,
    instruction_args: &[iceball::Argument],
) -> Result<Option<WrappedAstStatement>, DecompileError> {
    // Convert the CalcFlags operation to an AST expression for use in comparisons.
    let operation_expr = convert_expr(
        ast,
        function_id,
        function_version,
        calc_operation,
        calc_operation,
        var_map,
    )?;

    // Try to build a recovered comparison from the IR condition pattern.
    let Some(recovered_cond) = recover_ir_condition(condition, &operation_expr) else {
        return Ok(None);
    };

    // Convert true/false branches normally.
    let then_b = true_branch
        .iter()
        .map(|s| {
            convert_stmt(
                ast,
                function_id,
                function_version,
                s,
                stmt_position,
                None,
                var_map,
                instruction_args,
            )
        })
        .collect::<Result<_, _>>()?;
    let else_b = false_branch
        .iter()
        .map(|s| {
            convert_stmt(
                ast,
                function_id,
                function_version,
                s,
                stmt_position,
                None,
                var_map,
                instruction_args,
            )
        })
        .collect::<Result<_, _>>()?;

    Ok(Some(ws(
        AstStatement::If(wdn(recovered_cond), then_b, Some(else_b)),
        stmt_position.clone(),
    )))
}

/// Map an IR condition expression (built from flag registers) and a CalcFlags
/// operation (the computed value) into a high-level comparison expression.
///
/// The condition is the `Aos<IrData>` from `IrStatement::Condition`, e.g.:
///   - `Register(zf)` for je
///   - `Not(Register(zf))` for jne
///   - `And(Not(cf), Not(zf))` for ja
///   - `Equal(sf, of)` for jge
///
/// The operation_expr is the AST-converted CalcFlags operation (e.g., `a - b`
/// for cmp, `a & b` for test).
fn recover_ir_condition(
    condition: &Aos<IrData>,
    operation_expr: &Wrapped<AstExpression>,
) -> Option<AstExpression> {
    use crate::ir::{
        data::IrDataOperation,
        operator::{IrBinaryOperator as IrBinOp, IrUnaryOperator as IrUnOp},
    };

    match condition.as_ref() {
        // Bare flag register (e.g., zf for je, cf for jb, sf for js)
        IrData::Register(reg) => recover_single_flag(reg.name(), operation_expr, false),

        IrData::Operation(op) => match op {
            // NOT of a flag (e.g., !zf for jne, !cf for jae)
            IrDataOperation::Unary {
                operator: IrUnOp::Not,
                arg,
            } => match arg.as_ref() {
                IrData::Register(reg) => recover_single_flag(reg.name(), operation_expr, true),
                // NOT of compound (e.g., !(sf == of) for jl)
                _ => {
                    let inner = recover_ir_condition(arg, operation_expr)?;
                    Some(AstExpression::UnaryOp(
                        AstUnaryOperator::Not,
                        Box::new(wrap_expr(inner)),
                    ))
                }
            },

            // AND of flag expressions (e.g., !cf & !zf for ja, !zf & (sf==of) for jg)
            IrDataOperation::Binary {
                operator: IrBinOp::And,
                arg1,
                arg2,
            } => {
                let lhs = recover_ir_condition(arg1, operation_expr)?;
                let rhs = recover_ir_condition(arg2, operation_expr)?;
                Some(AstExpression::BinaryOp(
                    AstBinaryOperator::LogicAnd,
                    Box::new(wrap_expr(lhs)),
                    Box::new(wrap_expr(rhs)),
                ))
            }

            // OR of flag expressions (e.g., cf | zf for jbe, zf | !(sf==of) for jle)
            IrDataOperation::Binary {
                operator: IrBinOp::Or,
                arg1,
                arg2,
            } => {
                let lhs = recover_ir_condition(arg1, operation_expr)?;
                let rhs = recover_ir_condition(arg2, operation_expr)?;
                Some(AstExpression::BinaryOp(
                    AstBinaryOperator::LogicOr,
                    Box::new(wrap_expr(lhs)),
                    Box::new(wrap_expr(rhs)),
                ))
            }

            // Equal of two flags (e.g., sf == of for jge)
            IrDataOperation::Binary {
                operator: IrBinOp::Equal(..),
                arg1,
                arg2,
            } => recover_flag_equality_ir(arg1, arg2, operation_expr),

            _ => None,
        },

        _ => None,
    }
}

/// Recover a comparison from a single flag's semantics.
/// `negated` is true when the flag is used with NOT (e.g., `!zf`).
fn recover_single_flag(
    flag_name: &str,
    operation: &Wrapped<AstExpression>,
    negated: bool,
) -> Option<AstExpression> {
    match flag_name {
        "zf" => {
            // ZF = (operation == 0). So: zf → op == 0, !zf → op != 0
            let cmp_op = if negated {
                AstBinaryOperator::NotEqual
            } else {
                AstBinaryOperator::Equal
            };
            // Optimization: if op is `a & a` (test a, a) or `a - a`, simplify to compare a with 0
            if let Some((op_name, lhs, rhs)) = decompose_operation(&operation.item) {
                if op_name == "and" || op_name == "sub" {
                    if let Some(simplified) = simplify_self_operation(&lhs, &rhs, &cmp_op) {
                        return Some(simplified);
                    }
                }
                // For sub (cmp a, b): zf → a == b, !zf → a != b
                if op_name == "sub" {
                    return Some(AstExpression::BinaryOp(
                        cmp_op,
                        Box::new(wrap_expr(lhs)),
                        Box::new(wrap_expr(rhs)),
                    ));
                }
            }
            // General case: compare the full operation result against 0
            Some(AstExpression::BinaryOp(
                cmp_op,
                Box::new(operation.clone()),
                Box::new(wrap_expr(AstExpression::Literal(AstLiteral::Int(0)))),
            ))
        }
        "sf" => {
            // SF = (operation < 0). So: sf → op < 0, !sf → op >= 0
            let cmp_op = if negated {
                AstBinaryOperator::GreaterEqual
            } else {
                AstBinaryOperator::Less
            };
            Some(AstExpression::BinaryOp(
                cmp_op,
                Box::new(operation.clone()),
                Box::new(wrap_expr(AstExpression::Literal(AstLiteral::Int(0)))),
            ))
        }
        "cf" => {
            // CF for sub (cmp): CF = (a < b) unsigned.
            let cmp_op = if negated {
                AstBinaryOperator::GreaterEqual
            } else {
                AstBinaryOperator::Less
            };
            // For cmp (sub), CF means unsigned less-than
            Some(AstExpression::BinaryOp(
                cmp_op,
                Box::new(wrap_expr(AstExpression::UnaryOp(
                    AstUnaryOperator::CastUnsigned,
                    Box::new(operation.clone()),
                ))),
                Box::new(wrap_expr(AstExpression::Literal(AstLiteral::Int(0)))),
            ))
        }
        // OF, AF, PF: too complex or rarely used as direct condition. Skip.
        _ => None,
    }
}

/// Recover from IR-level sf == of or sf != of (used by jge, jl, etc.)
fn recover_flag_equality_ir(
    lhs: &Aos<IrData>,
    rhs: &Aos<IrData>,
    operation: &Wrapped<AstExpression>,
) -> Option<AstExpression> {
    let (lhs_name, rhs_name) = match (lhs.as_ref(), rhs.as_ref()) {
        (IrData::Register(l), IrData::Register(r)) => (l.name(), r.name()),
        _ => return None,
    };

    // sf == of → signed >=, relative to the operation source
    if (lhs_name == "sf" && rhs_name == "of") || (lhs_name == "of" && rhs_name == "sf") {
        // The operation is typically `a - b` from cmp. sf == of means a >= b (signed).
        let (op_name, lhs_operand, rhs_operand) = decompose_operation(&operation.item)?;
        if op_name == "sub" {
            return Some(AstExpression::BinaryOp(
                AstBinaryOperator::GreaterEqual,
                Box::new(wrap_expr(lhs_operand)),
                Box::new(wrap_expr(rhs_operand)),
            ));
        }
    }

    None
}

/// Decompose a binary operation expression into (operation_name, lhs, rhs).
fn decompose_operation(
    expr: &AstExpression,
) -> Option<(&'static str, AstExpression, AstExpression)> {
    match expr {
        AstExpression::BinaryOp(op, lhs, rhs) => {
            let name = match op {
                AstBinaryOperator::Add => "add",
                AstBinaryOperator::Sub => "sub",
                AstBinaryOperator::BitAnd => "and",
                AstBinaryOperator::BitOr => "or",
                AstBinaryOperator::BitXor => "xor",
                _ => "other",
            };
            Some((name, lhs.item.clone(), rhs.item.clone()))
        }
        // For unary ops like CastSigned wrapping a binary op, unwrap
        AstExpression::UnaryOp(AstUnaryOperator::CastSigned, inner) => {
            decompose_operation(&inner.item)
        }
        _ => None,
    }
}

/// If both operands of `a OP a` are structurally the same variable, simplify
/// to `a CMP 0` (for test/cmp self patterns).
fn simplify_self_operation(
    lhs: &AstExpression,
    rhs: &AstExpression,
    cmp_op: &AstBinaryOperator,
) -> Option<AstExpression> {
    // Check if lhs and rhs are the same variable
    match (lhs, rhs) {
        (AstExpression::Variable(_, l_id), AstExpression::Variable(_, r_id)) if l_id == r_id => {
            // `a & a` → test a against 0, or `a - a` → compare a to itself (always 0)
            // For `and`: comparing the operand against 0
            // For `sub`: the result is always 0, but the pattern from cmp is `a - b` not `a - a`
            Some(AstExpression::BinaryOp(
                cmp_op.clone(),
                Box::new(wrap_expr(lhs.clone())),
                Box::new(wrap_expr(AstExpression::Literal(AstLiteral::Int(0)))),
            ))
        }
        // Also handle: CastSigned(var) or CastUnsigned(var) matching against bare var
        _ => {
            let l_stripped = strip_cast(lhs);
            let r_stripped = strip_cast(rhs);
            match (l_stripped, r_stripped) {
                (AstExpression::Variable(_, l_id), AstExpression::Variable(_, r_id))
                    if l_id == r_id =>
                {
                    Some(AstExpression::BinaryOp(
                        cmp_op.clone(),
                        Box::new(wrap_expr(l_stripped.clone())),
                        Box::new(wrap_expr(AstExpression::Literal(AstLiteral::Int(0)))),
                    ))
                }
                _ => None,
            }
        }
    }
}

/// Strip CastSigned / CastUnsigned / ZeroExtend from an expression.
fn strip_cast(expr: &AstExpression) -> &AstExpression {
    match expr {
        AstExpression::UnaryOp(
            AstUnaryOperator::CastSigned | AstUnaryOperator::CastUnsigned,
            inner,
        ) => strip_cast(&inner.item),
        _ => expr,
    }
}

/// Wrap an expression in a Wrapped with Unknown origin.
fn wrap_expr(item: AstExpression) -> Wrapped<AstExpression> {
    Wrapped {
        item,
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}
