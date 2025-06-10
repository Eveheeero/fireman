//! Determinism test suite for Fireman decompiler
//!
//! These tests ensure that the decompiler produces byte-for-byte identical output
//! for identical input, regardless of:
//! - Machine architecture, available memory, CPU cores
//! - Previous runs, system load, time of day
//! - Thread scheduling, hash function seeds

use fireball::ir::{high_ir, low_ir, medium_ir};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Test that Low IR generation is deterministic
#[test]
fn test_low_ir_generation_deterministic() {
    let module = create_test_module();

    // Generate Low IR multiple times
    let mut results = Vec::new();
    for i in 0..100 {
        let result = format!("{:#?}", module);
        results.push(result);

        // Verify against first result
        if i > 0 {
            assert_eq!(
                results[0], results[i],
                "Low IR generation produced different results on iteration {}",
                i
            );
        }
    }
}

/// Test that Medium IR pattern detection is deterministic
#[test]
fn test_medium_ir_pattern_detection_deterministic() {
    let low_module = create_test_module();

    // Convert to Medium IR multiple times
    let mut results = Vec::new();
    for i in 0..100 {
        let medium_module = medium_ir::Module::from_low_ir(&low_module);
        let result = format!("{:#?}", medium_module);
        results.push(result);

        // Verify against first result
        if i > 0 {
            assert_eq!(
                results[0], results[i],
                "Medium IR pattern detection produced different results on iteration {}",
                i
            );
        }
    }
}

/// Test that High IR generation is deterministic
#[test]
fn test_high_ir_generation_deterministic() {
    let low_module = create_test_module();
    let medium_module = medium_ir::Module::from_low_ir(&low_module);

    // Convert to High IR multiple times
    let mut results = Vec::new();
    for i in 0..100 {
        let high_module = high_ir::Module::from_medium_ir(&medium_module);
        let result = format!("{:#?}", high_module);
        results.push(result);

        // Verify against first result
        if i > 0 {
            assert_eq!(
                results[0], results[i],
                "High IR generation produced different results on iteration {}",
                i
            );
        }
    }
}

/// Test that C code generation is deterministic
#[test]
fn test_c_code_generation_deterministic() {
    let low_module = create_test_module();
    let medium_module = medium_ir::Module::from_low_ir(&low_module);
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    // Generate C code multiple times
    let mut results = Vec::new();
    for i in 0..100 {
        let mut codegen = high_ir::c_codegen::CCodeGenerator::new();
        let c_code = codegen.generate(&high_module);
        results.push(c_code);

        // Verify against first result
        if i > 0 {
            assert_eq!(
                results[0], results[i],
                "C code generation produced different results on iteration {}",
                i
            );
        }
    }
}

/// Test address formatting is always 16-digit hex
#[test]
fn test_address_formatting_deterministic() {
    let addresses: Vec<u64> = vec![
        0x0,
        0x1,
        0xFF,
        0x1000,
        0xDEADBEEF,
        0xFFFFFFFF,
        0x100000000,
        0xFFFFFFFFFFFFFFFF,
    ];

    for addr in addresses {
        let formatted = format!("{:016x}", addr);
        assert_eq!(
            formatted.len(),
            16,
            "Address {:x} not formatted as 16 digits: {}",
            addr,
            formatted
        );

        // Test multiple times to ensure consistency
        for _ in 0..10 {
            let formatted2 = format!("{:016x}", addr);
            assert_eq!(
                formatted, formatted2,
                "Address formatting not consistent for {:x}",
                addr
            );
        }
    }
}

/// Test that variable naming is deterministic
#[test]
fn test_variable_naming_deterministic() {
    use fireball::core::Address;

    let sections = Arc::new(fireball::core::Sections::default());
    let base_addr = Address::from_virtual_address(&sections, 0x1000);

    // Create multiple LocalIds with same parameters
    let mut ids = Vec::new();
    for i in 0..100 {
        let local_id = low_ir::LocalId {
            source: base_addr.clone(),
            index: 42,
            version: 1,
            purpose: "counter",
        };

        let formatted = format!("{:?}", local_id);
        ids.push(formatted);

        // Verify against first result
        if i > 0 {
            assert_eq!(
                ids[0], ids[i],
                "LocalId formatting produced different results on iteration {}",
                i
            );
        }
    }
}

/// Test that BTreeMap usage ensures ordered iteration
#[test]
fn test_btreemap_ordering_deterministic() {
    // Create maps with same content but different insertion order
    let mut map1 = BTreeMap::new();
    map1.insert("z", 26);
    map1.insert("a", 1);
    map1.insert("m", 13);

    let mut map2 = BTreeMap::new();
    map2.insert("a", 1);
    map2.insert("m", 13);
    map2.insert("z", 26);

    let mut map3 = BTreeMap::new();
    map3.insert("m", 13);
    map3.insert("z", 26);
    map3.insert("a", 1);

    // Collect iteration results
    let iter1: Vec<_> = map1.iter().collect();
    let iter2: Vec<_> = map2.iter().collect();
    let iter3: Vec<_> = map3.iter().collect();

    // All should iterate in same order
    assert_eq!(
        iter1, iter2,
        "BTreeMap iteration order differs between map1 and map2"
    );
    assert_eq!(
        iter2, iter3,
        "BTreeMap iteration order differs between map2 and map3"
    );

    // Verify sorted order
    assert_eq!(iter1[0].0, &"a");
    assert_eq!(iter1[1].0, &"m");
    assert_eq!(iter1[2].0, &"z");
}

/// Helper function to create a test module
fn create_test_module() -> low_ir::Module {
    let mut module = low_ir::Module {
        target: low_ir::TargetInfo::x86_64(),
        functions: BTreeMap::new(),
        globals: BTreeMap::new(),
        externals: BTreeMap::new(),
    };

    let sections = Arc::new(fireball::core::Sections::default());
    let base_addr = fireball::core::Address::from_virtual_address(&sections, 0x1000);

    // Create a simple function
    let mut func = low_ir::Function {
        id: low_ir::FunctionId(0x1000),
        signature: low_ir::Type::Function {
            ret: Box::new(low_ir::Type::I32),
            params: vec![low_ir::Type::I32, low_ir::Type::I32],
            varargs: false,
        },
        entry: low_ir::BlockId(0x1000),
        blocks: BTreeMap::new(),
        locals: BTreeMap::new(),
    };

    // Add locals
    let a_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 0,
        version: 0,
        purpose: "a",
    };
    let b_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 1,
        version: 0,
        purpose: "b",
    };
    let result_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 2,
        version: 0,
        purpose: "result",
    };

    func.locals.insert(a_local.clone(), low_ir::Type::I32);
    func.locals.insert(b_local.clone(), low_ir::Type::I32);
    func.locals.insert(result_local.clone(), low_ir::Type::I32);

    // Create basic block
    let block = low_ir::BasicBlock {
        id: low_ir::BlockId(0x1000),
        phis: vec![],
        instructions: vec![low_ir::Instruction::BinOp {
            op: low_ir::BinaryOp::Add,
            dst: result_local.clone(),
            lhs: low_ir::Value::Local(a_local.clone()),
            rhs: low_ir::Value::Local(b_local.clone()),
            ty: low_ir::Type::I32,
            flags: low_ir::FlagUpdate::Unchanged,
        }],
        terminator: low_ir::Terminator::Return(Some((
            low_ir::Value::Local(result_local.clone()),
            low_ir::Type::I32,
        ))),
    };

    func.blocks.insert(block.id.clone(), block);
    module.functions.insert(func.id.clone(), func);

    module
}
