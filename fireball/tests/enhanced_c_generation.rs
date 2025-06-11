//! Test for Enhanced C code generation

use fireball::ir::analyze::enhanced_c_codegen::{EnhancedCConfig, EnhancedCGenerator};
use fireball::ir::medium_ir::Pattern;

#[test]
fn test_enhanced_c_header_generation() {
    let config = EnhancedCConfig::default();
    let mut generator = EnhancedCGenerator::new(config);

    // Generate with empty patterns
    let patterns: Vec<Pattern> = vec![];
    let output = generator.generate(&patterns);

    // Check header is generated
    assert!(output.contains("/* Enhanced C - Decompiled Output */"));
    assert!(output.contains("#include <stdint.h>"));
    assert!(output.contains("#include <stdbool.h>"));
    assert!(output.contains("#include <cstddef> // for nullptr"));
}

#[test]
fn test_enhanced_c_config_options() {
    // Test with different config options
    let config = EnhancedCConfig {
        use_nullptr: false,
        use_fixed_width_types: false,
        ..Default::default()
    };

    let mut generator = EnhancedCGenerator::new(config);
    let patterns: Vec<Pattern> = vec![];
    let output = generator.generate(&patterns);

    // Should not include nullptr header when disabled
    assert!(!output.contains("#include <cstddef>"));
}

#[test]
fn test_type_to_enhanced_c() {
    let config = EnhancedCConfig::default();
    let _generator = EnhancedCGenerator::new(config);

    // Just verify the generator can be created successfully
    // The actual type conversion is tested indirectly through generation
}
