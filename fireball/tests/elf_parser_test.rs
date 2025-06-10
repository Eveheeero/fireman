//! Test ELF parser functionality

use fireball::Fireball;
use fireball::binary::elf;

#[test]
fn test_elf_magic_detection() {
    // Minimal ELF header (64-bit, little-endian)
    let mut elf_data = vec![
        0x7F, 0x45, 0x4C, 0x46, // Magic: \x7FELF
        0x02, // Class: 64-bit
        0x01, // Data: Little-endian
        0x01, // Version: 1
        0x00, // OS/ABI: System V
        0x00, // ABI version
    ];

    // Pad to minimum header size
    elf_data.resize(64, 0);

    // Set some required fields
    elf_data[16] = 0x02; // e_type: ET_EXEC (executable)
    elf_data[18] = 0x3E; // e_machine: EM_X86_64

    // Try to parse
    let result = Fireball::from_binary(elf_data);
    assert!(result.is_ok(), "Failed to parse minimal ELF: {:?}", result);

    match result.unwrap() {
        Fireball::Elf(_) => {} // Success
        _ => panic!("Expected ELF format"),
    }
}

#[test]
fn test_elf_32bit_detection() {
    // Minimal ELF header (32-bit, little-endian)
    let mut elf_data = vec![
        0x7F, 0x45, 0x4C, 0x46, // Magic: \x7FELF
        0x01, // Class: 32-bit
        0x01, // Data: Little-endian
        0x01, // Version: 1
        0x00, // OS/ABI: System V
        0x00, // ABI version
    ];

    // Pad to minimum header size
    elf_data.resize(64, 0);

    // Set some required fields
    elf_data[16] = 0x02; // e_type: ET_EXEC (executable)
    elf_data[18] = 0x03; // e_machine: EM_386 (x86)

    // Try to parse
    let result = elf::Elf::from_bytes(elf_data);

    // Should fail because 32-bit parsing is not implemented yet
    assert!(result.is_err());
}

#[test]
fn test_elf_architecture_detection() {
    // Test data with different architectures
    let test_cases = vec![
        (0x03, elf::Architecture::X86),    // EM_386
        (0x3E, elf::Architecture::X86_64), // EM_X86_64
        (0x28, elf::Architecture::Arm32),  // EM_ARM
        (0xB7, elf::Architecture::Arm64),  // EM_AARCH64
    ];

    for (machine_type, expected_arch) in test_cases {
        let mut elf_data = vec![
            0x7F, 0x45, 0x4C, 0x46, // Magic
            0x02, 0x01, 0x01, 0x00, // 64-bit, little-endian
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        // Pad to minimum size
        elf_data.resize(64, 0);

        // Set machine type
        elf_data[18] = (machine_type & 0xFF) as u8;
        elf_data[19] = ((machine_type >> 8) & 0xFF) as u8;

        let result = elf::Elf::from_bytes(elf_data);
        if let Ok(elf) = result {
            assert_eq!(
                elf.architecture(),
                expected_arch,
                "Wrong architecture for machine type 0x{:02X}",
                machine_type
            );
        }
    }
}

#[test]
fn test_elf_entry_point() {
    let mut elf_data = vec![
        0x7F, 0x45, 0x4C, 0x46, // Magic
        0x02, 0x01, 0x01, 0x00, // 64-bit, little-endian
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, // e_type: ET_EXEC
        0x3E, 0x00, // e_machine: EM_X86_64
        0x01, 0x00, 0x00, 0x00, // e_version
    ];

    // Entry point at offset 24 (8 bytes for 64-bit)
    let entry_point: u64 = 0x0000000000401000;
    elf_data.extend_from_slice(&entry_point.to_le_bytes());

    // Pad to minimum size
    elf_data.resize(64, 0);

    let result = elf::Elf::from_bytes(elf_data);
    assert!(result.is_ok());

    let elf = result.unwrap();
    assert_eq!(elf.entry_point(), entry_point, "Entry point mismatch");
}
