//! Mach-O segment and section definitions
//!
//! This module defines the structures for Mach-O segments and sections.

/// Mach-O segment structure
#[derive(Debug, Clone)]
pub struct Segment {
    /// Segment name
    pub name: String,

    /// Virtual memory address
    pub vmaddr: u64,

    /// Virtual memory size
    pub vmsize: u64,

    /// File offset
    pub fileoff: u64,

    /// File size
    pub filesize: u64,

    /// Maximum VM protection
    pub maxprot: u32,

    /// Initial VM protection
    pub initprot: u32,

    /// Number of sections
    pub nsects: u32,

    /// Flags
    pub flags: u32,

    /// Sections in this segment
    pub sections: Vec<Section>,
}

/// Mach-O section structure
#[derive(Debug, Clone)]
pub struct Section {
    /// Section name
    pub name: String,

    /// Segment name
    pub segname: String,

    /// Virtual memory address
    pub addr: u64,

    /// Size in bytes
    pub size: u64,

    /// File offset
    pub offset: u32,

    /// Alignment
    pub align: u32,

    /// File offset of relocation entries
    pub reloff: u32,

    /// Number of relocation entries
    pub nreloc: u32,

    /// Flags
    pub flags: u32,
}

// Section flags
pub const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x00000400;
pub const S_ATTR_PURE_INSTRUCTIONS: u32 = 0x80000000;
pub const S_ATTR_ZERO_FILL: u32 = 0x00000001;
