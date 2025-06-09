use crate::{
    core::Address,
    ir::{
        analyze::{
            ir_to_c::c_abstract_syntax_tree::{
                AstDescriptor, BinaryOperator, CAst, CValue, Expression, FunctionId, JumpTarget,
                Literal, PrintWithConfig, Statement, UnaryOperator, VariableId, Wrapped,
                WrappedStatement,
            },
            variables::resolve_operand,
        },
        data::{AccessSize, IrData, IrDataOperation, IrIntrinsic, NumCondition},
        operator::{BinaryOperator as IrBinaryOp, UnaryOperator as IrUnaryOp},
        statements::{IrStatement, IrStatementSpecial},
    },
    prelude::*,
    utils::Aos,
};
use hashbrown::HashMap;
use num_bigint::BigInt;

/// Wrap Statement
pub(super) fn ws(statement: Statement, from: AstDescriptor) -> WrappedStatement {
    WrappedStatement {
        statement,
        from: Some(from),
        comment: None,
    }
}
/// Wrap Data
pub(super) fn wd<T>(item: T, origin_expr: &Aos<IrData>) -> Wrapped<T> {
    Wrapped {
        item,
        origin_expr: Some(origin_expr.clone()),
        comment: None,
    }
}
pub(super) fn wdn<T>(item: T) -> Wrapped<T> {
    Wrapped {
        item,
        origin_expr: None,
        comment: None,
    }
}

pub(super) fn convert_expr(
    ast: &mut CAst,
    function_id: FunctionId,
    root_expr: &Aos<IrData>,
    data: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Wrapped<Expression>, DecompileError> {
    let w = |x: Expression| wd(x, root_expr);

    if let Some(&vid) = var_map.get(data) {
        let vars = ast.get_variables(&function_id).unwrap();
        return Ok(w(Expression::Variable(vars, vid)));
    }

    let result = match data.as_ref() {
        IrData::Constant(c) => Expression::Literal(Literal::Int(*c as i64)),
        IrData::Dereference(inner) => Expression::Deref(Box::new(convert_expr(
            ast,
            function_id,
            root_expr,
            inner,
            var_map,
        )?)),
        IrData::Intrinsic(intr) => match intr {
            IrIntrinsic::ArchitectureByteSize => Expression::ArchitectureByteSize,
            IrIntrinsic::ArchitectureBitSize => Expression::ArchitectureBitSize,
            IrIntrinsic::ArchitectureBitPerByte => {
                Expression::Call("ARCH_BIT_PER_BYTE".into(), [].to_vec())
            }
            IrIntrinsic::InstructionByteSize => {
                Expression::Call("INSTRUCTION_BYTE_SIZE".into(), [].to_vec())
            }
            IrIntrinsic::ByteSizeOf(inner) => Expression::Call(
                "byte_size_of".into(),
                [convert_expr(ast, function_id, root_expr, inner, var_map)?].to_vec(),
            ),
            IrIntrinsic::BitSizeOf(inner) => Expression::Call(
                "bit_size_of".into(),
                [convert_expr(ast, function_id, root_expr, inner, var_map)?].to_vec(),
            ),
            IrIntrinsic::Sized(inner, size) => {
                let arg = convert_expr(ast, function_id, root_expr, inner, var_map)?;
                let sz = convert_size(ast, function_id, root_expr, size, var_map)?;
                Expression::Call("sized".into(), [arg, sz].to_vec())
            }
            IrIntrinsic::OperandExists(n) => Expression::Call(
                "operand_exists".into(),
                [w(Expression::Literal(Literal::UInt(n.get() as u64)))].to_vec(),
            ),
            IrIntrinsic::Unknown => Expression::Unknown,
            IrIntrinsic::Undefined => Expression::Undefined,
            IrIntrinsic::SignedMax(size) => Expression::Call(
                "signed_max".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)?].to_vec(),
            ),
            IrIntrinsic::SignedMin(size) => Expression::Call(
                "signed_min".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)?].to_vec(),
            ),
            IrIntrinsic::UnsignedMax(size) => Expression::Call(
                "unsigned_max".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)?].to_vec(),
            ),
            IrIntrinsic::UnsignedMin(size) => Expression::Call(
                "unsigned_min".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)?].to_vec(),
            ),
            IrIntrinsic::BitOnes(size) => Expression::Call(
                "bit_ones".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)?].to_vec(),
            ),
            IrIntrinsic::BitZeros(size) => Expression::Call(
                "bit_zeros".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)?].to_vec(),
            ),
            IrIntrinsic::ArchitectureByteSizeCondition(num_condition) => {
                let u = |v: &u16| Expression::Literal(Literal::UInt(*v as u64));

                match num_condition {
                    NumCondition::Higher(v) => {
                        let op = BinaryOperator::Greater;
                        let lhs = Expression::ArchitectureByteSize;
                        let rhs = u(v);
                        Expression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::HigherOrEqual(v) => {
                        let op = BinaryOperator::GreaterEqual;
                        let lhs = Expression::ArchitectureByteSize;
                        let rhs = u(v);
                        Expression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::Lower(v) => {
                        let op = BinaryOperator::Less;
                        let lhs = Expression::ArchitectureByteSize;
                        let rhs = u(v);
                        Expression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::LowerOrEqual(v) => {
                        let op = BinaryOperator::LessEqual;
                        let lhs = Expression::ArchitectureByteSize;
                        let rhs = u(v);
                        Expression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::Equal(v) => {
                        let op = BinaryOperator::Equal;
                        let lhs = Expression::ArchitectureByteSize;
                        let rhs = u(v);
                        Expression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::NotEqual(v) => {
                        let op = BinaryOperator::NotEqual;
                        let lhs = Expression::ArchitectureByteSize;
                        let rhs = u(v);
                        Expression::BinaryOp(op, Box::new(w(lhs)), Box::new(w(rhs)))
                    }
                    NumCondition::RangeInclusive(start, end) => {
                        let start = u(start);
                        let end = u(end);
                        let arch_size = w(Expression::ArchitectureByteSize);
                        let op1 = Expression::BinaryOp(
                            BinaryOperator::GreaterEqual,
                            Box::new(arch_size.clone()),
                            Box::new(w(start)),
                        );
                        let op2 = Expression::BinaryOp(
                            BinaryOperator::LessEqual,
                            Box::new(arch_size),
                            Box::new(w(end)),
                        );
                        Expression::BinaryOp(
                            BinaryOperator::LogicAnd,
                            Box::new(w(op1)),
                            Box::new(w(op2)),
                        )
                    }
                    NumCondition::ExcludesRange(start, end) => {
                        let start = u(start);
                        let end = u(end);
                        let arch_size = w(Expression::ArchitectureByteSize);
                        let op1 = Expression::BinaryOp(
                            BinaryOperator::Less,
                            Box::new(arch_size.clone()),
                            Box::new(w(start)),
                        );
                        let op2 = Expression::BinaryOp(
                            BinaryOperator::Greater,
                            Box::new(arch_size),
                            Box::new(w(end)),
                        );
                        Expression::BinaryOp(
                            BinaryOperator::LogicOr,
                            Box::new(w(op1)),
                            Box::new(w(op2)),
                        )
                    }
                }
            }
        },
        IrData::Operation(op) => match op {
            IrDataOperation::Unary { operator, arg } => {
                return convert_unary(ast, function_id, root_expr, operator, arg, var_map);
            }
            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => return convert_binary(ast, function_id, root_expr, operator, arg1, arg2, var_map),
        },
        IrData::Register(_) | IrData::Operand(_) => unreachable!("Should not be here"),
    };
    Ok(w(result))
}

pub(super) fn convert_stmt(
    ast: &mut CAst,
    function_id: FunctionId,
    stmt: &IrStatement,
    stmt_position: &AstDescriptor,
    root_expr: Option<&Aos<IrData>>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
    instruction_args: &[iceball::Argument],
) -> Result<WrappedStatement, DecompileError> {
    let result = match stmt {
        IrStatement::Assignment { from, to, .. } => {
            let from = &resolve_operand(from, instruction_args);
            let to = &resolve_operand(to, instruction_args);
            Statement::Assignment(
                convert_expr(ast, function_id, root_expr.unwrap_or(to), to, var_map)?,
                convert_expr(ast, function_id, root_expr.unwrap_or(to), from, var_map)?,
            )
        }
        IrStatement::JumpByCall { target } => {
            let target = &resolve_operand(target, instruction_args);
            let e = convert_expr(
                ast,
                function_id,
                root_expr.unwrap_or(target),
                target,
                var_map,
            )?;
            let name = match e.as_ref() {
                Expression::Variable(vars, id) => {
                    let vars = vars.read().unwrap();
                    let var = vars.get(id).unwrap();
                    var.name.to_string()
                }
                _ => {
                    warn!("Uncovered call target");
                    e.to_string_with_config(None)
                }
            };
            Statement::Call(JumpTarget::Unknown(name), Vec::new())
        }
        IrStatement::Jump { target } => {
            let target = &resolve_operand(target, instruction_args);
            let e = convert_expr(
                ast,
                function_id,
                root_expr.unwrap_or(target),
                target,
                var_map,
            )?;
            let label = match e.as_ref() {
                Expression::Variable(vars, id) => {
                    let vars = vars.read().unwrap();
                    let var = vars.get(id).unwrap();
                    var.name.to_string()
                }
                _ => {
                    warn!("Uncovered jump target");
                    e.to_string_with_config(None)
                }
            };
            Statement::Goto(JumpTarget::Unknown(label))
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
                        s,
                        stmt_position,
                        root_expr,
                        var_map,
                        instruction_args,
                    )
                })
                .collect::<Result<_, _>>()?;
            Statement::If(cond, then_b, Some(else_b))
        }
        IrStatement::Halt => Statement::Return(None),
        IrStatement::Undefined => Statement::Undefined,
        IrStatement::Exception(e) => Statement::Exception(e),
        IrStatement::Special(special) => match special {
            IrStatementSpecial::Assertion { .. } => Statement::Empty,
            IrStatementSpecial::CalcFlagsAutomatically {
                operation,
                size: _,
                flags,
            } => {
                let operation = &resolve_operand(operation, instruction_args);
                let stmts = calc_flags_automatically(
                    ast,
                    function_id,
                    operation,
                    stmt_position,
                    root_expr.unwrap_or(operation),
                    flags,
                    var_map,
                )?;
                Statement::Block(stmts)
            }
            IrStatementSpecial::TypeSpecified {
                location: _,
                size: _,
                data_type: _,
            } => Statement::Empty, // Used to detect types
        },
    };
    Ok(ws(result, stmt_position.clone()))
}

pub(super) fn convert_unary(
    ast: &mut CAst,
    function_id: FunctionId,
    root_expr: &Aos<IrData>,
    operator: &IrUnaryOp,
    arg: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Wrapped<Expression>, DecompileError> {
    let w = |x: Expression| wd(x, root_expr);

    let expr = convert_expr(ast, function_id, root_expr, arg, var_map)?;
    let op = match operator {
        IrUnaryOp::Not => UnaryOperator::Not,
        IrUnaryOp::Negation => UnaryOperator::Negate,
        IrUnaryOp::SignExtend => UnaryOperator::CastSigned,
        IrUnaryOp::ZeroExtend => UnaryOperator::CastUnsigned,
    };
    Ok(w(Expression::UnaryOp(op, Box::new(expr))))
}

pub(super) fn convert_binary(
    ast: &mut CAst,
    function_id: FunctionId,
    root_expr: &Aos<IrData>,
    operator: &IrBinaryOp,
    arg1: &Aos<IrData>,
    arg2: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Wrapped<Expression>, DecompileError> {
    let w = |x: Expression| wd(x, root_expr);

    let lhs = convert_expr(ast, function_id, root_expr, arg1, var_map)?;
    let rhs = convert_expr(ast, function_id, root_expr, arg2, var_map)?;

    let result = match operator {
        IrBinaryOp::Add => Expression::BinaryOp(BinaryOperator::Add, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::Sub => Expression::BinaryOp(BinaryOperator::Sub, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::Mul => Expression::BinaryOp(BinaryOperator::Mul, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::SignedDiv => Expression::BinaryOp(
            BinaryOperator::Div,
            Box::new(lhs),
            Box::new(w(Expression::UnaryOp(
                UnaryOperator::CastSigned,
                Box::new(rhs),
            ))),
        ),
        IrBinaryOp::UnsignedDiv => Expression::BinaryOp(
            BinaryOperator::Div,
            Box::new(lhs),
            Box::new(w(Expression::UnaryOp(
                UnaryOperator::CastUnsigned,
                Box::new(rhs),
            ))),
        ),
        IrBinaryOp::SignedRem => Expression::BinaryOp(
            BinaryOperator::Mod,
            Box::new(lhs),
            Box::new(w(Expression::UnaryOp(
                UnaryOperator::CastSigned,
                Box::new(rhs),
            ))),
        ),
        IrBinaryOp::UnsignedRem => Expression::BinaryOp(
            BinaryOperator::Mod,
            Box::new(lhs),
            Box::new(w(Expression::UnaryOp(
                UnaryOperator::CastUnsigned,
                Box::new(rhs),
            ))),
        ),
        IrBinaryOp::And => {
            Expression::BinaryOp(BinaryOperator::BitAnd, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Or => Expression::BinaryOp(BinaryOperator::BitOr, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::Xor => {
            Expression::BinaryOp(BinaryOperator::BitXor, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Shl => {
            Expression::BinaryOp(BinaryOperator::LeftShift, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Shr | IrBinaryOp::Sar => {
            Expression::BinaryOp(BinaryOperator::RightShift, Box::new(lhs), Box::new(rhs))
        }
        IrBinaryOp::Equal(size) => {
            let sz = convert_size(ast, function_id, root_expr, size, var_map)?;
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            Expression::BinaryOp(
                BinaryOperator::Equal,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_s)),
            )
        }
        IrBinaryOp::SignedLess(size) => {
            let sz = convert_size(ast, function_id, root_expr, size, var_map)?;
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec()); // TODO does lhs need to be sized?
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            Expression::BinaryOp(BinaryOperator::Less, Box::new(w(lhs_s)), Box::new(w(rhs_s)))
        }
        IrBinaryOp::UnsignedLess(size) => {
            let sz = convert_size(ast, function_id, root_expr, size, var_map)?;
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            let rhs_c = Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(w(rhs_s)));
            Expression::BinaryOp(BinaryOperator::Less, Box::new(w(lhs_s)), Box::new(w(rhs_c)))
        }
        IrBinaryOp::SignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, root_expr, size, var_map)?;
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            Expression::BinaryOp(
                BinaryOperator::LessEqual,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_s)),
            )
        }
        IrBinaryOp::UnsignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, root_expr, size, var_map)?;
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            let rhs_c = Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(w(rhs_s)));
            Expression::BinaryOp(
                BinaryOperator::LessEqual,
                Box::new(w(lhs_s)),
                Box::new(w(rhs_c)),
            )
        }
    };
    Ok(w(result))
}

pub(super) fn convert_size(
    ast: &mut CAst,
    function_id: FunctionId,
    root_expr: &Aos<IrData>,
    size: &AccessSize,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Wrapped<Expression>, DecompileError> {
    let w = |x: Expression| wd(x, root_expr);

    let result = match size {
        AccessSize::ResultOfBit(d) | AccessSize::ResultOfByte(d) | AccessSize::RelativeWith(d) => {
            return convert_expr(ast, function_id, root_expr, d, var_map);
        }
        AccessSize::ArchitectureSize => Expression::ArchitectureByteSize,
        AccessSize::Unlimited => Expression::Unknown,
    };
    Ok(w(result))
}

pub(super) fn calc_flags_automatically(
    ast: &mut CAst,
    function_id: FunctionId,
    operation: &Aos<IrData>,
    stmt_position: &AstDescriptor,
    root_expr: &Aos<IrData>,
    affected_registers: &[Aos<IrData>],
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Vec<WrappedStatement>, DecompileError> {
    let w = |x: Expression| wd(x, root_expr);

    // TODO INVALID
    let val = convert_expr(ast, function_id, root_expr, operation, var_map)?;
    let vars = ast.get_variables(&function_id).unwrap();
    let result = affected_registers
        .iter()
        .filter_map(|reg| {
            var_map.get(reg).map(|&vid| {
                Statement::Assignment(w(Expression::Variable(vars.clone(), vid)), val.clone())
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
) -> Result<Option<Wrapped<CValue>>, DecompileError> {
    let w = |x: CValue| wd(x, root_expr);

    let result = match data.as_ref() {
        IrData::Constant(c) => Some(CValue::Num(BigInt::from(*c))),
        IrData::Intrinsic(i) => match i {
            IrIntrinsic::Unknown => Some(CValue::Unknown),
            IrIntrinsic::Undefined => Some(CValue::Undefined),
            IrIntrinsic::SignedMax(..) | IrIntrinsic::UnsignedMax(..) => Some(CValue::Max),
            IrIntrinsic::SignedMin(..) | IrIntrinsic::UnsignedMin(..) => Some(CValue::Min),
            IrIntrinsic::OperandExists(non_zero) => {
                Some(CValue::Bool(non_zero.get() - 1 <= instruction_arg_size))
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
            "rip" | "eip" | "ip" => Some(CValue::Num(BigInt::from(position.get_virtual_address()))),
            _ => None,
        },
        IrData::Dereference(data) => {
            let Some(c) = resolve_constant(position, instruction_arg_size, root_expr, data)? else {
                return Ok(None);
            };
            Some(CValue::Pointer(Box::new(c)))
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
                    Some(CValue::Bool(!*arg))
                }
                IrUnaryOp::Negation => match arg.item {
                    CValue::Max => Some(CValue::Min),
                    CValue::Min => Some(CValue::Max),
                    CValue::Num(v) => Some(CValue::Num(-v)),
                    CValue::Double(v) => Some(CValue::Double(-v)),
                    CValue::Bool(v) => Some(CValue::Bool(!v)),
                    CValue::Char(..)
                    | CValue::Void
                    | CValue::Unknown
                    | CValue::Undefined
                    | CValue::Pointer(..)
                    | CValue::Array(..) => None,
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
                    Some(CValue::Bool(arg1 & arg2))
                }
                IrBinaryOp::Or => {
                    let Some(arg1) = arg1.bool() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.bool() else {
                        return Ok(None);
                    };
                    Some(CValue::Bool(arg1 | arg2))
                }
                IrBinaryOp::Xor => {
                    let Some(arg1) = arg1.bool() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.bool() else {
                        return Ok(None);
                    };
                    Some(CValue::Bool(arg1 ^ arg2))
                }
                IrBinaryOp::Shl => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num().and_then(|x| x.to_biguint()) else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 << arg2.to_u64_digits()[0]))
                }
                IrBinaryOp::Shr => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num().and_then(|x| x.to_biguint()) else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 >> arg2.to_u64_digits()[0]))
                }
                IrBinaryOp::Sar => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num().and_then(|x| x.to_biguint()) else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 >> arg2.to_u64_digits()[0]))
                }
                IrBinaryOp::Add => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 + arg2))
                }
                IrBinaryOp::Sub => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 - arg2))
                }
                IrBinaryOp::Mul => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 * arg2))
                }
                IrBinaryOp::SignedDiv => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 / arg2))
                }
                IrBinaryOp::SignedRem => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 % arg2))
                }
                IrBinaryOp::UnsignedDiv => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 / arg2))
                }
                IrBinaryOp::UnsignedRem => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Num(arg1 % arg2))
                }
                IrBinaryOp::Equal(..) => Some(CValue::Bool(arg1 == arg2)),
                IrBinaryOp::SignedLess(..) => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Bool(arg1 < arg2))
                }
                IrBinaryOp::SignedLessOrEqual(..) => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Bool(arg1 <= arg2))
                }
                IrBinaryOp::UnsignedLess(..) => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Bool(arg1 < arg2))
                }
                IrBinaryOp::UnsignedLessOrEqual(..) => {
                    let Some(arg1) = arg1.num() else {
                        return Ok(None);
                    };
                    let Some(arg2) = arg2.num() else {
                        return Ok(None);
                    };
                    Some(CValue::Bool(arg1 <= arg2))
                }
            }
        }
        IrData::Operand(..) => unreachable!("With {}, {}", position, data),
    };
    Ok(result.map(w))
}
