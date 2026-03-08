//! Conservative MSVC RTTI and vtable recovery for PE binaries.
//!
//! This additive first slice targets the common Win64/MSVC layout:
//! `CompleteObjectLocator` records in read-only data, paired with vtables whose
//! metadata slot points back to the locator. It intentionally does not attempt
//! RTTI-less class inference, inheritance graph recovery, or Win32-specific
//! layouts yet.

use crate::{
    core::{Address, PreDefinedOffset, PreDefinedOffsets, Section, Sections},
    prelude::*,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct RttiEntry {
    pub class_name: String,
    pub raw_type_name: String,
    pub type_descriptor_rva: u64,
    pub complete_object_locator_rva: u64,
    pub class_hierarchy_descriptor_rva: u64,
    pub vtable_rva: u64,
    pub method_count: usize,
    pub offset_to_top: u32,
    pub constructor_displacement_offset: u32,
}

#[derive(Debug, Clone, Default)]
pub struct RttiInfo {
    pub entries: Vec<RttiEntry>,
}

#[derive(Debug, Clone)]
struct TypeDescriptorInfo {
    raw_name: String,
    class_name: String,
}

#[derive(Debug, Clone)]
struct CompleteObjectLocator {
    rva: u64,
    type_descriptor_rva: u64,
    class_hierarchy_descriptor_rva: u64,
    offset_to_top: u32,
    constructor_displacement_offset: u32,
}

pub fn try_load_rtti(
    binary: &[u8],
    sections: &Sections,
    image_base: u64,
    is_64: bool,
) -> Option<RttiInfo> {
    if !is_64 {
        debug!("MSVC RTTI parser currently only supports 64-bit PE images");
        return None;
    }

    let section_snapshots = sections.all();
    let type_descriptors = collect_type_descriptors(binary, sections, &section_snapshots);
    if type_descriptors.is_empty() {
        debug!("No MSVC RTTI type descriptors found in PE data sections");
        return None;
    }

    let complete_object_locators =
        collect_complete_object_locators(binary, sections, &section_snapshots, &type_descriptors);
    if complete_object_locators.is_empty() {
        debug!("No MSVC RTTI complete object locators found in PE data sections");
        return None;
    }

    let entries = collect_vtables(
        binary,
        sections,
        &section_snapshots,
        image_base,
        &type_descriptors,
        &complete_object_locators,
    );

    if entries.is_empty() {
        debug!("MSVC RTTI metadata found but no RTTI-backed vtables passed validation");
    } else {
        info!(
            "Recovered {} RTTI-backed vtable(s) from PE data sections",
            entries.len()
        );
    }

    Some(RttiInfo { entries })
}

pub fn merge_rtti_symbols(info: &RttiInfo, defined: &PreDefinedOffsets, sections: &Sections) {
    if info.entries.is_empty() {
        return;
    }

    let mut known_addresses = collect_known_addresses(defined);
    let mut merged = 0usize;

    for entry in &info.entries {
        let class_name = display_class_name(entry);

        let vtable_address = Address::from_virtual_address(sections, entry.vtable_rva);
        if known_addresses.insert(vtable_address.clone()) {
            defined.insert(PreDefinedOffset {
                name: format!("vtable_for_{class_name}"),
                address: vtable_address,
            });
            merged += 1;
        }

        let typeinfo_address = Address::from_virtual_address(sections, entry.type_descriptor_rva);
        if known_addresses.insert(typeinfo_address.clone()) {
            defined.insert(PreDefinedOffset {
                name: format!("typeinfo_for_{class_name}"),
                address: typeinfo_address,
            });
            merged += 1;
        }
    }

    if merged != 0 {
        info!(
            "Merged {} RTTI-derived symbol(s) into predefined offsets",
            merged
        );
    }
}

fn collect_type_descriptors(
    binary: &[u8],
    sections: &Sections,
    section_snapshots: &[Section],
) -> HashMap<u64, TypeDescriptorInfo> {
    let mut out = HashMap::new();

    for section in iter_candidate_rtti_sections(section_snapshots) {
        let Some((start, end)) = section_file_range(section, binary.len()) else {
            continue;
        };

        let cursor_start = start.saturating_add(16);
        for cursor in cursor_start..end.saturating_sub(4) {
            if !starts_with_type_descriptor_name(binary, cursor) {
                continue;
            }

            let Some(raw_name) = read_c_string(binary, cursor, 512) else {
                continue;
            };
            let Some(class_name) = normalize_type_descriptor_name(&raw_name) else {
                continue;
            };

            let descriptor_rva = section.virtual_address + (cursor - start) as u64 - 16;
            let Some(descriptor_file_offset) = rva_to_file_offset(sections, descriptor_rva) else {
                continue;
            };
            if descriptor_file_offset != (cursor - 16) as u64 {
                continue;
            }

            out.entry(descriptor_rva)
                .or_insert_with(|| TypeDescriptorInfo {
                    raw_name,
                    class_name,
                });
        }
    }

    out
}

fn collect_complete_object_locators(
    binary: &[u8],
    sections: &Sections,
    section_snapshots: &[Section],
    type_descriptors: &HashMap<u64, TypeDescriptorInfo>,
) -> Vec<CompleteObjectLocator> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();

    for section in iter_candidate_rtti_sections(section_snapshots) {
        let Some((start, end)) = section_file_range(section, binary.len()) else {
            continue;
        };

        let mut cursor = start;
        while cursor + 24 <= end {
            let record_rva = section.virtual_address + (cursor - start) as u64;
            let signature = read_u32(binary, cursor).unwrap_or(u32::MAX);
            let offset_to_top = read_u32(binary, cursor + 4).unwrap_or(0);
            let constructor_displacement_offset = read_u32(binary, cursor + 8).unwrap_or(0);
            let type_descriptor_rva = read_u32(binary, cursor + 12).unwrap_or(0) as u64;
            let class_hierarchy_descriptor_rva = read_u32(binary, cursor + 16).unwrap_or(0) as u64;
            let self_rva = read_u32(binary, cursor + 20).unwrap_or(0) as u64;

            if signature <= 1
                && self_rva == record_rva
                && type_descriptors.contains_key(&type_descriptor_rva)
                && class_hierarchy_descriptor_rva != 0
                && sections
                    .from_virtual_address(class_hierarchy_descriptor_rva)
                    .is_some()
                && seen.insert(record_rva)
            {
                out.push(CompleteObjectLocator {
                    rva: record_rva,
                    type_descriptor_rva,
                    class_hierarchy_descriptor_rva,
                    offset_to_top,
                    constructor_displacement_offset,
                });
            }

            cursor += 4;
        }
    }

    out
}

fn collect_vtables(
    binary: &[u8],
    sections: &Sections,
    section_snapshots: &[Section],
    image_base: u64,
    type_descriptors: &HashMap<u64, TypeDescriptorInfo>,
    complete_object_locators: &[CompleteObjectLocator],
) -> Vec<RttiEntry> {
    let locator_by_pointer: HashMap<u64, &CompleteObjectLocator> = complete_object_locators
        .iter()
        .map(|locator| (image_base + locator.rva, locator))
        .collect();
    let mut out = Vec::new();
    let mut seen_vtables = HashSet::new();

    for section in iter_candidate_rtti_sections(section_snapshots) {
        let Some((start, end)) = section_file_range(section, binary.len()) else {
            continue;
        };

        let mut cursor = start;
        while cursor + 8 <= end {
            let meta_pointer = read_u64(binary, cursor).unwrap_or(0);
            let Some(locator) = locator_by_pointer.get(&meta_pointer) else {
                cursor += 8;
                continue;
            };

            let vtable_rva = section.virtual_address + (cursor - start) as u64 + 8;
            if !seen_vtables.insert(vtable_rva) {
                cursor += 8;
                continue;
            }

            let method_count = count_consecutive_virtual_methods(
                binary,
                sections,
                section_snapshots,
                image_base,
                vtable_rva,
            );
            if method_count < 2 {
                cursor += 8;
                continue;
            }

            let Some(type_descriptor) = type_descriptors.get(&locator.type_descriptor_rva) else {
                cursor += 8;
                continue;
            };

            out.push(RttiEntry {
                class_name: type_descriptor.class_name.clone(),
                raw_type_name: type_descriptor.raw_name.clone(),
                type_descriptor_rva: locator.type_descriptor_rva,
                complete_object_locator_rva: locator.rva,
                class_hierarchy_descriptor_rva: locator.class_hierarchy_descriptor_rva,
                vtable_rva,
                method_count,
                offset_to_top: locator.offset_to_top,
                constructor_displacement_offset: locator.constructor_displacement_offset,
            });

            cursor += 8;
        }
    }

    out.sort_by_key(|entry| entry.vtable_rva);
    out
}

fn count_consecutive_virtual_methods(
    binary: &[u8],
    sections: &Sections,
    section_snapshots: &[Section],
    image_base: u64,
    vtable_rva: u64,
) -> usize {
    let Some(mut file_offset) = rva_to_file_offset(sections, vtable_rva) else {
        return 0;
    };

    let mut count = 0usize;
    while (file_offset as usize) + 8 <= binary.len() {
        let Some(pointer_value) = read_u64(binary, file_offset as usize) else {
            break;
        };
        let Some(target_rva) = normalize_pointer(pointer_value, sections, image_base) else {
            break;
        };

        let Some(target_section) = section_snapshots.iter().find(|section| {
            section.virtual_address <= target_rva
                && target_rva < section.virtual_address + section.virtual_size
        }) else {
            break;
        };

        if !target_section.is_executable() {
            break;
        }

        count += 1;
        file_offset += 8;
    }

    count
}

fn iter_candidate_rtti_sections(section_snapshots: &[Section]) -> impl Iterator<Item = &Section> {
    section_snapshots.iter().filter(|section| {
        !section.is_executable()
            && (section.name.contains("rdata")
                || section.name.contains("data.rel.ro")
                || section.name == ".data")
    })
}

fn section_file_range(section: &Section, binary_len: usize) -> Option<(usize, usize)> {
    let start = usize::try_from(section.file_offset).ok()?;
    let size = usize::try_from(section.size_of_file).ok()?;
    let end = start.checked_add(size)?.min(binary_len);
    if start >= end {
        return None;
    }
    Some((start, end))
}

fn rva_to_file_offset(sections: &Sections, rva: u64) -> Option<u64> {
    let section = sections.from_virtual_address(rva)?;
    let relative = rva.checked_sub(section.virtual_address)?;
    Some(section.file_offset + relative)
}

fn starts_with_type_descriptor_name(binary: &[u8], offset: usize) -> bool {
    [
        b".?AV".as_slice(),
        b".?AU".as_slice(),
        b".PEAV".as_slice(),
        b".PEAU".as_slice(),
    ]
    .iter()
    .any(|prefix| binary.get(offset..offset + prefix.len()) == Some(*prefix))
}

fn read_c_string(binary: &[u8], offset: usize, max_len: usize) -> Option<String> {
    let end = binary
        .get(offset..)?
        .iter()
        .take(max_len)
        .position(|&byte| byte == 0)?;
    let bytes = &binary[offset..offset + end];
    let string = std::str::from_utf8(bytes).ok()?.trim();
    if string.is_empty() {
        return None;
    }
    Some(string.to_string())
}

fn read_u32(binary: &[u8], offset: usize) -> Option<u32> {
    let bytes: [u8; 4] = binary.get(offset..offset + 4)?.try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}

fn read_u64(binary: &[u8], offset: usize) -> Option<u64> {
    let bytes: [u8; 8] = binary.get(offset..offset + 8)?.try_into().ok()?;
    Some(u64::from_le_bytes(bytes))
}

fn normalize_pointer(value: u64, sections: &Sections, image_base: u64) -> Option<u64> {
    if sections.from_virtual_address(value).is_some() {
        return Some(value);
    }

    if image_base != 0 && value >= image_base {
        let rva = value - image_base;
        if sections.from_virtual_address(rva).is_some() {
            return Some(rva);
        }
    }

    None
}

fn normalize_type_descriptor_name(raw: &str) -> Option<String> {
    let body = raw
        .strip_prefix(".?AV")
        .or_else(|| raw.strip_prefix(".?AU"))
        .or_else(|| raw.strip_prefix(".PEAV"))
        .or_else(|| raw.strip_prefix(".PEAU"))?;
    let body = body.strip_suffix("@@").unwrap_or(body);
    let parts: Vec<&str> = body.split('@').filter(|part| !part.is_empty()).collect();
    if parts.is_empty() {
        return None;
    }

    let demangled_parts: Vec<String> = parts
        .into_iter()
        .rev()
        .map(|part| part.trim_start_matches('?').to_string())
        .collect();
    Some(demangled_parts.join("::"))
}

fn collect_known_addresses(defined: &PreDefinedOffsets) -> HashSet<Address> {
    defined
        .get_reader()
        .iter()
        .map(|offset| offset.address.clone())
        .collect()
}

fn display_class_name(entry: &RttiEntry) -> &str {
    if entry.class_name.is_empty() {
        &entry.raw_type_name
    } else {
        &entry.class_name
    }
}
