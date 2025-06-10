//! Test architecture detection functionality

use fireball::arch::{ArchType, ArchitectureDetector, Endianness};

#[test]
fn test_pe_architecture_detection() {
    // Minimal PE header for x86_64
    let mut pe_data = vec![
        0x4D, 0x5A, // MZ magic
    ];

    // Extend with DOS stub and PE signature location
    pe_data.resize(0x3C + 4, 0); // DOS header
    pe_data[0x3C..0x40].copy_from_slice(&[0x80, 0x00, 0x00, 0x00]); // PE offset at 0x80

    // Add PE signature and headers at offset 0x80
    pe_data.resize(0x80 + 24, 0);
    pe_data[0x80..0x84].copy_from_slice(b"PE\0\0"); // PE signature

    // COFF header
    pe_data[0x84..0x86].copy_from_slice(&[0x64, 0x86]); // Machine: x86_64

    // Optional header magic (determines 32/64 bit)
    pe_data.resize(0x80 + 24 + 64, 0);
    pe_data[0x98..0x9A].copy_from_slice(&[0x0B, 0x02]); // PE32+ (64-bit)

    let arch_info = ArchitectureDetector::detect_from_bytes(&pe_data);
    assert_eq!(arch_info.arch_type, ArchType::X86_64);
    assert_eq!(arch_info.pointer_size, 64);
    assert_eq!(arch_info.endianness, Endianness::Little);
}

#[test]
fn test_elf_x86_64_detection() {
    let elf_data = vec![
        0x7F, 0x45, 0x4C, 0x46, // Magic
        0x02, // 64-bit
        0x01, // Little-endian
        0x01, // Version
        0x00, // System V ABI
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
        0x02, 0x00, // Type: Executable
        0x3E, 0x00, // Machine: x86_64
    ];

    let arch_info = ArchitectureDetector::detect_from_bytes(&elf_data);
    assert_eq!(arch_info.arch_type, ArchType::X86_64);
    assert_eq!(arch_info.pointer_size, 64);
    assert_eq!(arch_info.endianness, Endianness::Little);
    assert_eq!(arch_info.register_count, 16);
}

#[test]
fn test_elf_arm64_detection() {
    let elf_data = vec![
        0x7F, 0x45, 0x4C, 0x46, // Magic
        0x02, // 64-bit
        0x01, // Little-endian
        0x01, // Version
        0x00, // System V ABI
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
        0x02, 0x00, // Type: Executable
        0xB7, 0x00, // Machine: ARM64
    ];

    let arch_info = ArchitectureDetector::detect_from_bytes(&elf_data);
    assert_eq!(arch_info.arch_type, ArchType::Arm64);
    assert_eq!(arch_info.pointer_size, 64);
    assert_eq!(arch_info.endianness, Endianness::Little);
    assert_eq!(arch_info.register_count, 31); // X0-X30
}

#[test]
fn test_elf_arm32_big_endian_detection() {
    let elf_data = vec![
        0x7F, 0x45, 0x4C, 0x46, // Magic
        0x01, // 32-bit
        0x02, // Big-endian
        0x01, // Version
        0x00, // System V ABI
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
        0x00, 0x02, // Type: Executable (big-endian)
        0x00, 0x28, // Machine: ARM (big-endian)
    ];

    let arch_info = ArchitectureDetector::detect_from_bytes(&elf_data);
    assert_eq!(arch_info.arch_type, ArchType::Arm32);
    assert_eq!(arch_info.pointer_size, 32);
    assert_eq!(arch_info.endianness, Endianness::Big);
    assert_eq!(arch_info.register_count, 16);
}

#[test]
fn test_macho_x86_64_detection() {
    let macho_data = vec![
        0xCF, 0xFA, 0xED, 0xFE, // Magic: MH_MAGIC_64
        0x07, 0x00, 0x00, 0x01, // CPU type: x86_64
        0x03, 0x00, 0x00, 0x00, // CPU subtype
    ];

    let arch_info = ArchitectureDetector::detect_from_bytes(&macho_data);
    assert_eq!(arch_info.arch_type, ArchType::X86_64);
    assert_eq!(arch_info.pointer_size, 64);
    assert_eq!(arch_info.endianness, Endianness::Little);
}

#[test]
fn test_macho_arm64_big_endian_detection() {
    let macho_data = vec![
        0xFE, 0xED, 0xFA, 0xCF, // Magic: MH_CIGAM_64 (big-endian)
        0x01, 0x00, 0x00, 0x0C, // CPU type: ARM64 (big-endian)
        0x00, 0x00, 0x00, 0x00, // CPU subtype
    ];

    let arch_info = ArchitectureDetector::detect_from_bytes(&macho_data);
    assert_eq!(arch_info.arch_type, ArchType::Arm64);
    assert_eq!(arch_info.pointer_size, 64);
    assert_eq!(arch_info.endianness, Endianness::Big);
}

#[test]
fn test_unknown_format_detection() {
    let unknown_data = vec![0xFF, 0xFF, 0xFF, 0xFF];

    let arch_info = ArchitectureDetector::detect_from_bytes(&unknown_data);
    assert_eq!(arch_info.arch_type, ArchType::Unknown);
}

#[test]
fn test_instruction_alignment() {
    assert_eq!(ArchType::X86.instruction_alignment(), 1);
    assert_eq!(ArchType::X86_64.instruction_alignment(), 1);
    assert_eq!(ArchType::Arm32.instruction_alignment(), 4);
    assert_eq!(ArchType::Arm64.instruction_alignment(), 4);
}
