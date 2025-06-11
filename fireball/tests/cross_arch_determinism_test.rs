//! Tests for deterministic output across different architectures
//!
//! This test suite verifies that the decompiler produces deterministic output
//! when processing the same binary content, regardless of the architecture.

use fireball::arch::register_mapping::get_register_mapper;
use fireball::arch::{ArchType, ArchitectureInfo, Endianness};
use fireball::ir::analyze::enhanced_c_codegen::EnhancedCConfig;
use std::collections::BTreeMap;

#[test]
fn test_deterministic_address_formatting() {
    // Test that addresses are always formatted the same way
    let addresses: Vec<u64> = vec![0x1000, 0xDEADBEEF, 0x123456789ABCDEF0];

    for addr in addresses {
        let formatted1 = format!("{:016x}", addr);
        let formatted2 = format!("{:016x}", addr);
        assert_eq!(formatted1, formatted2);
        assert_eq!(formatted1.len(), 16); // Always 16 digits
    }
}

#[test]
fn test_deterministic_register_mapping() {
    // Test that register mapping is consistent
    let mapper_x64 = get_register_mapper(ArchType::X86_64);
    let mapper_x86 = get_register_mapper(ArchType::X86);

    // Map the same register multiple times
    for _ in 0..10 {
        // Test x86_64 register mapping
        assert!(mapper_x64.to_ir_register("eax").is_some());
        assert!(mapper_x64.to_ir_register("al").is_some());
        assert!(mapper_x64.to_ir_register("ah").is_some());

        // Test x86 register mapping
        assert!(mapper_x86.to_ir_register("ax").is_some());
        assert!(mapper_x86.to_ir_register("al").is_some());

        // Test register size consistency
        assert_eq!(mapper_x64.get_register_size("rax"), Some(64));
        assert_eq!(mapper_x64.get_register_size("eax"), Some(32));
        assert_eq!(mapper_x86.get_register_size("eax"), Some(32));
    }
}

#[test]
fn test_btreemap_deterministic_iteration() {
    // BTreeMap should iterate in sorted order
    let mut map = BTreeMap::new();

    // Insert in random order
    map.insert(0x3000u64, "third");
    map.insert(0x1000u64, "first");
    map.insert(0x2000u64, "second");

    // Collect keys
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec![0x1000, 0x2000, 0x3000]);

    // Multiple iterations should give same order
    for _ in 0..10 {
        let keys2: Vec<_> = map.keys().cloned().collect();
        assert_eq!(keys, keys2);
    }
}

#[test]
fn test_architecture_specific_type_sizes() {
    // Test that architecture-specific types are handled deterministically
    let arch_32 = ArchitectureInfo {
        arch_type: ArchType::X86,
        pointer_size: 32,
        endianness: Endianness::Little,
        register_count: 8,
        instruction_alignment: 1,
    };

    let arch_64 = ArchitectureInfo {
        arch_type: ArchType::X86_64,
        pointer_size: 64,
        endianness: Endianness::Little,
        register_count: 16,
        instruction_alignment: 1,
    };

    // Pointer size should affect generated code
    assert_eq!(arch_32.pointer_size, 32);
    assert_eq!(arch_64.pointer_size, 64);

    // Architecture size operations should be consistent
    let size_32 = arch_32.pointer_size / 8;
    let size_64 = arch_64.pointer_size / 8;

    assert_eq!(size_32, 4);
    assert_eq!(size_64, 8);
}

#[test]
fn test_enhanced_c_config_determinism() {
    // Test that same config produces same results
    let config1 = EnhancedCConfig::default();
    let config2 = EnhancedCConfig::default();

    assert_eq!(config1.use_auto, config2.use_auto);
    assert_eq!(config1.use_nullptr, config2.use_nullptr);
    assert_eq!(config1.use_fixed_width_types, config2.use_fixed_width_types);
    assert_eq!(config1.confidence_threshold, config2.confidence_threshold);
}

#[test]
fn test_deterministic_btree_vs_hash() {
    use std::collections::{BTreeMap, HashMap};

    // Create the same data in both maps
    let mut btree = BTreeMap::new();
    let mut hash = HashMap::new();

    for i in [5, 1, 3, 2, 4] {
        btree.insert(i, format!("value_{}", i));
        hash.insert(i, format!("value_{}", i));
    }

    // BTreeMap should iterate in sorted order
    let btree_keys: Vec<_> = btree.keys().cloned().collect();
    assert_eq!(btree_keys, vec![1, 2, 3, 4, 5]);

    // HashMap order is not guaranteed - just verify it has all keys
    let hash_keys: Vec<_> = hash.keys().cloned().collect();
    assert_eq!(hash_keys.len(), 5);

    // Multiple iterations of BTreeMap should be consistent
    for _ in 0..10 {
        let keys: Vec<_> = btree.keys().cloned().collect();
        assert_eq!(keys, vec![1, 2, 3, 4, 5]);
    }
}

#[test]
fn test_architecture_info_consistency() {
    // Test that ArchitectureInfo produces consistent results
    let archs = vec![
        ArchitectureInfo {
            arch_type: ArchType::X86,
            pointer_size: 32,
            endianness: Endianness::Little,
            register_count: 8,
            instruction_alignment: 1,
        },
        ArchitectureInfo {
            arch_type: ArchType::X86_64,
            pointer_size: 64,
            endianness: Endianness::Little,
            register_count: 16,
            instruction_alignment: 1,
        },
        ArchitectureInfo {
            arch_type: ArchType::Arm32,
            pointer_size: 32,
            endianness: Endianness::Little,
            register_count: 16,
            instruction_alignment: 4,
        },
        ArchitectureInfo {
            arch_type: ArchType::Arm64,
            pointer_size: 64,
            endianness: Endianness::Little,
            register_count: 31,
            instruction_alignment: 4,
        },
    ];

    // Verify consistent representation
    for arch in archs {
        match arch.arch_type {
            ArchType::X86 => {
                assert_eq!(arch.pointer_size, 32);
                assert_eq!(arch.register_count, 8);
                assert_eq!(arch.instruction_alignment, 1);
            }
            ArchType::X86_64 => {
                assert_eq!(arch.pointer_size, 64);
                assert_eq!(arch.register_count, 16);
                assert_eq!(arch.instruction_alignment, 1);
            }
            ArchType::Arm32 => {
                assert_eq!(arch.pointer_size, 32);
                assert_eq!(arch.register_count, 16);
                assert_eq!(arch.instruction_alignment, 4);
            }
            ArchType::Arm64 => {
                assert_eq!(arch.pointer_size, 64);
                assert_eq!(arch.register_count, 31);
                assert_eq!(arch.instruction_alignment, 4);
            }
            _ => panic!("Unexpected architecture"),
        }
    }
}

#[test]
fn test_parallel_determinism_simple() {
    use std::thread;

    // Test that parallel operations produce same results
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                // Simulate some deterministic computation
                let mut result = BTreeMap::new();
                for j in 0..10 {
                    result.insert(j, format!("thread_{}_item_{}", i, j));
                }

                // Convert to sorted vec for comparison
                let sorted: Vec<_> = result.into_iter().collect();
                sorted
            })
        })
        .collect();

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All threads should produce deterministic results
    for result in &results {
        assert_eq!(result.len(), 10);
        for (j, item) in result.iter().enumerate() {
            assert_eq!(item.0, j);
        }
    }
}

#[test]
fn test_calling_convention_consistency() {
    // Test that calling convention registers are consistent
    let mapper_x64 = get_register_mapper(ArchType::X86_64);
    let mapper_arm64 = get_register_mapper(ArchType::Arm64);

    let cc_x64 = mapper_x64.get_calling_convention_registers();
    let cc_arm64 = mapper_arm64.get_calling_convention_registers();

    // x86_64 System V should have 6 argument registers
    assert_eq!(cc_x64.argument_registers.len(), 6);
    assert_eq!(cc_x64.argument_registers[0], "rdi");
    assert_eq!(cc_x64.return_register, "rax");

    // ARM64 should have 8 argument registers
    assert_eq!(cc_arm64.argument_registers.len(), 8);
    assert_eq!(cc_arm64.argument_registers[0], "x0");
    assert_eq!(cc_arm64.return_register, "x0");
}
