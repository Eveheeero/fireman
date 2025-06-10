//! ARM32 instruction decoder tests

use fireball::arch::arm32::{InstructionSet, decoder::Arm32Decoder};
use fireball::core::{Address, Sections};
use std::sync::Arc;

/// Helper function to create a test address
fn test_address(virtual_addr: u64) -> Address {
    // Create a default sections object for testing
    let sections = Arc::new(Sections::default());

    // Create address using from_virtual_address
    Address::from_virtual_address(&sections, virtual_addr)
}

#[test]
fn test_arm_mov_instruction() {
    let decoder = Arm32Decoder::new(InstructionSet::ARM);

    // MOV R0, #42 (E3A0002A in little-endian)
    let data = [0x2A, 0x00, 0xA0, 0xE3];
    let addr = test_address(0x1000);

    let result = decoder.decode_instruction(&data, &addr);
    assert!(result.is_ok());

    let (inst, size) = result.unwrap();
    assert_eq!(size, 4);

    // Check instruction
    assert!(inst.statement.is_ok());
    if let Ok(iceball::Statement::Arm32(stmt)) = inst.statement {
        assert_eq!(format!("{}", stmt), "mov");
    }

    // Check arguments
    assert_eq!(inst.arguments.len(), 2);

    // First argument should be R0
    if let iceball::Argument::Register(iceball::Register::Arm32(reg)) = &inst.arguments[0] {
        assert_eq!(format!("{}", reg), "r0");
    } else {
        panic!("Expected register argument");
    }

    // Second argument should be constant 42
    if let iceball::Argument::Constant(val) = &inst.arguments[1] {
        assert_eq!(*val, 42);
    } else {
        panic!("Expected constant argument");
    }
}

#[test]
fn test_arm_add_instruction() {
    let decoder = Arm32Decoder::new(InstructionSet::ARM);

    // ADD R1, R2, R3 (E0821003 in little-endian)
    let data = [0x03, 0x10, 0x82, 0xE0];
    let addr = test_address(0x1000);

    let result = decoder.decode_instruction(&data, &addr);
    assert!(result.is_ok());

    let (inst, size) = result.unwrap();
    assert_eq!(size, 4);

    // Check instruction
    assert!(inst.statement.is_ok());
    if let Ok(iceball::Statement::Arm32(stmt)) = inst.statement {
        assert_eq!(format!("{}", stmt), "add");
    }

    // Check arguments
    assert_eq!(inst.arguments.len(), 3);

    // Check registers
    if let iceball::Argument::Register(iceball::Register::Arm32(reg)) = &inst.arguments[0] {
        assert_eq!(format!("{}", reg), "r1");
    }
    if let iceball::Argument::Register(iceball::Register::Arm32(reg)) = &inst.arguments[1] {
        assert_eq!(format!("{}", reg), "r2");
    }
    if let iceball::Argument::Register(iceball::Register::Arm32(reg)) = &inst.arguments[2] {
        assert_eq!(format!("{}", reg), "r3");
    }
}

#[test]
fn test_arm_ldr_instruction() {
    let decoder = Arm32Decoder::new(InstructionSet::ARM);

    // LDR R0, [R1, #4] (E5910004 in little-endian)
    let data = [0x04, 0x00, 0x91, 0xE5];
    let addr = test_address(0x1000);

    let result = decoder.decode_instruction(&data, &addr);
    assert!(result.is_ok());

    let (inst, size) = result.unwrap();
    assert_eq!(size, 4);

    // Check instruction
    assert!(inst.statement.is_ok());
    if let Ok(iceball::Statement::Arm32(stmt)) = inst.statement {
        assert_eq!(format!("{}", stmt), "ldr");
    }

    // Check arguments
    assert_eq!(inst.arguments.len(), 2);

    // First argument should be R0
    if let iceball::Argument::Register(iceball::Register::Arm32(reg)) = &inst.arguments[0] {
        assert_eq!(format!("{}", reg), "r0");
    }

    // Second argument should be memory [R1, #4]
    assert!(matches!(&inst.arguments[1], iceball::Argument::Memory(_)));
}

#[test]
fn test_arm_branch_instruction() {
    let decoder = Arm32Decoder::new(InstructionSet::ARM);

    // B 0x1000 (forward branch by 4092 bytes) (EAFFFFFE in little-endian)
    let data = [0xFE, 0xFF, 0xFF, 0xEA];
    let addr = test_address(0x1000);

    let result = decoder.decode_instruction(&data, &addr);
    assert!(result.is_ok());

    let (inst, size) = result.unwrap();
    assert_eq!(size, 4);

    // Check instruction
    assert!(inst.statement.is_ok());
    if let Ok(iceball::Statement::Arm32(stmt)) = inst.statement {
        assert_eq!(format!("{}", stmt), "b");
    }

    // Check arguments
    assert_eq!(inst.arguments.len(), 1);
}

#[test]
fn test_arm_conditional_instruction() {
    let decoder = Arm32Decoder::new(InstructionSet::ARM);

    // ADDEQ R0, R1, R2 (00810002 in little-endian)
    let data = [0x02, 0x00, 0x81, 0x00];
    let addr = test_address(0x1000);

    let result = decoder.decode_instruction(&data, &addr);
    assert!(result.is_ok());

    let (inst, size) = result.unwrap();
    assert_eq!(size, 4);

    // Check instruction has condition suffix
    assert!(inst.statement.is_ok());
    if let Ok(iceball::Statement::Arm32(stmt)) = inst.statement {
        // Statement should be parsed as "add" (condition is handled separately)
        assert_eq!(format!("{}", stmt), "add");
    }
}

#[test]
fn test_thumb_mov_instruction() {
    let decoder = Arm32Decoder::new(InstructionSet::Thumb);

    // MOV R0, #42 (202A in little-endian)
    let data = [0x2A, 0x20];
    let addr = test_address(0x1000);

    let result = decoder.decode_instruction(&data, &addr);
    assert!(result.is_ok());

    let (inst, size) = result.unwrap();
    assert_eq!(size, 2);

    // Check instruction
    assert!(inst.statement.is_ok());
    if let Ok(iceball::Statement::Arm32(stmt)) = inst.statement {
        assert_eq!(format!("{}", stmt), "mov");
    }

    // Check arguments
    assert_eq!(inst.arguments.len(), 2);
}

#[test]
fn test_thumb_add_registers() {
    let decoder = Arm32Decoder::new(InstructionSet::Thumb);

    // ADD R0, R1, R2 (8818 in little-endian)
    let data = [0x88, 0x18];
    let addr = test_address(0x1000);

    let result = decoder.decode_instruction(&data, &addr);
    assert!(result.is_ok());

    let (inst, size) = result.unwrap();
    assert_eq!(size, 2);

    // Check instruction
    assert!(inst.statement.is_ok());
    if let Ok(iceball::Statement::Arm32(stmt)) = inst.statement {
        assert_eq!(format!("{}", stmt), "add");
    }
}
