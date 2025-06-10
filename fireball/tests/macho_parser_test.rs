//! Test Mach-O parser functionality

use fireball::Fireball;
use fireball::binary::macho;

#[test]
fn test_macho_magic_detection() {
    // Minimal Mach-O header (64-bit, little-endian)
    let macho_data = vec![
        0xCF, 0xFA, 0xED, 0xFE, // Magic: MH_MAGIC_64 (little-endian)
        0x07, 0x00, 0x00, 0x01, // CPU type: x86_64
        0x03, 0x00, 0x00, 0x00, // CPU subtype
        0x02, 0x00, 0x00, 0x00, // File type: MH_EXECUTE
        0x00, 0x00, 0x00, 0x00, // Number of load commands
        0x00, 0x00, 0x00, 0x00, // Size of load commands
        0x00, 0x00, 0x00, 0x00, // Flags
        0x00, 0x00, 0x00, 0x00, // Reserved (64-bit)
    ];

    // Try to parse
    let result = Fireball::from_binary(macho_data);
    assert!(
        result.is_ok(),
        "Failed to parse minimal Mach-O: {:?}",
        result
    );

    match result.unwrap() {
        Fireball::MachO(_) => {} // Success
        _ => panic!("Expected Mach-O format"),
    }
}

#[test]
fn test_macho_32bit_detection() {
    // Minimal Mach-O header (32-bit, little-endian)
    let macho_data = vec![
        0xCE, 0xFA, 0xED, 0xFE, // Magic: MH_MAGIC (little-endian)
        0x07, 0x00, 0x00, 0x00, // CPU type: x86
        0x03, 0x00, 0x00, 0x00, // CPU subtype
        0x02, 0x00, 0x00, 0x00, // File type: MH_EXECUTE
        0x00, 0x00, 0x00, 0x00, // Number of load commands
        0x00, 0x00, 0x00, 0x00, // Size of load commands
        0x00, 0x00, 0x00, 0x00, // Flags
    ];

    // Try to parse
    let result = macho::MachO::from_bytes(macho_data);

    // Should fail because 32-bit parsing is not implemented yet
    assert!(result.is_err());
}

#[test]
fn test_macho_architecture_detection() {
    // Test data with different architectures
    let test_cases = vec![
        (0x00000007u32, macho::Architecture::X86),
        (0x01000007u32, macho::Architecture::X86_64),
        (0x0000000Cu32, macho::Architecture::Arm32),
        (0x0100000Cu32, macho::Architecture::Arm64),
    ];

    for (cpu_type, expected_arch) in test_cases {
        let mut macho_data = vec![
            0xCF, 0xFA, 0xED, 0xFE, // Magic: MH_MAGIC_64
        ];

        // CPU type
        macho_data.extend_from_slice(&cpu_type.to_le_bytes());

        // Rest of header
        macho_data.extend_from_slice(&[
            0x03, 0x00, 0x00, 0x00, // CPU subtype
            0x02, 0x00, 0x00, 0x00, // File type
            0x00, 0x00, 0x00, 0x00, // ncmds
            0x00, 0x00, 0x00, 0x00, // sizeofcmds
            0x00, 0x00, 0x00, 0x00, // flags
            0x00, 0x00, 0x00, 0x00, // reserved
        ]);

        let result = macho::MachO::from_bytes(macho_data);
        if let Ok(mach) = result {
            assert_eq!(
                mach.architecture(),
                expected_arch,
                "Wrong architecture for CPU type 0x{:08X}",
                cpu_type
            );
        }
    }
}

#[test]
fn test_macho_big_endian() {
    // Test big-endian Mach-O
    let macho_data = vec![
        0xFE, 0xED, 0xFA, 0xCF, // Magic: MH_CIGAM_64 (big-endian)
        0x01, 0x00, 0x00, 0x07, // CPU type: x86_64 (big-endian)
        0x00, 0x00, 0x00, 0x03, // CPU subtype
        0x00, 0x00, 0x00, 0x02, // File type: MH_EXECUTE
        0x00, 0x00, 0x00, 0x00, // ncmds
        0x00, 0x00, 0x00, 0x00, // sizeofcmds
        0x00, 0x00, 0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x00, // reserved
    ];

    let result = macho::MachO::from_bytes(macho_data);
    assert!(result.is_ok());

    let mach = result.unwrap();
    assert!(mach.is_64bit());
    assert_eq!(mach.architecture(), macho::Architecture::X86_64);
}
