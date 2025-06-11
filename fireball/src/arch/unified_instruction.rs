//! Unified instruction interface for all architectures
//!
//! This module provides a common interface for handling instructions across
//! different architectures (x86, x86_64, ARM32, ARM64) while supporting
//! architecture-specific features.

use crate::arch::{ArchType, ArchitectureInfo};
use crate::core::Instruction;
use crate::ir::statements::IrStatement;
use crate::utils::error::DecompileError;
use std::collections::BTreeMap;

/// Common instruction categories shared across architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionCategory {
    /// Arithmetic operations (ADD, SUB, MUL, DIV)
    Arithmetic,
    /// Logical operations (AND, OR, XOR, NOT)
    Logic,
    /// Data movement (MOV, LOAD, STORE)
    DataTransfer,
    /// Control flow (JMP, CALL, RET, conditional branches)
    ControlFlow,
    /// Comparison operations
    Compare,
    /// Stack operations (PUSH, POP)
    Stack,
    /// SIMD/Vector operations
    Simd,
    /// System/privileged instructions
    System,
    /// Atomic/synchronization operations
    Atomic,
    /// Unknown/unsupported
    Unknown,
}

/// Architecture-specific features and modifiers
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InstructionModifiers {
    /// x86/x86_64: LOCK prefix for atomic operations
    pub lock_prefix: bool,
    /// x86/x86_64: REP/REPE/REPNE prefixes
    pub rep_prefix: Option<RepPrefix>,
    /// ARM: Condition codes (EQ, NE, LT, GT, etc.)
    pub condition: Option<ConditionCode>,
    /// ARM: Update flags after operation
    pub update_flags: bool,
    /// Size override (for variable-size architectures)
    pub size_override: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepPrefix {
    Rep,
    Repe,
    Repne,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionCode {
    // Common ARM condition codes
    Equal,
    NotEqual,
    CarrySet,
    CarryClear,
    Negative,
    NonNegative,
    Overflow,
    NoOverflow,
    UnsignedHigher,
    UnsignedLowerOrSame,
    SignedGreaterOrEqual,
    SignedLess,
    SignedGreater,
    SignedLessOrEqual,
    Always,
    Never,
}

/// Processor mode for x86 family
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorMode {
    Real16,      // 16-bit real mode
    Protected32, // 32-bit protected mode
    Long64,      // 64-bit long mode
}

/// Instruction set for ARM family
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmInstructionSet {
    Arm32,   // 32-bit ARM instructions
    Thumb,   // 16-bit Thumb instructions
    Thumb2,  // Mixed 16/32-bit Thumb2
    AArch64, // 64-bit ARM instructions
}

/// Architecture context for instruction analysis
pub struct UnifiedArchContext {
    pub arch_type: ArchType,
    pub arch_info: ArchitectureInfo,
    pub mode: ArchMode,
    /// Architecture-specific features
    pub features: ArchFeatures,
}

#[derive(Debug, Clone)]
pub enum ArchMode {
    X86(ProcessorMode),
    Arm(ArmInstructionSet),
}

#[derive(Debug, Clone, Default)]
pub struct ArchFeatures {
    /// x86: SSE, AVX, etc.
    pub x86_extensions: Vec<String>,
    /// ARM: NEON, SVE, etc.
    pub arm_extensions: Vec<String>,
    /// Endianness override (if different from default)
    pub endianness_override: Option<bool>, // true = big endian
}

/// Unified instruction trait for cross-architecture support
pub trait UnifiedInstruction {
    /// Get the instruction mnemonic
    fn mnemonic(&self) -> &str;

    /// Get the instruction category
    fn category(&self) -> InstructionCategory;

    /// Get the number of operands
    fn operand_count(&self) -> usize;

    /// Get architecture-specific modifiers
    fn modifiers(&self) -> InstructionModifiers;

    /// Convert to IR statements with architecture context
    fn to_ir(&self, ctx: &UnifiedArchContext) -> Result<Vec<IrStatement>, DecompileError>;

    /// Check if this instruction can be executed atomically
    fn supports_atomic(&self) -> bool;

    /// Get the base operation (without modifiers)
    fn base_operation(&self) -> BaseOperation;
}

/// Base operations common across architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseOperation {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Negate,

    // Logical
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,

    // Data Transfer
    Move,
    Load,
    Store,
    Exchange,
    CompareExchange,

    // Control Flow
    Jump,
    ConditionalJump,
    Call,
    Return,

    // Comparison
    Compare,
    Test,

    // Stack
    Push,
    Pop,

    // Special
    Nop,
    Undefined,
}

/// Unified instruction analyzer that dispatches to architecture-specific handlers
pub struct UnifiedInstructionAnalyzer {
    /// Mapping of common operations across architectures
    operation_map: BTreeMap<(ArchType, String), BaseOperation>,
}

impl UnifiedInstructionAnalyzer {
    pub fn new() -> Self {
        let mut operation_map = BTreeMap::new();

        // x86/x86_64 mappings
        Self::init_x86_mappings(&mut operation_map);

        // ARM32/ARM64 mappings
        Self::init_arm_mappings(&mut operation_map);

        Self { operation_map }
    }

    fn init_x86_mappings(map: &mut BTreeMap<(ArchType, String), BaseOperation>) {
        // Common x86/x86_64 instructions
        let x86_common = vec![
            ("add", BaseOperation::Add),
            ("sub", BaseOperation::Subtract),
            ("imul", BaseOperation::Multiply),
            ("mul", BaseOperation::Multiply),
            ("idiv", BaseOperation::Divide),
            ("div", BaseOperation::Divide),
            ("and", BaseOperation::And),
            ("or", BaseOperation::Or),
            ("xor", BaseOperation::Xor),
            ("not", BaseOperation::Not),
            ("shl", BaseOperation::ShiftLeft),
            ("shr", BaseOperation::ShiftRight),
            ("rol", BaseOperation::RotateLeft),
            ("ror", BaseOperation::RotateRight),
            ("mov", BaseOperation::Move),
            ("movzx", BaseOperation::Move), // zero extend
            ("movsx", BaseOperation::Move), // sign extend
            ("xchg", BaseOperation::Exchange),
            ("cmpxchg", BaseOperation::CompareExchange),
            ("jmp", BaseOperation::Jump),
            ("call", BaseOperation::Call),
            ("ret", BaseOperation::Return),
            ("cmp", BaseOperation::Compare),
            ("test", BaseOperation::Test),
            ("push", BaseOperation::Push),
            ("pop", BaseOperation::Pop),
            ("nop", BaseOperation::Nop),
        ];

        // Add for both x86 and x86_64
        for (mnemonic, op) in &x86_common {
            map.insert((ArchType::X86, mnemonic.to_string()), *op);
            map.insert((ArchType::X86_64, mnemonic.to_string()), *op);
        }

        // Conditional jumps
        let x86_cond_jumps = vec![
            "je", "jne", "jg", "jge", "jl", "jle", "ja", "jae", "jb", "jbe", "jo", "jno", "js",
            "jns", "jp", "jnp",
        ];

        for jcc in &x86_cond_jumps {
            map.insert(
                (ArchType::X86, jcc.to_string()),
                BaseOperation::ConditionalJump,
            );
            map.insert(
                (ArchType::X86_64, jcc.to_string()),
                BaseOperation::ConditionalJump,
            );
        }
    }

    fn init_arm_mappings(map: &mut BTreeMap<(ArchType, String), BaseOperation>) {
        // Common ARM32/ARM64 instructions
        let arm_common = vec![
            ("add", BaseOperation::Add),
            ("sub", BaseOperation::Subtract),
            ("mul", BaseOperation::Multiply),
            ("sdiv", BaseOperation::Divide), // signed divide
            ("udiv", BaseOperation::Divide), // unsigned divide
            ("and", BaseOperation::And),
            ("orr", BaseOperation::Or),
            ("eor", BaseOperation::Xor),
            ("mvn", BaseOperation::Not), // move not
            ("lsl", BaseOperation::ShiftLeft),
            ("lsr", BaseOperation::ShiftRight),
            ("ror", BaseOperation::RotateRight),
            ("mov", BaseOperation::Move),
            ("ldr", BaseOperation::Load),
            ("str", BaseOperation::Store),
            ("b", BaseOperation::Jump),    // branch
            ("bl", BaseOperation::Call),   // branch with link
            ("bx", BaseOperation::Return), // branch exchange
            ("cmp", BaseOperation::Compare),
            ("tst", BaseOperation::Test),
            ("push", BaseOperation::Push),
            ("pop", BaseOperation::Pop),
            ("nop", BaseOperation::Nop),
        ];

        // Add for both ARM32 and ARM64
        for (mnemonic, op) in &arm_common {
            map.insert((ArchType::Arm32, mnemonic.to_string()), *op);
            map.insert((ArchType::Arm64, mnemonic.to_string()), *op);
        }

        // Conditional branches (handled with condition codes)
        map.insert(
            (ArchType::Arm32, "beq".to_string()),
            BaseOperation::ConditionalJump,
        );
        map.insert(
            (ArchType::Arm32, "bne".to_string()),
            BaseOperation::ConditionalJump,
        );
        map.insert(
            (ArchType::Arm64, "beq".to_string()),
            BaseOperation::ConditionalJump,
        );
        map.insert(
            (ArchType::Arm64, "bne".to_string()),
            BaseOperation::ConditionalJump,
        );
        // ... more conditional branches
    }

    /// Analyze an instruction and convert to IR
    pub fn analyze_instruction(
        &self,
        instruction: &Instruction,
        ctx: &UnifiedArchContext,
    ) -> Result<Vec<IrStatement>, DecompileError> {
        // Extract base operation
        let mnemonic = self.extract_mnemonic(instruction)?;
        let base_op =
            self.get_base_operation(ctx.arch_type, &mnemonic)
                .ok_or(DecompileError::Unknown(Some(format!(
                    "Unsupported instruction: {}",
                    mnemonic
                ))))?;

        // Extract modifiers
        let modifiers = self.extract_modifiers(instruction, ctx)?;

        // Handle architecture-specific logic
        match ctx.arch_type {
            ArchType::X86 | ArchType::X86_64 => {
                self.analyze_x86_instruction(instruction, ctx, base_op, modifiers)
            }
            ArchType::Arm32 | ArchType::Arm64 => {
                self.analyze_arm_instruction(instruction, ctx, base_op, modifiers)
            }
            _ => Err(DecompileError::Unknown(Some(
                "Unsupported architecture".to_string(),
            ))),
        }
    }

    fn extract_mnemonic(&self, instruction: &Instruction) -> Result<String, DecompileError> {
        // Extract mnemonic from iceball instruction
        // This is a simplified version - real implementation would parse properly
        Ok(format!("{:?}", instruction.inner().statement).to_lowercase())
    }

    fn get_base_operation(&self, arch: ArchType, mnemonic: &str) -> Option<BaseOperation> {
        self.operation_map
            .get(&(arch, mnemonic.to_string()))
            .copied()
    }

    fn extract_modifiers(
        &self,
        instruction: &Instruction,
        ctx: &UnifiedArchContext,
    ) -> Result<InstructionModifiers, DecompileError> {
        let mut modifiers = InstructionModifiers {
            lock_prefix: false,
            rep_prefix: None,
            condition: None,
            update_flags: false,
            size_override: None,
        };

        // Extract architecture-specific modifiers
        match ctx.arch_type {
            ArchType::X86 | ArchType::X86_64 => {
                // Check for LOCK prefix in instruction bytes
                if let Ok(bytes) = instruction.inner().get_bytes() {
                    modifiers.lock_prefix = bytes.contains(&0xF0); // LOCK prefix

                    // Check for REP prefixes
                    if bytes.contains(&0xF3) {
                        modifiers.rep_prefix = Some(RepPrefix::Rep);
                    } else if bytes.contains(&0xF2) {
                        modifiers.rep_prefix = Some(RepPrefix::Repne);
                    }
                }
            }
            ArchType::Arm32 | ArchType::Arm64 => {
                // ARM instructions have condition codes and flag updates
                // This would parse the instruction encoding
            }
            _ => {}
        }

        Ok(modifiers)
    }

    fn analyze_x86_instruction(
        &self,
        instruction: &Instruction,
        ctx: &UnifiedArchContext,
        base_op: BaseOperation,
        modifiers: InstructionModifiers,
    ) -> Result<Vec<IrStatement>, DecompileError> {
        // Handle LOCK prefix for atomic operations
        if modifiers.lock_prefix && !Self::supports_lock_prefix(base_op) {
            return Err(DecompileError::Unknown(Some(
                "LOCK prefix not supported for this instruction".to_string(),
            )));
        }

        // Dispatch to existing x86_64 implementation for now
        match ctx.arch_type {
            ArchType::X86_64 => {
                crate::arch::x86_64::instruction_analyze::create_ir_statement(instruction)
                    .ok_or(DecompileError::Unknown(Some(
                        "Unknown x86_64 instruction".to_string(),
                    )))
                    .map(|stmts| stmts.to_vec())
            }
            ArchType::X86 => {
                // x86 uses x86_64 handler with 32-bit mode adjustments
                // First check if we're in the right processor mode
                if let ArchMode::X86(ProcessorMode::Protected32) = ctx.mode {
                    crate::arch::x86_64::instruction_analyze::create_ir_statement(instruction)
                        .ok_or(DecompileError::Unknown(Some(
                            "Unknown x86 instruction".to_string(),
                        )))
                        .map(|stmts| self.adjust_for_32bit_mode(stmts))
                } else {
                    Err(DecompileError::Unknown(Some(
                        "x86 instruction in wrong processor mode".to_string(),
                    )))
                }
            }
            _ => unreachable!(),
        }
    }

    fn analyze_arm_instruction(
        &self,
        _instruction: &Instruction,
        _ctx: &UnifiedArchContext,
        _base_op: BaseOperation,
        _modifiers: InstructionModifiers,
    ) -> Result<Vec<IrStatement>, DecompileError> {
        // TODO: Implement ARM instruction analysis
        Err(DecompileError::Unknown(Some(
            "ARM instruction analysis not implemented".to_string(),
        )))
    }

    fn supports_lock_prefix(op: BaseOperation) -> bool {
        matches!(
            op,
            BaseOperation::Add
                | BaseOperation::Subtract
                | BaseOperation::And
                | BaseOperation::Or
                | BaseOperation::Xor
                | BaseOperation::Not
                | BaseOperation::Exchange
                | BaseOperation::CompareExchange
        )
    }

    fn adjust_for_32bit_mode(&self, statements: &'static [IrStatement]) -> Vec<IrStatement> {
        // In 32-bit mode, operations on 32-bit registers don't zero-extend to 64 bits
        // Also need to adjust register sizes and addressing modes
        statements
            .iter()
            .map(|stmt| self.transform_for_32bit(stmt))
            .collect()
    }

    fn transform_for_32bit(&self, stmt: &IrStatement) -> IrStatement {
        match stmt {
            IrStatement::Assignment { from, to, size } => {
                // In 32-bit mode:
                // 1. No zero-extension when writing to 32-bit registers
                // 2. Default pointer size is 32 bits
                // 3. Segment registers are more relevant

                // Check if this is a register operation that needs adjustment
                if let Some(adjusted_size) = self.adjust_size_for_32bit(size) {
                    IrStatement::Assignment {
                        from: from.clone(),
                        to: to.clone(),
                        size: adjusted_size,
                    }
                } else {
                    stmt.clone()
                }
            }
            IrStatement::Condition {
                condition,
                true_branch,
                false_branch,
            } => {
                // Recursively transform branches
                IrStatement::Condition {
                    condition: condition.clone(),
                    true_branch: true_branch
                        .iter()
                        .map(|s| self.transform_for_32bit(s))
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                    false_branch: false_branch
                        .iter()
                        .map(|s| self.transform_for_32bit(s))
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                }
            }
            // Other statements pass through unchanged
            _ => stmt.clone(),
        }
    }

    fn adjust_size_for_32bit(
        &self,
        size: &crate::ir::data::AccessSize,
    ) -> Option<crate::ir::data::AccessSize> {
        use crate::ir::data::AccessSize;

        match size {
            AccessSize::ArchitectureSize => {
                // In 32-bit mode, architecture size is 32 bits
                Some(AccessSize::ResultOfBit(crate::utils::Aos::new(
                    crate::ir::data::IrData::Constant(32),
                )))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_operation_mapping() {
        let analyzer = UnifiedInstructionAnalyzer::new();

        // Test x86_64 mappings
        assert_eq!(
            analyzer.get_base_operation(ArchType::X86_64, "add"),
            Some(BaseOperation::Add)
        );
        assert_eq!(
            analyzer.get_base_operation(ArchType::X86_64, "cmpxchg"),
            Some(BaseOperation::CompareExchange)
        );

        // Test ARM mappings
        assert_eq!(
            analyzer.get_base_operation(ArchType::Arm64, "add"),
            Some(BaseOperation::Add)
        );
        assert_eq!(
            analyzer.get_base_operation(ArchType::Arm64, "ldr"),
            Some(BaseOperation::Load)
        );
    }

    #[test]
    fn test_lock_prefix_support() {
        assert!(UnifiedInstructionAnalyzer::supports_lock_prefix(
            BaseOperation::Add
        ));
        assert!(UnifiedInstructionAnalyzer::supports_lock_prefix(
            BaseOperation::CompareExchange
        ));
        assert!(!UnifiedInstructionAnalyzer::supports_lock_prefix(
            BaseOperation::Jump
        ));
        assert!(!UnifiedInstructionAnalyzer::supports_lock_prefix(
            BaseOperation::Move
        ));
    }
}
