//! ELF section structures

/// Section header
#[derive(Debug)]
pub struct SectionHeader {
    /// Section name (index into string table)
    pub name: u32,
    /// Section type
    pub sh_type: u32,
    /// Section flags
    pub flags: u64,
    /// Virtual address
    pub addr: u64,
    /// Offset in file
    pub offset: u64,
    /// Section size
    pub size: u64,
    /// Link to another section
    pub link: u32,
    /// Additional information
    pub info: u32,
    /// Alignment
    pub addralign: u64,
    /// Entry size if fixed
    pub entsize: u64,
}

// Section types
pub const SHT_NULL: u32 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHT_SYMTAB: u32 = 2;
pub const SHT_STRTAB: u32 = 3;
pub const SHT_RELA: u32 = 4;
pub const SHT_HASH: u32 = 5;
pub const SHT_DYNAMIC: u32 = 6;
pub const SHT_NOTE: u32 = 7;
pub const SHT_NOBITS: u32 = 8;
pub const SHT_REL: u32 = 9;
pub const SHT_SHLIB: u32 = 10;
pub const SHT_DYNSYM: u32 = 11;

// Section flags
pub const SHF_WRITE: u64 = 0x1;
pub const SHF_ALLOC: u64 = 0x2;
pub const SHF_EXECINSTR: u64 = 0x4;
