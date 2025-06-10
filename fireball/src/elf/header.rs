//! ELF header structures and constants

/// ELF class (32-bit or 64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfClass {
    Elf32 = 1,
    Elf64 = 2,
}

/// ELF data encoding (endianness)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfData {
    LittleEndian = 1,
    BigEndian = 2,
}

/// ELF header
#[derive(Debug)]
pub struct ElfHeader {
    /// File class (32-bit or 64-bit)
    pub class: ElfClass,
    /// Data encoding
    pub data: ElfData,
    /// File version
    pub version: u8,
    /// OS/ABI
    pub osabi: u8,
    /// ABI version
    pub abiversion: u8,
    /// Object file type
    pub e_type: u16,
    /// Machine type
    pub machine: u16,
    /// Entry point
    pub entry: u64,
    /// Program header offset
    pub phoff: u64,
    /// Section header offset
    pub shoff: u64,
    /// Flags
    pub flags: u32,
    /// ELF header size
    pub ehsize: u16,
    /// Program header entry size
    pub phentsize: u16,
    /// Program header count
    pub phnum: u16,
    /// Section header entry size
    pub shentsize: u16,
    /// Section header count
    pub shnum: u16,
    /// Section header string table index
    pub shstrndx: u16,
}

/// Program header (segment)
#[derive(Debug)]
pub struct ProgramHeader {
    /// Segment type
    pub p_type: u32,
    /// Segment flags
    pub p_flags: u32,
    /// Offset in file
    pub p_offset: u64,
    /// Virtual address
    pub p_vaddr: u64,
    /// Physical address
    pub p_paddr: u64,
    /// File size
    pub p_filesz: u64,
    /// Memory size
    pub p_memsz: u64,
    /// Alignment
    pub p_align: u64,
}

// Program header types
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;

// Program header flags
pub const PF_X: u32 = 0x1; // Execute
pub const PF_W: u32 = 0x2; // Write
pub const PF_R: u32 = 0x4; // Read
