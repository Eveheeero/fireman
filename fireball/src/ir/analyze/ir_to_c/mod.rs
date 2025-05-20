pub mod c_abstract_syntax_tree;

use crate::{
    core::{Address, Block},
    ir::{
        analyze::{
            ir_block_merger::merge_blocks,
            ir_to_c::c_abstract_syntax_tree::{
                BinaryOperator, CAst, CType, CValue, Expression, FunctionId, JumpTarget, Literal,
                Statement, UnaryOperator, Variable, VariableId, Wrapped, WrappedStatement,
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
use num_bigint::BigInt;
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
fn wd<T>(item: T, root_expr: &Aos<IrData>) -> Wrapped<T> {
    Wrapped {
        item,
        root_expr: Some(root_expr.clone()),
        comment: None,
    }
}
fn wdn<T>(item: T) -> Wrapped<T> {
    Wrapped {
        item,
        root_expr: None,
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
            DataType::Int => CType::Int,
            DataType::Float => CType::Double,
            DataType::StringPointer => CType::Pointer(Box::new(CType::Char)),
            DataType::Char => CType::Char,
            DataType::Address => CType::Pointer(Box::new(CType::Void)),
        };
        let mut const_value = None;
        for (position, accesses) in var.get_data_accesses().iter() {
            let instruction_arg_size = data.get_instructions()[position.ir_index() as usize]
                .inner
                .arguments
                .len() as u8;
            let position = &data.get_ir()[position.ir_index() as usize].address;
            for da in accesses.iter() {
                var_map.insert(da.location().clone(), var_id);
                // Resolve constant value
                if let Some(c) = resolve_constant(
                    position,
                    instruction_arg_size,
                    &da.location(),
                    &da.location(),
                ) {
                    trace!("Constant value found in {}: {}", position, c);
                    if const_value.is_some() && const_value.as_ref().unwrap() != &c {
                        warn!(
                            "Constant value mismatch in position {}: {} != {}",
                            position,
                            const_value.unwrap(),
                            c
                        );
                    }
                    const_value = Some(c);
                }
            }
        }
        locals.insert(
            var_id,
            Variable {
                name: var_id.get_default_name(),
                id: var_id,
                var_type: c_type,
                const_value,
            },
        );
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
                    None,
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
    root_expr: &Aos<IrData>,
    data: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Wrapped<Expression> {
    if let Some(&vid) = var_map.get(data) {
        let vars = ast.get_variables(&function_id).unwrap();
        return wd(Expression::Variable(vars, vid), root_expr);
    }

    let result = match data.as_ref() {
        IrData::Constant(c) => Expression::Literal(Literal::Int(*c as i64)),
        IrData::Dereference(inner) => Expression::Deref(Box::new(convert_expr(
            ast,
            function_id,
            root_expr,
            inner,
            var_map,
        ))),
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
                [convert_expr(ast, function_id, root_expr, inner, var_map)].to_vec(),
            ),
            IrIntrinsic::BitSizeOf(inner) => Expression::Call(
                "bit_size_of".into(),
                [convert_expr(ast, function_id, root_expr, inner, var_map)].to_vec(),
            ),
            IrIntrinsic::Sized(inner, size) => {
                let arg = convert_expr(ast, function_id, root_expr, inner, var_map);
                let sz = convert_size(ast, function_id, root_expr, size, var_map);
                Expression::Call("sized".into(), [arg, sz].to_vec())
            }
            IrIntrinsic::OperandExists(n) => Expression::Call(
                "operand_exists".into(),
                [wd(
                    Expression::Literal(Literal::UInt(n.get() as u64)),
                    root_expr,
                )]
                .to_vec(),
            ),
            IrIntrinsic::Unknown => Expression::Unknown,
            IrIntrinsic::Undefined => Expression::Undefined,
            IrIntrinsic::SignedMax(size) => Expression::Call(
                "signed_max".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)].to_vec(),
            ),
            IrIntrinsic::SignedMin(size) => Expression::Call(
                "signed_min".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)].to_vec(),
            ),
            IrIntrinsic::UnsignedMax(size) => Expression::Call(
                "unsigned_max".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)].to_vec(),
            ),
            IrIntrinsic::UnsignedMin(size) => Expression::Call(
                "unsigned_min".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)].to_vec(),
            ),
            IrIntrinsic::BitOnes(size) => Expression::Call(
                "bit_ones".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)].to_vec(),
            ),
            IrIntrinsic::BitZeros(size) => Expression::Call(
                "bit_zeros".into(),
                [convert_size(ast, function_id, root_expr, size, var_map)].to_vec(),
            ),
        },
        IrData::Operation(op) => match op {
            IrDataOperation::Unary { operator, arg } => {
                return convert_unary(ast, function_id, root_expr, operator, arg, var_map)
            }
            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => return convert_binary(ast, function_id, root_expr, operator, arg1, arg2, var_map),
        },
        IrData::Register(_) | IrData::Operand(_) => unreachable!("Should not be here"),
    };
    wd(result, root_expr)
}

fn convert_stmt(
    ast: &mut CAst,
    function_id: FunctionId,
    stmt: &IrStatement,
    stmt_position: &IrStatementDescriptor,
    root_expr: Option<&Aos<IrData>>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
    instruction_args: &[iceball::Argument],
) -> WrappedStatement {
    let result = match stmt {
        IrStatement::Assignment { from, to, .. } => {
            let from = &resolve_operand(from, instruction_args);
            let to = &resolve_operand(to, instruction_args);
            Statement::Assignment(
                convert_expr(ast, function_id, root_expr.unwrap_or(to), to, var_map),
                convert_expr(ast, function_id, root_expr.unwrap_or(to), from, var_map),
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
            );
            let name = match e.as_ref() {
                Expression::Variable(vars, id) => {
                    let vars = vars.read().unwrap();
                    let var = vars.get(id).unwrap();
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
            let e = convert_expr(
                ast,
                function_id,
                root_expr.unwrap_or(target),
                target,
                var_map,
            );
            let label = match e.as_ref() {
                Expression::Variable(vars, id) => {
                    let vars = vars.read().unwrap();
                    let var = vars.get(id).unwrap();
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
            let cond = convert_expr(
                ast,
                function_id,
                root_expr.unwrap_or(condition),
                condition,
                var_map,
            );
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
                .collect();
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
                        Box::new(wdn(Expression::ArchitectureByteSize)),
                        Box::new(wdn(Expression::Literal(Literal::UInt(*v as u64)))),
                    ),
                    NumCondition::HigherOrEqual(v) => Expression::BinaryOp(
                        BinaryOperator::GreaterEqual,
                        Box::new(wdn(Expression::ArchitectureByteSize)),
                        Box::new(wdn(Expression::Literal(Literal::UInt(*v as u64)))),
                    ),
                    NumCondition::Lower(v) => Expression::BinaryOp(
                        BinaryOperator::Less,
                        Box::new(wdn(Expression::ArchitectureByteSize)),
                        Box::new(wdn(Expression::Literal(Literal::UInt(*v as u64)))),
                    ),
                    NumCondition::LowerOrEqual(v) => Expression::BinaryOp(
                        BinaryOperator::LessEqual,
                        Box::new(wdn(Expression::ArchitectureByteSize)),
                        Box::new(wdn(Expression::Literal(Literal::UInt(*v as u64)))),
                    ),
                    NumCondition::Equal(v) => Expression::BinaryOp(
                        BinaryOperator::Equal,
                        Box::new(wdn(Expression::ArchitectureByteSize)),
                        Box::new(wdn(Expression::Literal(Literal::UInt(*v as u64)))),
                    ),
                    NumCondition::NotEqual(v) => Expression::BinaryOp(
                        BinaryOperator::NotEqual,
                        Box::new(wdn(Expression::ArchitectureByteSize)),
                        Box::new(wdn(Expression::Literal(Literal::UInt(*v as u64)))),
                    ),
                    NumCondition::RangeInclusive(v1, v2) => {
                        let ge = Expression::BinaryOp(
                            BinaryOperator::GreaterEqual,
                            Box::new(wdn(Expression::ArchitectureByteSize)),
                            Box::new(wdn(Expression::Literal(Literal::UInt(*v1 as u64)))),
                        );
                        let le = Expression::BinaryOp(
                            BinaryOperator::LessEqual,
                            Box::new(wdn(Expression::ArchitectureByteSize)),
                            Box::new(wdn(Expression::Literal(Literal::UInt(*v2 as u64)))),
                        );
                        let ge = wdn(ge);
                        let le = wdn(le);
                        Expression::BinaryOp(BinaryOperator::LogicAnd, Box::new(ge), Box::new(le))
                    }
                    NumCondition::ExcludesRange(v1, v2) => {
                        let ge = Expression::BinaryOp(
                            BinaryOperator::GreaterEqual,
                            Box::new(wdn(Expression::ArchitectureByteSize)),
                            Box::new(wdn(Expression::Literal(Literal::UInt(*v1 as u64)))),
                        );
                        let le = Expression::BinaryOp(
                            BinaryOperator::LessEqual,
                            Box::new(wdn(Expression::ArchitectureByteSize)),
                            Box::new(wdn(Expression::Literal(Literal::UInt(*v2 as u64)))),
                        );
                        let between = Expression::BinaryOp(
                            BinaryOperator::LogicAnd,
                            Box::new(wdn(ge)),
                            Box::new(wdn(le)),
                        );
                        Expression::UnaryOp(UnaryOperator::Not, Box::new(wdn(between)))
                    }
                };
                let cond_expr = wdn(cond_expr);
                let tb = true_branch
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
                    .collect();
                let fb = false_branch
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
                    root_expr.unwrap_or(operation),
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
    root_expr: &Aos<IrData>,
    operator: &IrUnaryOp,
    arg: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Wrapped<Expression> {
    let expr = convert_expr(ast, function_id, root_expr, arg, var_map);
    let op = match operator {
        IrUnaryOp::Not => UnaryOperator::Not,
        IrUnaryOp::Negation => UnaryOperator::Negate,
        IrUnaryOp::SignExtend => UnaryOperator::CastSigned,
        IrUnaryOp::ZeroExtend => UnaryOperator::CastUnsigned,
    };
    wd(Expression::UnaryOp(op, Box::new(expr)), root_expr)
}

fn convert_binary(
    ast: &mut CAst,
    function_id: FunctionId,
    root_expr: &Aos<IrData>,
    operator: &IrBinaryOp,
    arg1: &Aos<IrData>,
    arg2: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Wrapped<Expression> {
    let lhs = convert_expr(ast, function_id, root_expr, arg1, var_map);
    let rhs = convert_expr(ast, function_id, root_expr, arg2, var_map);

    let result = match operator {
        IrBinaryOp::Add => Expression::BinaryOp(BinaryOperator::Add, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::Sub => Expression::BinaryOp(BinaryOperator::Sub, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::Mul => Expression::BinaryOp(BinaryOperator::Mul, Box::new(lhs), Box::new(rhs)),
        IrBinaryOp::SignedDiv => Expression::BinaryOp(
            BinaryOperator::Div,
            Box::new(lhs),
            Box::new(wd(
                Expression::UnaryOp(UnaryOperator::CastSigned, Box::new(rhs)),
                root_expr,
            )),
        ),
        IrBinaryOp::UnsignedDiv => Expression::BinaryOp(
            BinaryOperator::Div,
            Box::new(lhs),
            Box::new(wd(
                Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(rhs)),
                root_expr,
            )),
        ),
        IrBinaryOp::SignedRem => Expression::BinaryOp(
            BinaryOperator::Mod,
            Box::new(lhs),
            Box::new(wd(
                Expression::UnaryOp(UnaryOperator::CastSigned, Box::new(rhs)),
                root_expr,
            )),
        ),
        IrBinaryOp::UnsignedRem => Expression::BinaryOp(
            BinaryOperator::Mod,
            Box::new(lhs),
            Box::new(wd(
                Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(rhs)),
                root_expr,
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
            let sz = convert_size(ast, function_id, root_expr, size, var_map);
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            Expression::BinaryOp(
                BinaryOperator::Equal,
                Box::new(wd(lhs_s, root_expr)),
                Box::new(wd(rhs_s, root_expr)),
            )
        }
        IrBinaryOp::SignedLess(size) => {
            let sz = convert_size(ast, function_id, root_expr, size, var_map);
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec()); // TODO does lhs need to be sized?
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            Expression::BinaryOp(
                BinaryOperator::Less,
                Box::new(wd(lhs_s, root_expr)),
                Box::new(wd(rhs_s, root_expr)),
            )
        }
        IrBinaryOp::UnsignedLess(size) => {
            let sz = convert_size(ast, function_id, root_expr, size, var_map);
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            let rhs_c =
                Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(wd(rhs_s, root_expr)));
            Expression::BinaryOp(
                BinaryOperator::Less,
                Box::new(wd(lhs_s, root_expr)),
                Box::new(wd(rhs_c, root_expr)),
            )
        }
        IrBinaryOp::SignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, root_expr, size, var_map);
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            Expression::BinaryOp(
                BinaryOperator::LessEqual,
                Box::new(wd(lhs_s, root_expr)),
                Box::new(wd(rhs_s, root_expr)),
            )
        }
        IrBinaryOp::UnsignedLessOrEqual(size) => {
            let sz = convert_size(ast, function_id, root_expr, size, var_map);
            let lhs_s = Expression::Call("sized".into(), [lhs.clone(), sz.clone()].to_vec());
            let rhs_s = Expression::Call("sized".into(), [rhs.clone(), sz].to_vec());
            let rhs_c =
                Expression::UnaryOp(UnaryOperator::CastUnsigned, Box::new(wd(rhs_s, root_expr)));
            Expression::BinaryOp(
                BinaryOperator::LessEqual,
                Box::new(wd(lhs_s, root_expr)),
                Box::new(wd(rhs_c, root_expr)),
            )
        }
    };
    wd(result, root_expr)
}

fn convert_size(
    ast: &mut CAst,
    function_id: FunctionId,
    root_expr: &Aos<IrData>,
    size: &AccessSize,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Wrapped<Expression> {
    match size {
        AccessSize::ResultOfBit(d) | AccessSize::ResultOfByte(d) | AccessSize::RelativeWith(d) => {
            convert_expr(ast, function_id, root_expr, d, var_map)
        }
        AccessSize::ArchitectureSize => wd(Expression::ArchitectureByteSize, root_expr),
        AccessSize::Unlimited => wd(Expression::Unknown, root_expr),
    }
}

fn calc_flags_automatically(
    ast: &mut CAst,
    function_id: FunctionId,
    operation: &Aos<IrData>,
    stmt_position: &IrStatementDescriptor,
    root_expr: &Aos<IrData>,
    affected_registers: &[Aos<IrData>],
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Vec<WrappedStatement> {
    // TODO INVALID
    let val = convert_expr(ast, function_id, root_expr, operation, var_map);
    let vars = ast.get_variables(&function_id).unwrap();
    affected_registers
        .iter()
        .filter_map(|reg| {
            var_map.get(reg).map(|&vid| {
                Statement::Assignment(
                    wd(Expression::Variable(vars.clone(), vid), root_expr),
                    val.clone(),
                )
            })
        })
        .map(|stmt| ws(stmt, *stmt_position))
        .collect()
}

/// TODO Need implement for constant access size
fn resolve_constant(
    position: &Address,
    instruction_arg_size: u8,
    root_expr: &Aos<IrData>,
    data: &Aos<IrData>,
) -> Option<Wrapped<CValue>> {
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
            | IrIntrinsic::InstructionByteSize => None,
        },
        IrData::Register(register) => match register.name() {
            "rip" | "eip" | "ip" => Some(CValue::Num(BigInt::from(position.get_virtual_address()))),
            _ => None,
        },
        IrData::Dereference(data) => {
            let c = resolve_constant(position, instruction_arg_size, root_expr, data)?;
            Some(CValue::Pointer(Box::new(c)))
        }
        IrData::Operation(IrDataOperation::Unary { operator, arg }) => {
            let arg = resolve_constant(position, instruction_arg_size, root_expr, arg)?;
            match operator {
                IrUnaryOp::Not => Some(CValue::Bool(!*arg.bool()?)),
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
            let arg1 = resolve_constant(position, instruction_arg_size, root_expr, arg1)?;
            let arg2 = resolve_constant(position, instruction_arg_size, root_expr, arg2)?;
            match operator {
                IrBinaryOp::And => Some(CValue::Bool(arg1.bool()? & arg2.bool()?)),
                IrBinaryOp::Or => Some(CValue::Bool(arg1.bool()? | arg2.bool()?)),
                IrBinaryOp::Xor => Some(CValue::Bool(arg1.bool()? ^ arg2.bool()?)),
                IrBinaryOp::Shl => Some(CValue::Num(
                    arg1.num()? << arg2.num()?.to_biguint()?.to_u64_digits()[0],
                )),
                IrBinaryOp::Shr => Some(CValue::Num(
                    arg1.num()? >> arg2.num()?.to_biguint()?.to_u64_digits()[0],
                )),
                IrBinaryOp::Sar => Some(CValue::Num(
                    arg1.num()? >> arg2.num()?.to_biguint()?.to_u64_digits()[0],
                )),
                IrBinaryOp::Add => Some(CValue::Num(arg1.num()? + arg2.num()?)),
                IrBinaryOp::Sub => Some(CValue::Num(arg1.num()? - arg2.num()?)),
                IrBinaryOp::Mul => Some(CValue::Num(arg1.num()? * arg2.num()?)),
                IrBinaryOp::SignedDiv => Some(CValue::Num(arg1.num()? / arg2.num()?)),
                IrBinaryOp::SignedRem => Some(CValue::Num(arg1.num()? % arg2.num()?)),
                IrBinaryOp::UnsignedDiv => Some(CValue::Num(arg1.num()? / arg2.num()?)),
                IrBinaryOp::UnsignedRem => Some(CValue::Num(arg1.num()? % arg2.num()?)),
                IrBinaryOp::Equal(..) => Some(CValue::Bool(arg1 == arg2)),
                IrBinaryOp::SignedLess(..) => Some(CValue::Bool(arg1.num()? < arg2.num()?)),
                IrBinaryOp::SignedLessOrEqual(..) => Some(CValue::Bool(arg1.num()? <= arg2.num()?)),
                IrBinaryOp::UnsignedLess(..) => Some(CValue::Bool(arg1.num()? < arg2.num()?)),
                IrBinaryOp::UnsignedLessOrEqual(..) => {
                    Some(CValue::Bool(arg1.num()? <= arg2.num()?))
                }
            }
        }
        IrData::Operand(..) => unreachable!("With {}, {}", position, data),
    };
    result.map(|c| wd(c, root_expr))
}
