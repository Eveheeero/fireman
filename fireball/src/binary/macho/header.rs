//! Mach-O header definitions
//!
//! This module defines the structures and constants for Mach-O headers.

/// Magic number for 32-bit Mach-O files (little endian)
pub const MH_MAGIC: u32 = 0xfeedface;

/// Magic number for 32-bit Mach-O files (big endian)
pub const MH_CIGAM: u32 = 0xcefaedfe;

/// Magic number for 64-bit Mach-O files (little endian)
pub const MH_MAGIC_64: u32 = 0xfeedfacf;

/// Magic number for 64-bit Mach-O files (big endian)
pub const MH_CIGAM_64: u32 = 0xcffaedfe;

/// Mach-O header structure
#[derive(Debug, Clone)]
pub struct MachHeader {
    /// Magic number
    pub magic: u32,

    /// CPU type
    pub cpu_type: u32,

    /// CPU subtype
    pub cpu_subtype: u32,

    /// File type
    pub filetype: u32,

    /// Number of load commands
    pub ncmds: u32,

    /// Size of load commands
    pub sizeofcmds: u32,

    /// Flags
    pub flags: u32,

    /// Reserved field (64-bit only)
    pub reserved: Option<u32>,
}

/// File types
pub const MH_OBJECT: u32 = 0x1; // Object file
pub const MH_EXECUTE: u32 = 0x2; // Executable
pub const MH_DYLIB: u32 = 0x6; // Dynamic library
pub const MH_BUNDLE: u32 = 0x8; // Bundle
pub const MH_DYLINKER: u32 = 0x7; // Dynamic linker
pub const MH_DSYM: u32 = 0xa; // Debug symbols
