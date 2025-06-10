//! IR statement executor for simulation
//!
//! This module provides the execution engine that interprets IR statements
//! and updates the simulation context accordingly.

use crate::ir::{data::*, operator::*, statements::*};
use crate::simulation::symbolic::SymbolicValue;
use crate::simulation::{SimulationContext, SimulationError, SimulationResult};
use crate::utils::Aos;

/// Jump type for execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JumpType {
    Jump,
    Call,
    Return,
}

/// Executor for IR statements
pub struct Executor<'a> {
    context: &'a mut SimulationContext,
}

impl<'a> Executor<'a> {
    /// Create a new executor
    pub fn new(context: &'a mut SimulationContext) -> Self {
        Self { context }
    }

    /// Execute a single IR statement
    pub fn execute_statement(&mut self, statement: &IrStatement) -> SimulationResult<()> {
        match statement {
            IrStatement::Assignment { from, to, size } => self.execute_assignment(size, from, to),
            IrStatement::Jump { target } => self.execute_jump(target, JumpType::Jump),
            IrStatement::JumpByCall { target } => self.execute_jump(target, JumpType::Call),
            IrStatement::Halt => {
                self.execute_jump(&Aos::new(IrData::Constant(0)), JumpType::Return)
            }
            IrStatement::Condition {
                condition,
                true_branch,
                false_branch,
            } => self.execute_condition(condition, true_branch, false_branch),
            IrStatement::Special(special) => self.execute_special(special),
            IrStatement::Undefined => {
                // Undefined values are handled as unknown
                Ok(())
            }
            IrStatement::Exception(msg) => Err(SimulationError::InvalidOperation(format!(
                "Exception: {}",
                msg
            ))),
            IrStatement::Atomic { statement, .. } => {
                // For simulation, atomic operations are treated the same as non-atomic
                // The memory ordering is only relevant for code generation
                self.execute_statement(statement)
            }
        }
    }

    /// Execute an assignment statement
    fn execute_assignment(
        &mut self,
        size: &AccessSize,
        from: &Aos<IrData>,
        to: &Aos<IrData>,
    ) -> SimulationResult<()> {
        let value = self.evaluate_data(from)?;
        self.store_data(to, value, size)?;
        Ok(())
    }

    /// Execute a jump statement
    fn execute_jump(&mut self, target: &Aos<IrData>, jump_type: JumpType) -> SimulationResult<()> {
        let target_addr = self.evaluate_data(target)?;

        match jump_type {
            JumpType::Jump => {
                // Direct jump
                self.context.cpu_state.rip = target_addr;
            }
            JumpType::Call => {
                // Push return address and jump
                let return_addr = self.context.cpu_state.rip;
                let new_rsp = self
                    .context
                    .memory
                    .allocate_stack(self.context.cpu_state.rsp, 8)?;
                self.context.memory.write_u64(new_rsp, return_addr)?;
                self.context.cpu_state.rsp = new_rsp;
                self.context.cpu_state.rip = target_addr;
            }
            JumpType::Return => {
                // Pop return address and jump
                let return_addr = self.context.memory.read_u64(self.context.cpu_state.rsp)?;
                self.context.cpu_state.rsp += 8;
                self.context.cpu_state.rip = return_addr;
            }
        }

        Ok(())
    }

    /// Execute a conditional statement
    fn execute_condition(
        &mut self,
        condition: &Aos<IrData>,
        true_branch: &[IrStatement],
        false_branch: &[IrStatement],
    ) -> SimulationResult<()> {
        let cond_value = self.evaluate_data(condition)?;

        let statements = if cond_value != 0 {
            true_branch
        } else {
            false_branch
        };

        for stmt in statements {
            self.execute_statement(stmt)?;
        }

        Ok(())
    }

    /// Execute special IR statements
    fn execute_special(&mut self, special: &IrStatementSpecial) -> SimulationResult<()> {
        match special {
            IrStatementSpecial::TypeSpecified { .. } => {
                // Type specifications don't affect execution
                Ok(())
            }
            IrStatementSpecial::CalcFlagsAutomatically { .. } => {
                // TODO: Implement flag calculation
                Ok(())
            }
            IrStatementSpecial::Assertion { condition } => {
                let value = self.evaluate_data(condition)?;
                if value == 0 {
                    return Err(SimulationError::InvalidOperation(
                        "Assertion failed".to_string(),
                    ));
                }
                Ok(())
            }
        }
    }

    /// Evaluate IR data to a concrete value
    fn evaluate_data(&mut self, data: &Aos<IrData>) -> SimulationResult<u64> {
        // If symbolic execution is enabled, create symbolic values
        if let Some(ref mut _symbolic) = self.context.symbolic_engine {
            let sym_value = self.evaluate_symbolic(data)?;

            // Try to get concrete value if possible
            if let Some(concrete) = sym_value.as_concrete() {
                return Ok(concrete);
            }

            // Otherwise, we need to handle symbolic execution
            return Err(SimulationError::UnsupportedFeature(
                "Full symbolic execution not yet implemented".to_string(),
            ));
        }

        // Concrete execution
        match data.as_ref() {
            IrData::Constant(v) => Ok(*v as u64),

            IrData::Register(reg) => self.context.cpu_state.get_register(reg.name()),

            IrData::Dereference(addr_data) => {
                let address = self.evaluate_data(addr_data)?;
                // Assume 64-bit dereference by default
                self.context.memory.read_u64(address)
            }

            IrData::Operation(op) => self.evaluate_operation(op),

            IrData::Intrinsic(intrinsic) => self.evaluate_intrinsic(intrinsic),

            _ => Err(SimulationError::UnsupportedFeature(format!(
                "IR data type: {:?}",
                data
            ))),
        }
    }

    /// Evaluate symbolic data
    fn evaluate_symbolic(&mut self, data: &Aos<IrData>) -> SimulationResult<SymbolicValue> {
        match data.as_ref() {
            IrData::Constant(v) => Ok(SymbolicValue::concrete(*v as u64)),

            IrData::Register(reg) => {
                let symbolic = self.context.symbolic_engine.as_mut().unwrap();
                Ok(symbolic.get_register(reg.name()))
            }

            IrData::Dereference(addr_data) => {
                let addr = self.evaluate_symbolic(addr_data)?;
                if let Some(concrete_addr) = addr.as_concrete() {
                    let symbolic = self.context.symbolic_engine.as_mut().unwrap();
                    Ok(symbolic.get_memory(concrete_addr))
                } else {
                    Ok(SymbolicValue::Dereference(Box::new(addr)))
                }
            }

            IrData::Operation(op) => self.evaluate_symbolic_operation(op),

            _ => Ok(SymbolicValue::Unknown),
        }
    }

    /// Evaluate an operation
    fn evaluate_operation(&mut self, op: &IrDataOperation) -> SimulationResult<u64> {
        match op {
            IrDataOperation::Unary { operator, arg } => {
                let arg_val = self.evaluate_data(arg)?;
                match operator {
                    UnaryOperator::Not => Ok(!arg_val),
                    UnaryOperator::Negation => Ok((!arg_val).wrapping_add(1)),
                    UnaryOperator::ZeroExtend => Ok(arg_val), // Already zero-extended
                    UnaryOperator::SignExtend => {
                        // TODO: Implement proper sign extension based on source size
                        Ok(arg_val)
                    }
                }
            }

            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => {
                let val1 = self.evaluate_data(arg1)?;
                let val2 = self.evaluate_data(arg2)?;

                match operator {
                    BinaryOperator::Add => Ok(val1.wrapping_add(val2)),
                    BinaryOperator::Sub => Ok(val1.wrapping_sub(val2)),
                    BinaryOperator::Mul => Ok(val1.wrapping_mul(val2)),
                    BinaryOperator::And => Ok(val1 & val2),
                    BinaryOperator::Or => Ok(val1 | val2),
                    BinaryOperator::Xor => Ok(val1 ^ val2),
                    BinaryOperator::Shl => Ok(val1 << (val2 & 63)),
                    BinaryOperator::Shr => Ok(val1 >> (val2 & 63)),
                    BinaryOperator::Sar => Ok(((val1 as i64) >> (val2 & 63)) as u64),
                    BinaryOperator::Equal(_) => Ok(if val1 == val2 { 1 } else { 0 }),
                    BinaryOperator::SignedLess(_) => {
                        Ok(if (val1 as i64) < (val2 as i64) { 1 } else { 0 })
                    }
                    BinaryOperator::UnsignedLess(_) => Ok(if val1 < val2 { 1 } else { 0 }),
                    BinaryOperator::SignedLessOrEqual(_) => {
                        Ok(if (val1 as i64) <= (val2 as i64) { 1 } else { 0 })
                    }
                    BinaryOperator::UnsignedLessOrEqual(_) => Ok(if val1 <= val2 { 1 } else { 0 }),
                    BinaryOperator::SignedDiv => {
                        if val2 == 0 {
                            return Err(SimulationError::DivisionByZero);
                        }
                        Ok(((val1 as i64) / (val2 as i64)) as u64)
                    }
                    BinaryOperator::UnsignedDiv => {
                        if val2 == 0 {
                            return Err(SimulationError::DivisionByZero);
                        }
                        Ok(val1 / val2)
                    }
                    BinaryOperator::SignedRem => {
                        if val2 == 0 {
                            return Err(SimulationError::DivisionByZero);
                        }
                        Ok(((val1 as i64) % (val2 as i64)) as u64)
                    }
                    BinaryOperator::UnsignedRem => {
                        if val2 == 0 {
                            return Err(SimulationError::DivisionByZero);
                        }
                        Ok(val1 % val2)
                    }
                }
            }
        }
    }

    /// Evaluate a symbolic operation
    fn evaluate_symbolic_operation(
        &mut self,
        op: &IrDataOperation,
    ) -> SimulationResult<SymbolicValue> {
        match op {
            IrDataOperation::Unary { operator, arg } => {
                let arg_val = self.evaluate_symbolic(arg)?;
                Ok(SymbolicValue::UnaryOp {
                    op: *operator,
                    operand: Box::new(arg_val),
                })
            }

            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => {
                let val1 = self.evaluate_symbolic(arg1)?;
                let val2 = self.evaluate_symbolic(arg2)?;
                Ok(SymbolicValue::BinaryOp {
                    op: operator.clone(),
                    left: Box::new(val1),
                    right: Box::new(val2),
                })
            }
        }
    }

    /// Evaluate an intrinsic
    fn evaluate_intrinsic(&mut self, intrinsic: &IrIntrinsic) -> SimulationResult<u64> {
        match intrinsic {
            IrIntrinsic::ArchitectureByteSize => Ok(8), // x86_64
            IrIntrinsic::ArchitectureBitSize => Ok(64), // x86_64
            IrIntrinsic::ArchitectureBitPerByte => Ok(8),
            IrIntrinsic::Unknown => Ok(0), // Default to 0 for unknown values
            _ => Err(SimulationError::UnsupportedFeature(format!(
                "Intrinsic: {:?}",
                intrinsic
            ))),
        }
    }

    /// Store data to a destination
    fn store_data(
        &mut self,
        destination: &Aos<IrData>,
        value: u64,
        size: &AccessSize,
    ) -> SimulationResult<()> {
        match destination.as_ref() {
            IrData::Register(reg) => {
                self.context.cpu_state.set_register(reg.name(), value)?;

                // Update symbolic state if enabled
                if let Some(ref mut symbolic) = self.context.symbolic_engine {
                    symbolic.set_register(reg.name().to_string(), SymbolicValue::concrete(value));
                }
                Ok(())
            }

            IrData::Dereference(addr_data) => {
                let address = self.evaluate_data(addr_data)?;

                // Determine size in bytes
                let byte_size = match size {
                    AccessSize::Unlimited => 8,        // Default to 64-bit
                    AccessSize::ArchitectureSize => 8, // x86_64
                    AccessSize::RelativeWith(_) => 8,  // TODO: Proper size calculation
                    AccessSize::ResultOfBit(_) => 1,
                    AccessSize::ResultOfByte(data) => self.evaluate_data(data)? as usize,
                };

                // Write to memory based on size
                match byte_size {
                    1 => self.context.memory.write_u8(address, value as u8)?,
                    2 => self.context.memory.write_u16(address, value as u16)?,
                    4 => self.context.memory.write_u32(address, value as u32)?,
                    8 => self.context.memory.write_u64(address, value)?,
                    _ => {
                        return Err(SimulationError::UnsupportedFeature(format!(
                            "Memory write size: {}",
                            byte_size
                        )));
                    }
                }

                // Update symbolic state if enabled
                if let Some(ref mut symbolic) = self.context.symbolic_engine {
                    symbolic.set_memory(address, SymbolicValue::concrete(value));
                }
                Ok(())
            }

            _ => Err(SimulationError::InvalidOperation(
                "Cannot store to non-lvalue".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{VirtualMachine, x86_64::X64Range as X64};

    #[test]
    fn test_simple_assignment() {
        let mut ctx = SimulationContext::new();

        // Create assignment: rax = 0x42
        let rax = Aos::new(IrData::Register(<VirtualMachine as X64>::rax()));
        let stmt = IrStatement::Assignment {
            size: AccessSize::ArchitectureSize,
            from: Aos::new(IrData::Constant(0x42)),
            to: rax,
        };

        ctx.execute_statement(&stmt).unwrap();
        assert_eq!(ctx.cpu_state.get_register("rax").unwrap(), 0x42);
    }

    #[test]
    fn test_binary_operation() {
        let mut ctx = SimulationContext::new();

        // Set initial values
        ctx.cpu_state.set_register("rax", 10).unwrap();
        ctx.cpu_state.set_register("rbx", 20).unwrap();

        // Create assignment: rcx = rax + rbx
        let rax = Aos::new(IrData::Register(<VirtualMachine as X64>::rax()));
        let rbx = Aos::new(IrData::Register(<VirtualMachine as X64>::rbx()));
        let rcx = Aos::new(IrData::Register(<VirtualMachine as X64>::rcx()));

        let add_op = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Add,
            arg1: rax,
            arg2: rbx,
        });

        let stmt = IrStatement::Assignment {
            size: AccessSize::ArchitectureSize,
            from: Aos::new(add_op),
            to: rcx,
        };

        ctx.execute_statement(&stmt).unwrap();
        assert_eq!(ctx.cpu_state.get_register("rcx").unwrap(), 30);
    }

    #[test]
    fn test_memory_operations() {
        let mut ctx = SimulationContext::new();

        // Write to memory
        let addr = 0x1000;
        ctx.memory.write_u64(addr, 0xdeadbeef).unwrap();

        // Create dereference: rax = [0x1000]
        let rax = Aos::new(IrData::Register(<VirtualMachine as X64>::rax()));
        let deref = IrData::Dereference(Aos::new(IrData::Constant(addr as usize)));
        let stmt = IrStatement::Assignment {
            size: AccessSize::ArchitectureSize,
            from: Aos::new(deref),
            to: rax,
        };

        ctx.execute_statement(&stmt).unwrap();
        assert_eq!(ctx.cpu_state.get_register("rax").unwrap(), 0xdeadbeef);
    }

    #[test]
    fn test_condition_execution() {
        let mut ctx = SimulationContext::new();

        // Set initial values
        ctx.cpu_state.set_register("rax", 5).unwrap();
        ctx.cpu_state.set_register("rbx", 10).unwrap();

        // Create comparison: rax < rbx
        let rax = Aos::new(IrData::Register(<VirtualMachine as X64>::rax()));
        let rbx = Aos::new(IrData::Register(<VirtualMachine as X64>::rbx()));
        let rcx = Aos::new(IrData::Register(<VirtualMachine as X64>::rcx()));
        let rdx = Aos::new(IrData::Register(<VirtualMachine as X64>::rdx()));

        let cmp = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::UnsignedLess(AccessSize::ArchitectureSize),
            arg1: rax,
            arg2: rbx,
        });

        // Create conditional: if (rax < rbx) { rcx = 1 } else { rdx = 1 }
        let true_branch = vec![IrStatement::Assignment {
            size: AccessSize::ArchitectureSize,
            from: Aos::new(IrData::Constant(1)),
            to: rcx,
        }];

        let false_branch = vec![IrStatement::Assignment {
            size: AccessSize::ArchitectureSize,
            from: Aos::new(IrData::Constant(1)),
            to: rdx,
        }];

        let stmt = IrStatement::Condition {
            condition: Aos::new(cmp),
            true_branch: true_branch.into_boxed_slice(),
            false_branch: false_branch.into_boxed_slice(),
        };

        ctx.execute_statement(&stmt).unwrap();

        // Since 5 < 10, rcx should be 1 and rdx should be 0
        assert_eq!(ctx.cpu_state.get_register("rcx").unwrap(), 1);
        assert_eq!(ctx.cpu_state.get_register("rdx").unwrap(), 0);
    }
}
