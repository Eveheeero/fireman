pub mod c_abstract_syntax_tree;

use crate::{
    ir::{
        analyze::{
            ir_to_c::c_abstract_syntax_tree::{
                BinaryOperator, CAst, CType, Expression, Function, Literal, Statement,
                UnaryOperator, Variable,
            },
            variables::resolve_operand,
            DataType, MergedIr,
        },
        data::{AccessSize, IrData, IrDataOperation, IrIntrinsic},
        operator::{BinaryOperator as IrBinaryOp, UnaryOperator as IrUnaryOp},
        statements::{IrStatement, IrStatementSpecial, NumCondition},
    },
    utils::Aos,
};
use hashbrown::HashMap;

pub fn generate_c(data: &MergedIr) -> CAst {
    let mut ast = CAst::new();

    let mut locals = HashMap::new();
    for (i, var) in data.variables.iter().enumerate() {
        let c_type = match var.data_type {
            DataType::Unknown => CType::Unknown,
            DataType::Int => CType::Int32,
            DataType::Float => CType::Double,
            DataType::StringPointer => CType::Pointer(Box::new(CType::Char)),
            DataType::Char => CType::Char,
            DataType::Address => CType::Pointer(Box::new(CType::Void)),
        };
        locals.insert(
            i as u32,
            Variable {
                name: None,
                id: i as u32,
                var_type: c_type,
                is_const: false,
            },
        );
    }

    let mut func = Function {
        name: None,
        id: data
            .ir
            .first()
            .map(|ir| ir.address.get_virtual_address())
            .unwrap_or(0),
        return_type: CType::Void,
        parameters: Vec::new(),
        variables: locals,
        body: Vec::new(),
    };

    let mut var_map: HashMap<Aos<IrData>, u32> = HashMap::new();
    for (var_id, mvar) in data.variables.iter().enumerate() {
        for (_key, accesses) in mvar.accesses.iter() {
            for da in accesses.iter() {
                var_map.insert(da.location().clone(), var_id as u32);
            }
        }
    }

    for (ir, instruction) in data.ir.iter().zip(data.instructions.iter()) {
        func.body.push(Statement::Comment(instruction.to_string()));
        if let Some(stmts) = ir.statements {
            let instruction_args = instruction.inner.arguments.as_ref();
            for stmt in stmts.iter() {
                func.body.push(Statement::Comment(stmt.to_string()));
                func.body
                    .push(convert_stmt(stmt, &var_map, instruction_args));
            }
        } else {
            func.body
                .push(Statement::Assembly(instruction.inner.to_string()));
        }
    }

    ast.functions.push(func);
    ast
}

fn convert_expr(data: &Aos<IrData>, var_map: &HashMap<Aos<IrData>, u32>) -> Expression {
    if let Some(&vid) = var_map.get(data) {
        return Expression::Variable(vid);
    }

    match data.as_ref() {
        IrData::Constant(c) => Expression::Literal(Literal::Int(*c as i64)),
        IrData::Dereference(inner) => Expression::Deref(Box::new(convert_expr(inner, var_map))),
        IrData::Intrinsic(intr) => match intr {
            IrIntrinsic::ArchitectureByteSize => Expression::ArchitectureByteSize,
            IrIntrinsic::ArchitectureBitSize => Expression::ArchitectureBitSize,
            IrIntrinsic::ArchitectureBitPerByte => {
                Expression::Call("ARCH_BIT_PER_BYTE".into(), vec![])
            }
            IrIntrinsic::InstructionByteSize => {
                Expression::Call("INSTRUCTION_BYTE_SIZE".into(), vec![])
            }
            IrIntrinsic::ByteSizeOf(inner) => {
                Expression::Call("byte_size_of".into(), vec![convert_expr(inner, var_map)])
            }
            IrIntrinsic::BitSizeOf(inner) => {
                Expression::Call("bit_size_of".into(), vec![convert_expr(inner, var_map)])
            }
            IrIntrinsic::Sized(inner, size) => {
                let arg = convert_expr(inner, var_map);
                let sz = convert_size(size, var_map);
                Expression::Call("sized".into(), vec![arg, sz])
            }
            IrIntrinsic::OperandExists(n) => Expression::Call(
                "operand_exists".into(),
                vec![Expression::Literal(Literal::UInt(n.get() as u64))],
            ),
            IrIntrinsic::Unknown => Expression::Unknown,
            IrIntrinsic::Undefined => Expression::Undefined,
            IrIntrinsic::SignedMax(size) => {
                Expression::Call("signed_max".into(), vec![convert_size(size, var_map)])
            }
            IrIntrinsic::SignedMin(size) => {
                Expression::Call("signed_min".into(), vec![convert_size(size, var_map)])
            }
            IrIntrinsic::UnsignedMax(size) => {
                Expression::Call("unsigned_max".into(), vec![convert_size(size, var_map)])
            }
            IrIntrinsic::UnsignedMin(size) => {
                Expression::Call("unsigned_min".into(), vec![convert_size(size, var_map)])
            }
            IrIntrinsic::BitOnes(size) => {
                Expression::Call("bit_ones".into(), vec![convert_size(size, var_map)])
            }
            IrIntrinsic::BitZeros(size) => {
                Expression::Call("bit_zeros".into(), vec![convert_size(size, var_map)])
            }
        },
        IrData::Operation(op) => match op {
            IrDataOperation::Unary { operator, arg } => convert_unary(operator, arg, var_map),
            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => convert_binary(operator, arg1, arg2, var_map),
        },
        IrData::Register(_) | IrData::Operand(_) => unreachable!("Should not be here"),
    }
}

fn convert_stmt(
    stmt: &IrStatement,
    var_map: &HashMap<Aos<IrData>, u32>,
    instruction_args: &[iceball::Argument],
) -> Statement {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            let from = &resolve_operand(from, instruction_args);
            let to = &resolve_operand(to, instruction_args);
            Statement::Assignment(convert_expr(to, var_map), convert_expr(from, var_map))
        }
        IrStatement::JumpByCall { target } => {
            let target = &resolve_operand(target, instruction_args);
            let e = convert_expr(target, var_map);
            let name = match e {
                Expression::Variable(id) => format!("v{}", id),
                _ => e.to_string(),
            };
            Statement::Call(name, Vec::new())
        }
        IrStatement::Jump { target } => {
            let target = &resolve_operand(target, instruction_args);
            let e = convert_expr(target, var_map);
            let label = match e {
                Expression::Variable(id) => format!("L{}", id),
                _ => e.to_string(),
            };
            Statement::Goto(label)
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            let condition = &resolve_operand(condition, instruction_args);
            let cond = convert_expr(condition, var_map);
            let then_b = true_branch
                .iter()
                .map(|s| convert_stmt(s, var_map, instruction_args))
                .collect();
            let else_b = false_branch
                .iter()
                .map(|s| convert_stmt(s, var_map, instruction_args))
                .collect();
            Statement::If(cond, then_b, Some(else_b))
        }
        IrStatement::Halt => Statement::Return(None),
        IrStatement::Undefined => Statement::Undefined,
        IrStatement::Exception(e) => Statement::Exception(e),
        IrStatement::Special(special) => match special {
            IrStatementSpecial::Assertion { .. } => Statement::Empty,
            IrStatementSpecial::ArchitectureByteSizeCondition {
                condition,
                true_branch,
                false_branch,
            } => {
                let cond_expr = match condition {
                    NumCondition::Higher(v) => Expression::BinaryOp(
                        BinaryOperator::Greater,
                        Box::new(Expression::ArchitectureByteSize),
                        Box::new(Expression::Literal(Literal::UInt(*v as u64))),
                    ),
                    NumCondition::HigherOrEqual(v) => Expression::BinaryOp(
                        BinaryOperator::GreaterEqual,
                        Box::new(Expression::ArchitectureByteSize),
                        Box::new(Expression::Literal(Literal::UInt(*v as u64))),
                    ),
                    NumCondition::Lower(v) => Expression::BinaryOp(
                        BinaryOperator::Less,
                        Box::new(Expression::ArchitectureByteSize),
                        Box::new(Expression::Literal(Literal::UInt(*v as u64))),
                    ),
                    NumCondition::LowerOrEqual(v) => Expression::BinaryOp(
                        BinaryOperator::LessEqual,
                        Box::new(Expression::ArchitectureByteSize),
                        Box::new(Expression::Literal(Literal::UInt(*v as u64))),
                    ),
                    NumCondition::Equal(v) => Expression::BinaryOp(
                        BinaryOperator::Equal,
                        Box::new(Expression::ArchitectureByteSize),
                        Box::new(Expression::Literal(Literal::UInt(*v as u64))),
                    ),
                    NumCondition::NotEqual(v) => Expression::BinaryOp(
                        BinaryOperator::NotEqual,
                        Box::new(Expression::ArchitectureByteSize),
                        Box::new(Expression::Literal(Literal::UInt(*v as u64))),
                    ),
                    NumCondition::RangeInclusive(v1, v2) => {
                        let ge = Expression::BinaryOp(
                            BinaryOperator::GreaterEqual,
                            Box::new(Expression::ArchitectureByteSize),
                            Box::new(Expression::Literal(Literal::UInt(*v1 as u64))),
                        );
                        let le = Expression::BinaryOp(
                            BinaryOperator::LessEqual,
                            Box::new(Expression::ArchitectureByteSize),
                            Box::new(Expression::Literal(Literal::UInt(*v2 as u64))),
                        );
                        Expression::BinaryOp(BinaryOperator::LogicAnd, Box::new(ge), Box::new(le))
                    }
                    NumCondition::ExcludesRange(v1, v2) => {
                        let ge = Expression::BinaryOp(
                            BinaryOperator::GreaterEqual,
                            Box::new(Expression::ArchitectureByteSize),
                            Box::new(Expression::Literal(Literal::UInt(*v1 as u64))),
                        );
                        let le = Expression::BinaryOp(
                            BinaryOperator::LessEqual,
                            Box::new(Expression::ArchitectureByteSize),
                            Box::new(Expression::Literal(Literal::UInt(*v2 as u64))),
                        );
                        let between = Expression::BinaryOp(
                            BinaryOperator::LogicAnd,
                            Box::new(ge),
                            Box::new(le),
                        );
                        Expression::UnaryOp(UnaryOperator::Not, Box::new(between))
                    }
                };
                let tb = true_branch
                    .iter()
                    .map(|s| convert_stmt(s, var_map, instruction_args))
                    .collect();
                let fb = false_branch
                    .iter()
                    .map(|s| convert_stmt(s, var_map, instruction_args))
                    .collect();
                Statement::If(cond_expr, tb, Some(fb))
            }
            IrStatementSpecial::CalcFlagsAutomatically {
                operation,
                size: _,
                flags,
            } => {
                let operation = &resolve_operand(operation, instruction_args);
                let stmts = calc_flags_automatically(operation, flags, var_map);
                Statement::Block(stmts)
            }
            IrStatementSpecial::TypeSpecified {
                location: _,
                size: _,
                data_type: _,
            } => Statement::Empty, // Used to detect types
        },
    }
}

fn convert_unary(
    operator: &IrUnaryOp,
    arg: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, u32>,
) -> Expression {
    let expr = convert_expr(arg, var_map);
    let op = match operator {
        IrUnaryOp::Not => UnaryOperator::Not,
        IrUnaryOp::Negation => UnaryOperator::Negate,
        IrUnaryOp::SignExtend => UnaryOperator::CastSigned,
        IrUnaryOp::ZeroExtend => UnaryOperator::CastUnsigned,
    };
    Expression::UnaryOp(op, Box::new(expr))
}

fn convert_binary(
    operator: &IrBinaryOp,
    arg1: &Aos<IrData>,
    arg2: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, u32>,
) -> Expression {
    let lhs = convert_expr(arg1, var_map);
    let rhs = convert_expr(arg2, var_map);

    match operator {
        IrBinaryOp::Add => Expression::BinaryOp(BinaryOperator::Add, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::Sub => Expression::BinaryOp(BinaryOperator::Sub, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::Mul => Expression::BinaryOp(BinaryOperator::Mul, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::SignedDiv => Expression::BinaryOp(
            BinaryOperator::Div,
            Box::new(lhs),
            Box::new(Expression::UnaryOp(
                UnaryOperator::CastSigned,
                Box::new(rhs),
            )),
        ),
        IrBinaryOp::UnsignedDiv => Expression::BinaryOp(
            BinaryOperator::Div,
            Box::new(lhs),
            Box::new(Expression::UnaryOp(
                UnaryOperator::CastUnsigned,
                Box::new(rhs),
            )),
        ),
        IrBinaryOp::SignedRem => Expression::BinaryOp(
            BinaryOperator::Mod,
            Box::new(lhs),
            Box::new(Expression::UnaryOp(
                UnaryOperator::CastSigned,
                Box::new(rhs),
            )),
        ),
        IrBinaryOp::UnsignedRem => Expression::BinaryOp(
            BinaryOperator::Mod,
            Box::new(lhs),
            Box::new(Expression::UnaryOp(
                UnaryOperator::CastUnsigned,
                Box::new(rhs),
            )),
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
            let sz = convert_size(size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]);
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            Expression::BinaryOp(BinaryOperator::Equal, Box::new(lhs_s), Box::new(rhs_s))
        }
        IrBinaryOp::SignedLess(size) => {
            let sz = convert_size(size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]); // TODO does lhs need to be sized?
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            Expression::BinaryOp(BinaryOperator::Less, Box::new(lhs_s), Box::new(rhs_s))
        }
        IrBinaryOp::UnsignedLess(size) => {
            let sz = convert_size(size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]);
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            let rhs_c = Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(rhs_s));
            Expression::BinaryOp(BinaryOperator::Less, Box::new(lhs_s), Box::new(rhs_c))
        }
        IrBinaryOp::SignedLessOrEqual(size) => {
            let sz = convert_size(size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]);
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            Expression::BinaryOp(BinaryOperator::LessEqual, Box::new(lhs_s), Box::new(rhs_s))
        }
        IrBinaryOp::UnsignedLessOrEqual(size) => {
            let sz = convert_size(size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]);
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            let rhs_c = Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(rhs_s));
            Expression::BinaryOp(BinaryOperator::LessEqual, Box::new(lhs_s), Box::new(rhs_c))
        }
    }
}

fn convert_size(size: &AccessSize, var_map: &HashMap<Aos<IrData>, u32>) -> Expression {
    match size {
        AccessSize::ResultOfBit(d) | AccessSize::ResultOfByte(d) | AccessSize::RelativeWith(d) => {
            convert_expr(d, var_map)
        }
        AccessSize::ArchitectureSize => Expression::ArchitectureByteSize,
        AccessSize::Unlimited => Expression::Unknown,
    }
}

fn calc_flags_automatically(
    operation: &Aos<IrData>,
    affected_registers: &[Aos<IrData>],
    var_map: &HashMap<Aos<IrData>, u32>,
) -> Vec<Statement> {
    // TODO INVALID
    let val = convert_expr(operation, var_map);
    affected_registers
        .iter()
        .filter_map(|reg| {
            var_map
                .get(reg)
                .map(|&vid| Statement::Assignment(Expression::Variable(vid), val.clone()))
        })
        .collect()
}
