pub mod c_abstract_syntax_tree;

use crate::{
    core::Block,
    ir::{
        analyze::{
            ir_block_merger::merge_blocks,
            ir_to_c::c_abstract_syntax_tree::{
                BinaryOperator, CAst, CType, Expression, FunctionId, JumpTarget, Literal,
                Statement, UnaryOperator, Variable, VariableId, WrappedData, WrappedStatement,
            },
            variables::resolve_operand,
            ControlFlowGraphAnalyzer, DataType, MergedIr,
        },
        data::{AccessSize, IrData, IrDataOperation, IrIntrinsic},
        operator::{BinaryOperator as IrBinaryOp, UnaryOperator as IrUnaryOp},
        statements::{IrStatement, IrStatementSpecial, NumCondition},
        utils::IrStatementDescriptor,
    },
    prelude::*,
    utils::Aos,
};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

/// Wrap Statement
fn ws(statement: Statement, from: IrStatementDescriptor) -> WrappedStatement {
    WrappedStatement {
        statement,
        from: Some(from),
        comment: None,
    }
}
/// Wrap Data
fn wd<T>(item: T, from: impl Into<Option<Aos<IrData>>>) -> WrappedData<T> {
    WrappedData {
        item,
        from: from.into(),
        comment: None,
    }
}

/// Generate C AST from targets
pub fn generate_c(targets: impl IntoIterator<Item = Arc<Block>>) -> CAst {
    let mut ast = CAst::new();
    let mut cfg_analyzer = ControlFlowGraphAnalyzer::new();
    cfg_analyzer.add_targets(targets);
    let cfgs = cfg_analyzer.analyze();
    for cfg in cfgs.into_iter() {
        let merged = merge_blocks(&cfg.get_blocks());
        generate_c_function(&mut ast, &merged);
    }
    ast
}

/// Generate C function and add it to AST
///
/// ```rust, ignore
/// let mut ast = fireball::ir::analyze::ir_to_c::c_abstract_syntax_tree::CAst::new();
/// let merged = fireball::ir::analyze::ir_block_merger::merge_blocks(want_to_merge);
/// generate_c_function(&mut ast, &merged);
/// ```
///
/// ### Arguments
/// * `ast: &mut CAst` - The C AST to which the function will be added.
/// * `data: &MergedIr` - The merged IR data containing the function's instructions and variables.
pub fn generate_c_function(ast: &mut CAst, data: &MergedIr) {
    let func_id = ast.generate_default_function(data.get_ir().first().map(|x| &x.address).unwrap());

    let mut locals = HashMap::new();
    let mut var_map: HashMap<Aos<IrData>, VariableId> = HashMap::new();
    for var in data.get_variables().iter() {
        let var_id = ast.new_variable_id(&func_id);
        let c_type = match var.data_type {
            DataType::Unknown => CType::Unknown,
            DataType::Int => CType::Int32,
            DataType::Float => CType::Double,
            DataType::StringPointer => CType::Pointer(Box::new(CType::Char)),
            DataType::Char => CType::Char,
            DataType::Address => CType::Pointer(Box::new(CType::Void)),
        };
        locals.insert(
            var_id,
            Variable {
                name: var_id.get_default_name(),
                id: var_id,
                var_type: c_type,
                is_const: false,
            },
        );
        for accesses in var.get_data_accesses().values() {
            for da in accesses.iter() {
                var_map.insert(da.location().clone(), var_id);
            }
        }
    }
    ast.functions
        .write()
        .unwrap()
        .get_mut(&func_id)
        .unwrap()
        .variables = Arc::new(RwLock::new(locals));

    let mut func_body = Vec::new();
    for (ir_index, (ir, instruction)) in data
        .get_ir()
        .iter()
        .zip(data.get_instructions().iter())
        .enumerate()
    {
        let ir_index = ir_index as u32;
        func_body.push(ws(
            Statement::Comment(instruction.to_string()),
            IrStatementDescriptor::new(ir_index, None),
        ));
        if let Some(stmts) = ir.statements {
            let instruction_args = instruction.inner.arguments.as_ref();
            for (stmt_index, stmt) in stmts.iter().enumerate() {
                let stmt_index = stmt_index as u8;
                let stmt_position = IrStatementDescriptor::new(ir_index, Some(stmt_index));
                func_body.push(ws(Statement::Comment(stmt.to_string()), stmt_position));
                func_body.push(convert_stmt(
                    ast,
                    func_id,
                    stmt,
                    &stmt_position,
                    &var_map,
                    instruction_args,
                ));
            }
        } else {
            func_body.push(ws(
                Statement::Assembly(instruction.inner.to_string()),
                IrStatementDescriptor::new(ir_index, None),
            ));
        }
    }
    ast.functions
        .write()
        .unwrap()
        .get_mut(&func_id)
        .unwrap()
        .body = func_body;
}

fn convert_expr(
    ast: &mut CAst,
    function_id: FunctionId,
    data: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Expression {
    if let Some(&vid) = var_map.get(data) {
        let vars = ast.get_variables(&function_id).unwrap();
        return Expression::Variable(vars, vid);
    }

    match data.as_ref() {
        IrData::Constant(c) => Expression::Literal(Literal::Int(*c as i64)),
        IrData::Dereference(inner) => {
            Expression::Deref(Box::new(convert_expr(ast, function_id, inner, var_map)))
        }
        IrData::Intrinsic(intr) => match intr {
            IrIntrinsic::ArchitectureByteSize => Expression::ArchitectureByteSize,
            IrIntrinsic::ArchitectureBitSize => Expression::ArchitectureBitSize,
            IrIntrinsic::ArchitectureBitPerByte => {
                Expression::Call("ARCH_BIT_PER_BYTE".into(), vec![])
            }
            IrIntrinsic::InstructionByteSize => {
                Expression::Call("INSTRUCTION_BYTE_SIZE".into(), vec![])
            }
            IrIntrinsic::ByteSizeOf(inner) => Expression::Call(
                "byte_size_of".into(),
                vec![convert_expr(ast, function_id, inner, var_map)],
            ),
            IrIntrinsic::BitSizeOf(inner) => Expression::Call(
                "bit_size_of".into(),
                vec![convert_expr(ast, function_id, inner, var_map)],
            ),
            IrIntrinsic::Sized(inner, size) => {
                let arg = convert_expr(ast, function_id, inner, var_map);
                let sz = convert_size(ast, function_id, size, var_map);
                Expression::Call("sized".into(), vec![arg, sz])
            }
            IrIntrinsic::OperandExists(n) => Expression::Call(
                "operand_exists".into(),
                vec![Expression::Literal(Literal::UInt(n.get() as u64))],
            ),
            IrIntrinsic::Unknown => Expression::Unknown,
            IrIntrinsic::Undefined => Expression::Undefined,
            IrIntrinsic::SignedMax(size) => Expression::Call(
                "signed_max".into(),
                vec![convert_size(ast, function_id, size, var_map)],
            ),
            IrIntrinsic::SignedMin(size) => Expression::Call(
                "signed_min".into(),
                vec![convert_size(ast, function_id, size, var_map)],
            ),
            IrIntrinsic::UnsignedMax(size) => Expression::Call(
                "unsigned_max".into(),
                vec![convert_size(ast, function_id, size, var_map)],
            ),
            IrIntrinsic::UnsignedMin(size) => Expression::Call(
                "unsigned_min".into(),
                vec![convert_size(ast, function_id, size, var_map)],
            ),
            IrIntrinsic::BitOnes(size) => Expression::Call(
                "bit_ones".into(),
                vec![convert_size(ast, function_id, size, var_map)],
            ),
            IrIntrinsic::BitZeros(size) => Expression::Call(
                "bit_zeros".into(),
                vec![convert_size(ast, function_id, size, var_map)],
            ),
        },
        IrData::Operation(op) => match op {
            IrDataOperation::Unary { operator, arg } => {
                convert_unary(ast, function_id, operator, arg, var_map)
            }
            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => convert_binary(ast, function_id, operator, arg1, arg2, var_map),
        },
        IrData::Register(_) | IrData::Operand(_) => unreachable!("Should not be here"),
    }
}

fn convert_stmt(
    ast: &mut CAst,
    function_id: FunctionId,
    stmt: &IrStatement,
    stmt_position: &IrStatementDescriptor,
    var_map: &HashMap<Aos<IrData>, VariableId>,
    instruction_args: &[iceball::Argument],
) -> WrappedStatement {
    let result = match stmt {
        IrStatement::Assignment { from, to, .. } => {
            let from = &resolve_operand(from, instruction_args);
            let to = &resolve_operand(to, instruction_args);
            Statement::Assignment(
                convert_expr(ast, function_id, to, var_map),
                convert_expr(ast, function_id, from, var_map),
            )
        }
        IrStatement::JumpByCall { target } => {
            let target = &resolve_operand(target, instruction_args);
            let e = convert_expr(ast, function_id, target, var_map);
            let name = match e {
                Expression::Variable(vars, id) => {
                    let vars = vars.read().unwrap();
                    let var = vars.get(&id).unwrap();
                    var.name.to_string()
                }
                _ => {
                    warn!("Uncovered call target");
                    e.to_string()
                }
            };
            Statement::Call(JumpTarget::Unknown(name), Vec::new())
        }
        IrStatement::Jump { target } => {
            let target = &resolve_operand(target, instruction_args);
            let e = convert_expr(ast, function_id, target, var_map);
            let label = match e {
                Expression::Variable(vars, id) => {
                    let vars = vars.read().unwrap();
                    let var = vars.get(&id).unwrap();
                    var.name.to_string()
                }
                _ => {
                    warn!("Uncovered jump target");
                    e.to_string()
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
            let cond = convert_expr(ast, function_id, condition, var_map);
            let then_b = true_branch
                .iter()
                .map(|s| {
                    convert_stmt(
                        ast,
                        function_id,
                        s,
                        stmt_position,
                        var_map,
                        instruction_args,
                    )
                })
                .collect();
            let else_b = false_branch
                .iter()
                .map(|s| {
                    convert_stmt(
                        ast,
                        function_id,
                        s,
                        stmt_position,
                        var_map,
                        instruction_args,
                    )
                })
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
                    .map(|s| {
                        convert_stmt(
                            ast,
                            function_id,
                            s,
                            stmt_position,
                            var_map,
                            instruction_args,
                        )
                    })
                    .collect();
                let fb = false_branch
                    .iter()
                    .map(|s| {
                        convert_stmt(
                            ast,
                            function_id,
                            s,
                            stmt_position,
                            var_map,
                            instruction_args,
                        )
                    })
                    .collect();
                Statement::If(cond_expr, tb, Some(fb))
            }
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
                    flags,
                    var_map,
                );
                Statement::Block(stmts)
            }
            IrStatementSpecial::TypeSpecified {
                location: _,
                size: _,
                data_type: _,
            } => Statement::Empty, // Used to detect types
        },
    };
    ws(result, *stmt_position)
}

fn convert_unary(
    ast: &mut CAst,
    function_id: FunctionId,
    operator: &IrUnaryOp,
    arg: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Expression {
    let expr = convert_expr(ast, function_id, arg, var_map);
    let op = match operator {
        IrUnaryOp::Not => UnaryOperator::Not,
        IrUnaryOp::Negation => UnaryOperator::Negate,
        IrUnaryOp::SignExtend => UnaryOperator::CastSigned,
        IrUnaryOp::ZeroExtend => UnaryOperator::CastUnsigned,
    };
    Expression::UnaryOp(op, Box::new(expr))
}

fn convert_binary(
    ast: &mut CAst,
    function_id: FunctionId,
    operator: &IrBinaryOp,
    arg1: &Aos<IrData>,
    arg2: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Expression {
    let lhs = convert_expr(ast, function_id, arg1, var_map);
    let rhs = convert_expr(ast, function_id, arg2, var_map);

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
            let sz = convert_size(ast, function_id, size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]);
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            Expression::BinaryOp(BinaryOperator::Equal, Box::new(lhs_s), Box::new(rhs_s))
        }
        IrBinaryOp::SignedLess(size) => {
            let sz = convert_size(ast, function_id, size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]); // TODO does lhs need to be sized?
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            Expression::BinaryOp(BinaryOperator::Less, Box::new(lhs_s), Box::new(rhs_s))
        }
        IrBinaryOp::UnsignedLess(size) => {
            let sz = convert_size(ast, function_id, size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]);
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            let rhs_c = Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(rhs_s));
            Expression::BinaryOp(BinaryOperator::Less, Box::new(lhs_s), Box::new(rhs_c))
        }
        IrBinaryOp::SignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]);
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            Expression::BinaryOp(BinaryOperator::LessEqual, Box::new(lhs_s), Box::new(rhs_s))
        }
        IrBinaryOp::UnsignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, size, var_map);
            let lhs_s = Expression::Call("sized".into(), vec![lhs.clone(), sz.clone()]);
            let rhs_s = Expression::Call("sized".into(), vec![rhs.clone(), sz]);
            let rhs_c = Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(rhs_s));
            Expression::BinaryOp(BinaryOperator::LessEqual, Box::new(lhs_s), Box::new(rhs_c))
        }
    }
}

fn convert_size(
    ast: &mut CAst,
    function_id: FunctionId,
    size: &AccessSize,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Expression {
    match size {
        AccessSize::ResultOfBit(d) | AccessSize::ResultOfByte(d) | AccessSize::RelativeWith(d) => {
            convert_expr(ast, function_id, d, var_map)
        }
        AccessSize::ArchitectureSize => Expression::ArchitectureByteSize,
        AccessSize::Unlimited => Expression::Unknown,
    }
}

fn calc_flags_automatically(
    ast: &mut CAst,
    function_id: FunctionId,
    operation: &Aos<IrData>,
    stmt_position: &IrStatementDescriptor,
    affected_registers: &[Aos<IrData>],
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Vec<WrappedStatement> {
    // TODO INVALID
    let val = convert_expr(ast, function_id, operation, var_map);
    let vars = ast.get_variables(&function_id).unwrap();
    affected_registers
        .iter()
        .filter_map(|reg| {
            var_map.get(reg).map(|&vid| {
                Statement::Assignment(Expression::Variable(vars.clone(), vid), val.clone())
            })
        })
        .map(|stmt| ws(stmt, *stmt_position))
        .collect()
}
