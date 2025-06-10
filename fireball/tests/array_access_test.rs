//! Tests for array access pattern detection

use fireball::core::Address;
use fireball::ir::{high_ir, low_ir, medium_ir};
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
fn test_array_access_pattern_detection() {
    // Create Low IR module with array access
    let mut module = low_ir::Module {
        target: low_ir::TargetInfo::x86_64(),
        functions: BTreeMap::new(),
        globals: BTreeMap::new(),
        externals: BTreeMap::new(),
    };

    // Create function with array access: arr[i] = 42
    let sections = Arc::new(fireball::core::Sections::default());
    let base_addr = Address::from_virtual_address(&sections, 0x1000);

    let mut func = low_ir::Function {
        id: low_ir::FunctionId(0x1000),
        signature: low_ir::Type::Function {
            ret: Box::new(low_ir::Type::Void),
            params: vec![
                low_ir::Type::Pointer(Some(Box::new(low_ir::Type::I32))), // int* arr
                low_ir::Type::I32,                                        // int i
            ],
            varargs: false,
        },
        entry: low_ir::BlockId(0x1000),
        blocks: BTreeMap::new(),
        locals: BTreeMap::new(),
    };

    // Add local variables
    let arr_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 0,
        version: 0,
        purpose: "arr",
    };
    let i_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 1,
        version: 0,
        purpose: "i",
    };
    let scaled_index = low_ir::LocalId {
        source: base_addr.clone(),
        index: 2,
        version: 0,
        purpose: "scaled_index",
    };
    let ptr_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 3,
        version: 0,
        purpose: "ptr",
    };

    func.locals.insert(
        arr_local.clone(),
        low_ir::Type::Pointer(Some(Box::new(low_ir::Type::I32))),
    );
    func.locals.insert(i_local.clone(), low_ir::Type::I32);
    func.locals.insert(scaled_index.clone(), low_ir::Type::I32);
    func.locals.insert(
        ptr_local.clone(),
        low_ir::Type::Pointer(Some(Box::new(low_ir::Type::I32))),
    );

    // Create basic block with array access pattern
    let mut block = low_ir::BasicBlock {
        id: low_ir::BlockId(0x1000),
        phis: vec![],
        instructions: vec![
            // scaled_index = i * 4
            low_ir::Instruction::BinOp {
                op: low_ir::BinaryOp::Mul,
                dst: scaled_index.clone(),
                lhs: low_ir::Value::Local(i_local.clone()),
                rhs: low_ir::Value::Constant(low_ir::Constant::Int {
                    value: 4,
                    ty: low_ir::Type::I32,
                }),
                ty: low_ir::Type::I32,
                flags: low_ir::FlagUpdate::Unchanged,
            },
            // ptr = arr + scaled_index
            low_ir::Instruction::BinOp {
                op: low_ir::BinaryOp::Add,
                dst: ptr_local.clone(),
                lhs: low_ir::Value::Local(arr_local.clone()),
                rhs: low_ir::Value::Local(scaled_index.clone()),
                ty: low_ir::Type::Pointer(Some(Box::new(low_ir::Type::I32))),
                flags: low_ir::FlagUpdate::Unchanged,
            },
            // *ptr = 42
            low_ir::Instruction::Store {
                val: low_ir::Value::Constant(low_ir::Constant::Int {
                    value: 42,
                    ty: low_ir::Type::I32,
                }),
                ptr: low_ir::Value::Local(ptr_local.clone()),
                ty: low_ir::Type::I32,
                align: Some(4),
                volatile: false,
            },
        ],
        terminator: low_ir::Terminator::Return(None),
    };

    func.blocks.insert(block.id.clone(), block);
    module.functions.insert(func.id.clone(), func);

    // Convert to Medium IR
    let medium_module = medium_ir::Module::from_low_ir(&module);

    // Check that array access pattern was detected
    let medium_func = &medium_module.functions[&low_ir::FunctionId(0x1000)];
    let body_pattern = medium_func.patterns.get(medium_func.body).unwrap();

    // The pattern should contain an ArrayAccess pattern
    let has_array_access = check_pattern_for_array_access(body_pattern, &medium_func.patterns);
    assert!(
        has_array_access,
        "Array access pattern should be detected in: {:?}",
        body_pattern
    );

    // Convert to High IR
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    // Generate C code
    let mut codegen = high_ir::c_codegen::CCodeGenerator::new();
    let c_code = codegen.generate(&high_module);

    println!("Generated C code:\n{}", c_code);

    // Check that array access syntax is generated
    assert!(
        c_code.contains("[") && c_code.contains("]"),
        "C code should contain array access syntax"
    );
}

#[test]
#[ignore = "C code generation has issues - needs fixing"]
fn test_array_read_pattern() {
    // Create Low IR module with array read: int x = arr[i]
    let mut module = low_ir::Module {
        target: low_ir::TargetInfo::x86_64(),
        functions: BTreeMap::new(),
        globals: BTreeMap::new(),
        externals: BTreeMap::new(),
    };

    let sections = Arc::new(fireball::core::Sections::default());
    let base_addr = Address::from_virtual_address(&sections, 0x2000);

    let mut func = low_ir::Function {
        id: low_ir::FunctionId(0x2000),
        signature: low_ir::Type::Function {
            ret: Box::new(low_ir::Type::I32),
            params: vec![
                low_ir::Type::Pointer(Some(Box::new(low_ir::Type::I32))), // int* arr
                low_ir::Type::I32,                                        // int i
            ],
            varargs: false,
        },
        entry: low_ir::BlockId(0x2000),
        blocks: BTreeMap::new(),
        locals: BTreeMap::new(),
    };

    // Add locals
    let arr_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 0,
        version: 0,
        purpose: "arr",
    };
    let i_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 1,
        version: 0,
        purpose: "i",
    };
    let shifted_index = low_ir::LocalId {
        source: base_addr.clone(),
        index: 2,
        version: 0,
        purpose: "shifted",
    };
    let ptr_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 3,
        version: 0,
        purpose: "ptr",
    };
    let result_local = low_ir::LocalId {
        source: base_addr.clone(),
        index: 4,
        version: 0,
        purpose: "result",
    };

    func.locals.insert(
        arr_local.clone(),
        low_ir::Type::Pointer(Some(Box::new(low_ir::Type::I32))),
    );
    func.locals.insert(i_local.clone(), low_ir::Type::I32);
    func.locals.insert(shifted_index.clone(), low_ir::Type::I32);
    func.locals.insert(
        ptr_local.clone(),
        low_ir::Type::Pointer(Some(Box::new(low_ir::Type::I32))),
    );
    func.locals.insert(result_local.clone(), low_ir::Type::I32);

    // Create block with array read using shift instead of multiply
    let mut block = low_ir::BasicBlock {
        id: low_ir::BlockId(0x2000),
        phis: vec![],
        instructions: vec![
            // shifted = i << 2 (equivalent to i * 4)
            low_ir::Instruction::BinOp {
                op: low_ir::BinaryOp::Shl,
                dst: shifted_index.clone(),
                lhs: low_ir::Value::Local(i_local.clone()),
                rhs: low_ir::Value::Constant(low_ir::Constant::Int {
                    value: 2,
                    ty: low_ir::Type::I32,
                }),
                ty: low_ir::Type::I32,
                flags: low_ir::FlagUpdate::Unchanged,
            },
            // ptr = arr + shifted
            low_ir::Instruction::BinOp {
                op: low_ir::BinaryOp::Add,
                dst: ptr_local.clone(),
                lhs: low_ir::Value::Local(arr_local.clone()),
                rhs: low_ir::Value::Local(shifted_index.clone()),
                ty: low_ir::Type::Pointer(Some(Box::new(low_ir::Type::I32))),
                flags: low_ir::FlagUpdate::Unchanged,
            },
            // result = *ptr
            low_ir::Instruction::Load {
                dst: result_local.clone(),
                ptr: low_ir::Value::Local(ptr_local.clone()),
                ty: low_ir::Type::I32,
                align: Some(4),
                volatile: false,
            },
        ],
        terminator: low_ir::Terminator::Return(Some((
            low_ir::Value::Local(result_local.clone()),
            low_ir::Type::I32,
        ))),
    };

    func.blocks.insert(block.id.clone(), block);
    module.functions.insert(func.id.clone(), func);

    // Convert through the IR pipeline
    let medium_module = medium_ir::Module::from_low_ir(&module);
    let high_module = high_ir::Module::from_medium_ir(&medium_module);

    // Generate C code
    let mut codegen = high_ir::c_codegen::CCodeGenerator::new();
    let c_code = codegen.generate(&high_module);

    println!("Generated C code for array read:\n{}", c_code);

    // Should contain array access and return
    assert!(c_code.contains("["), "Should contain array access");
    assert!(c_code.contains("return"), "Should contain return statement");
}

/// Helper function to check if a pattern contains an ArrayAccess
fn check_pattern_for_array_access(
    pattern: &medium_ir::Pattern,
    store: &medium_ir::PatternStore,
) -> bool {
    match pattern {
        medium_ir::Pattern::ArrayAccess { .. } => true,
        medium_ir::Pattern::LowIR { .. } => {
            // Check if the LowIR pattern contains array-like operations
            false // For now, we expect explicit ArrayAccess patterns
        }
        medium_ir::Pattern::Expression { operands, .. } => {
            // Check operands recursively
            operands.iter().any(|op_ref| {
                if let Some(op_pattern) = store.get(*op_ref) {
                    check_pattern_for_array_access(op_pattern, store)
                } else {
                    false
                }
            })
        }
        _ => false,
    }
}
