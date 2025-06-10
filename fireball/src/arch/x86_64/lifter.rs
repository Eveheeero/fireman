//! x86_64 to Low IR Lifter
//!
//! This module bridges between the existing x86_64 instruction analysis
//! and the new Low IR representation.

use crate::{
    core::Address,
    ir::{IrBlock, data::IrDataOperation, low_ir::*},
};
use std::collections::BTreeMap;

/// x86_64 to Low IR lifter that converts from existing IR to Low IR
pub struct X64Lifter {
    /// Temporary allocator for deterministic naming
    temp_alloc: TempAllocator,

    /// Current function's locals
    locals: BTreeMap<LocalId, Type>,
}

impl X64Lifter {
    pub fn new() -> Self {
        Self {
            temp_alloc: TempAllocator::new(),
            locals: BTreeMap::new(),
        }
    }

    /// Reset state for new function (CRITICAL for determinism)
    pub fn reset_for_function(&mut self) {
        self.temp_alloc.reset();
        self.locals.clear();
    }

    /// Convert IrBlock to Low IR Module
    pub fn lift_block(
        &mut self,
        ir_block: &IrBlock,
        start_addr: Address,
    ) -> Result<Module, String> {
        let mut module = Module {
            target: TargetInfo {
                arch: "x86_64".to_string(),
                bits: 64,
                endian: Endianness::Little,
            },
            globals: BTreeMap::new(),
            functions: BTreeMap::new(),
            externals: BTreeMap::new(),
        };

        // Create a single function for the block
        let func_id = FunctionId(start_addr.get_virtual_address());

        let mut function = Function {
            id: func_id.clone(),
            signature: Type::Function {
                ret: Box::new(Type::Void),
                params: Vec::new(),
                varargs: false,
            },
            entry: BlockId(start_addr.get_virtual_address()),
            blocks: BTreeMap::new(),
            locals: BTreeMap::new(),
        };

        // Convert IR statements to Low IR
        let mut instructions = Vec::new();
        for ir in ir_block.ir() {
            if let Some(stmts) = ir.statements {
                for stmt in stmts {
                    let low_ir_insts = self.convert_ir_statement(stmt, ir.address.clone())?;
                    instructions.extend(low_ir_insts);
                }
            }
        }

        // Create basic block
        let bb = BasicBlock {
            id: BlockId(start_addr.get_virtual_address()),
            phis: Vec::new(),
            instructions,
            terminator: Terminator::Return(None), // Default terminator
        };

        function.blocks.insert(bb.id.clone(), bb);
        function.locals = self.locals.clone();
        module.functions.insert(func_id, function);

        Ok(module)
    }

    /// Convert a single IR statement to Low IR instructions
    fn convert_ir_statement(
        &mut self,
        stmt: &crate::ir::statements::IrStatement,
        addr: Address,
    ) -> Result<Vec<Instruction>, String> {
        use crate::ir::statements::IrStatement;

        match stmt {
            IrStatement::Assignment { from, to, size } => {
                self.convert_assignment(from, to, size, addr)
            }
            IrStatement::Jump { target } => self.convert_jump(target, addr),
            IrStatement::JumpByCall { target } => self.convert_call(target, addr),
            IrStatement::Halt => {
                Ok(vec![]) // Return instruction - handled by terminator
            }
            IrStatement::Condition {
                condition,
                true_branch,
                false_branch,
            } => self.convert_condition(condition, true_branch, false_branch, addr),
            IrStatement::Special(special) => self.convert_special(special, addr),
            _ => Err(format!("Unimplemented IR statement: {:?}", stmt)),
        }
    }

    /// Convert Assignment IR to Low IR
    fn convert_assignment(
        &mut self,
        from: &crate::utils::Aos<crate::ir::data::IrData>,
        to: &crate::utils::Aos<crate::ir::data::IrData>,
        _size: &crate::ir::data::AccessSize,
        addr: Address,
    ) -> Result<Vec<Instruction>, String> {
        let mut result = Vec::new();

        // Convert source value
        let (src_val, _src_ty, mut pre_src) = self.convert_ir_data(from, addr.clone(), false)?;
        result.append(&mut pre_src);

        // Convert destination
        let (dst_local, mut store_insts) = self.convert_ir_data_lvalue(to, addr.clone())?;

        // Create assignment
        result.push(Instruction::Assign {
            dst: dst_local,
            value: src_val,
            source_addr: addr,
        });

        result.append(&mut store_insts);
        Ok(result)
    }

    /// Convert Jump IR to Low IR
    fn convert_jump(
        &mut self,
        _target: &crate::utils::Aos<crate::ir::data::IrData>,
        _addr: Address,
    ) -> Result<Vec<Instruction>, String> {
        // Jumps are handled as terminators, not regular instructions
        Ok(vec![])
    }

    /// Convert Call IR to Low IR
    fn convert_call(
        &mut self,
        _target: &crate::utils::Aos<crate::ir::data::IrData>,
        _addr: Address,
    ) -> Result<Vec<Instruction>, String> {
        // Calls are handled as terminators after saving return address
        Ok(vec![])
    }

    /// Convert Condition IR to Low IR
    fn convert_condition(
        &mut self,
        condition: &crate::utils::Aos<crate::ir::data::IrData>,
        _true_branch: &[crate::ir::statements::IrStatement],
        _false_branch: &[crate::ir::statements::IrStatement],
        addr: Address,
    ) -> Result<Vec<Instruction>, String> {
        let mut result = Vec::new();

        // Convert condition
        let (_cond_val, _, mut pre_cond) = self.convert_ir_data(condition, addr.clone(), false)?;
        result.append(&mut pre_cond);

        // For now, we'll need to handle branches at a higher level
        // This would require control flow analysis

        Ok(result)
    }

    /// Convert Special IR statements
    fn convert_special(
        &mut self,
        special: &crate::ir::statements::IrStatementSpecial,
        _addr: Address,
    ) -> Result<Vec<Instruction>, String> {
        use crate::ir::statements::IrStatementSpecial;

        match special {
            IrStatementSpecial::CalcFlagsAutomatically {
                operation: _,
                size: _,
                flags: _,
            } => {
                // Flag calculations are implicit in Low IR operations
                Ok(vec![])
            }
            _ => Ok(vec![]),
        }
    }

    /// Convert IrData to Low IR Value
    fn convert_ir_data(
        &mut self,
        data: &crate::utils::Aos<crate::ir::data::IrData>,
        addr: Address,
        is_write: bool,
    ) -> Result<(Value, Type, Vec<Instruction>), String> {
        use crate::ir::data::IrData;

        match data.as_ref() {
            IrData::Constant(c) => {
                let ty = Type::I64; // Default type
                let const_val = Constant::Int {
                    value: *c as i128,
                    ty: ty.clone(),
                };
                Ok((Value::Constant(const_val), ty, vec![]))
            }
            IrData::Register(reg) => {
                let local = self.register_from_ir(reg, addr, is_write)?;
                let ty = self.register_type_from_ir(reg)?;
                Ok((Value::Local(local), ty, vec![]))
            }
            IrData::Dereference(inner) => {
                let mut insts = Vec::new();

                // Get address
                let (addr_val, _, mut addr_insts) =
                    self.convert_ir_data(inner, addr.clone(), false)?;
                insts.append(&mut addr_insts);

                // Load from memory
                let load_temp = self.temp_alloc.new_temp(addr, "load");
                let ty = Type::I64; // Default to 64-bit
                self.locals.insert(load_temp.clone(), ty.clone());

                insts.push(Instruction::Load {
                    dst: load_temp.clone(),
                    ptr: addr_val,
                    ty: ty.clone(),
                    align: None,
                    volatile: false,
                });

                Ok((Value::Local(load_temp), ty, insts))
            }
            IrData::Operation(IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            }) => {
                let mut insts = Vec::new();

                // Convert operands
                let (lhs_val, lhs_ty, mut lhs_insts) =
                    self.convert_ir_data(arg1, addr.clone(), false)?;
                let (rhs_val, _rhs_ty, mut rhs_insts) =
                    self.convert_ir_data(arg2, addr.clone(), false)?;
                insts.append(&mut lhs_insts);
                insts.append(&mut rhs_insts);

                // Convert operator
                let op = self.convert_operator(operator)?;

                // Create temporary for result
                let result_temp = self.temp_alloc.new_temp(addr, "binop");
                let ty = lhs_ty; // Use LHS type
                self.locals.insert(result_temp.clone(), ty.clone());

                insts.push(Instruction::BinOp {
                    op,
                    dst: result_temp.clone(),
                    lhs: lhs_val,
                    rhs: rhs_val,
                    ty: ty.clone(),
                    flags: FlagUpdate::Unchanged,
                });

                Ok((Value::Local(result_temp), ty, insts))
            }
            IrData::Operation(IrDataOperation::Unary { operator, arg }) => {
                let mut insts = Vec::new();

                // Convert operand
                let (val, ty, mut val_insts) = self.convert_ir_data(arg, addr.clone(), false)?;
                insts.append(&mut val_insts);

                // Convert operator
                let op = self.convert_unary_operator(operator)?;

                // Create temporary for result
                let result_temp = self.temp_alloc.new_temp(addr, "unop");
                self.locals.insert(result_temp.clone(), ty.clone());

                insts.push(Instruction::UnOp {
                    op,
                    dst: result_temp.clone(),
                    src: val,
                    ty: ty.clone(),
                    flags: FlagUpdate::Unchanged,
                });

                Ok((Value::Local(result_temp), ty, insts))
            }
            _ => Err(format!("Unsupported IrData: {:?}", data)),
        }
    }

    /// Convert IrData to lvalue (for assignments)
    fn convert_ir_data_lvalue(
        &mut self,
        data: &crate::utils::Aos<crate::ir::data::IrData>,
        addr: Address,
    ) -> Result<(LocalId, Vec<Instruction>), String> {
        use crate::ir::data::IrData;

        match data.as_ref() {
            IrData::Register(reg) => {
                let local = self.register_from_ir(reg, addr, true)?;
                Ok((local, vec![]))
            }
            IrData::Dereference(_inner) => {
                // For memory stores, we return a temporary and generate store instruction
                let temp = self.temp_alloc.new_temp(addr, "store_temp");
                self.locals.insert(temp.clone(), Type::I64);
                Ok((temp, vec![]))
            }
            _ => Err(format!("Cannot use {:?} as lvalue", data)),
        }
    }

    /// Convert IR Register to LocalId
    fn register_from_ir(
        &mut self,
        reg: &crate::ir::Register,
        addr: Address,
        is_write: bool,
    ) -> Result<LocalId, String> {
        // Extract register name from bit range
        // This is a simplified version - in reality we'd map bit ranges to register names
        let name = "reg"; // Placeholder

        let local = if is_write {
            self.temp_alloc.new_temp(addr, name)
        } else {
            LocalId {
                source: addr,
                purpose: name,
                index: 0,
                version: 0,
            }
        };

        let ty = self.register_type_from_ir(reg)?;
        self.locals.insert(local.clone(), ty);

        Ok(local)
    }

    /// Get register type from IR Register
    fn register_type_from_ir(&self, reg: &crate::ir::Register) -> Result<Type, String> {
        // Map bit size to type
        let bit_size = reg.bit_range().end - reg.bit_range().start;
        match bit_size {
            8 => Ok(Type::I8),
            16 => Ok(Type::I16),
            32 => Ok(Type::I32),
            64 => Ok(Type::I64),
            128 => Ok(Type::I128),
            _ => Err(format!("Unsupported register size: {} bits", bit_size)),
        }
    }

    /// Convert binary operator
    fn convert_operator(
        &self,
        op: &crate::ir::operator::BinaryOperator,
    ) -> Result<BinaryOp, String> {
        use crate::ir::operator::BinaryOperator;

        match op {
            BinaryOperator::Add => Ok(BinaryOp::Add),
            BinaryOperator::Sub => Ok(BinaryOp::Sub),
            BinaryOperator::Mul => Ok(BinaryOp::Mul),
            BinaryOperator::SignedDiv => Ok(BinaryOp::SDiv),
            BinaryOperator::SignedRem => Ok(BinaryOp::SRem),
            BinaryOperator::UnsignedDiv => Ok(BinaryOp::UDiv),
            BinaryOperator::UnsignedRem => Ok(BinaryOp::URem),
            BinaryOperator::And => Ok(BinaryOp::And),
            BinaryOperator::Or => Ok(BinaryOp::Or),
            BinaryOperator::Xor => Ok(BinaryOp::Xor),
            BinaryOperator::Shl => Ok(BinaryOp::Shl),
            BinaryOperator::Shr => Ok(BinaryOp::LShr),
            BinaryOperator::Sar => Ok(BinaryOp::AShr),
            BinaryOperator::Equal(_) => Ok(BinaryOp::Eq),
            BinaryOperator::SignedLess(_) => Ok(BinaryOp::Slt),
            BinaryOperator::SignedLessOrEqual(_) => Ok(BinaryOp::Sle),
            BinaryOperator::UnsignedLess(_) => Ok(BinaryOp::Ult),
            BinaryOperator::UnsignedLessOrEqual(_) => Ok(BinaryOp::Ule),
        }
    }

    /// Convert unary operator
    fn convert_unary_operator(
        &self,
        op: &crate::ir::operator::UnaryOperator,
    ) -> Result<UnaryOp, String> {
        use crate::ir::operator::UnaryOperator;

        match op {
            UnaryOperator::Not => Ok(UnaryOp::Not),
            UnaryOperator::Negation => Ok(UnaryOp::Neg),
            UnaryOperator::SignExtend | UnaryOperator::ZeroExtend => {
                // These are handled as Cast operations, not UnaryOp
                Err(format!(
                    "Extension operators should be Cast operations: {:?}",
                    op
                ))
            }
        }
    }
}
