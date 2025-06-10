//! Tests for ARM64 instruction decoder

use fireball::arch::arm64::instruction_analyze::create_ir_statement;
use fireball::core::Instruction;
use fireball::ir::statements::IrStatement;
use iceball::{
    Argument, Arm64Register, Arm64Statement, Instruction as IceballInstruction, Register, Statement,
};

/// Helper to create a test instruction
fn create_test_instruction(statement: Arm64Statement, args: Vec<Argument>) -> Instruction {
    Instruction::new(
        0x1000,
        IceballInstruction {
            statement: Ok(Statement::Arm64(statement)),
            arguments: args.into_boxed_slice(),
            bytes: None,
        },
    )
}

/// Helper to create a register argument
fn reg_arg(reg: Arm64Register) -> Argument {
    Argument::Register(Register::Arm64(reg))
}

/// Helper to create a constant argument
fn const_arg(val: u64) -> Argument {
    Argument::Constant(val)
}

#[test]
fn test_arm64_add_instruction() {
    // ADD X0, X1, X2
    let inst = create_test_instruction(
        Arm64Statement::Add,
        vec![
            reg_arg(Arm64Register::X0),
            reg_arg(Arm64Register::X1),
            reg_arg(Arm64Register::X2),
        ],
    );

    let ir_statements = create_ir_statement(&inst);
    assert!(ir_statements.is_some(), "ADD should generate IR statements");

    let statements = ir_statements.unwrap();
    assert_eq!(statements.len(), 1, "ADD should generate 1 statement");
}

#[test]
fn test_arm64_mov_instruction() {
    // MOV X0, X1
    let inst = create_test_instruction(
        Arm64Statement::Mov,
        vec![reg_arg(Arm64Register::X0), reg_arg(Arm64Register::X1)],
    );

    let ir_statements = create_ir_statement(&inst);
    assert!(ir_statements.is_some(), "MOV should generate IR statements");

    let statements = ir_statements.unwrap();
    assert_eq!(statements.len(), 1, "MOV should generate 1 statement");
}

#[test]
fn test_arm64_branch_instruction() {
    // B 0x2000
    let inst = create_test_instruction(Arm64Statement::B, vec![const_arg(0x2000)]);

    let ir_statements = create_ir_statement(&inst);
    assert!(ir_statements.is_some(), "B should generate IR statements");

    let statements = ir_statements.unwrap();
    assert_eq!(statements.len(), 1, "B should generate 1 statement");

    match &statements[0] {
        IrStatement::Jump { .. } => {}
        _ => panic!("B should generate a Jump statement"),
    }
}

#[test]
fn test_arm64_conditional_branch() {
    // BEQ 0x3000
    let inst = create_test_instruction(Arm64Statement::Beq, vec![const_arg(0x3000)]);

    let ir_statements = create_ir_statement(&inst);
    assert!(ir_statements.is_some(), "BEQ should generate IR statements");

    let statements = ir_statements.unwrap();
    assert_eq!(statements.len(), 1, "BEQ should generate 1 statement");

    match &statements[0] {
        IrStatement::Condition { .. } => {}
        _ => panic!("BEQ should generate a Condition statement"),
    }
}

#[test]
fn test_arm64_cmp_instruction() {
    // CMP X0, X1
    let inst = create_test_instruction(
        Arm64Statement::Cmp,
        vec![reg_arg(Arm64Register::X0), reg_arg(Arm64Register::X1)],
    );

    let ir_statements = create_ir_statement(&inst);
    assert!(ir_statements.is_some(), "CMP should generate IR statements");

    let statements = ir_statements.unwrap();
    assert_eq!(statements.len(), 1, "CMP should generate 1 statement");
}

#[test]
fn test_arm64_ret_instruction() {
    // RET
    let inst = create_test_instruction(Arm64Statement::Ret, vec![]);

    let ir_statements = create_ir_statement(&inst);
    assert!(ir_statements.is_some(), "RET should generate IR statements");

    let statements = ir_statements.unwrap();
    assert_eq!(statements.len(), 1, "RET should generate 1 statement");

    match &statements[0] {
        IrStatement::Halt => {}
        _ => panic!("RET should generate a Halt statement"),
    }
}

#[test]
fn test_arm64_ldr_instruction() {
    use iceball::Memory;

    // LDR X0, [X1]
    let inst = create_test_instruction(
        Arm64Statement::Ldr,
        vec![
            reg_arg(Arm64Register::X0),
            Argument::Memory(Memory {
                base: Some(Register::Arm64(Arm64Register::X1)),
                index: None,
                scale: 1,
                displacement: 0,
                size: None,
            }),
        ],
    );

    let ir_statements = create_ir_statement(&inst);
    assert!(ir_statements.is_some(), "LDR should generate IR statements");

    let statements = ir_statements.unwrap();
    assert_eq!(statements.len(), 1, "LDR should generate 1 statement");
}

#[test]
fn test_arm64_str_instruction() {
    use iceball::Memory;

    // STR X0, [X1]
    let inst = create_test_instruction(
        Arm64Statement::Str,
        vec![
            reg_arg(Arm64Register::X0),
            Argument::Memory(Memory {
                base: Some(Register::Arm64(Arm64Register::X1)),
                index: None,
                scale: 1,
                displacement: 0,
                size: None,
            }),
        ],
    );

    let ir_statements = create_ir_statement(&inst);
    assert!(ir_statements.is_some(), "STR should generate IR statements");

    let statements = ir_statements.unwrap();
    assert_eq!(statements.len(), 1, "STR should generate 1 statement");
}

#[test]
fn test_arm64_nop_instruction() {
    // NOP
    let inst = create_test_instruction(Arm64Statement::Nop, vec![]);

    let ir_statements = create_ir_statement(&inst);
    assert!(ir_statements.is_some(), "NOP should generate IR statements");

    let statements = ir_statements.unwrap();
    assert_eq!(statements.len(), 0, "NOP should generate no statements");
}
