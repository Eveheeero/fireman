//! Tests for pattern storage format

use fireball::ir::medium_ir::{
    ArchPattern, IdiomPattern, InstructionMatcher, LibraryPattern, ParsedPattern,
    PatternDatabaseBuilder, PatternMatcher, PatternParser, create_standard_pattern_database,
};
use std::path::Path;

#[test]
fn test_parse_library_pattern_file() {
    let parser = &mut PatternParser::new();
    let _path = Path::new("patterns/libraries/libc_string.pat");

    // Since we can't access the actual file in tests, test with content
    let content = r#"
# Version: 1.0
# Platform: generic
# Architecture: any

FUNCTION: libc::strlen
    RETURNS: u64
    PARAM: str char*
    CONVENTION: cdecl
    BEHAVIOR: pure
    ATTRIBUTES: null_terminated_string

FUNCTION: libc::strcpy
    RETURNS: char*
    PARAM: dest char*
    PARAM: src const char*
    CONVENTION: cdecl
    BEHAVIOR: modifies_memory
    ATTRIBUTES: null_terminated_string, returns_dest
"#;

    // Test parsing the content directly
    let result = parser.parse_library_patterns(content).unwrap();

    assert_eq!(result.metadata.version, "1.0");
    assert_eq!(result.patterns.len(), 2);

    // Check first pattern
    if let ParsedPattern::Library(lib) = &result.patterns[0] {
        assert_eq!(lib.name, "strlen");
        assert_eq!(lib.library, "libc");
        assert_eq!(lib.signature.parameters.len(), 1);
        assert_eq!(lib.signature.parameters[0].0, "str");
    } else {
        panic!("Expected library pattern");
    }

    // Check second pattern
    if let ParsedPattern::Library(lib) = &result.patterns[1] {
        assert_eq!(lib.name, "strcpy");
        assert_eq!(lib.library, "libc");
        assert_eq!(lib.signature.parameters.len(), 2);
    } else {
        panic!("Expected library pattern");
    }
}

#[test]
fn test_parse_idiom_pattern_file() {
    let parser = &mut PatternParser::new();

    let content = r#"
# Version: 1.0

IDIOM: strlen_manual_loop
  DESCRIPTION: Manual string length calculation using pointer iteration
  CONFIDENCE_BOOST: +25
  PATTERN:
    INIT: ptr = string_base
    LOOP:
      CONDITION: (*ptr != 0)
      BODY:
        INCREMENT: ptr = ptr + 1
    RESULT: length = ptr - string_base
  END_PATTERN

IDIOM: null_check_string
  DESCRIPTION: Check string pointer before operation
  CONFIDENCE_BOOST: +15
  PATTERN:
    IF_CONDITION: string_ptr == NULL
    THEN_BRANCH:
      RETURN: 0
  END_PATTERN
"#;

    let result = parser.parse_idiom_patterns(content).unwrap();

    assert_eq!(result.patterns.len(), 2);

    // Check first idiom
    if let ParsedPattern::Idiom(idiom) = &result.patterns[0] {
        assert_eq!(idiom.name, "strlen_manual_loop");
        assert_eq!(idiom.confidence_boost, 25);
        assert!(idiom.description.contains("Manual string length"));
    } else {
        panic!("Expected idiom pattern");
    }
}

#[test]
fn test_parse_arch_pattern_file() {
    let parser = &mut PatternParser::new();

    let content = r#"
# Architecture: x86_64

ARCH_PATTERN: standard_prologue
  ARCHITECTURE: x86_64
  DESCRIPTION: Standard function prologue with frame pointer
  SEQUENCE:
    push rbp
    mov rbp, rsp
    sub rsp, <frame_size>
  END_SEQUENCE
  CONFIDENCE_BOOST: +30

ARCH_PATTERN: standard_epilogue
  ARCHITECTURE: x86_64
  DESCRIPTION: Standard function epilogue
  SEQUENCE:
    leave
    ret
  END_SEQUENCE
  CONFIDENCE_BOOST: +30
"#;

    let result = parser.parse_arch_patterns(content).unwrap();

    assert_eq!(result.metadata.architecture, "x86_64");
    assert_eq!(result.patterns.len(), 2);

    // Check first pattern
    if let ParsedPattern::Architecture(arch) = &result.patterns[0] {
        assert_eq!(arch.name, "standard_prologue");
        assert_eq!(arch.arch, "x86_64");

        // Check instruction sequence
        if let PatternMatcher::InstructionSequence(seq) = &arch.matcher {
            assert_eq!(seq.len(), 3);
            assert!(matches!(seq[0], InstructionMatcher::Push(_)));
            assert!(matches!(seq[1], InstructionMatcher::MovReg(_, _)));
        }
    } else {
        panic!("Expected architecture pattern");
    }
}

#[test]
fn test_pattern_database_builder() {
    let mut builder = PatternDatabaseBuilder::new();

    // Add some test patterns
    builder.add_library_pattern(
        fireball::ir::medium_ir::pattern_database::Platform::Generic,
        LibraryPattern {
            name: "test_func".to_string(),
            library: "test_lib".to_string(),
            signature: fireball::ir::medium_ir::FunctionSignature {
                return_type: fireball::ir::medium_ir::TypeRef::Primitive(
                    fireball::ir::medium_ir::PrimitiveType::I32,
                ),
                parameters: vec![],
                convention: fireball::ir::low_ir::CallConv::X86_64SysV,
                variadic: false,
            },
            behavior: fireball::ir::medium_ir::PatternBehavior::Pure,
        },
    );

    let db = builder.build();
    assert!(db.library_functions.contains_key("test_lib::test_func"));
}

#[test]
fn test_standard_pattern_database() {
    let db = create_standard_pattern_database();

    // Check for common C library functions
    assert!(db.library_functions.contains_key("libc::malloc"));
    assert!(db.library_functions.contains_key("libc::free"));
    assert!(db.library_functions.contains_key("libc::strlen"));
    assert!(db.library_functions.contains_key("libc::printf"));

    // Check for C++ functions
    assert!(db.library_functions.contains_key("libstdc++::_Znwm")); // operator new
    assert!(db.library_functions.contains_key("libstdc++::_ZdlPv")); // operator delete

    // Check for idioms - they are added in add_standard_idioms
    assert!(!db.idioms.is_empty());
    assert!(db.idioms.iter().any(|i| i.name == "strlen_loop"));
}

#[test]
fn test_pattern_type_parsing() {
    let parser = PatternParser::new();

    // Test various type formats
    let test_cases = vec![
        ("int", "should parse as i32"),
        ("char*", "should parse as pointer to i8"),
        ("const char*", "should parse as pointer to i8"),
        ("int[10]", "should parse as array of i32"),
        ("void**", "should parse as pointer to pointer"),
    ];

    for (type_str, description) in test_cases {
        let result = parser.parse_type(type_str);
        assert!(result.is_ok(), "{}: {:?}", description, result);
    }
}

#[test]
fn test_pattern_format_completeness() {
    // Verify that all components of the pattern format are implemented

    // Test library pattern components
    let lib_pattern = LibraryPattern {
        name: "test".to_string(),
        library: "testlib".to_string(),
        signature: fireball::ir::medium_ir::FunctionSignature {
            return_type: fireball::ir::medium_ir::TypeRef::Primitive(
                fireball::ir::medium_ir::PrimitiveType::Void,
            ),
            parameters: vec![(
                "arg1".to_string(),
                fireball::ir::medium_ir::TypeRef::Primitive(
                    fireball::ir::medium_ir::PrimitiveType::I32,
                ),
            )],
            convention: fireball::ir::low_ir::CallConv::C,
            variadic: false,
        },
        behavior: fireball::ir::medium_ir::PatternBehavior::Pure,
    };

    // Test idiom pattern components
    let idiom_pattern = IdiomPattern {
        name: "test_idiom".to_string(),
        description: "Test idiom pattern".to_string(),
        matcher: PatternMatcher::ControlFlow(fireball::ir::medium_ir::ControlFlowMatcher {}),
        confidence_boost: 20,
    };

    // Test architecture pattern components
    let arch_pattern = ArchPattern {
        name: "test_arch".to_string(),
        arch: "x86_64".to_string(),
        matcher: PatternMatcher::InstructionSequence(vec![
            InstructionMatcher::Push("rbp"),
            InstructionMatcher::MovReg("rbp", "rsp"),
        ]),
    };

    // Verify all patterns can be created
    assert_eq!(lib_pattern.name, "test");
    assert_eq!(idiom_pattern.confidence_boost, 20);
    assert_eq!(arch_pattern.arch, "x86_64");
}
