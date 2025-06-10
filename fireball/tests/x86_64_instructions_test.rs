//! Tests for specific x86_64 instructions

use fireball::arch::x86_64::instruction_analyze::create_ir_statement;
use fireball::core::Instruction;
use iceball::{
    Argument, Instruction as IceballInstruction, Memory, Register, Statement, X64Register,
    X64Statement,
};

/// Helper to create a test instruction
fn create_test_instruction(statement: X64Statement, args: Vec<Argument>) -> Instruction {
    Instruction::new(
        0x1000,
        IceballInstruction {
            statement: Ok(Statement::X64(statement)),
            arguments: args.into_boxed_slice(),
            bytes: None,
        },
    )
}

#[test]
fn test_sahf_instruction() {
    // SAHF - Store AH into Flags
    let inst = create_test_instruction(X64Statement::Sahf, vec![]);

    let ir_statements = create_ir_statement(&inst);
    assert!(
        ir_statements.is_some(),
        "SAHF should generate IR statements"
    );

    let statements = ir_statements.unwrap();
    assert!(
        !statements.is_empty(),
        "SAHF should generate non-empty IR statements"
    );

    // SAHF should generate 5 statements (one for each flag: CF, PF, AF, ZF, SF)
    assert_eq!(statements.len(), 5, "SAHF should set 5 flags");
}

#[test]
fn test_xchg_instruction() {
    // XCHG RAX, RBX - Exchange two registers
    let inst = create_test_instruction(
        X64Statement::Xchg,
        vec![
            Argument::Register(Register::X64(X64Register::Rax)),
            Argument::Register(Register::X64(X64Register::Rbx)),
        ],
    );

    let ir_statements = create_ir_statement(&inst);
    assert!(
        ir_statements.is_some(),
        "XCHG should generate IR statements"
    );

    let statements = ir_statements.unwrap();
    assert!(
        !statements.is_empty(),
        "XCHG should generate non-empty IR statements"
    );

    // XCHG should generate 3 statements (temp = a, a = b, b = temp)
    assert_eq!(
        statements.len(),
        3,
        "XCHG should generate 3 assignment statements"
    );
}

#[test]
fn test_cmpxchg_instruction() {
    // CMPXCHG [RDI], RSI - Compare and exchange
    let inst = create_test_instruction(
        X64Statement::Cmpxchg,
        vec![
            Argument::Memory(Memory::RelativeAddressing(
                vec![iceball::RelativeAddressingArgument::Register(
                    Register::X64(X64Register::Rdi),
                )]
                .into_boxed_slice(),
            )),
            Argument::Register(Register::X64(X64Register::Rsi)),
        ],
    );

    let ir_statements = create_ir_statement(&inst);
    assert!(
        ir_statements.is_some(),
        "CMPXCHG should generate IR statements"
    );

    let statements = ir_statements.unwrap();
    assert!(
        !statements.is_empty(),
        "CMPXCHG should generate non-empty IR statements"
    );

    // CMPXCHG should generate statements for comparison, conditional assignment, and flag updates
    assert!(
        statements.len() >= 3,
        "CMPXCHG should generate at least 3 statements"
    );
}

#[test]
fn test_lock_prefix_handling() {
    // For now, we just test that CMPXCHG works without lock prefix
    // TODO: Add proper lock prefix handling when implemented

    // This test documents that lock prefix handling is not yet implemented
    // The lock prefix should make the operation atomic, but currently
    // we generate the same IR regardless of the lock prefix

    // When lock prefix is implemented, it should:
    // 1. Generate atomic memory operations
    // 2. Include memory barriers or fence instructions
    // 3. Ensure the operation is not interruptible
}
