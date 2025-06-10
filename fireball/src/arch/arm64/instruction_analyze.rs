//! ARM64 instruction analysis and IR generation

use crate::core::Instruction;
use crate::ir::statements::IrStatement;

/// Converts ARM64 assembly instructions into IR statements.
///
/// ### Arguments
/// - `instruction: &Instruction` : ARM64 assembly instruction
///
/// ### Returns
/// `Option<&'static [IrStatement]>` : IR statements corresponding to the ARM64 instruction
/// or `None` if the instruction is not supported.
pub fn create_ir_statement(instruction: &Instruction) -> Option<&'static [IrStatement]> {
    let op = match &instruction.inner.statement {
        Ok(iceball::Statement::Arm64(op)) => op,
        _ => return None,
    };

    use iceball::Arm64Statement;
    Some(match op {
        // Data processing instructions
        Arm64Statement::Add => data_processing::add(instruction),
        Arm64Statement::Sub => data_processing::sub(instruction),
        Arm64Statement::Mov => data_processing::mov(instruction),
        Arm64Statement::Movz => data_processing::movz(instruction),
        Arm64Statement::Movk => data_processing::movk(instruction),
        Arm64Statement::Movn => data_processing::movn(instruction),
        Arm64Statement::And => data_processing::and(instruction),
        Arm64Statement::Orr => data_processing::orr(instruction),
        Arm64Statement::Eor => data_processing::eor(instruction),
        Arm64Statement::Lsl => data_processing::lsl(instruction),
        Arm64Statement::Lsr => data_processing::lsr(instruction),
        Arm64Statement::Asr => data_processing::asr(instruction),

        // Load/Store instructions
        Arm64Statement::Ldr => load_store::ldr(instruction),
        Arm64Statement::Ldrb => load_store::ldrb(instruction),
        Arm64Statement::Ldrh => load_store::ldrh(instruction),
        Arm64Statement::Ldrsb => load_store::ldrsb(instruction),
        Arm64Statement::Ldrsh => load_store::ldrsh(instruction),
        Arm64Statement::Str => load_store::str(instruction),
        Arm64Statement::Strb => load_store::strb(instruction),
        Arm64Statement::Strh => load_store::strh(instruction),
        Arm64Statement::Ldp => load_store::ldp(instruction),
        Arm64Statement::Stp => load_store::stp(instruction),

        // Branch instructions
        Arm64Statement::B => branch::b(instruction),
        Arm64Statement::Bl => branch::bl(instruction),
        Arm64Statement::Br => branch::br(instruction),
        Arm64Statement::Blr => branch::blr(instruction),
        Arm64Statement::Ret => branch::ret(instruction),
        Arm64Statement::Beq => branch::beq(instruction),
        Arm64Statement::Bne => branch::bne(instruction),
        Arm64Statement::Bcs => branch::bcs(instruction),
        Arm64Statement::Bcc => branch::bcc(instruction),
        Arm64Statement::Bmi => branch::bmi(instruction),
        Arm64Statement::Bpl => branch::bpl(instruction),
        Arm64Statement::Bvs => branch::bvs(instruction),
        Arm64Statement::Bvc => branch::bvc(instruction),
        Arm64Statement::Bhi => branch::bhi(instruction),
        Arm64Statement::Bls => branch::bls(instruction),
        Arm64Statement::Bge => branch::bge(instruction),
        Arm64Statement::Blt => branch::blt(instruction),
        Arm64Statement::Bgt => branch::bgt(instruction),
        Arm64Statement::Ble => branch::ble(instruction),

        // Compare instructions
        Arm64Statement::Cmp => comparison::cmp(instruction),
        Arm64Statement::Cmn => comparison::cmn(instruction),
        Arm64Statement::Tst => comparison::tst(instruction),

        // Nop
        Arm64Statement::Nop => &[],

        _ => return None,
    })
}

// Data processing instruction implementations
mod data_processing {
    use crate::core::Instruction;
    use crate::ir::{
        data::{AccessSize, IrData, IrDataOperation},
        operator::BinaryOperator,
        statements::IrStatement,
    };

    /// ADD Rd, Rn, Rm - Add two registers
    pub fn add(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Add,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: (&inst.inner.arguments[2]).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// SUB Rd, Rn, Rm - Subtract two registers
    pub fn sub(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Sub,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: (&inst.inner.arguments[2]).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// MOV Rd, Rm - Move register to register
    pub fn mov(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: (&inst.inner.arguments[1]).into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// MOVZ Rd, #imm - Move immediate with zero
    pub fn movz(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: (&inst.inner.arguments[1]).into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// MOVK Rd, #imm - Move immediate with keep
    pub fn movk(inst: &Instruction) -> &'static [IrStatement] {
        // MOVK keeps other bits, so it's an OR operation with shifted immediate
        // This is simplified - full implementation would need shift amount
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Or,
                arg1: (&inst.inner.arguments[0]).into(),
                arg2: (&inst.inner.arguments[1]).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// MOVN Rd, #imm - Move NOT immediate
    pub fn movn(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Xor,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: IrData::Constant(0xFFFFFFFFFFFFFFFF).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// AND Rd, Rn, Rm - Bitwise AND
    pub fn and(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::And,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: (&inst.inner.arguments[2]).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// ORR Rd, Rn, Rm - Bitwise OR
    pub fn orr(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Or,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: (&inst.inner.arguments[2]).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// EOR Rd, Rn, Rm - Bitwise exclusive OR
    pub fn eor(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Xor,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: (&inst.inner.arguments[2]).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// LSL Rd, Rn, Rm - Logical shift left
    pub fn lsl(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Shl,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: (&inst.inner.arguments[2]).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// LSR Rd, Rn, Rm - Logical shift right
    pub fn lsr(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Shr,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: (&inst.inner.arguments[2]).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// ASR Rd, Rn, Rm - Arithmetic shift right
    pub fn asr(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Sar,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: (&inst.inner.arguments[2]).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }
}

mod load_store {
    use crate::core::Instruction;
    use crate::ir::{
        data::{AccessSize, IrData, IrDataOperation, IrIntrinsic},
        operator::BinaryOperator,
        statements::IrStatement,
    };

    /// LDR Rt, [Rn] - Load register (64-bit)
    pub fn ldr(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: (&inst.inner.arguments[1]).into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// LDRB Rt, [Rn] - Load register byte
    pub fn ldrb(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::And,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: IrData::Constant(0xFF).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// LDRH Rt, [Rn] - Load register halfword
    pub fn ldrh(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::And,
                arg1: (&inst.inner.arguments[1]).into(),
                arg2: IrData::Constant(0xFFFF).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// LDRSB Rt, [Rn] - Load register signed byte
    pub fn ldrsb(inst: &Instruction) -> &'static [IrStatement] {
        // Sign extend from byte
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Intrinsic(IrIntrinsic::Sized(
                (&inst.inner.arguments[1]).into(),
                AccessSize::ResultOfByte(IrData::Constant(8).into()),
            ))
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// LDRSH Rt, [Rn] - Load register signed halfword
    pub fn ldrsh(inst: &Instruction) -> &'static [IrStatement] {
        // Sign extend from halfword
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[0]).into(),
            from: IrData::Intrinsic(IrIntrinsic::Sized(
                (&inst.inner.arguments[1]).into(),
                AccessSize::ResultOfByte(IrData::Constant(16).into()),
            ))
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// STR Rt, [Rn] - Store register (64-bit)
    pub fn str(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[1]).into(),
            from: (&inst.inner.arguments[0]).into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// STRB Rt, [Rn] - Store register byte
    pub fn strb(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[1]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::And,
                arg1: (&inst.inner.arguments[0]).into(),
                arg2: IrData::Constant(0xFF).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// STRH Rt, [Rn] - Store register halfword
    pub fn strh(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Assignment {
            to: (&inst.inner.arguments[1]).into(),
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::And,
                arg1: (&inst.inner.arguments[0]).into(),
                arg2: IrData::Constant(0xFFFF).into(),
            })
            .into(),
            size: AccessSize::ArchitectureSize,
        }]))
    }

    /// LDP Rt1, Rt2, [Rn] - Load pair of registers
    pub fn ldp(inst: &Instruction) -> &'static [IrStatement] {
        // Load two consecutive 64-bit values
        Box::leak(Box::new([
            IrStatement::Assignment {
                to: (&inst.inner.arguments[0]).into(),
                from: (&inst.inner.arguments[2]).into(),
                size: AccessSize::ArchitectureSize,
            },
            IrStatement::Assignment {
                to: (&inst.inner.arguments[1]).into(),
                from: IrData::Dereference(
                    IrData::Operation(IrDataOperation::Binary {
                        operator: BinaryOperator::Add,
                        arg1: (&inst.inner.arguments[2]).into(),
                        arg2: IrData::Constant(8).into(),
                    })
                    .into(),
                )
                .into(),
                size: AccessSize::ArchitectureSize,
            },
        ]))
    }

    /// STP Rt1, Rt2, [Rn] - Store pair of registers
    pub fn stp(inst: &Instruction) -> &'static [IrStatement] {
        // Store two consecutive 64-bit values
        Box::leak(Box::new([
            IrStatement::Assignment {
                to: (&inst.inner.arguments[2]).into(),
                from: (&inst.inner.arguments[0]).into(),
                size: AccessSize::ArchitectureSize,
            },
            IrStatement::Assignment {
                to: IrData::Dereference(
                    IrData::Operation(IrDataOperation::Binary {
                        operator: BinaryOperator::Add,
                        arg1: (&inst.inner.arguments[2]).into(),
                        arg2: IrData::Constant(8).into(),
                    })
                    .into(),
                )
                .into(),
                from: (&inst.inner.arguments[1]).into(),
                size: AccessSize::ArchitectureSize,
            },
        ]))
    }
}

mod branch {
    use crate::core::Instruction;
    use crate::ir::{
        data::{AccessSize, IrData, IrDataOperation},
        operator::{BinaryOperator, UnaryOperator},
        statements::IrStatement,
    };
    use crate::utils::Aos;

    /// B target - Unconditional branch
    pub fn b(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Jump {
            target: (&inst.inner.arguments[0]).into(),
        }]))
    }

    /// BL target - Branch with link (call)
    pub fn bl(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::JumpByCall {
            target: (&inst.inner.arguments[0]).into(),
        }]))
    }

    /// BR Xn - Branch to register
    pub fn br(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Jump {
            target: (&inst.inner.arguments[0]).into(),
        }]))
    }

    /// BLR Xn - Branch with link to register
    pub fn blr(inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::JumpByCall {
            target: (&inst.inner.arguments[0]).into(),
        }]))
    }

    /// RET - Return from subroutine
    pub fn ret(_inst: &Instruction) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Halt]))
    }

    // Conditional branch helpers
    fn conditional_branch(inst: &Instruction, condition: Aos<IrData>) -> &'static [IrStatement] {
        Box::leak(Box::new([IrStatement::Condition {
            condition,
            true_branch: Box::new([IrStatement::Jump {
                target: (&inst.inner.arguments[0]).into(),
            }]),
            false_branch: Box::new([]),
        }]))
    }

    /// BEQ target - Branch if equal
    pub fn beq(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;

        let condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Equal(AccessSize::ArchitectureSize),
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0).into(), // Z flag set
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BNE target - Branch if not equal
    pub fn bne(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;

        // Not equal is implemented as NOT(Equal)
        let eq_condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Equal(AccessSize::ArchitectureSize),
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0).into(), // Z flag clear
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::Not,
            arg: eq_condition,
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BCS target - Branch if carry set (unsigned >=)
    pub fn bcs(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;

        // Check C flag is set
        let condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x20000000).into(), // C flag bit
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BCC target - Branch if carry clear (unsigned <)
    pub fn bcc(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;
        use crate::ir::operator::UnaryOperator;

        // Check C flag is clear
        let c_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x20000000).into(), // C flag bit
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::Not,
            arg: c_flag,
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BMI target - Branch if minus (negative)
    pub fn bmi(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;

        // Check N flag is set
        let condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x80000000).into(), // N flag bit
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BPL target - Branch if plus (positive or zero)
    pub fn bpl(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;
        use crate::ir::operator::UnaryOperator;

        // Check N flag is clear
        let n_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x80000000).into(), // N flag bit
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::Not,
            arg: n_flag,
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BVS target - Branch if overflow set
    pub fn bvs(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;

        // Check V flag is set
        let condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x10000000).into(), // V flag bit
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BVC target - Branch if overflow clear
    pub fn bvc(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;
        use crate::ir::operator::UnaryOperator;

        // Check V flag is clear
        let v_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x10000000).into(), // V flag bit
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::Not,
            arg: v_flag,
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BHI target - Branch if unsigned higher (C set and Z clear)
    pub fn bhi(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;

        // Check C set AND Z clear
        let c_set = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x20000000).into(), // C flag bit
        })
        .into();

        let z_clear = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x40000000).into(), // Z flag bit
        })
        .into();

        let z_is_clear = IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::Not,
            arg: z_clear,
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: c_set,
            arg2: z_is_clear,
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BLS target - Branch if unsigned lower or same (C clear or Z set)
    pub fn bls(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;
        use crate::ir::operator::UnaryOperator;

        // Check C clear OR Z set
        let c_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x20000000).into(), // C flag bit
        })
        .into();

        let c_clear = IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::Not,
            arg: c_flag,
        })
        .into();

        let z_set = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x40000000).into(), // Z flag bit
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Or,
            arg1: c_clear,
            arg2: z_set,
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BGE target - Branch if signed greater or equal (N == V)
    pub fn bge(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;

        // Check N == V
        let n_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x80000000).into(), // N flag bit
        })
        .into();

        let v_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x10000000).into(), // V flag bit
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Equal(AccessSize::ArchitectureSize),
            arg1: n_flag,
            arg2: v_flag,
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BLT target - Branch if signed less than (N != V)
    pub fn blt(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;

        // Check N != V
        let n_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x80000000).into(), // N flag bit
        })
        .into();

        let v_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x10000000).into(), // V flag bit
        })
        .into();

        let eq_cond = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Equal(AccessSize::ArchitectureSize),
            arg1: n_flag,
            arg2: v_flag,
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::Not,
            arg: eq_cond,
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BGT target - Branch if signed greater than (Z clear and N == V)
    pub fn bgt(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;
        use crate::ir::operator::UnaryOperator;

        // Check Z clear AND (N == V)
        let z_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x40000000).into(), // Z flag bit
        })
        .into();

        let z_clear = IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::Not,
            arg: z_flag,
        })
        .into();

        let n_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x80000000).into(), // N flag bit
        })
        .into();

        let v_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x10000000).into(), // V flag bit
        })
        .into();

        let n_eq_v = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Equal(AccessSize::ArchitectureSize),
            arg1: n_flag,
            arg2: v_flag,
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: z_clear,
            arg2: n_eq_v,
        })
        .into();
        conditional_branch(inst, condition)
    }

    /// BLE target - Branch if signed less or equal (Z set or N != V)
    pub fn ble(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;

        // Check Z set OR (N != V)
        let z_set = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x40000000).into(), // Z flag bit
        })
        .into();

        let n_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x80000000).into(), // N flag bit
        })
        .into();

        let v_flag = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: register::NZCV.clone(),
            arg2: IrData::Constant(0x10000000).into(), // V flag bit
        })
        .into();

        let n_neq_v = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Equal(AccessSize::ArchitectureSize),
            arg1: n_flag,
            arg2: v_flag,
        })
        .into();

        let condition = IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Or,
            arg1: z_set,
            arg2: n_neq_v,
        })
        .into();
        conditional_branch(inst, condition)
    }
}

mod comparison {
    use crate::core::Instruction;
    use crate::ir::{
        data::{AccessSize, IrData, IrDataOperation},
        operator::BinaryOperator,
        statements::IrStatement,
    };

    /// CMP Rn, Rm - Compare (Rn - Rm and set flags)
    pub fn cmp(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;
        Box::leak(Box::new([
            // Set NZCV flags based on subtraction result
            IrStatement::Assignment {
                to: register::NZCV.clone(),
                from: IrData::Operation(IrDataOperation::Binary {
                    operator: BinaryOperator::Sub,
                    arg1: (&inst.inner.arguments[0]).into(),
                    arg2: (&inst.inner.arguments[1]).into(),
                })
                .into(),
                size: AccessSize::ArchitectureSize,
            },
        ]))
    }

    /// CMN Rn, Rm - Compare negative (Rn + Rm and set flags)
    pub fn cmn(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;
        Box::leak(Box::new([
            // Set NZCV flags based on addition result
            IrStatement::Assignment {
                to: register::NZCV.clone(),
                from: IrData::Operation(IrDataOperation::Binary {
                    operator: BinaryOperator::Add,
                    arg1: (&inst.inner.arguments[0]).into(),
                    arg2: (&inst.inner.arguments[1]).into(),
                })
                .into(),
                size: AccessSize::ArchitectureSize,
            },
        ]))
    }

    /// TST Rn, Rm - Test bits (Rn & Rm and set flags)
    pub fn tst(inst: &Instruction) -> &'static [IrStatement] {
        use crate::arch::arm64::register;
        Box::leak(Box::new([
            // Set NZCV flags based on AND result
            IrStatement::Assignment {
                to: register::NZCV.clone(),
                from: IrData::Operation(IrDataOperation::Binary {
                    operator: BinaryOperator::And,
                    arg1: (&inst.inner.arguments[0]).into(),
                    arg2: (&inst.inner.arguments[1]).into(),
                })
                .into(),
                size: AccessSize::ArchitectureSize,
            },
        ]))
    }
}
