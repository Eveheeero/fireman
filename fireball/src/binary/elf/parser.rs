//! ELF file parser implementation

use super::{Elf, header::*, section::SectionHeader, symbol};
use crate::utils::error::DecompileError;
use std::collections::BTreeMap;

/// Parse ELF binary data
pub fn parse_elf(data: Vec<u8>) -> Result<Elf, DecompileError> {
    if data.len() < 64 {
        return Err(DecompileError::HeaderParsingFailed);
    }

    // Verify magic number
    if &data[0..4] != b"\x7FELF" {
        return Err(DecompileError::HeaderParsingFailed);
    }

    // Parse identification bytes
    let class = match data[4] {
        1 => ElfClass::Elf32,
        2 => ElfClass::Elf64,
        _ => return Err(DecompileError::HeaderParsingFailed),
    };

    let endian = match data[5] {
        1 => ElfData::LittleEndian,
        2 => ElfData::BigEndian,
        _ => return Err(DecompileError::HeaderParsingFailed),
    };

    // Parse based on class
    match class {
        ElfClass::Elf32 => parse_elf32(data, endian),
        ElfClass::Elf64 => parse_elf64(data, endian),
    }
}

/// Parse 32-bit ELF
fn parse_elf32(_data: Vec<u8>, _endian: ElfData) -> Result<Elf, DecompileError> {
    // TODO: Implement 32-bit ELF parsing
    Err(DecompileError::Unknown(Some(
        "32-bit ELF parsing not implemented".to_string(),
    )))
}

/// Parse 64-bit ELF
fn parse_elf64(data: Vec<u8>, endian: ElfData) -> Result<Elf, DecompileError> {
    let header = parse_elf64_header(&data, endian)?;
    let program_headers = parse_program_headers(&data, &header, endian)?;
    let section_headers = parse_section_headers(&data, &header, endian)?;

    // Parse string table section
    let string_table_data =
        if header.shstrndx != 0 && header.shstrndx < section_headers.len() as u16 {
            let str_section = &section_headers[header.shstrndx as usize];
            let offset = str_section.offset as usize;
            let size = str_section.size as usize;

            if offset + size > data.len() {
                return Err(DecompileError::HeaderParsingFailed);
            }

            data[offset..offset + size].to_vec()
        } else {
            Vec::new()
        };

    // Create a BTreeMap for string tables with the string table at index 0
    let mut string_tables = BTreeMap::new();
    string_tables.insert(0, string_table_data);

    // Detect architecture from the binary
    let architecture = crate::arch::architecture::ArchitectureDetector::detect_from_bytes(&data);

    Ok(Elf {
        header,
        program_headers,
        section_headers,
        symbols: symbol::SymbolTables::default(),
        string_tables,
        data,
        architecture,
    })
}

/// Parse 64-bit ELF header
fn parse_elf64_header(data: &[u8], endian: ElfData) -> Result<ElfHeader, DecompileError> {
    if data.len() < 64 {
        return Err(DecompileError::HeaderParsingFailed);
    }

    let read_u16 = |offset: usize| -> u16 {
        match endian {
            ElfData::LittleEndian => u16::from_le_bytes([data[offset], data[offset + 1]]),
            ElfData::BigEndian => u16::from_be_bytes([data[offset], data[offset + 1]]),
        }
    };

    let read_u32 = |offset: usize| -> u32 {
        match endian {
            ElfData::LittleEndian => u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]),
            ElfData::BigEndian => u32::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]),
        }
    };

    let read_u64 = |offset: usize| -> u64 {
        match endian {
            ElfData::LittleEndian => u64::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]),
            ElfData::BigEndian => u64::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]),
        }
    };

    Ok(ElfHeader {
        class: ElfClass::Elf64,
        data: endian,
        version: data[6],
        osabi: data[7],
        abiversion: data[8],
        e_type: read_u16(16),
        machine: read_u16(18),
        entry: read_u64(24),
        phoff: read_u64(32),
        shoff: read_u64(40),
        flags: read_u32(48),
        ehsize: read_u16(52),
        phentsize: read_u16(54),
        phnum: read_u16(56),
        shentsize: read_u16(58),
        shnum: read_u16(60),
        shstrndx: read_u16(62),
    })
}

/// Parse program headers
fn parse_program_headers(
    data: &[u8],
    header: &ElfHeader,
    endian: ElfData,
) -> Result<Vec<ProgramHeader>, DecompileError> {
    let mut headers = Vec::new();
    let offset = header.phoff as usize;
    let size = header.phentsize as usize;
    let count = header.phnum as usize;

    if offset + size * count > data.len() {
        return Err(DecompileError::HeaderParsingFailed);
    }

    for i in 0..count {
        let ph_offset = offset + i * size;
        let ph = parse_program_header64(&data[ph_offset..ph_offset + size], endian)?;
        headers.push(ph);
    }

    Ok(headers)
}

/// Parse a single 64-bit program header
fn parse_program_header64(data: &[u8], endian: ElfData) -> Result<ProgramHeader, DecompileError> {
    if data.len() < 56 {
        return Err(DecompileError::HeaderParsingFailed);
    }

    let read_u32 = |offset: usize| -> u32 {
        match endian {
            ElfData::LittleEndian => u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]),
            ElfData::BigEndian => u32::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]),
        }
    };

    let read_u64 = |offset: usize| -> u64 {
        match endian {
            ElfData::LittleEndian => u64::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]),
            ElfData::BigEndian => u64::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]),
        }
    };

    Ok(ProgramHeader {
        p_type: read_u32(0),
        p_flags: read_u32(4),
        p_offset: read_u64(8),
        p_vaddr: read_u64(16),
        p_paddr: read_u64(24),
        p_filesz: read_u64(32),
        p_memsz: read_u64(40),
        p_align: read_u64(48),
    })
}

/// Parse section headers
fn parse_section_headers(
    data: &[u8],
    header: &ElfHeader,
    endian: ElfData,
) -> Result<Vec<SectionHeader>, DecompileError> {
    let mut headers = Vec::new();
    let offset = header.shoff as usize;
    let size = header.shentsize as usize;
    let count = header.shnum as usize;

    if offset + size * count > data.len() {
        return Err(DecompileError::HeaderParsingFailed);
    }

    for i in 0..count {
        let sh_offset = offset + i * size;
        let sh = parse_section_header64(&data[sh_offset..sh_offset + size], endian)?;
        headers.push(sh);
    }

    Ok(headers)
}

/// Parse a single 64-bit section header
fn parse_section_header64(data: &[u8], endian: ElfData) -> Result<SectionHeader, DecompileError> {
    if data.len() < 64 {
        return Err(DecompileError::HeaderParsingFailed);
    }

    let read_u32 = |offset: usize| -> u32 {
        match endian {
            ElfData::LittleEndian => u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]),
            ElfData::BigEndian => u32::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]),
        }
    };

    let read_u64 = |offset: usize| -> u64 {
        match endian {
            ElfData::LittleEndian => u64::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]),
            ElfData::BigEndian => u64::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]),
        }
    };

    Ok(SectionHeader {
        name: read_u32(0),
        sh_type: read_u32(4),
        flags: read_u64(8),
        addr: read_u64(16),
        offset: read_u64(24),
        size: read_u64(32),
        link: read_u32(40),
        info: read_u32(44),
        addralign: read_u64(48),
        entsize: read_u64(56),
    })
}

/// Parse string table from section
fn parse_string_table(
    data: &[u8],
    section: &SectionHeader,
) -> Result<BTreeMap<u32, String>, DecompileError> {
    let mut strings = BTreeMap::new();
    let offset = section.offset as usize;
    let size = section.size as usize;

    if offset + size > data.len() {
        return Err(DecompileError::HeaderParsingFailed);
    }

    let str_data = &data[offset..offset + size];
    let mut current_offset = 0;

    while current_offset < str_data.len() {
        let start = current_offset;

        // Find null terminator
        while current_offset < str_data.len() && str_data[current_offset] != 0 {
            current_offset += 1;
        }

        if current_offset < str_data.len() {
            let string = String::from_utf8_lossy(&str_data[start..current_offset]).to_string();
            strings.insert(start as u32, string);
            current_offset += 1; // Skip null byte
        }
    }

    Ok(strings)
}
