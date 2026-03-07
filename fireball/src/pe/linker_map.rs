//! Linker map file parser for PE binaries.
//!
//! Supports both MSVC (`link.exe`) and GNU (`ld`) map file formats.
//! Discovered symbols are merged directly into the decompiler's
//! [`PreDefinedOffsets`] table.

use crate::{
    core::{Address, PreDefinedOffset, PreDefinedOffsets, Sections},
    prelude::*,
};

/// Auto-detect the map file format and parse symbols into [`PreDefinedOffsets`].
///
/// ### Arguments
/// - `content`  - full text of the `.map` file
/// - `defined`  - target offset table (interior-mutable via `RwLock`)
/// - `sections` - PE section layout used for virtual-address resolution
pub fn parse_and_merge_map(content: &str, defined: &PreDefinedOffsets, sections: &Sections) {
    let count = if content.contains("Publics by Value") {
        parse_msvc(content, defined, sections)
    } else {
        parse_gnu(content, defined, sections)
    };

    debug!("linker map: inserted {count} symbols");
}

/// Parse an MSVC linker map.
///
/// Relevant lines look like:
/// ```text
///  0001:00001000       _main              00401000 f   main.obj
/// ```
/// We extract the hex address in the 3rd whitespace-delimited column and the
/// symbol name in the 2nd column.
fn parse_msvc(content: &str, defined: &PreDefinedOffsets, sections: &Sections) -> usize {
    let mut count = 0usize;

    // The "Publics by Value" section starts after that header line.
    // We process every line that matches the expected column layout.
    for line in content.lines() {
        let trimmed = line.trim();

        // A symbol line starts with a section:offset pair like "0001:00001234".
        if !trimmed
            .as_bytes()
            .first()
            .is_some_and(|b| b.is_ascii_hexdigit())
        {
            continue;
        }

        let mut cols = trimmed.split_whitespace();

        // Column 0: section:offset (e.g. "0001:00001000") -- skip
        let Some(_seg_off) = cols.next() else {
            continue;
        };
        // Column 1: symbol name
        let Some(name) = cols.next() else {
            continue;
        };
        // Column 2: flat virtual address (hex without 0x prefix)
        let Some(va_str) = cols.next() else {
            continue;
        };

        let Ok(va) = u64::from_str_radix(va_str, 16) else {
            continue;
        };

        let address = Address::from_virtual_address(sections, va);
        defined.insert(PreDefinedOffset {
            name: name.to_string(),
            address,
        });
        count += 1;
    }

    count
}

/// Parse a GNU ld linker map.
///
/// Relevant lines look like:
/// ```text
///                 0x0000000000401000                main
/// ```
/// We look for lines whose first non-whitespace token is a `0x` hex address
/// followed by a symbol name.
fn parse_gnu(content: &str, defined: &PreDefinedOffsets, sections: &Sections) -> usize {
    let mut count = 0usize;

    for line in content.lines() {
        let trimmed = line.trim();

        // Must start with "0x"
        if !trimmed.starts_with("0x") {
            continue;
        }

        let mut cols = trimmed.split_whitespace();

        // Column 0: hex address with 0x prefix
        let Some(va_str) = cols.next() else {
            continue;
        };
        // Column 1: symbol name
        let Some(name) = cols.next() else {
            continue;
        };

        // Reject lines with more than 2 columns (section size entries, etc.)
        if cols.next().is_some() {
            continue;
        }

        let Some(hex_digits) = va_str.strip_prefix("0x") else {
            continue;
        };
        let Ok(va) = u64::from_str_radix(hex_digits, 16) else {
            continue;
        };

        let address = Address::from_virtual_address(sections, va);
        defined.insert(PreDefinedOffset {
            name: name.to_string(),
            address,
        });
        count += 1;
    }

    count
}
