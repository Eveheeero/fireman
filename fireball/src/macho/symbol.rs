//! Mach-O symbol table definitions
//!
//! This module defines the structures for Mach-O symbol tables.

/// Symbol table entry
#[derive(Debug, Clone)]
pub struct NList {
    /// String table index
    pub n_strx: u32,

    /// Type flag
    pub n_type: u8,

    /// Section number
    pub n_sect: u8,

    /// Description
    pub n_desc: u16,

    /// Value (address)
    pub n_value: u64,
}

/// Symbol table
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    /// Local symbols
    pub local_symbols: Vec<Symbol>,

    /// External symbols
    pub external_symbols: Vec<Symbol>,

    /// Undefined symbols
    pub undefined_symbols: Vec<Symbol>,
}

/// Symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol name
    pub name: String,

    /// Symbol address
    pub address: u64,

    /// Symbol size (if known)
    pub size: Option<u64>,

    /// Symbol type
    pub symbol_type: SymbolType,
}

/// Symbol types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType {
    /// Unknown type
    Unknown,

    /// Function
    Function,

    /// Data
    Data,

    /// Section
    Section,

    /// File
    File,
}

impl SymbolTable {
    /// Get imported symbols
    pub fn get_imports(&self) -> Vec<crate::macho::Import> {
        self.undefined_symbols
            .iter()
            .map(|sym| crate::macho::Import {
                name: sym.name.clone(),
                library: None,
                address: sym.address,
            })
            .collect()
    }

    /// Get exported symbols
    pub fn get_exports(&self) -> Vec<crate::macho::Export> {
        self.external_symbols
            .iter()
            .map(|sym| crate::macho::Export {
                name: sym.name.clone(),
                address: sym.address,
                flags: 0, // Default flags
                size: sym.size,
            })
            .collect()
    }
}
