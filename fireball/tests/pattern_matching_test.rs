//! Tests for pattern matching engine

use fireball::ir::low_ir::{
    BasicBlock, BlockId, Constant, Function, FunctionId, Terminator, Type, Value,
};
use fireball::ir::medium_ir::{
    Confidence, Pattern, PatternDatabase, PatternMatcherEngine as PatternMatcher,
    create_standard_pattern_database,
};
use std::collections::BTreeMap;

#[test]
fn test_pattern_matcher_creation() {
    let db = create_standard_pattern_database();
    let _matcher = PatternMatcher::new(db);

    // Pattern matcher created successfully
}

#[test]
fn test_pattern_matcher_confidence_setting() {
    let db = PatternDatabase::default();
    let mut matcher = PatternMatcher::new(db);

    // Set higher confidence threshold
    matcher.set_min_confidence(Confidence::HIGH);

    // Create a simple function to test confidence filtering
    let mut blocks = BTreeMap::new();
    let entry_id = BlockId(0x1000);

    let entry_block = BasicBlock {
        id: entry_id.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(None),
    };

    blocks.insert(entry_id.clone(), entry_block);

    let func = Function {
        id: FunctionId(0x1000),
        entry: entry_id,
        blocks,
        locals: BTreeMap::new(),
        signature: Type::Function {
            ret: Box::new(Type::Void),
            params: vec![],
            varargs: false,
        },
    };

    // Match patterns - should only return high confidence matches
    let matches = matcher.match_function(&func);

    // All results should have high confidence or above
    for m in &matches {
        assert!(
            m.confidence >= Confidence::HIGH,
            "Expected only high confidence matches"
        );
    }
}

#[test]
fn test_simple_if_pattern_detection() {
    let db = PatternDatabase::default();
    let mut matcher = PatternMatcher::new(db);

    // Create a simple function with conditional branch
    let mut blocks = BTreeMap::new();
    let entry_id = BlockId(0x1000);

    let entry_block = BasicBlock {
        id: entry_id.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::CondBranch {
            cond: Value::Constant(Constant::Int {
                value: 1,
                ty: Type::Bool,
            }),
            true_dest: BlockId(0x2000),
            false_dest: BlockId(0x3000),
        },
    };

    blocks.insert(entry_id.clone(), entry_block);

    // Add dummy target blocks
    for addr in [0x2000, 0x3000] {
        let block_id = BlockId(addr);
        let block = BasicBlock {
            id: block_id.clone(),
            phis: vec![],
            instructions: vec![],
            terminator: Terminator::Return(None),
        };
        blocks.insert(block_id, block);
    }

    let func = Function {
        id: FunctionId(0x1000),
        entry: entry_id,
        blocks,
        locals: BTreeMap::new(),
        signature: Type::Function {
            ret: Box::new(Type::Void),
            params: vec![],
            varargs: false,
        },
    };

    // Match patterns
    let matches = matcher.match_function(&func);

    // Should detect if-else pattern
    let has_if_else = matches
        .iter()
        .any(|m| matches!(m.pattern, Pattern::IfElse { .. }));
    assert!(has_if_else, "Should detect if-else pattern");
}

#[test]
fn test_loop_back_edge_detection() {
    let db = PatternDatabase::default();
    let mut matcher = PatternMatcher::new(db);

    // Create a simple loop with back edge
    let mut blocks = BTreeMap::new();

    let entry_id = BlockId(0x1000);
    let loop_id = BlockId(0x1100);
    let exit_id = BlockId(0x1200);

    // Entry block
    let entry_block = BasicBlock {
        id: entry_id.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Branch(loop_id.clone()),
    };

    // Loop block with back edge
    let loop_block = BasicBlock {
        id: loop_id.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::CondBranch {
            cond: Value::Constant(Constant::Int {
                value: 1,
                ty: Type::Bool,
            }),
            true_dest: loop_id.clone(), // Back edge
            false_dest: exit_id.clone(),
        },
    };

    // Exit block
    let exit_block = BasicBlock {
        id: exit_id.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(None),
    };

    blocks.insert(entry_id.clone(), entry_block);
    blocks.insert(loop_id, loop_block);
    blocks.insert(exit_id, exit_block);

    let func = Function {
        id: FunctionId(0x1000),
        entry: entry_id,
        blocks,
        locals: BTreeMap::new(),
        signature: Type::Function {
            ret: Box::new(Type::Void),
            params: vec![],
            varargs: false,
        },
    };

    // Match patterns
    let matches = matcher.match_function(&func);

    // Should detect loop pattern
    let has_loop = matches.iter().any(|m| {
        matches!(
            m.pattern,
            Pattern::WhileLoop { .. } | Pattern::ForLoop { .. } | Pattern::DoWhileLoop { .. }
        )
    });
    assert!(has_loop, "Should detect loop pattern from back edge");
}

#[test]
fn test_match_result_sorting() {
    let db = PatternDatabase::default();
    let mut matcher = PatternMatcher::new(db);

    // Create a function with multiple patterns
    let mut blocks = BTreeMap::new();
    let entry_id = BlockId(0x1000);

    // Entry with two sequential conditions
    let entry_block = BasicBlock {
        id: entry_id.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::CondBranch {
            cond: Value::Constant(Constant::Int {
                value: 1,
                ty: Type::Bool,
            }),
            true_dest: BlockId(0x2000),
            false_dest: BlockId(0x3000),
        },
    };

    blocks.insert(entry_id.clone(), entry_block);

    // Add second conditional block
    let second_block = BasicBlock {
        id: BlockId(0x2000),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::CondBranch {
            cond: Value::Constant(Constant::Int {
                value: 0,
                ty: Type::Bool,
            }),
            true_dest: BlockId(0x3000),
            false_dest: BlockId(0x4000),
        },
    };
    blocks.insert(BlockId(0x2000), second_block);

    // Add terminal blocks
    for addr in [0x3000, 0x4000] {
        let block_id = BlockId(addr);
        let block = BasicBlock {
            id: block_id.clone(),
            phis: vec![],
            instructions: vec![],
            terminator: Terminator::Return(None),
        };
        blocks.insert(block_id, block);
    }

    let func = Function {
        id: FunctionId(0x1000),
        entry: entry_id,
        blocks,
        locals: BTreeMap::new(),
        signature: Type::Function {
            ret: Box::new(Type::Void),
            params: vec![],
            varargs: false,
        },
    };

    // Match patterns
    let matches = matcher.match_function(&func);

    // Should have multiple matches
    assert!(matches.len() >= 2, "Should detect multiple patterns");

    // Check that results are sorted by confidence (descending)
    for i in 1..matches.len() {
        assert!(
            matches[i - 1].confidence >= matches[i].confidence,
            "Results should be sorted by confidence descending"
        );
    }
}

#[test]
fn test_pattern_database_standard() {
    let db = create_standard_pattern_database();

    // Should have standard library functions
    assert!(!db.library_functions.is_empty());
    assert!(db.library_functions.contains_key("libc::malloc"));
    assert!(db.library_functions.contains_key("libc::free"));

    // Should have idioms
    assert!(!db.idioms.is_empty());
}

#[test]
fn test_empty_function_pattern_matching() {
    let db = PatternDatabase::default();
    let mut matcher = PatternMatcher::new(db);

    // Create empty function
    let mut blocks = BTreeMap::new();
    let entry_id = BlockId(0x1000);

    let entry_block = BasicBlock {
        id: entry_id.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Return(None),
    };

    blocks.insert(entry_id.clone(), entry_block);

    let func = Function {
        id: FunctionId(0x1000),
        entry: entry_id,
        blocks,
        locals: BTreeMap::new(),
        signature: Type::Function {
            ret: Box::new(Type::Void),
            params: vec![],
            varargs: false,
        },
    };

    // Match patterns
    let matches = matcher.match_function(&func);

    // Even empty function might have some patterns (e.g., LowIR pattern)
    // but they should be low confidence
    for m in &matches {
        assert!(m.confidence <= Confidence::MEDIUM);
    }
}

#[test]
fn test_switch_pattern() {
    let db = PatternDatabase::default();
    let mut matcher = PatternMatcher::new(db);

    // Create function with switch
    let mut blocks = BTreeMap::new();
    let entry_id = BlockId(0x1000);

    let mut cases = BTreeMap::new();
    cases.insert(
        Constant::Int {
            value: 1,
            ty: Type::I32,
        },
        BlockId(0x2000),
    );
    cases.insert(
        Constant::Int {
            value: 2,
            ty: Type::I32,
        },
        BlockId(0x3000),
    );

    let entry_block = BasicBlock {
        id: entry_id.clone(),
        phis: vec![],
        instructions: vec![],
        terminator: Terminator::Switch {
            value: Value::Constant(Constant::Int {
                value: 0,
                ty: Type::I32,
            }),
            default: BlockId(0x4000),
            cases,
        },
    };

    blocks.insert(entry_id.clone(), entry_block);

    // Add case blocks
    for addr in [0x2000, 0x3000, 0x4000] {
        let block_id = BlockId(addr);
        let block = BasicBlock {
            id: block_id.clone(),
            phis: vec![],
            instructions: vec![],
            terminator: Terminator::Return(None),
        };
        blocks.insert(block_id, block);
    }

    let func = Function {
        id: FunctionId(0x1000),
        entry: entry_id,
        blocks,
        locals: BTreeMap::new(),
        signature: Type::Function {
            ret: Box::new(Type::Void),
            params: vec![],
            varargs: false,
        },
    };

    // Match patterns
    let matches = matcher.match_function(&func);

    // Should detect switch pattern
    let has_switch = matches
        .iter()
        .any(|m| matches!(m.pattern, Pattern::SwitchCase { .. }));
    assert!(has_switch, "Should detect switch pattern");
}
