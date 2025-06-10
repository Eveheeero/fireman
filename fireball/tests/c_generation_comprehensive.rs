//! Comprehensive tests for C code generation fixes

use fireball::core::{Address, Sections};
use fireball::ir::high_ir::{self, c_codegen::CCodeGenerator};
use fireball::ir::low_ir::{
    BasicBlock, BinaryOp, BlockId, Constant, FlagUpdate, Function, FunctionId, Instruction,
    LocalId, Module as LowModule, TargetInfo, Terminator, Type, Value,
};
use fireball::ir::medium_ir;
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
fn test_return_statement_generation() {
    let module = create_add_function();
    let medium_module = medium_ir::Module::from_low_ir(&module);
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    let mut generator = CCodeGenerator::new();
    let c_code = generator.generate(&high_module);

    println!("Generated C code:\n{}", c_code);

    // Must have return
    assert!(
        c_code.contains("return result;"),
        "Missing return statement"
    );
    // Parameters should now have good names
    assert!(
        c_code.contains("(int a, int b)"),
        "Parameters should be named a and b"
    );
    assert!(
        !c_code.contains("param_0"),
        "Should not have generic parameter names"
    );
    assert!(
        !c_code.contains("result_0"),
        "Should not have indexed variable names"
    );
}

#[test]
fn test_multiple_returns() {
    let module = create_conditional_return_function();
    let medium_module = medium_ir::Module::from_low_ir(&module);
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    let mut generator = CCodeGenerator::new();
    let c_code = generator.generate(&high_module);

    println!("Generated C code with conditional returns:\n{}", c_code);

    // Should have multiple return statements
    assert!(c_code.contains("return"), "Missing return statements");
}

#[test]
fn test_void_return() {
    let module = create_void_function();
    let medium_module = medium_ir::Module::from_low_ir(&module);
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    let mut generator = CCodeGenerator::new();
    let c_code = generator.generate(&high_module);

    println!("Generated C code for void function:\n{}", c_code);

    // Should have return statement for void function
    assert!(c_code.contains("return;"), "Missing void return statement");
}

// Helper functions to create test modules

fn create_add_function() -> LowModule {
    let sections = Arc::new(Sections::default());
    let mut module = LowModule::new(TargetInfo::x86_64());

    let func_id = FunctionId(0x1000);
    let entry = BlockId(0x1000);

    let mut function = Function {
        id: func_id.clone(),
        signature: Type::Function {
            ret: Box::new(Type::I32),
            params: vec![Type::I32, Type::I32],
            varargs: false,
        },
        entry: entry.clone(),
        blocks: BTreeMap::new(),
        locals: BTreeMap::new(),
    };

    // Entry block: return a + b
    let entry_block = BasicBlock {
        id: entry.clone(),
        phis: vec![],
        instructions: vec![Instruction::BinOp {
            op: BinaryOp::Add,
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x1000),
                purpose: "result",
                index: 0,
                version: 0,
            },
            lhs: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x1000),
                purpose: "a",
                index: 1,
                version: 0,
            }),
            rhs: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x1000),
                purpose: "b",
                index: 2,
                version: 0,
            }),
            ty: Type::I32,
            flags: FlagUpdate::Unchanged,
        }],
        terminator: Terminator::Return(Some((
            Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x1000),
                purpose: "result",
                index: 0,
                version: 0,
            }),
            Type::I32,
        ))),
    };

    function.blocks.insert(entry, entry_block);
    module.functions.insert(func_id, function);
    module
}

fn create_conditional_return_function() -> LowModule {
    let sections = Arc::new(Sections::default());
    let mut module = LowModule::new(TargetInfo::x86_64());

    let func_id = FunctionId(0x2000);
    let entry = BlockId(0x2000);
    let then_block = BlockId(0x2010);
    let else_block = BlockId(0x2020);

    let mut function = Function {
        id: func_id.clone(),
        signature: Type::Function {
            ret: Box::new(Type::I32),
            params: vec![Type::I32],
            varargs: false,
        },
        entry: entry.clone(),
        blocks: BTreeMap::new(),
        locals: BTreeMap::new(),
    };

    // Entry block: if (x > 0)
    let entry_block = BasicBlock {
        id: entry.clone(),
        phis: vec![],
        instructions: vec![Instruction::BinOp {
            op: BinaryOp::Sgt,
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x2000),
                purpose: "cond",
                index: 0,
                version: 0,
            },
            lhs: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x2000),
                purpose: "a",
                index: 1,
                version: 0,
            }),
            rhs: Value::Constant(Constant::Int {
                value: 0,
                ty: Type::I32,
            }),
            ty: Type::Bool,
            flags: FlagUpdate::Unchanged,
        }],
        terminator: Terminator::CondBranch {
            cond: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x2000),
                purpose: "cond",
                index: 0,
                version: 0,
            }),
            true_dest: then_block.clone(),
            false_dest: else_block.clone(),
        },
    };

    // Then block: return 1
    let then = BasicBlock {
        id: then_block.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(Some((
            Value::Constant(Constant::Int {
                value: 1,
                ty: Type::I32,
            }),
            Type::I32,
        ))),
    };

    // Else block: return 0
    let else_b = BasicBlock {
        id: else_block.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(Some((
            Value::Constant(Constant::Int {
                value: 0,
                ty: Type::I32,
            }),
            Type::I32,
        ))),
    };

    function.blocks.insert(entry, entry_block);
    function.blocks.insert(then_block, then);
    function.blocks.insert(else_block, else_b);

    module.functions.insert(func_id, function);
    module
}

fn create_void_function() -> LowModule {
    let _sections = Arc::new(Sections::default());
    let mut module = LowModule::new(TargetInfo::x86_64());

    let func_id = FunctionId(0x3000);
    let entry = BlockId(0x3000);

    let mut function = Function {
        id: func_id.clone(),
        signature: Type::Function {
            ret: Box::new(Type::Void),
            params: vec![],
            varargs: false,
        },
        entry: entry.clone(),
        blocks: BTreeMap::new(),
        locals: BTreeMap::new(),
    };

    // Entry block: just return
    let entry_block = BasicBlock {
        id: entry.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(None),
    };

    function.blocks.insert(entry, entry_block);
    module.functions.insert(func_id, function);
    module
}
