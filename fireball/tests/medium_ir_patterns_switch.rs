//! Test for switch/case pattern detection

use fireball::core::{Address, Sections};
use fireball::ir::low_ir::{
    BasicBlock, BlockId, Constant, Function, FunctionId, Instruction, LocalId, Module as LowModule,
    TargetInfo, Terminator, Type, Value,
};
use fireball::ir::medium_ir::{self, Confidence, Pattern};
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
#[ignore = "Switch/case pattern detection not implemented yet"]
fn test_switch_case_pattern_detection() {
    let module = create_switch_case_module();
    let medium_module = medium_ir::Module::from_low_ir(&module);

    let func = medium_module.functions.values().next().unwrap();

    // Look for switch/case patterns
    let mut found_switch = false;
    let body_ref = func.body;

    if let Some(pattern) = func.patterns.get(body_ref) {
        if let Pattern::Expression { operands, .. } = pattern {
            // Check operands for switch patterns
            for op_ref in operands {
                if let Some(Pattern::SwitchCase {
                    cases, confidence, ..
                }) = func.patterns.get(*op_ref)
                {
                    found_switch = true;
                    assert!(!cases.is_empty(), "Switch should have cases");
                    assert!(
                        confidence >= &Confidence::MEDIUM,
                        "Switch patterns should have at least medium confidence"
                    );
                }
            }
        }
    }

    assert!(found_switch, "Should detect switch/case pattern");
}

fn create_switch_case_module() -> LowModule {
    let sections = Arc::new(Sections::default());
    let mut module = LowModule::new(TargetInfo::x86_64());

    let func_id = FunctionId(0x4000);
    let entry = BlockId(0x4000);
    let case0 = BlockId(0x4010);
    let case1 = BlockId(0x4020);
    let case2 = BlockId(0x4030);
    let default = BlockId(0x4040);
    let exit = BlockId(0x4050);

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

    // Entry block with switch
    let entry_block = BasicBlock {
        id: entry.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Switch {
            value: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x4000),
                purpose: "x",
                index: 0,
                version: 0,
            }),
            cases: {
                let mut cases = BTreeMap::new();
                cases.insert(
                    Constant::Int {
                        value: 0,
                        ty: Type::I32,
                    },
                    case0.clone(),
                );
                cases.insert(
                    Constant::Int {
                        value: 1,
                        ty: Type::I32,
                    },
                    case1.clone(),
                );
                cases.insert(
                    Constant::Int {
                        value: 2,
                        ty: Type::I32,
                    },
                    case2.clone(),
                );
                cases
            },
            default: default.clone(),
        },
    };

    // Case 0: return 10
    let case0_block = BasicBlock {
        id: case0.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x4010),
                purpose: "result",
                index: 1,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 10,
                ty: Type::I32,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x4010),
        }],
        terminator: Terminator::Branch(exit.clone()),
    };

    // Case 1: return 20
    let case1_block = BasicBlock {
        id: case1.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x4020),
                purpose: "result",
                index: 1,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 20,
                ty: Type::I32,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x4020),
        }],
        terminator: Terminator::Branch(exit.clone()),
    };

    // Case 2: return 30
    let case2_block = BasicBlock {
        id: case2.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x4030),
                purpose: "result",
                index: 1,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 30,
                ty: Type::I32,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x4030),
        }],
        terminator: Terminator::Branch(exit.clone()),
    };

    // Default: return -1
    let default_block = BasicBlock {
        id: default.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x4040),
                purpose: "result",
                index: 1,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: -1,
                ty: Type::I32,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x4040),
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
                source: Address::from_virtual_address(&sections, 0x4040),
                purpose: "result",
                index: 1,
                version: 0,
            }),
            Type::I32,
        ))),
    };

    function.blocks.insert(entry, entry_block);
    function.blocks.insert(case0, case0_block);
    function.blocks.insert(case1, case1_block);
    function.blocks.insert(case2, case2_block);
    function.blocks.insert(default, default_block);
    function.blocks.insert(exit, exit_block);

    module.functions.insert(func_id, function);
    module
}
