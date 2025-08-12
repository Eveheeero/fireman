use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstDescriptor, AstExpression, AstFunctionId, AstFunctionVersion,
        AstJumpTarget, AstLiteral, AstStatement, AstStatementOrigin, AstUnaryOperator, AstValue,
        AstValueOrigin, AstVariableId, PrintWithConfig, Wrapped, WrappedAstStatement,
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
            IrIntrinsic::ArchitectureBitPerByte => {
                AstExpression::Call("ARCH_BIT_PER_BYTE".into(), [].to_vec())
            }
            IrIntrinsic::InstructionByteSize => {
                AstExpression::Call("INSTRUCTION_BYTE_SIZE".into(), [].to_vec())
            }
            IrIntrinsic::ByteSizeOf(inner) => AstExpression::Call(
                "byte_size_of".into(),
                [convert_expr(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    inner,
                    var_map,
                )?]
                .to_vec(),
            ),
            IrIntrinsic::BitSizeOf(inner) => AstExpression::Call(
                "bit_size_of".into(),
                [convert_expr(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    inner,
                    var_map,
                )?]
                .to_vec(),
            ),
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
                AstExpression::Call("sized".into(), [arg, sz].to_vec())
            }
            IrIntrinsic::OperandExists(n) => AstExpression::Call(
                "operand_exists".into(),
                [w(AstExpression::Literal(AstLiteral::UInt(n.get() as u64)))].to_vec(),
            ),
            IrIntrinsic::Unknown => AstExpression::Unknown,
            IrIntrinsic::Undefined => AstExpression::Undefined,
            IrIntrinsic::SignedMax(size) => AstExpression::Call(
                "signed_max".into(),
                [convert_size(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    size,
                    var_map,
                )?]
                .to_vec(),
            ),
            IrIntrinsic::SignedMin(size) => AstExpression::Call(
                "signed_min".into(),
                [convert_size(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    size,
                    var_map,
                )?]
                .to_vec(),
            ),
            IrIntrinsic::UnsignedMax(size) => AstExpression::Call(
                "unsigned_max".into(),
                [convert_size(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    size,
                    var_map,
                )?]
                .to_vec(),
            ),
            IrIntrinsic::UnsignedMin(size) => AstExpression::Call(
                "unsigned_min".into(),
                [convert_size(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    size,
                    var_map,
                )?]
                .to_vec(),
            ),
            IrIntrinsic::BitOnes(size) => AstExpression::Call(
                "bit_ones".into(),
                [convert_size(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    size,
                    var_map,
                )?]
                .to_vec(),
            ),
            IrIntrinsic::BitZeros(size) => AstExpression::Call(
                "bit_zeros".into(),
                [convert_size(
                    ast,
                    function_id,
                    function_version,
                    root_expr,
                    size,
                    var_map,
                )?]
                .to_vec(),
            ),
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
        IrData::Register(_) | IrData::Operand(_) => unreachable!("Should not be here"),
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
        IrStatement::Assignment { from, to, .. } => {
            let from = &resolve_operand(from, instruction_args);
            let to = &resolve_operand(to, instruction_args);
            AstStatement::Assignment(
                convert_expr(
                    ast,
                    function_id,
                    function_version,
                    root_expr.unwrap_or(to),
                    to,
                    var_map,
                )?,
                convert_expr(
                    ast,
                    function_id,
                    function_version,
                    root_expr.unwrap_or(to),
                    from,
                    var_map,
                )?,
            )
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
                AstExpression::Variable(vars, id) => AstStatement::Call(
                    AstJumpTarget::Variable {
                        scope: function_id,
                        var_map: vars.clone(),
                        var_id: *id,
                    },
                    Vec::new(),
                ),
                _ => {
                    warn!("Uncovered call target");
                    let name = e.to_string_with_config(None);
                    AstStatement::Call(AstJumpTarget::Unknown(name), Vec::new())
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
            let cond = convert_expr(
                ast,
                function_id,
                function_version,
                root_expr.unwrap_or(condition),
                condition,
                var_map,
            )?;
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
            let lhs_s = AstExpression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = AstExpression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            AstExpression::BinaryOp(
                AstBinaryOperator::Equal,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_s)),
            )
        }
        IrBinaryOp::SignedLess(size) => {
            let sz = convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
            let lhs_s = AstExpression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec()); // TODO does lhs need to be sized?
            let rhs_s = AstExpression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            AstExpression::BinaryOp(
                AstBinaryOperator::Less,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_s)),
            )
        }
        IrBinaryOp::UnsignedLess(size) => {
            let sz = convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
            let lhs_s = AstExpression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = AstExpression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            let rhs_c = AstExpression::UnaryOp(AstUnaryOperator::CastUnsigned, Box::new(w(rhs_s)));
            AstExpression::BinaryOp(
                AstBinaryOperator::Less,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_c)),
            )
        }
        IrBinaryOp::SignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
            let lhs_s = AstExpression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = AstExpression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            AstExpression::BinaryOp(
                AstBinaryOperator::LessEqual,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_s)),
            )
        }
        IrBinaryOp::UnsignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, function_version, root_expr, size, var_map)?;
            let lhs_s = AstExpression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = AstExpression::Call("sized".into(), [rhs.clone(), sz].to_vec());
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
            "rip" | "eip" | "ip" => {
                Some(AstValue::Num(BigInt::from(position.get_virtual_address())))
            }
            _ => None,
        },
        IrData::Dereference(data) => {
            let Some(c) = resolve_constant(position, instruction_arg_size, root_expr, data)? else {
                return Ok(None);
            };
            Some(AstValue::Pointer(Box::new(c)))
        }
        IrData::Operation(IrDataOperation::Unary { operator, arg }) => {
            let Some(arg) = resolve_constant(position, instruction_arg_size, root_expr, arg)?
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
            let Some(arg1) = resolve_constant(position, instruction_arg_size, root_expr, arg1)?
            else {
                return Ok(None);
            };
            let Some(arg2) = resolve_constant(position, instruction_arg_size, root_expr, arg2)?
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
        IrData::Operand(..) => unreachable!("With {}, {}", position, data),
    };
    Ok(result.map(w))
}
