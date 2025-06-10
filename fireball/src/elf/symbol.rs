//! ELF symbol table parsing and management

use super::section::*;
use crate::utils::error::FireballError;
use std::collections::BTreeMap;

/// Symbol table entry
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol name (index into string table)
    pub name: u32,
    /// Symbol type and binding
    pub info: u8,
    /// Symbol visibility
    pub other: u8,
    /// Section index
    pub shndx: u16,
    /// Symbol value
    pub value: u64,
    /// Symbol size
    pub size: u64,
}

impl Symbol {
    /// Get symbol type
    pub fn get_type(&self) -> SymbolType {
        match self.info & 0xf {
            0 => SymbolType::NoType,
            1 => SymbolType::Object,
            2 => SymbolType::Func,
            3 => SymbolType::Section,
            4 => SymbolType::File,
            5 => SymbolType::Common,
            6 => SymbolType::TLS,
            _ => SymbolType::Unknown,
        }
    }

    /// Get symbol binding
    pub fn get_binding(&self) -> SymbolBinding {
        match self.info >> 4 {
            0 => SymbolBinding::Local,
            1 => SymbolBinding::Global,
            2 => SymbolBinding::Weak,
            _ => SymbolBinding::Unknown,
        }
    }

    /// Get symbol visibility
    pub fn get_visibility(&self) -> SymbolVisibility {
        match self.other & 0x3 {
            0 => SymbolVisibility::Default,
            1 => SymbolVisibility::Internal,
            2 => SymbolVisibility::Hidden,
            3 => SymbolVisibility::Protected,
            _ => SymbolVisibility::Default,
        }
    }
}

/// Symbol type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    NoType,
    Object,
    Func,
    Section,
    File,
    Common,
    TLS,
    Unknown,
}

/// Symbol binding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolBinding {
    Local,
    Global,
    Weak,
    Unknown,
}

/// Symbol visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolVisibility {
    Default,
    Internal,
    Hidden,
    Protected,
}

/// Parse symbol table from section data
pub fn parse_symbol_table(
    data: &[u8],
    section: &SectionHeader,
    is_64bit: bool,
) -> Result<Vec<Symbol>, FireballError> {
    let mut symbols = Vec::new();
    let entry_size = if is_64bit { 24 } else { 16 };

    if section.entsize != entry_size {
        return Err(FireballError::InvalidBinary(format!(
            "Invalid symbol entry size: expected {}, got {}",
            entry_size, section.entsize
        )));
    }

    let offset = section.offset as usize;
    let size = section.size as usize;
    let count = size / entry_size as usize;

    if offset + size > data.len() {
        return Err(FireballError::InvalidBinary(
            "Symbol table beyond file".to_string(),
        ));
    }

    let sym_data = &data[offset..offset + size];

    for i in 0..count {
        let entry_offset = i * entry_size as usize;
        let symbol = if is_64bit {
            parse_symbol64(&sym_data[entry_offset..entry_offset + 24])?
        } else {
            parse_symbol32(&sym_data[entry_offset..entry_offset + 16])?
        };
        symbols.push(symbol);
    }

    Ok(symbols)
}

/// Parse 64-bit symbol entry
fn parse_symbol64(data: &[u8]) -> Result<Symbol, FireballError> {
    if data.len() < 24 {
        return Err(FireballError::InvalidBinary(
            "Symbol entry too small".to_string(),
        ));
    }

    Ok(Symbol {
        name: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
        info: data[4],
        other: data[5],
        shndx: u16::from_le_bytes([data[6], data[7]]),
        value: u64::from_le_bytes([
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
        ]),
        size: u64::from_le_bytes([
            data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23],
        ]),
    })
}

/// Parse 32-bit symbol entry
fn parse_symbol32(data: &[u8]) -> Result<Symbol, FireballError> {
    if data.len() < 16 {
        return Err(FireballError::InvalidBinary(
            "Symbol entry too small".to_string(),
        ));
    }

    Ok(Symbol {
        name: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
        value: u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as u64,
        size: u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as u64,
        info: data[12],
        other: data[13],
        shndx: u16::from_le_bytes([data[14], data[15]]),
    })
}

/// Resolve symbol names using string table
pub fn resolve_symbol_names(symbols: &mut Vec<Symbol>, string_table: &BTreeMap<u32, String>) {
    for symbol in symbols {
        // Name resolution would be done by looking up symbol.name in string_table
        // For now, we just store the index
    }
}

/// Symbol tables container
#[derive(Debug, Default)]
pub struct SymbolTables {
    /// Dynamic symbol table
    pub dynsym: Vec<Symbol>,
    /// Static symbol table
    pub symtab: Vec<Symbol>,
    /// String table for dynamic symbols
    pub dynstr: Vec<u8>,
    /// String table for static symbols
    pub strtab: Vec<u8>,
}

impl SymbolTables {
    /// Get imported functions from dynamic symbol table
    pub fn get_imports(&self) -> Vec<super::Import> {
        let mut imports = Vec::new();

        for symbol in &self.dynsym {
            if symbol.get_binding() == SymbolBinding::Global
                && symbol.get_type() == SymbolType::Func
                && symbol.shndx == 0
            // Undefined symbols
            {
                let name = self.get_symbol_name(symbol, &self.dynstr);
                imports.push(super::Import {
                    name,
                    library: None, // TODO: Parse from DT_NEEDED
                    address: symbol.value,
                });
            }
        }

        imports
    }

    /// Get exported functions from symbol tables
    pub fn get_exports(&self) -> Vec<super::Export> {
        let mut exports = Vec::new();

        // Check dynamic symbols
        for symbol in &self.dynsym {
            if symbol.get_binding() == SymbolBinding::Global
                && symbol.get_type() == SymbolType::Func
                && symbol.shndx != 0
            // Defined symbols
            {
                let name = self.get_symbol_name(symbol, &self.dynstr);
                exports.push(super::Export {
                    name,
                    address: symbol.value,
                    size: Some(symbol.size),
                });
            }
        }

        // Check static symbols
        for symbol in &self.symtab {
            if symbol.get_binding() == SymbolBinding::Global
                && symbol.get_type() == SymbolType::Func
                && symbol.shndx != 0
            // Defined symbols
            {
                let name = self.get_symbol_name(symbol, &self.strtab);
                // Avoid duplicates
                if !exports.iter().any(|e| e.address == symbol.value) {
                    exports.push(super::Export {
                        name,
                        address: symbol.value,
                        size: Some(symbol.size),
                    });
                }
            }
        }

        exports
    }

    /// Get symbol name from string table
    fn get_symbol_name(&self, symbol: &Symbol, strtab: &[u8]) -> String {
        let offset = symbol.name as usize;
        if offset >= strtab.len() {
            return String::new();
        }

        // Find null terminator
        let end = strtab[offset..]
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(strtab.len() - offset);

        String::from_utf8_lossy(&strtab[offset..offset + end]).to_string()
    }
}
