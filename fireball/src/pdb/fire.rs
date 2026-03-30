//! [`Fire`] and [`FireRaw`] implementations for [`StandalonePdb`].
//!
//! Since a PDB file contains no executable code, most analysis methods return
//! errors or empty results.  `decompile_all()` returns a pre-built C-like dump
//! of the type and symbol information extracted at construction time.

use super::StandalonePdb;
use crate::{
    BinaryKind,
    core::{
        Address, Block, Blocks, Fire, FireRaw, PreDefinedOffset, PreDefinedOffsets, Relations,
        Sections,
    },
    prelude::*,
};
use pdb::FallibleIterator;
use std::sync::Arc;

/// Concrete PDB source type used throughout this module.
type PdbSource = std::io::Cursor<Vec<u8>>;

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

impl StandalonePdb {
    /// Creates a [`StandalonePdb`] by parsing the given PDB bytes.
    pub(crate) fn new(path: Option<String>, binary: Vec<u8>) -> Result<Self, FireballError> {
        let sections = Sections::new();
        let relations = Relations::new();
        let blocks = Blocks::new(relations.clone());
        let defined = PreDefinedOffsets::new();

        // Parse the PDB to extract symbols and types.
        let dump = match build_dump(&binary, &defined, &sections) {
            Ok(d) => d,
            Err(e) => {
                warn!("PDB parsing partially failed: {e}");
                String::from("// PDB parsing failed\n")
            }
        };

        Ok(Self {
            path,
            binary,
            defined,
            sections,
            blocks,
            relations,
            dump,
        })
    }

    pub fn kind(&self) -> BinaryKind {
        BinaryKind::DebugInfo
    }

    pub fn cancel_analysis(&self) {}

    pub fn reset_analysis_cancellation(&self) {}
}

// ---------------------------------------------------------------------------
// Fire trait
// ---------------------------------------------------------------------------

impl Fire for StandalonePdb {
    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_binary(&self) -> &Vec<u8> {
        &self.binary
    }

    fn decompile_all(&self) -> Result<String, DecompileError> {
        Ok(self.dump.clone())
    }

    fn decompile_from_entry(&self) -> Result<String, DecompileError> {
        Err(DecompileError::NoEntryPoint)
    }

    fn decompile_from_file_offset(&self, _address: u64) -> Result<String, DecompileError> {
        Err(DecompileError::NoEntryPoint)
    }

    fn decompile_from_virtual_address(&self, _address: u64) -> Result<String, DecompileError> {
        Err(DecompileError::NoEntryPoint)
    }
}

// ---------------------------------------------------------------------------
// FireRaw trait
// ---------------------------------------------------------------------------

impl FireRaw for StandalonePdb {
    fn analyze_all(&self) -> Result<Vec<Arc<Block>>, DecompileError> {
        Ok(Vec::new())
    }

    fn analyze_from_entry(&self) -> Result<Arc<Block>, DecompileError> {
        Err(DecompileError::NoEntryPoint)
    }

    fn analyze_from_file_offset(&self, _address: u64) -> Result<Arc<Block>, DecompileError> {
        Err(DecompileError::NoEntryPoint)
    }

    fn analyze_from_virtual_address(&self, _address: u64) -> Result<Arc<Block>, DecompileError> {
        Err(DecompileError::NoEntryPoint)
    }

    fn analyze_block(&self, _address: &Address) -> Result<Arc<Block>, DecompileError> {
        Err(DecompileError::NoEntryPoint)
    }

    fn get_sections(&self) -> Arc<Sections> {
        self.sections.clone()
    }

    fn get_defined(&self) -> Arc<PreDefinedOffsets> {
        self.defined.clone()
    }

    fn get_blocks(&self) -> Arc<Blocks> {
        self.blocks.clone()
    }

    fn get_relations(&self) -> Arc<Relations> {
        self.relations.clone()
    }
}

// ---------------------------------------------------------------------------
// PDB parsing — standalone functions to avoid lifetime issues with pdb crate
// ---------------------------------------------------------------------------

fn build_dump(
    binary: &[u8],
    defined: &PreDefinedOffsets,
    sections: &Sections,
) -> Result<String, String> {
    let cursor: PdbSource = std::io::Cursor::new(binary.to_vec());
    let mut pdb_file = pdb::PDB::open(cursor).map_err(|e| format!("Failed to open PDB: {e}"))?;

    let mut output = String::with_capacity(8192);

    // --- Type information ---
    dump_types(&mut pdb_file, &mut output);

    // --- Symbol information ---
    dump_symbols(&mut pdb_file, &mut output, defined, sections);

    if output.is_empty() {
        output.push_str("// PDB file contains no type or symbol information\n");
    }

    Ok(output)
}

fn dump_types(pdb_file: &mut pdb::PDB<'_, PdbSource>, output: &mut String) {
    let type_info = match pdb_file.type_information() {
        Ok(t) => t,
        Err(_) => return,
    };

    let mut iter = type_info.iter();
    let mut type_section = String::new();
    let mut type_count = 0u32;

    while let Ok(Some(item)) = iter.next() {
        let parsed = match item.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        match parsed {
            pdb::TypeData::Class(class) => {
                let kind_str = match class.kind {
                    pdb::ClassKind::Class => "class",
                    pdb::ClassKind::Struct => "struct",
                    pdb::ClassKind::Interface => "interface",
                };
                type_section.push_str(&format!(
                    "{} {} {{ /* size: {} */ }};\n",
                    kind_str, class.name, class.size
                ));
                type_count += 1;
            }
            pdb::TypeData::Union(union) => {
                type_section.push_str(&format!(
                    "union {} {{ /* size: {} */ }};\n",
                    union.name, union.size
                ));
                type_count += 1;
            }
            pdb::TypeData::Enumeration(enum_type) => {
                type_section.push_str(&format!("enum {} {{ ... }};\n", enum_type.name));
                type_count += 1;
            }
            _ => {}
        }
    }

    if type_count > 0 {
        output.push_str(&format!("// --- Types ({type_count} definitions) ---\n\n"));
        output.push_str(&type_section);
        output.push('\n');
    }
}

fn dump_symbols(
    pdb_file: &mut pdb::PDB<'_, PdbSource>,
    output: &mut String,
    defined: &PreDefinedOffsets,
    sections: &Sections,
) {
    let address_map = match pdb_file.address_map() {
        Ok(m) => m,
        Err(_) => return,
    };

    let mut functions: Vec<(String, u32, u32)> = Vec::new();
    let mut data_syms: Vec<(String, u32)> = Vec::new();

    // Global symbols
    if let Ok(symbol_table) = pdb_file.global_symbols() {
        let mut iter = symbol_table.iter();
        while let Ok(Some(symbol)) = iter.next() {
            if let Ok(pdb::SymbolData::Public(data)) = symbol.parse() {
                if let Some(rva) = data.offset.to_rva(&address_map) {
                    let name = demangle_name(&data.name.to_string());
                    if data.function {
                        functions.push((name, rva.0, 0u32));
                    } else {
                        data_syms.push((name, rva.0));
                    }
                }
            }
        }
    }

    // Module symbols (richer: has Procedure with size)
    if let Ok(dbi) = pdb_file.debug_information() {
        if let Ok(mut modules) = dbi.modules() {
            while let Ok(Some(module)) = modules.next() {
                let info = match pdb_file.module_info(&module) {
                    Ok(Some(info)) => info,
                    _ => continue,
                };
                let mut syms = match info.symbols() {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                while let Ok(Some(symbol)) = syms.next() {
                    match symbol.parse() {
                        Ok(pdb::SymbolData::Procedure(proc)) => {
                            if let Some(rva) = proc.offset.to_rva(&address_map) {
                                functions.push((
                                    demangle_name(&proc.name.to_string()),
                                    rva.0,
                                    proc.len,
                                ));
                            }
                        }
                        Ok(pdb::SymbolData::Data(data)) => {
                            if let Some(rva) = data.offset.to_rva(&address_map) {
                                data_syms.push((demangle_name(&data.name.to_string()), rva.0));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Deduplicate by RVA (module symbols are more precise)
    functions.sort_by_key(|(_, rva, _)| *rva);
    functions.dedup_by_key(|(_, rva, _)| *rva);
    data_syms.sort_by_key(|(_, rva)| *rva);
    data_syms.dedup_by_key(|(_, rva)| *rva);

    // Merge into PreDefinedOffsets
    for (name, rva, _) in &functions {
        if *rva == 0 {
            continue;
        }
        defined.insert(PreDefinedOffset {
            name: name.clone(),
            address: Address::from_virtual_address(sections, *rva as u64),
        });
    }
    for (name, rva) in &data_syms {
        if *rva == 0 {
            continue;
        }
        defined.insert(PreDefinedOffset {
            name: name.clone(),
            address: Address::from_virtual_address(sections, *rva as u64),
        });
    }

    // Render functions
    if !functions.is_empty() {
        output.push_str(&format!(
            "// --- Functions ({} symbols) ---\n\n",
            functions.len()
        ));
        for (name, rva, size) in &functions {
            if *size > 0 {
                output.push_str(&format!(
                    "void {name}(void); // RVA: 0x{rva:08X}, size: {size}\n"
                ));
            } else {
                output.push_str(&format!("void {name}(void); // RVA: 0x{rva:08X}\n"));
            }
        }
        output.push('\n');
    }

    // Render data symbols
    if !data_syms.is_empty() {
        output.push_str(&format!(
            "// --- Data ({} symbols) ---\n\n",
            data_syms.len()
        ));
        for (name, rva) in &data_syms {
            output.push_str(&format!("extern void* {name}; // RVA: 0x{rva:08X}\n"));
        }
        output.push('\n');
    }
}

fn demangle_name(raw: &str) -> String {
    if let Ok(sym) = cpp_demangle::Symbol::new(raw) {
        if let Ok(demangled) = sym.demangle() {
            return demangled;
        }
    }
    let demangled = rustc_demangle::demangle(raw).to_string();
    if demangled != raw {
        return demangled;
    }
    raw.to_string()
}
