use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstBuiltinFunction, AstBuiltinFunctionArgument, AstCall,
        AstDescriptor, AstExpression, AstFunctionId, AstFunctionVersion, AstJumpTarget, AstLiteral,
        AstStatement, AstStatementOrigin, AstUnaryOperator, AstValue, AstValueOrigin,
        AstValueType, AstVariableId, PrintWithConfig, Wrapped, WrappedAstStatement,
    },
    core::Address,
    ir::{
        analyze::variables::resolve_operand,
        data::{IrAccessSize, IrData, IrDataOperation, IrIntrinsic, NumCondition},
        operator::{IrBinaryOperator as IrBinaryOp, IrUnaryOperator as IrUnaryOp},
        statements::{IrStatement, IrStatementSpecial},
    },
    prelude::*,
    utils::Aos,
};
use hashbrown::HashMap;
use num_bigint::BigInt;

/// Wrap Statement
pub(super) fn ws(statement: AstStatement, from: AstDescriptor) -> WrappedAstStatement {
    WrappedAstStatement {
        statement,
        origin: AstStatementOrigin::Ir(from),
        comment: None,
    }
}
/// Wrap Data
pub(super) fn wd<T>(item: T, origin_expr: &Aos<IrData>) -> Wrapped<T> {
    Wrapped {
        item,
        origin: AstValueOrigin::Expression(origin_expr.clone()),
        comment: None,
    }
}
pub(super) fn wdn<T>(item: T) -> Wrapped<T> {
    Wrapped {
        item,
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

pub(super) fn convert_expr(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    root_expr: &Aos<IrData>,
    data: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, AstVariableId>,
) -> Result<Wrapped<AstExpression>, DecompileError> {
    let w = |x: AstExpression| wd(x, root_expr);

    if let Some(&vid) = var_map.get(data) {
        let vars = ast.get_variables(&function_id, &function_version).unwrap();
        return Ok(w(AstExpression::Variable(vars, vid)));
    }

    let result = match data.as_ref() {
        IrData::Constant(c) => AstExpression::Literal(AstLiteral::Int(*c as i64)),
        IrData::Dereference(inner) => AstExpression::Deref(Box::new(convert_expr(
            ast,
            function_id,
            function_version,
            root_expr,
            inner,
            var_map,
        )?)),
        IrData::Intrinsic(intr) => match intr {
            IrIntrinsic::ArchitectureByteSize => AstExpression::ArchitectureByteSize,
            IrIntrinsic::ArchitectureBitSize => AstExpression::ArchitectureBitSize,
            IrIntrinsic::ArchitectureBitPerByte => AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::ArchBitPerByte,
                Box::new(AstBuiltinFunctionArgument::None),
            )),
            IrIntrinsic::InstructionByteSize => AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::InstructionByteSize,
                Box::new(AstBuiltinFunctionArgument::None),
            )),
            IrIntrinsic::ByteSizeOf(inner) => {
                let inner = convert_expr(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    inner,
                    var_map,
                )?;
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::ByteSizeOf,
                    Box::new(AstBuiltinFunctionArgument::ByteSizeOf(inner)),
                ))
            }
            IrIntrinsic::BitSizeOf(inner) => {
                let inner = convert_expr(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    inner,
                    var_map,
                )?;
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::BitSizeOf,
                    Box::new(AstBuiltinFunctionArgument::BitSizeOf(inner)),
                ))
            }
            IrIntrinsic::Sized(inner, size) => {
                let arg = convert_expr(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    inner,
                    var_map,
                )?;
                let sz =
                    convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::Sized,
                    Box::new(AstBuiltinFunctionArgument::Sized(arg, sz)),
                ))
            }
            IrIntrinsic::OperandExists(n) => {
                let n = w(AstExpression::Literal(AstLiteral::UInt(n.get() as u64)));
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::OperandExists,
                    Box::new(AstBuiltinFunctionArgument::OperandExists(n)),
                ))
            }
            IrIntrinsic::Unknown => AstExpression::Unknown,
            IrIntrinsic::Undefined => AstExpression::Undefined,
            IrIntrinsic::SignedMax(size) => {
                let sz =
                    convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::SignedMax,
                    Box::new(AstBuiltinFunctionArgument::SignedMax(sz)),
                ))
            }
            IrIntrinsic::SignedMin(size) => {
                let sz =
                    convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::SignedMin,
                    Box::new(AstBuiltinFunctionArgument::SignedMin(sz)),
                ))
            }
            IrIntrinsic::UnsignedMax(size) => {
                let sz =
                    convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::UnsignedMax,
                    Box::new(AstBuiltinFunctionArgument::UnsignedMax(sz)),
                ))
            }
            IrIntrinsic::UnsignedMin(size) => {
                let sz =
                    convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::UnsignedMin,
                    Box::new(AstBuiltinFunctionArgument::UnsignedMin(sz)),
                ))
            }
            IrIntrinsic::BitOnes(size) => {
                let sz =
                    convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::BitOnes,
                    Box::new(AstBuiltinFunctionArgument::BitOnes(sz)),
                ))
            }
            IrIntrinsic::BitZeros(size) => {
                let sz =
                    convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
                AstExpression::Call(AstCall::Builtin(
                    AstBuiltinFunction::BitZeros,
                    Box::new(AstBuiltinFunctionArgument::BitZeros(sz)),
                ))
            }
            IrIntrinsic::ArchitectureByteSizeCondition(num_condition) => {
                let u = |v: &u16| AstExpression::Literal(AstLiteral::UInt(*v as u64));
                let result = match num_condition {
                    NumCondition::Higher(v) => {
                        let op = AstBinaryOperator::Greater;
                        let lhs = AstExpression::ArchitectureByteSize;
                        let rhs = u(v);
                        AstExpression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::HigherOrEqual(v) => {
                        let op = AstBinaryOperator::GreaterEqual;
                        let lhs = AstExpression::ArchitectureByteSize;
                        let rhs = u(v);
                        AstExpression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::Lower(v) => {
                        let op = AstBinaryOperator::Less;
                        let lhs = AstExpression::ArchitectureByteSize;
                        let rhs = u(v);
                        AstExpression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::LowerOrEqual(v) => {
                        let op = AstBinaryOperator::LessEqual;
                        let lhs = AstExpression::ArchitectureByteSize;
                        let rhs = u(v);
                        AstExpression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::Equal(v) => {
                        let op = AstBinaryOperator::Equal;
                        let lhs = AstExpression::ArchitectureByteSize;
                        let rhs = u(v);
                        AstExpression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::NotEqual(v) => {
                        let op = AstBinaryOperator::NotEqual;
                        let lhs = AstExpression::ArchitectureByteSize;
                        let rhs = u(v);
                        AstExpression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::RangeInclusive(start, end) => {
                        let start = u(start);
                        let end = u(end);
                        let arch_size = w(AstExpression::ArchitectureByteSize);
                        let op1 = AstExpression::BinaryOp(
                            AstBinaryOperator::GreaterEqual,
                            Box::new(arch_size.clone()),
                            Box::new(w(start)),
                        );
                        let op2 = AstExpression::BinaryOp(
                            AstBinaryOperator::LessEqual,
                            Box::new(arch_size),
                            Box::new(w(end)),
                        );
                        AstExpression::BinaryOp(
                            AstBinaryOperator::LogicAnd,
                            Box::new(w(op1)),
                            Box::new(w(op2)),
                        )
                    }
                    NumCondition::ExcludesRange(start, end) => {
                        let start = u(start);
                        let end = u(end);
                        let arch_size = w(AstExpression::ArchitectureByteSize);
                        let op1 = AstExpression::BinaryOp(
                            AstBinaryOperator::Less,
                            Box::new(arch_size.clone()),
                            Box::new(w(start)),
                        );
                        let op2 = AstExpression::BinaryOp(
                            AstBinaryOperator::Greater,
                            Box::new(arch_size),
                            Box::new(w(end)),
                        );
                        AstExpression::BinaryOp(
                            AstBinaryOperator::LogicOr,
                            Box::new(w(op1)),
                            Box::new(w(op2)),
                        )
                    }
                };
                result
            }
        },
        IrData::Operation(op) => match op {
            IrDataOperation::Unary { operator, arg } => {
                return convert_unary(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    operator,
                    arg,
                    var_map,
                );
            }
            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => {
                return convert_binary(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    operator,
                    arg1,
                    arg2,
                    var_map,
                );
            }
        },
        IrData::Register(_) | IrData::Operand(_) => AstExpression::Unknown,
    };
    Ok(w(result))
}

pub(super) fn convert_stmt(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    stmt: &IrStatement,
    stmt_position: &AstDescriptor,
    root_expr: Option<&Aos<IrData>>,
    var_map: &HashMap<Aos<IrData>, AstVariableId>,
    instruction_args: &[iceball::Argument],
) -> Result<WrappedAstStatement, DecompileError> {
    let result = match stmt {
        IrStatement::Assignment { from, to, size } => {
            let from = &resolve_operand(from, instruction_args);
            let to = &resolve_operand(to, instruction_args);
            let lhs = convert_expr(
                ast,
                function_id,
                function_version,
                root_expr.unwrap_or(to),
                to,
                var_map,
            )?;
            let mut rhs = convert_expr(
                ast,
                function_id,
                function_version,
                root_expr.unwrap_or(to),
                from,
                var_map,
            )?;
            // Refine unsized CastSigned/CastUnsigned into explicit typed casts
            // using the assignment's target size.
            refine_extend_cast(&mut rhs, size, instruction_args);
            // Reject assignments to non-assignable LHS (e.g. literal targets
            // from unresolved IR data). Emit as a comment instead of producing
            // malformed AST like `0x0 = v31`.
            if is_assignable_lhs(&lhs.item) {
                AstStatement::Assignment(lhs, rhs)
            } else {
                use crate::abstract_syntax_tree::PrintWithConfig;
                let lhs_str = lhs.to_string_with_config(None);
                let rhs_str = rhs.to_string_with_config(None);
                AstStatement::Comment(format!("invalid assignment: {} = {}", lhs_str, rhs_str))
            }
        }
        IrStatement::JumpByCall { target } => {
            let target = &resolve_operand(target, instruction_args);
            let e = convert_expr(
                ast,
                function_id,
                function_version,
                root_expr.unwrap_or(target),
                target,
                var_map,
            )?;
            match e.as_ref() {
                AstExpression::Variable(vars, id) => AstStatement::Call(AstCall::Variable {
                    scope: function_id,
                    var_map: vars.clone(),
                    var_id: *id,
                    args: Vec::new(),
                }),
                _ => {
                    warn!("Uncovered call target");
                    let name = e.to_string_with_config(None);
                    AstStatement::Call(AstCall::Unknown(name, Vec::new()))
                }
            }
        }
        IrStatement::Jump { target } => {
            let target = &resolve_operand(target, instruction_args);
            let e = convert_expr(
                ast,
                function_id,
                function_version,
                root_expr.unwrap_or(target),
                target,
                var_map,
            )?;
            match e.as_ref() {
                AstExpression::Variable(vars, id) => AstStatement::Goto(AstJumpTarget::Variable {
                    scope: function_id,
                    var_map: vars.clone(),
                    var_id: *id,
                }),
                _ => {
                    warn!("Uncovered jump target");
                    let label = e.to_string_with_config(None);
                    AstStatement::Goto(AstJumpTarget::Unknown(label))
                }
            }
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            let condition = &resolve_operand(condition, instruction_args);
            let mut cond = convert_expr(
                ast,
                function_id,
                function_version,
                root_expr.unwrap_or(condition),
                condition,
                var_map,
            )?;
            // Evaluate OperandExists at conversion time since we have instruction arg count
            try_fold_operand_exists(&mut cond, instruction_args.len() as u8);
            // If the condition resolved to a variable with a boolean const_value, use it
            try_fold_bool_const_var(ast, function_id, function_version, &mut cond);
            let then_b = true_branch
                .iter()
                .map(|s| {
                    convert_stmt(
                        ast,
                        function_id,
                        function_version,
                        s,
                        stmt_position,
                        root_expr,
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
                        root_expr,
                        var_map,
                        instruction_args,
                    )
                })
                .collect::<Result<_, _>>()?;
            AstStatement::If(cond, then_b, Some(else_b))
        }
        IrStatement::Halt => AstStatement::Return(None),
        IrStatement::Undefined => AstStatement::Undefined,
        IrStatement::Exception(e) => AstStatement::Exception(e),
        IrStatement::Special(special) => match special {
            IrStatementSpecial::Assertion { .. } => AstStatement::Empty,
            IrStatementSpecial::CalcFlagsAutomatically {
                operation,
                size: _,
                flags,
            } => {
                let operation = &resolve_operand(operation, instruction_args);
                let stmts = calc_flags_automatically(
                    ast,
                    function_id,
                    function_version,
                    operation,
                    stmt_position,
                    root_expr.unwrap_or(operation),
                    flags,
                    var_map,
                )?;
                AstStatement::Block(stmts)
            }
            IrStatementSpecial::TypeSpecified {
                location: _,
                size: _,
                data_type: _,
            } => AstStatement::Empty, // Used to detect types
        },
    };
    Ok(ws(result, stmt_position.clone()))
}

pub(super) fn convert_unary(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    root_expr: &Aos<IrData>,
    operator: &IrUnaryOp,
    arg: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, AstVariableId>,
) -> Result<Wrapped<AstExpression>, DecompileError> {
    let w = |x: AstExpression| wd(x, root_expr);

    let expr = convert_expr(ast, function_id, function_version, root_expr, arg, var_map)?;
    let op = match operator {
        IrUnaryOp::Not => AstUnaryOperator::Not,
        IrUnaryOp::Negation => AstUnaryOperator::Negate,
        IrUnaryOp::SignExtend => AstUnaryOperator::CastSigned,
        IrUnaryOp::ZeroExtend => AstUnaryOperator::CastUnsigned,
    };
    Ok(w(AstExpression::UnaryOp(op, Box::new(expr))))
}

pub(super) fn convert_binary(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    root_expr: &Aos<IrData>,
    operator: &IrBinaryOp,
    arg1: &Aos<IrData>,
    arg2: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, AstVariableId>,
) -> Result<Wrapped<AstExpression>, DecompileError> {
    let w = |x: AstExpression| wd(x, root_expr);

    let lhs = convert_expr(ast, function_id, function_version, root_expr, arg1, var_map)?;
    let rhs = convert_expr(ast, function_id, function_version, root_expr, arg2, var_map)?;

    let result = match operator {
        IrBinaryOp::Add => {
            AstExpression::BinaryOp(AstBinaryOperator::Add, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Sub => {
            AstExpression::BinaryOp(AstBinaryOperator::Sub, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Mul => {
            AstExpression::BinaryOp(AstBinaryOperator::Mul, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::SignedDiv => AstExpression::BinaryOp(
            AstBinaryOperator::Div,
            Box::new(lhs),
            Box::new(w(AstExpression::UnaryOp(
                AstUnaryOperator::CastSigned,
                Box::new(rhs),
            ))),
        ),
        IrBinaryOp::UnsignedDiv => AstExpression::BinaryOp(
            AstBinaryOperator::Div,
            Box::new(lhs),
            Box::new(w(AstExpression::UnaryOp(
                AstUnaryOperator::CastUnsigned,
                Box::new(rhs),
            ))),
        ),
        IrBinaryOp::SignedRem => AstExpression::BinaryOp(
            AstBinaryOperator::Mod,
            Box::new(lhs),
            Box::new(w(AstExpression::UnaryOp(
                AstUnaryOperator::CastSigned,
                Box::new(rhs),
            ))),
        ),
        IrBinaryOp::UnsignedRem => AstExpression::BinaryOp(
            AstBinaryOperator::Mod,
            Box::new(lhs),
            Box::new(w(AstExpression::UnaryOp(
                AstUnaryOperator::CastUnsigned,
                Box::new(rhs),
            ))),
        ),
        IrBinaryOp::And => {
            AstExpression::BinaryOp(AstBinaryOperator::BitAnd, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Or => {
            AstExpression::BinaryOp(AstBinaryOperator::BitOr, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Xor => {
            AstExpression::BinaryOp(AstBinaryOperator::BitXor, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Shl => {
            AstExpression::BinaryOp(AstBinaryOperator::LeftShift, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Shr | IrBinaryOp::Sar => {
            AstExpression::BinaryOp(AstBinaryOperator::RightShift, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Equal(size) => {
            let sz = convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
            let lhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(lhs.clone(), sz.clone())),
            ));
            let rhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(rhs.clone(), sz)),
            ));
            AstExpression::BinaryOp(
                AstBinaryOperator::Equal,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_s)),
            )
        }
        IrBinaryOp::SignedLess(size) => {
            let sz = convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
            let lhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(lhs.clone(), sz.clone())),
            )); // TODO does lhs need to be sized?
            let rhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(rhs.clone(), sz)),
            ));
            AstExpression::BinaryOp(
                AstBinaryOperator::Less,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_s)),
            )
        }
        IrBinaryOp::UnsignedLess(size) => {
            let sz = convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
            let lhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(lhs.clone(), sz.clone())),
            ));
            let rhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(rhs.clone(), sz)),
            ));
            let rhs_c = AstExpression::UnaryOp(AstUnaryOperator::CastUnsigned, Box::new(w(rhs_s)));
            AstExpression::BinaryOp(
                AstBinaryOperator::Less,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_c)),
            )
        }
        IrBinaryOp::SignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
            let lhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(lhs.clone(), sz.clone())),
            ));
            let rhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(rhs.clone(), sz)),
            ));
            AstExpression::BinaryOp(
                AstBinaryOperator::LessEqual,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_s)),
            )
        }
        IrBinaryOp::UnsignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
            let lhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(lhs.clone(), sz.clone())),
            ));
            let rhs_s = AstExpression::Call(AstCall::Builtin(
                AstBuiltinFunction::Sized,
                Box::new(AstBuiltinFunctionArgument::Sized(rhs.clone(), sz)),
            ));
            let rhs_c = AstExpression::UnaryOp(AstUnaryOperator::CastUnsigned, Box::new(w(rhs_s)));
            AstExpression::BinaryOp(
                AstBinaryOperator::LessEqual,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_c)),
            )
        }
    };
    Ok(w(result))
}

pub(super) fn convert_size(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    root_expr: &Aos<IrData>,
    size: &IrAccessSize,
    var_map: &HashMap<Aos<IrData>, AstVariableId>,
) -> Result<Wrapped<AstExpression>, DecompileError> {
    let w = |x: AstExpression| wd(x, root_expr);

    let result = match size {
        IrAccessSize::ResultOfBit(d)
        | IrAccessSize::ResultOfByte(d)
        | IrAccessSize::RelativeWith(d) => {
            return convert_expr(ast, function_id, function_version, root_expr, d, var_map);
        }
        IrAccessSize::ArchitectureSize => AstExpression::ArchitectureByteSize,
        IrAccessSize::Unlimited => AstExpression::Unknown,
    };
    Ok(w(result))
}

pub(super) fn calc_flags_automatically(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    operation: &Aos<IrData>,
    stmt_position: &AstDescriptor,
    root_expr: &Aos<IrData>,
    affected_registers: &[Aos<IrData>],
    var_map: &HashMap<Aos<IrData>, AstVariableId>,
) -> Result<Vec<WrappedAstStatement>, DecompileError> {
    let w = |x: AstExpression| wd(x, root_expr);

    // TODO INVALID
    let val = convert_expr(
        ast,
        function_id,
        function_version,
        root_expr,
        operation,
        var_map,
    )?;
    let vars = ast.get_variables(&function_id, &function_version).unwrap();
    let result = affected_registers
        .iter()
        .filter_map(|reg| {
            var_map.get(reg).map(|&vid| {
                AstStatement::Assignment(w(AstExpression::Variable(vars.clone(), vid)), val.clone())
            })
        })
        .map(|stmt| ws(stmt, stmt_position.clone()))
        .collect();
    Ok(result)
}

/// TODO Need implement for constant access size
pub(super) fn resolve_constant(
    position: &Address,
    instruction_arg_size: u8,
    instruction_byte_size: u8,
    root_expr: &Aos<IrData>,
    data: &Aos<IrData>,
) -> Result<Option<Wrapped<AstValue>>, DecompileError> {
    let w = |x: AstValue| wd(x, root_expr);

    let result = match data.as_ref() {
        IrData::Constant(c) => Some(AstValue::Num(BigInt::from(*c))),
        IrData::Intrinsic(i) => match i {
            IrIntrinsic::Unknown => Some(AstValue::Unknown),
            IrIntrinsic::Undefined => Some(AstValue::Undefined),
            IrIntrinsic::SignedMax(..) | IrIntrinsic::UnsignedMax(..) => Some(AstValue::Max),
            IrIntrinsic::SignedMin(..) | IrIntrinsic::UnsignedMin(..) => Some(AstValue::Min),
            IrIntrinsic::OperandExists(non_zero) => {
                Some(AstValue::Bool(non_zero.get() - 1 <= instruction_arg_size))
            }
            IrIntrinsic::ByteSizeOf(..)
            | IrIntrinsic::BitSizeOf(..)
            | IrIntrinsic::Sized(..)
            | IrIntrinsic::BitOnes(..)
            | IrIntrinsic::BitZeros(..)
            | IrIntrinsic::ArchitectureByteSize
            | IrIntrinsic::ArchitectureBitSize
            | IrIntrinsic::ArchitectureBitPerByte
            | IrIntrinsic::InstructionByteSize
            | IrIntrinsic::ArchitectureByteSizeCondition(..) => None,
        },
        IrData::Register(register) => match register.name() {
            "rip" | "eip" | "ip" => Some(AstValue::Num(BigInt::from(
                position.get_virtual_address() + u64::from(instruction_byte_size),
            ))),
            _ => None,
        },
        IrData::Dereference(data) => {
            let Some(c) = resolve_constant(
                position,
                instruction_arg_size,
                instruction_byte_size,
                root_expr,
                data,
            )?
            else {
                return Ok(None);
            };
            Some(AstValue::Pointer(Box::new(c)))
        }
        IrData::Operation(IrDataOperation::Unary { operator, arg }) => {
            let Some(arg) = resolve_constant(
                position,
                instruction_arg_size,
                instruction_byte_size,
                root_expr,
                arg,
            )?
            else {
                return Ok(None);
            };
            match operator {
                IrUnaryOp::Not => {
                    let Some(arg) = arg.bool() else {
                        return Ok(None);
                    };
                    Some(AstValue::Bool(!*arg))
                }
                IrUnaryOp::Negation => match arg.item {
                    AstValue::Max => Some(AstValue::Min),
                    AstValue::Min => Some(AstValue::Max),
                    AstValue::Num(v) => Some(AstValue::Num(-v)),
                    AstValue::Double(v) => Some(AstValue::Double(-v)),
                    AstValue::Bool(v) => Some(AstValue::Bool(!v)),
                    AstValue::Char(..)
                    | AstValue::Void
                    | AstValue::Unknown
                    | AstValue::Undefined
                    | AstValue::Pointer(..)
                    | AstValue::Array(..) => None,
                },
                IrUnaryOp::SignExtend | IrUnaryOp::ZeroExtend => None,
            }
        }
        IrData::Operation(IrDataOperation::Binary {
            operator,
            arg1,
            arg2,
        }) => {
            let Some(arg1) = resolve_constant(
                position,
                instruction_arg_size,
                instruction_byte_size,
                root_expr,
                arg1,
            )?
            else {
                return Ok(None);
            };
            let Some(arg2) = resolve_constant(
                position,
                instruction_arg_size,
                instruction_byte_size,
                root_expr,
                arg2,
            )?
            else {
                return Ok(None);
            };
            match operator {
                IrBinaryOp::And => {
                    let Some(arg1) = arg1.bool() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.bool() else {
                        return Ok(None);
                    };
                    Some(AstValue::Bool(arg1 & arg2))
                }
                IrBinaryOp::Or => {
                    let Some(arg1) = arg1.bool() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.bool() else {
                        return Ok(None);
                    };
                    Some(AstValue::Bool(arg1 | arg2))
                }
                IrBinaryOp::Xor => {
                    let Some(arg1) = arg1.bool() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.bool() else {
                        return Ok(None);
                    };
                    Some(AstValue::Bool(arg1 ^ arg2))
                }
                IrBinaryOp::Shl => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num().and_then(|x| x.to_biguint()) else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 << arg2.to_u64_digits()[0]))
                }
                IrBinaryOp::Shr => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num().and_then(|x| x.to_biguint()) else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 >> arg2.to_u64_digits()[0]))
                }
                IrBinaryOp::Sar => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num().and_then(|x| x.to_biguint()) else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 >> arg2.to_u64_digits()[0]))
                }
                IrBinaryOp::Add => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 + arg2))
                }
                IrBinaryOp::Sub => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 - arg2))
                }
                IrBinaryOp::Mul => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 * arg2))
                }
                IrBinaryOp::SignedDiv => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 / arg2))
                }
                IrBinaryOp::SignedRem => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 % arg2))
                }
                IrBinaryOp::UnsignedDiv => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 / arg2))
                }
                IrBinaryOp::UnsignedRem => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Num(arg1 % arg2))
                }
                IrBinaryOp::Equal(..) => Some(AstValue::Bool(arg1 == arg2)),
                IrBinaryOp::SignedLess(..) => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Bool(arg1 < arg2))
                }
                IrBinaryOp::SignedLessOrEqual(..) => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Bool(arg1 <= arg2))
                }
                IrBinaryOp::UnsignedLess(..) => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Bool(arg1 < arg2))
                }
                IrBinaryOp::UnsignedLessOrEqual(..) => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(AstValue::Bool(arg1 <= arg2))
                }
            }
        }
        IrData::Operand(..) => None,
    };
    Ok(result.map(w))
}

/// If the expression is a variable whose const_value is a boolean, fold it to a literal.
fn try_fold_bool_const_var(
    ast: &Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    expr: &mut Wrapped<AstExpression>,
) {
    if let AstExpression::Variable(_, var_id) = &expr.item {
        let Ok(vars) = ast.get_variables(&function_id, &function_version) else {
            return;
        };
        let vars = vars.read().unwrap();
        if let Some(var) = vars.get(var_id) {
            if let Some(const_val) = &var.const_value {
                if let AstValue::Bool(b) = &const_val.item {
                    expr.item = AstExpression::Literal(AstLiteral::Bool(*b));
                }
            }
        }
    }
}

/// Check whether an expression is a valid assignment target.
fn is_assignable_lhs(expr: &AstExpression) -> bool {
    matches!(
        expr,
        AstExpression::Variable(_, _)
            | AstExpression::Deref(_)
            | AstExpression::ArrayAccess(_, _)
            | AstExpression::MemberAccess(_, _)
            | AstExpression::Unknown
    )
}

/// If the expression is `operand_exists(N)`, evaluate it to a bool literal
/// using the known instruction argument count.
fn try_fold_operand_exists(expr: &mut Wrapped<AstExpression>, instruction_arg_count: u8) {
    if let AstExpression::Call(AstCall::Builtin(
        crate::abstract_syntax_tree::AstBuiltinFunction::OperandExists,
        args,
    )) = &expr.item
    {
        if let AstBuiltinFunctionArgument::OperandExists(arg) = args.as_ref() {
            if let AstExpression::Literal(AstLiteral::UInt(n)) = &arg.item {
                // operand_exists(n): true if instruction has at least n operands
                // n is 1-based operand index
                let exists = n
                    .checked_sub(1)
                    .is_some_and(|idx| idx < instruction_arg_count as u64);
                expr.item = AstExpression::Literal(AstLiteral::Bool(exists));
            }
        }
    }
}

/// Resolve an `IrAccessSize` to a concrete byte count when possible.
fn resolve_size_bytes(size: &IrAccessSize, instruction_args: &[iceball::Argument]) -> Option<usize> {
    use crate::ir::analyze::variables::resolve_ir_operand_of_access_size;
    let resolved = resolve_ir_operand_of_access_size(size, instruction_args);
    match &resolved {
        IrAccessSize::ResultOfByte(data) => match data.as_ref() {
            IrData::Constant(n) => Some(*n),
            _ => None,
        },
        IrAccessSize::ResultOfBit(data) => match data.as_ref() {
            IrData::Constant(n) => {
                if *n % 8 == 0 {
                    Some(*n / 8)
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    }
}

/// Convert a byte count to the corresponding signed `AstValueType`.
fn bytes_to_signed_type(bytes: usize) -> Option<AstValueType> {
    match bytes {
        1 => Some(AstValueType::Int8),
        2 => Some(AstValueType::Int16),
        4 => Some(AstValueType::Int32),
        8 => Some(AstValueType::Int64),
        _ => None,
    }
}

/// Convert a byte count to the corresponding unsigned `AstValueType`.
fn bytes_to_unsigned_type(bytes: usize) -> Option<AstValueType> {
    match bytes {
        1 => Some(AstValueType::UInt8),
        2 => Some(AstValueType::UInt16),
        4 => Some(AstValueType::UInt32),
        8 => Some(AstValueType::UInt64),
        _ => None,
    }
}

/// Refine `CastSigned(x)` / `CastUnsigned(x)` into `Cast(sized_type, x)`
/// using the assignment's target size to determine the appropriate C type.
fn refine_extend_cast(
    rhs: &mut Wrapped<AstExpression>,
    size: &IrAccessSize,
    instruction_args: &[iceball::Argument],
) {
    let AstExpression::UnaryOp(op, inner) = &rhs.item else {
        return;
    };
    let target_type = match op {
        AstUnaryOperator::CastSigned => {
            let Some(bytes) = resolve_size_bytes(size, instruction_args) else {
                return;
            };
            bytes_to_signed_type(bytes)
        }
        AstUnaryOperator::CastUnsigned => {
            let Some(bytes) = resolve_size_bytes(size, instruction_args) else {
                return;
            };
            bytes_to_unsigned_type(bytes)
        }
        _ => return,
    };
    if let Some(ty) = target_type {
        rhs.item = AstExpression::Cast(ty, inner.clone());
    }
}
