//! Conservative DWARF symbol extraction for PE and ELF binaries.
//!
//! This module intentionally lands a narrow first slice: it looks for embedded
//! `.debug_*` sections, extracts `DW_TAG_subprogram` names plus `DW_AT_low_pc`,
//! and merges those names into [`PreDefinedOffsets`]. It does not yet surface
//! variables, types, scopes, or source mappings.

use crate::{
    core::{Address, PreDefinedOffset, PreDefinedOffsets, Sections},
    prelude::*,
};
use gimli::{AttributeValue, Dwarf, EndianSlice, LittleEndian, SectionId, constants};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone)]
pub struct DwarfSymbol {
    pub name: String,
    pub address: u64,
}

#[derive(Debug, Clone)]
pub struct DwarfInfo {
    pub symbols: Vec<DwarfSymbol>,
}

pub fn try_load_dwarf(binary: &[u8]) -> Option<DwarfInfo> {
    let object = goblin::Object::parse(binary).ok()?;
    let dwarf_sections = match &object {
        goblin::Object::PE(pe) => collect_dwarf_sections_pe(binary, &pe.sections),
        goblin::Object::Elf(elf) => collect_dwarf_sections_elf(binary, elf),
        _ => return None,
    };
    if dwarf_sections.is_empty() {
        debug!("DWARF debug sections not present in binary");
        return None;
    }

    let dwarf_cow = match Dwarf::load(|id| load_section(&dwarf_sections, id)) {
        Ok(dwarf) => dwarf,
        Err(error) => {
            warn!("Failed to parse embedded DWARF sections: {error}");
            return None;
        }
    };
    #[allow(deprecated)]
    let dwarf = dwarf_cow.borrow(|section| EndianSlice::new(section.as_ref(), LittleEndian));

    let mut symbols = Vec::new();
    let mut units = dwarf.units();
    loop {
        let Some(unit_header) = next_unit(&mut units) else {
            break;
        };
        let unit = match dwarf.unit(unit_header) {
            Ok(unit) => unit,
            Err(error) => {
                warn!("Failed to load DWARF unit: {error}");
                continue;
            }
        };

        let mut entries = unit.entries();
        loop {
            let entry = match entries.next_dfs() {
                Ok(Some(entry)) => entry,
                Ok(None) => break,
                Err(error) => {
                    warn!("Failed to iterate DWARF entries: {error}");
                    break;
                }
            };
            if entry.tag() != constants::DW_TAG_subprogram {
                continue;
            }

            let Some(address) = extract_symbol_address(&entry) else {
                continue;
            };
            let Some(name) = extract_symbol_name(&dwarf, &unit, &entry) else {
                continue;
            };

            symbols.push(DwarfSymbol { name, address });
        }
    }

    debug!(
        "Recovered {} DWARF subprogram symbols from embedded debug sections",
        symbols.len()
    );

    Some(DwarfInfo { symbols })
}

pub fn merge_dwarf_symbols(
    dwarf_info: &DwarfInfo,
    defined: &PreDefinedOffsets,
    sections: &Sections,
    image_base: u64,
) {
    let mut known_addresses = collect_known_addresses(defined);
    let mut merged = 0u64;

    for symbol in &dwarf_info.symbols {
        let Some(virtual_address) = normalize_dwarf_address(symbol.address, sections, image_base)
        else {
            continue;
        };

        let address = Address::from_virtual_address(sections, virtual_address);
        if !known_addresses.insert(address.clone()) {
            continue;
        }

        defined.insert(PreDefinedOffset {
            name: demangle_name(&symbol.name),
            address,
        });
        merged += 1;
    }

    debug!("Merged {} DWARF symbols into predefined offsets", merged);
}

fn collect_dwarf_sections_pe<'a>(
    binary: &'a [u8],
    sections: &[goblin::pe::section_table::SectionTable],
) -> HashMap<String, &'a [u8]> {
    let mut result = HashMap::new();

    for section in sections {
        let Ok(name) = section.name() else {
            continue;
        };
        if !name.starts_with(".debug_") {
            continue;
        }

        let start = section.pointer_to_raw_data as usize;
        let size = section.size_of_raw_data as usize;
        let Some(end) = start.checked_add(size) else {
            continue;
        };
        if size == 0 || end > binary.len() {
            warn!(
                "Skipping malformed DWARF section {} with file range {}..{}",
                name, start, end
            );
            continue;
        }

        result.insert(name.to_string(), &binary[start..end]);
    }

    result
}

fn collect_dwarf_sections_elf<'a>(
    binary: &'a [u8],
    elf: &goblin::elf::Elf,
) -> HashMap<String, &'a [u8]> {
    let mut result = HashMap::new();

    for sh in &elf.section_headers {
        let Some(name) = elf.shdr_strtab.get_at(sh.sh_name) else {
            continue;
        };
        if !name.starts_with(".debug_") {
            continue;
        }

        let start = sh.sh_offset as usize;
        let size = sh.sh_size as usize;
        let Some(end) = start.checked_add(size) else {
            continue;
        };
        if size == 0 || end > binary.len() {
            warn!(
                "Skipping malformed DWARF section {} with file range {}..{}",
                name, start, end
            );
            continue;
        }

        result.insert(name.to_string(), &binary[start..end]);
    }

    result
}

fn load_section<'a>(
    sections: &HashMap<String, &'a [u8]>,
    id: SectionId,
) -> Result<Cow<'a, [u8]>, gimli::Error> {
    Ok(Cow::Borrowed(
        sections.get(id.name()).copied().unwrap_or(&[]),
    ))
}

fn next_unit<R: gimli::Reader<Offset = usize>>(
    units: &mut gimli::DebugInfoUnitHeadersIter<R>,
) -> Option<gimli::UnitHeader<R>> {
    match units.next() {
        Ok(Some(header)) => Some(header),
        Ok(None) => None,
        Err(error) => {
            debug!("Stopped DWARF unit iteration early: {error}");
            None
        }
    }
}

fn extract_symbol_address<R: gimli::Reader<Offset = usize>>(
    entry: &gimli::DebuggingInformationEntry<R>,
) -> Option<u64> {
    let attribute = entry.attr(constants::DW_AT_low_pc)?;
    match attribute.value() {
        AttributeValue::Addr(address) => Some(address),
        _ => None,
    }
}

fn extract_symbol_name<R: gimli::Reader<Offset = usize>>(
    dwarf: &Dwarf<R>,
    unit: &gimli::Unit<R>,
    entry: &gimli::DebuggingInformationEntry<R>,
) -> Option<String> {
    for name_attr in [
        constants::DW_AT_linkage_name,
        constants::DW_AT_MIPS_linkage_name,
        constants::DW_AT_name,
    ] {
        let Some(attribute) = entry.attr(name_attr) else {
            continue;
        };
        let value = attribute.value();
        let Ok(name) = dwarf.attr_string(unit, value) else {
            continue;
        };
        let Ok(name) = name.to_string_lossy() else {
            continue;
        };
        let name = name.into_owned();
        if !name.is_empty() {
            return Some(name);
        }
    }

    None
}

fn normalize_dwarf_address(address: u64, sections: &Sections, image_base: u64) -> Option<u64> {
    if sections.from_virtual_address(address).is_some() {
        return Some(address);
    }

    if image_base != 0 && address >= image_base {
        let rva = address - image_base;
        if sections.from_virtual_address(rva).is_some() {
            return Some(rva);
        }
    }

    None
}

fn collect_known_addresses(defined: &PreDefinedOffsets) -> HashSet<Address> {
    defined
        .get_reader()
        .iter()
        .map(|offset| offset.address.clone())
        .collect()
}

fn demangle_name(raw: &str) -> String {
    if let Ok(symbol) = cpp_demangle::Symbol::new(raw) {
        if let Ok(demangled) = symbol.demangle() {
            return demangled;
        }
    }

    let demangled = rustc_demangle::demangle(raw).to_string();
    if demangled != raw {
        return demangled;
    }

    raw.to_string()
}
