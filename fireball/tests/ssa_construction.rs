//! SSA construction tests

use fireball::core::{Address, Sections};
use fireball::ir::low_ir::{
    BasicBlock, BinaryOp, BlockId, Constant, FlagUpdate, Function, FunctionId, Instruction,
    LocalId, Module, SSABuilder, TargetInfo, Terminator, Type, Value,
};
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
fn test_simple_ssa_construction() {
    // Create a simple module with a function that has a conditional
    let _module = Module::new(TargetInfo::x86_64());

    let func_id = FunctionId(0x1000);
    let entry_block = BlockId(0x1000);
    let then_block = BlockId(0x1010);
    let else_block = BlockId(0x1020);
    let merge_block = BlockId(0x1030);

    let sections = Arc::new(Sections::default());

    // Create function
    let mut function = Function {
        id: func_id.clone(),
        signature: Type::Function {
            ret: Box::new(Type::Void),
            params: vec![],
            varargs: false,
        },
        entry: entry_block.clone(),
        blocks: BTreeMap::new(),
        locals: BTreeMap::new(),
    };

    // Entry block: if (condition) goto then_block else goto else_block
    let entry_bb = BasicBlock {
        id: entry_block.clone(),
        phis: vec![],
        instructions: vec![
            // x = 1
            Instruction::Assign {
                dst: LocalId {
                    source: Address::from_virtual_address(&sections, 0x1000),
                    purpose: "x",
                    index: 0,
                    version: 0,
                },
                value: Value::Constant(Constant::Int {
                    value: 1,
                    ty: Type::I64,
                }),
                source_addr: Address::from_virtual_address(&sections, 0x1000),
            },
            // cond = ...
            Instruction::Assign {
                dst: LocalId {
                    source: Address::from_virtual_address(&sections, 0x1001),
                    purpose: "cond",
                    index: 0,
                    version: 0,
                },
                value: Value::Constant(Constant::Int {
                    value: 1,
                    ty: Type::Bool,
                }),
                source_addr: Address::from_virtual_address(&sections, 0x1001),
            },
        ],
        terminator: Terminator::CondBranch {
            cond: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x1001),
                purpose: "cond",
                index: 0,
                version: 0,
            }),
            true_dest: then_block.clone(),
            false_dest: else_block.clone(),
        },
    };

    // Then block: x = 2; goto merge
    let then_bb = BasicBlock {
        id: then_block.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x1010),
                purpose: "x",
                index: 0,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 2,
                ty: Type::I64,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x1010),
        }],
        terminator: Terminator::Branch(merge_block.clone()),
    };

    // Else block: x = 3; goto merge
    let else_bb = BasicBlock {
        id: else_block.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x1020),
                purpose: "x",
                index: 0,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 3,
                ty: Type::I64,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x1020),
        }],
        terminator: Terminator::Branch(merge_block.clone()),
    };

    // Merge block: return x
    let merge_bb = BasicBlock {
        id: merge_block.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(Some((
            Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x1030),
                purpose: "x",
                index: 0,
                version: 0,
            }),
            Type::I64,
        ))),
    };

    // Add blocks to function
    function.blocks.insert(entry_block.clone(), entry_bb);
    function.blocks.insert(then_block.clone(), then_bb);
    function.blocks.insert(else_block.clone(), else_bb);
    function.blocks.insert(merge_block.clone(), merge_bb);

    // Add locals
    function.locals.insert(
        LocalId {
            source: Address::from_virtual_address(&sections, 0x1000),
            purpose: "x",
            index: 0,
            version: 0,
        },
        Type::I64,
    );
    function.locals.insert(
        LocalId {
            source: Address::from_virtual_address(&sections, 0x1001),
            purpose: "cond",
            index: 0,
            version: 0,
        },
        Type::Bool,
    );

    // Build SSA
    let mut ssa_builder = SSABuilder::new();
    ssa_builder
        .build_ssa(&mut function)
        .expect("SSA construction failed");

    // Verify results
    // 1. Check that merge block has a phi for x
    let merge_block_ref = function.blocks.get(&merge_block).unwrap();
    assert!(
        !merge_block_ref.phis.is_empty(),
        "Merge block should have phi nodes"
    );

    // 2. Check that variables have been versioned
    let mut has_versioned_vars = false;
    for block in function.blocks.values() {
        for inst in &block.instructions {
            if let Instruction::Assign { dst, .. } = inst {
                if dst.version > 0 {
                    has_versioned_vars = true;
                    break;
                }
            }
        }
    }
    assert!(
        has_versioned_vars,
        "Variables should be versioned in SSA form"
    );

    println!("✓ Simple SSA construction test passed");
}

#[test]
fn test_ssa_determinism() {
    // Create the same CFG multiple times and verify SSA is deterministic
    let sections = Arc::new(Sections::default());
    let mut hashes = Vec::new();

    for i in 0..10 {
        // Pollute memory differently
        let _garbage: Vec<_> = (0..i * 100).map(|x| vec![x as u8; x % 100]).collect();

        let mut module = create_test_module(&sections);
        let func_id = FunctionId(0x2000);
        let function = module.functions.get_mut(&func_id).unwrap();

        let mut ssa_builder = SSABuilder::new();
        ssa_builder
            .build_ssa(function)
            .expect("SSA construction failed");

        // Hash the SSA form
        let hash = hash_ssa_function(function);
        hashes.push(hash);
    }

    // All hashes must be identical
    let first_hash = &hashes[0];
    for (i, hash) in hashes.iter().enumerate() {
        assert_eq!(
            first_hash, hash,
            "SSA construction produced different output on iteration {}!",
            i
        );
    }

    println!("✓ SSA determinism test passed");
}

fn create_test_module(sections: &Arc<Sections>) -> Module {
    let mut module = Module::new(TargetInfo::x86_64());
    let func_id = FunctionId(0x2000);
    let entry = BlockId(0x2000);
    let loop_header = BlockId(0x2010);
    let loop_body = BlockId(0x2020);
    let loop_exit = BlockId(0x2030);

    let mut function = Function {
        id: func_id.clone(),
        signature: Type::Function {
            ret: Box::new(Type::I64),
            params: vec![],
            varargs: false,
        },
        entry: entry.clone(),
        blocks: BTreeMap::new(),
        locals: BTreeMap::new(),
    };

    // Create a simple loop structure
    // entry -> loop_header -> loop_body -> loop_header | loop_exit

    // Entry: i = 0; goto loop_header
    let entry_bb = BasicBlock {
        id: entry.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(sections, 0x2000),
                purpose: "i",
                index: 0,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 0,
                ty: Type::I64,
            }),
            source_addr: Address::from_virtual_address(sections, 0x2000),
        }],
        terminator: Terminator::Branch(loop_header.clone()),
    };

    // Loop header: if (i < 10) goto loop_body else goto loop_exit
    let header_bb = BasicBlock {
        id: loop_header.clone(),
        phis: vec![],
        instructions: vec![
            // cond = i < 10
            Instruction::BinOp {
                op: BinaryOp::Slt,
                dst: LocalId {
                    source: Address::from_virtual_address(sections, 0x2010),
                    purpose: "cond",
                    index: 0,
                    version: 0,
                },
                lhs: Value::Local(LocalId {
                    source: Address::from_virtual_address(sections, 0x2010),
                    purpose: "i",
                    index: 0,
                    version: 0,
                }),
                rhs: Value::Constant(Constant::Int {
                    value: 10,
                    ty: Type::I64,
                }),
                ty: Type::Bool,
                flags: FlagUpdate::Unchanged,
            },
        ],
        terminator: Terminator::CondBranch {
            cond: Value::Local(LocalId {
                source: Address::from_virtual_address(sections, 0x2010),
                purpose: "cond",
                index: 0,
                version: 0,
            }),
            true_dest: loop_body.clone(),
            false_dest: loop_exit.clone(),
        },
    };

    // Loop body: i = i + 1; goto loop_header
    let body_bb = BasicBlock {
        id: loop_body.clone(),
        phis: vec![],
        instructions: vec![
            // i = i + 1
            Instruction::BinOp {
                op: BinaryOp::Add,
                dst: LocalId {
                    source: Address::from_virtual_address(sections, 0x2020),
                    purpose: "i",
                    index: 0,
                    version: 0,
                },
                lhs: Value::Local(LocalId {
                    source: Address::from_virtual_address(sections, 0x2020),
                    purpose: "i",
                    index: 0,
                    version: 0,
                }),
                rhs: Value::Constant(Constant::Int {
                    value: 1,
                    ty: Type::I64,
                }),
                ty: Type::I64,
                flags: FlagUpdate::Unchanged,
            },
        ],
        terminator: Terminator::Branch(loop_header.clone()),
    };

    // Loop exit: return i
    let exit_bb = BasicBlock {
        id: loop_exit.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(Some((
            Value::Local(LocalId {
                source: Address::from_virtual_address(sections, 0x2030),
                purpose: "i",
                index: 0,
                version: 0,
            }),
            Type::I64,
        ))),
    };

    function.blocks.insert(entry, entry_bb);
    function.blocks.insert(loop_header, header_bb);
    function.blocks.insert(loop_body, body_bb);
    function.blocks.insert(loop_exit, exit_bb);

    // Add locals
    function.locals.insert(
        LocalId {
            source: Address::from_virtual_address(sections, 0x2000),
            purpose: "i",
            index: 0,
            version: 0,
        },
        Type::I64,
    );
    function.locals.insert(
        LocalId {
            source: Address::from_virtual_address(sections, 0x2010),
            purpose: "cond",
            index: 0,
            version: 0,
        },
        Type::Bool,
    );

    module.functions.insert(func_id, function);
    module
}

fn hash_ssa_function(function: &Function) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();

    // Hash blocks in order
    for (block_id, block) in &function.blocks {
        hasher.update(&block_id.0.to_le_bytes());

        // Hash phis
        hasher.update(&(block.phis.len() as u64).to_le_bytes());
        for phi in &block.phis {
            if let Instruction::Phi { dst, incoming, .. } = phi {
                hasher.update(&dst.version.to_le_bytes());
                hasher.update(&(incoming.len() as u64).to_le_bytes());
            }
        }

        // Hash instructions with versions
        hasher.update(&(block.instructions.len() as u64).to_le_bytes());
        for inst in &block.instructions {
            // Hash version of definition if any
            match inst {
                Instruction::Assign { dst, .. }
                | Instruction::BinOp { dst, .. }
                | Instruction::UnOp { dst, .. }
                | Instruction::Load { dst, .. }
                | Instruction::Cast { dst, .. } => {
                    hasher.update(&dst.version.to_le_bytes());
                }
                _ => {}
            }
        }
    }

    let result = hasher.finalize();
    format!("{:x}", result)
}
