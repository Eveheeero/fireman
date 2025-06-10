//! Tests for the simple C code generator

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
fn test_simple_function_codegen() {
    let module = create_simple_function_module();
    let medium_module = medium_ir::Module::from_low_ir(&module);
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    // Debug print the High IR module
    println!("High IR module: {:#?}", high_module);

    // Generate C code using simple generator
    let mut generator = CCodeGenerator::new();
    let c_code = generator.generate(&high_module);
    println!("Generated C code:\n{}", c_code);

    // Check that the C code contains expected elements
    assert!(
        c_code.contains("#include <stdio.h>"),
        "Should contain standard includes"
    );
    assert!(c_code.contains("sub_"), "Should contain function name");
    assert!(c_code.contains("result"), "Should contain result variable");
    assert!(c_code.contains("return"), "Should contain return statement");
}

#[test]
fn test_while_loop_codegen() {
    let module = create_while_loop_module();
    let medium_module = medium_ir::Module::from_low_ir(&module);
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    // Generate C code using simple generator
    let mut generator = CCodeGenerator::new();
    let c_code = generator.generate(&high_module);
    println!("Generated C code for while loop:\n{}", c_code);

    // Check that the C code contains while loop
    assert!(c_code.contains("while ("), "Should contain while loop");
    assert!(c_code.contains("{"), "Should contain block braces");
    assert!(c_code.contains("}"), "Should contain block braces");
}

#[test]
fn test_if_else_codegen() {
    let module = create_if_else_module();
    let medium_module = medium_ir::Module::from_low_ir(&module);
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    // Generate C code using simple generator
    let mut generator = CCodeGenerator::new();
    let c_code = generator.generate(&high_module);
    println!("Generated C code for if-else:\n{}", c_code);

    // Check that the C code contains if statement
    assert!(c_code.contains("if ("), "Should contain if statement");
    assert!(c_code.contains("} else {"), "Should contain else clause");
}

#[test]
fn test_for_loop_codegen() {
    let module = create_for_loop_module();
    let medium_module = medium_ir::Module::from_low_ir(&module);
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    // Generate C code using simple generator
    let mut generator = CCodeGenerator::new();
    let c_code = generator.generate(&high_module);
    println!("Generated C code for for loop:\n{}", c_code);

    // Check that the C code contains for loop
    assert!(c_code.contains("for ("), "Should contain for loop");
    assert!(
        c_code.contains("; i < 10; i++"),
        "Should contain loop condition and update"
    );
}

// Helper functions (same as in other test files)

fn create_simple_function_module() -> LowModule {
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

fn create_while_loop_module() -> LowModule {
    let sections = Arc::new(Sections::default());
    let mut module = LowModule::new(TargetInfo::x86_64());

    let func_id = FunctionId(0x2000);
    let entry = BlockId(0x2000);
    let loop_header = BlockId(0x2010);
    let loop_body = BlockId(0x2020);
    let loop_exit = BlockId(0x2030);

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

    // Entry block: goto header
    let entry_block = BasicBlock {
        id: entry.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Branch(loop_header.clone()),
    };

    // Loop header: while (x != 0)
    let header_block = BasicBlock {
        id: loop_header.clone(),
        phis: vec![],
        instructions: vec![Instruction::BinOp {
            op: BinaryOp::Ne,
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x2010),
                purpose: "cond",
                index: 0,
                version: 0,
            },
            lhs: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x2000),
                purpose: "x",
                index: 0,
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
                source: Address::from_virtual_address(&sections, 0x2010),
                purpose: "cond",
                index: 0,
                version: 0,
            }),
            true_dest: loop_body.clone(),
            false_dest: loop_exit.clone(),
        },
    };

    // Loop body
    let body_block = BasicBlock {
        id: loop_body.clone(),
        phis: vec![],
        instructions: vec![
            // x = x - 1
            Instruction::BinOp {
                op: BinaryOp::Sub,
                dst: LocalId {
                    source: Address::from_virtual_address(&sections, 0x2020),
                    purpose: "x",
                    index: 0,
                    version: 1,
                },
                lhs: Value::Local(LocalId {
                    source: Address::from_virtual_address(&sections, 0x2000),
                    purpose: "x",
                    index: 0,
                    version: 0,
                }),
                rhs: Value::Constant(Constant::Int {
                    value: 1,
                    ty: Type::I32,
                }),
                ty: Type::I32,
                flags: FlagUpdate::Unchanged,
            },
        ],
        terminator: Terminator::Branch(loop_header.clone()),
    };

    // Exit block
    let exit_block = BasicBlock {
        id: loop_exit.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(None),
    };

    function.blocks.insert(entry, entry_block);
    function.blocks.insert(loop_header, header_block);
    function.blocks.insert(loop_body, body_block);
    function.blocks.insert(loop_exit, exit_block);

    module.functions.insert(func_id, function);
    module
}

fn create_if_else_module() -> LowModule {
    let sections = Arc::new(Sections::default());
    let mut module = LowModule::new(TargetInfo::x86_64());

    let func_id = FunctionId(0x3000);
    let entry = BlockId(0x3000);
    let then_block_id = BlockId(0x3010);
    let else_block_id = BlockId(0x3020);
    let exit = BlockId(0x3030);

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
                source: Address::from_virtual_address(&sections, 0x3000),
                purpose: "cond",
                index: 0,
                version: 0,
            },
            lhs: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x3000),
                purpose: "x",
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
                source: Address::from_virtual_address(&sections, 0x3000),
                purpose: "cond",
                index: 0,
                version: 0,
            }),
            true_dest: then_block_id.clone(),
            false_dest: else_block_id.clone(),
        },
    };

    // Then block: return 1
    let then_block = BasicBlock {
        id: then_block_id.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x3010),
                purpose: "result",
                index: 2,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 1,
                ty: Type::I32,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x3010),
        }],
        terminator: Terminator::Branch(exit.clone()),
    };

    // Else block: return 0
    let else_block = BasicBlock {
        id: else_block_id.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x3020),
                purpose: "result",
                index: 2,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 0,
                ty: Type::I32,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x3020),
        }],
        terminator: Terminator::Branch(exit.clone()),
    };

    // Exit block
    let exit_block = BasicBlock {
        id: exit.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(Some((
            Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x3010),
                purpose: "result",
                index: 2,
                version: 0,
            }),
            Type::I32,
        ))),
    };

    function.blocks.insert(entry, entry_block);
    function.blocks.insert(then_block_id, then_block);
    function.blocks.insert(else_block_id, else_block);
    function.blocks.insert(exit, exit_block);

    module.functions.insert(func_id, function);
    module
}

fn create_for_loop_module() -> LowModule {
    let sections = Arc::new(Sections::default());
    let mut module = LowModule::new(TargetInfo::x86_64());

    let func_id = FunctionId(0x4000);
    let entry = BlockId(0x4000);
    let loop_header = BlockId(0x4010);
    let loop_body = BlockId(0x4020);
    let loop_exit = BlockId(0x4030);

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

    // Entry block: i = 0, goto header
    let entry_block = BasicBlock {
        id: entry.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x4000),
                purpose: "i",
                index: 0,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 0,
                ty: Type::I32,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x4000),
        }],
        terminator: Terminator::Branch(loop_header.clone()),
    };

    // Loop header: if (i < 10)
    let header_block = BasicBlock {
        id: loop_header.clone(),
        phis: vec![],
        instructions: vec![Instruction::BinOp {
            op: BinaryOp::Slt,
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x4010),
                purpose: "cond",
                index: 1,
                version: 0,
            },
            lhs: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x4000),
                purpose: "i",
                index: 0,
                version: 0,
            }),
            rhs: Value::Constant(Constant::Int {
                value: 10,
                ty: Type::I32,
            }),
            ty: Type::Bool,
            flags: FlagUpdate::Unchanged,
        }],
        terminator: Terminator::CondBranch {
            cond: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x4010),
                purpose: "cond",
                index: 1,
                version: 0,
            }),
            true_dest: loop_body.clone(),
            false_dest: loop_exit.clone(),
        },
    };

    // Loop body: i++
    let body_block = BasicBlock {
        id: loop_body.clone(),
        phis: vec![],
        instructions: vec![
            // i = i + 1
            Instruction::BinOp {
                op: BinaryOp::Add,
                dst: LocalId {
                    source: Address::from_virtual_address(&sections, 0x4020),
                    purpose: "i",
                    index: 0,
                    version: 1,
                },
                lhs: Value::Local(LocalId {
                    source: Address::from_virtual_address(&sections, 0x4000),
                    purpose: "i",
                    index: 0,
                    version: 0,
                }),
                rhs: Value::Constant(Constant::Int {
                    value: 1,
                    ty: Type::I32,
                }),
                ty: Type::I32,
                flags: FlagUpdate::Unchanged,
            },
        ],
        terminator: Terminator::Branch(loop_header.clone()),
    };

    // Exit block
    let exit_block = BasicBlock {
        id: loop_exit.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(None),
    };

    function.blocks.insert(entry, entry_block);
    function.blocks.insert(loop_header, header_block);
    function.blocks.insert(loop_body, body_block);
    function.blocks.insert(loop_exit, exit_block);

    module.functions.insert(func_id, function);
    module
}
