//! Common binary file operations
//!
//! This module provides common functionality for working with binary files,
//! shared across different file formats (PE, ELF, Mach-O).

/// Detect the architecture of a binary file
pub fn detect_architecture(data: &[u8]) -> Option<&'static str> {
    if data.len() < 4 {
        return None;
    }

    match &data[0..4] {
        // PE format: MZ header
        [0x4D, 0x5A, _, _] => {
            // TODO: Implement PE architecture detection
            Some("x86_64") // Default assumption
        }

        // ELF format: 0x7F ELF
        [0x7F, 0x45, 0x4C, 0x46] => {
            if data.len() < 5 {
                return None;
            }

            // Check ELF class (32/64-bit)
            let is_64bit = data[4] == 2;

            // Check machine type (at offset 18 for both 32/64-bit)
            if data.len() < 20 {
                return None;
            }

            let machine = u16::from_le_bytes([data[18], data[19]]);
            match machine {
                0x03 => Some(if is_64bit { "x86_64" } else { "x86" }),
                0x3E => Some("x86_64"),
                0x28 => Some("arm32"),
                0xB7 => Some("arm64"),
                _ => None,
            }
        }

        // Mach-O format
        [0xFE, 0xED, 0xFA, 0xCE] => Some("x86"), // 32-bit little-endian
        [0xFE, 0xED, 0xFA, 0xCF] => Some("x86_64"), // 64-bit little-endian
        [0xCE, 0xFA, 0xED, 0xFE] => Some("x86"), // 32-bit big-endian
        [0xCF, 0xFA, 0xED, 0xFE] => Some("x86_64"), // 64-bit big-endian

        _ => None,
    }
}

/// Check if a binary is 64-bit
pub fn is_64bit(data: &[u8]) -> Option<bool> {
    if data.len() < 4 {
        return None;
    }

    match &data[0..4] {
        // PE format: MZ header
        [0x4D, 0x5A, _, _] => {
            // TODO: Implement PE 64-bit detection
            Some(true) // Default assumption
        }

        // ELF format: 0x7F ELF
        [0x7F, 0x45, 0x4C, 0x46] => {
            if data.len() < 5 {
                return None;
            }

            // ELF class is at offset 4
            Some(data[4] == 2)
        }

        // Mach-O format
        [0xFE, 0xED, 0xFA, 0xCE] => Some(false), // 32-bit little-endian
        [0xFE, 0xED, 0xFA, 0xCF] => Some(true),  // 64-bit little-endian
        [0xCE, 0xFA, 0xED, 0xFE] => Some(false), // 32-bit big-endian
        [0xCF, 0xFA, 0xED, 0xFE] => Some(true),  // 64-bit big-endian

        _ => None,
    }
}

/// Detect the endianness of a binary file
pub fn detect_endianness(data: &[u8]) -> Option<Endianness> {
    if data.len() < 4 {
        return None;
    }

    match &data[0..4] {
        // PE format: MZ header (always little-endian)
        [0x4D, 0x5A, _, _] => Some(Endianness::Little),

        // ELF format: 0x7F ELF
        [0x7F, 0x45, 0x4C, 0x46] => {
            if data.len() < 6 {
                return None;
            }

            // ELF data encoding is at offset 5
            match data[5] {
                1 => Some(Endianness::Little),
                2 => Some(Endianness::Big),
                _ => None,
            }
        }

        // Mach-O format
        [0xFE, 0xED, 0xFA, 0xCE] | [0xFE, 0xED, 0xFA, 0xCF] => Some(Endianness::Little),
        [0xCE, 0xFA, 0xED, 0xFE] | [0xCF, 0xFA, 0xED, 0xFE] => Some(Endianness::Big),

        _ => None,
    }
}

/// Endianness of a binary file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    /// Little-endian (least significant byte first)
    Little,
    /// Big-endian (most significant byte first)
    Big,
}
