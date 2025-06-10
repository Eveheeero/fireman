use fireball::core::{Address, Sections};
use fireball::ir::{
    Ir,
    analyze::{
        DataType,
        type_recovery::{InferredType, TypeRecoveryEngine},
    },
    data::{AccessSize, IrData},
    statements::{IrStatement, IrStatementSpecial},
};
use fireball::utils::Aos;
use std::sync::Arc;

fn make_test_address(addr: u64) -> Address {
    let sections = Arc::new(Sections::default());
    Address::from_virtual_address(&sections, addr)
}

#[test]
fn test_type_recovery_basic() {
    let mut engine = TypeRecoveryEngine::new();

    // Create a simple IR with type information
    static STATEMENTS: &[IrStatement] = &[
        // Explicit type declaration
        IrStatement::Special(IrStatementSpecial::TypeSpecified {
            location: Aos::StaticRef(&IrData::Constant(0x1000)),
            size: AccessSize::ResultOfByte(Aos::StaticRef(&IrData::Constant(4))),
            data_type: DataType::Int,
        }),
        // Assignment that should propagate type
        IrStatement::Assignment {
            from: Aos::StaticRef(&IrData::Constant(0x1000)),
            to: Aos::StaticRef(&IrData::Constant(0x2000)),
            size: AccessSize::ResultOfByte(Aos::StaticRef(&IrData::Constant(4))),
        },
    ];

    let ir = Ir {
        address: make_test_address(0x1000),
        statements: Some(STATEMENTS),
    };

    let types = engine.recover_types(&ir);

    // Check that type was inferred
    assert!(types.contains_key("const_1000"));
    assert!(types.contains_key("const_2000"));

    let type1000 = &types["const_1000"];
    assert!(matches!(type1000.ty, InferredType::Integer { .. }));
    assert_eq!(type1000.confidence, 1.0);

    let type2000 = &types["const_2000"];
    assert!(matches!(type2000.ty, InferredType::Integer { .. }));
    assert!(type2000.confidence > 0.8);
}

#[test]
fn test_function_call_type_inference() {
    let mut engine = TypeRecoveryEngine::new();

    static STATEMENTS: &[IrStatement] = &[
        // Function call
        IrStatement::JumpByCall {
            target: Aos::StaticRef(&IrData::Constant(0x5000)),
        },
    ];

    let ir = Ir {
        address: make_test_address(0x1000),
        statements: Some(STATEMENTS),
    };

    let types = engine.recover_types(&ir);

    // Check that the target was recognized as a function
    assert!(types.contains_key("const_5000"));
    let func_type = &types["const_5000"];
    assert!(matches!(func_type.ty, InferredType::Function { .. }));
}

#[test]
fn test_pointer_type_inference() {
    let mut engine = TypeRecoveryEngine::new();

    static STATEMENTS: &[IrStatement] = &[
        // Jump to an address (should infer pointer type)
        IrStatement::Jump {
            target: Aos::StaticRef(&IrData::Constant(0x4000)),
        },
    ];

    let ir = Ir {
        address: make_test_address(0x1000),
        statements: Some(STATEMENTS),
    };

    let types = engine.recover_types(&ir);

    assert!(types.contains_key("const_4000"));
    let type4000 = &types["const_4000"];
    assert!(matches!(type4000.ty, InferredType::Pointer { .. }));
}
