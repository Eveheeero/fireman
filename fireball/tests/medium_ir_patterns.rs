//! Tests for Medium IR pattern matching

use fireball::core::{Address, Sections};
use fireball::ir::low_ir::{
    BasicBlock, BinaryOp, BlockId, Constant, FlagUpdate, Function, FunctionId, Instruction,
    LocalId, Module as LowModule, TargetInfo, Terminator, Type, Value,
};
use fireball::ir::medium_ir::{self, Confidence, Pattern, PatternRef};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Test pattern detection in a simple for loop
#[test]
fn test_for_loop_pattern_detection() {
    let module = create_for_loop_module();
    let medium_module = medium_ir::Module::from_low_ir(&module);

    // Check that we detected a function
    assert_eq!(medium_module.functions.len(), 1);

    let func = medium_module.functions.values().next().unwrap();

    // Check that patterns were detected
    // Note: We can't directly access patterns field as it's private
    // Instead, we check the body pattern
    let body_ref = func.body;

    // Look for a for loop pattern
    let mut found_for_loop = false;
    if let Some(pattern) = func.patterns.get(body_ref) {
        // The body might be wrapped in an Expression pattern
        match pattern {
            Pattern::Expression { operands, .. } => {
                // Check operands for for loop
                for op_ref in operands {
                    if let Some(Pattern::ForLoop {
                        init,
                        condition: _,
                        increment,
                        confidence,
                        ..
                    }) = func.patterns.get(*op_ref)
                    {
                        found_for_loop = true;
                        assert!(init.is_some(), "For loop should have initialization");
                        assert!(increment.is_some(), "For loop should have increment");
                        assert!(
                            confidence >= &Confidence::LOW,
                            "Confidence should be at least LOW"
                        );
                    }
                }
            }
            Pattern::ForLoop {
                init,
                condition: _,
                increment,
                confidence,
                ..
            } => {
                found_for_loop = true;
                assert!(init.is_some(), "For loop should have initialization");
                assert!(increment.is_some(), "For loop should have increment");
                assert!(
                    confidence >= &Confidence::LOW,
                    "Confidence should be at least LOW"
                );
            }
            _ => {}
        }
    }

    assert!(found_for_loop, "Should detect for loop pattern");
}

/// Test pattern detection in a while loop
#[test]
fn test_while_loop_pattern_detection() {
    let module = create_while_loop_module();
    let medium_module = medium_ir::Module::from_low_ir(&module);

    let func = medium_module.functions.values().next().unwrap();

    // Look for a while loop pattern
    let mut found_while_loop = false;
    let body_ref = func.body;

    if let Some(pattern) = func.patterns.get(body_ref) {
        match pattern {
            Pattern::Expression { operands, .. } => {
                // Check operands for while loop
                for op_ref in operands {
                    if let Some(Pattern::WhileLoop {
                        condition: _,
                        confidence,
                        ..
                    }) = func.patterns.get(*op_ref)
                    {
                        found_while_loop = true;
                        assert!(
                            confidence >= &Confidence::LOW,
                            "Confidence should be at least LOW"
                        );
                    }
                }
            }
            Pattern::WhileLoop {
                condition: _,
                confidence,
                ..
            } => {
                found_while_loop = true;
                assert!(
                    confidence >= &Confidence::LOW,
                    "Confidence should be at least LOW"
                );
            }
            _ => {}
        }
    }

    assert!(found_while_loop, "Should detect while loop pattern");
}

/// Test confidence scoring
#[test]
fn test_confidence_scoring() {
    use fireball::ir::medium_ir::confidence::ConfidenceCalculator;

    let calc = ConfidenceCalculator::default();

    // Test LowIR pattern (should have high confidence)
    let low_ir_pattern = Pattern::LowIR {
        instructions: vec![],
        terminator: None,
        source_block: BlockId(0),
        confidence: Confidence::CERTAIN,
    };
    let confidence = calc.calculate(&low_ir_pattern);
    assert!(
        confidence >= Confidence::HIGH,
        "LowIR should have high confidence"
    );

    // Test incomplete for loop (should have lower confidence)
    let incomplete_for = Pattern::ForLoop {
        init: None,
        condition: PatternRef(0),
        increment: None,
        body: PatternRef(1),
        confidence: Confidence::MEDIUM,
    };
    let confidence = calc.calculate(&incomplete_for);
    assert!(
        confidence < Confidence::HIGH,
        "Incomplete for loop should have lower confidence"
    );
}

/// Test pattern store functionality
#[test]
fn test_pattern_store() {
    let mut store = medium_ir::PatternStore::new();

    // Insert patterns
    let pattern1 = Pattern::Expression {
        operation: medium_ir::ExpressionOp::Add,
        operands: vec![],
        confidence: Confidence::HIGH,
    };
    let ref1 = store.insert(pattern1.clone());

    let pattern2 = Pattern::Expression {
        operation: medium_ir::ExpressionOp::Mul,
        operands: vec![],
        confidence: Confidence::MEDIUM,
    };
    let ref2 = store.insert(pattern2.clone());

    // Check retrieval
    assert_eq!(store.get(ref1), Some(&pattern1));
    assert_eq!(store.get(ref2), Some(&pattern2));
    assert_ne!(ref1, ref2);
}

// Helper functions to create test modules

fn create_for_loop_module() -> LowModule {
    let sections = Arc::new(Sections::default());
    let mut module = LowModule::new(TargetInfo::x86_64());

    let func_id = FunctionId(0x1000);
    let entry = BlockId(0x1000);
    let loop_header = BlockId(0x1010);
    let loop_body = BlockId(0x1020);
    let loop_exit = BlockId(0x1030);

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

    // Entry block: i = 0
    let entry_block = BasicBlock {
        id: entry.clone(),
        phis: vec![],
        instructions: vec![Instruction::Assign {
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x1000),
                purpose: "i",
                index: 0,
                version: 0,
            },
            value: Value::Constant(Constant::Int {
                value: 0,
                ty: Type::I32,
            }),
            source_addr: Address::from_virtual_address(&sections, 0x1000),
        }],
        terminator: Terminator::Branch(loop_header.clone()),
    };

    // Loop header: if (i < 10) goto body else goto exit
    let header_block = BasicBlock {
        id: loop_header.clone(),
        phis: vec![],
        instructions: vec![Instruction::BinOp {
            op: BinaryOp::Slt,
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x1010),
                purpose: "cond",
                index: 0,
                version: 0,
            },
            lhs: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x1000),
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
                source: Address::from_virtual_address(&sections, 0x1010),
                purpose: "cond",
                index: 0,
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
        instructions: vec![Instruction::BinOp {
            op: BinaryOp::Add,
            dst: LocalId {
                source: Address::from_virtual_address(&sections, 0x1020),
                purpose: "i",
                index: 0,
                version: 1,
            },
            lhs: Value::Local(LocalId {
                source: Address::from_virtual_address(&sections, 0x1000),
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
        }],
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
