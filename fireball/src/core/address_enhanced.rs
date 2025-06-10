//! Enhanced Address implementation with proper ordering
//!
//! This module provides an improved Address design that handles the case
//! where different sections can have the same virtual address.

use crate::core::{Section, Sections};
use std::sync::Arc;

/// Enhanced Address with deterministic ordering that considers sections
#[derive(Debug, Clone)]
pub struct Address {
    /// Section information indicating which section the Address instance belongs to
    section: Option<Arc<Section>>,
    /// The virtual address of the Address instance
    virtual_offset: u64,
}

impl Address {
    /// Creates an Address from a file offset.
    pub fn from_file_offset(sections: &Sections, offset: u64) -> Self {
        let section = sections.from_file_offset(offset);
        let virtual_offset = if let Some(ref sec) = section {
            offset - sec.file_offset + sec.virtual_address
        } else {
            offset
        };

        Self {
            section,
            virtual_offset,
        }
    }

    /// Creates an Address from a virtual address.
    pub fn from_virtual_address(sections: &Sections, offset: u64) -> Self {
        let section = sections.from_virtual_address(offset);
        Self {
            section,
            virtual_offset: offset,
        }
    }

    /// Returns a unique identifier for deterministic ordering
    /// Format: "section_name:virtual_address" or "none:virtual_address"
    pub fn deterministic_id(&self) -> String {
        if let Some(ref section) = self.section {
            format!("{}:{:016x}", section.name, self.virtual_offset)
        } else {
            format!("none:{:016x}", self.virtual_offset)
        }
    }

    /// Returns section priority for ordering (lower = higher priority)
    fn section_priority(&self) -> u32 {
        if let Some(ref section) = self.section {
            // Common section priority order
            match section.name.as_str() {
                ".text" => 1,
                ".rodata" => 2,
                ".data" => 3,
                ".bss" => 4,
                _ => 100 + (section.name.as_bytes()[0] as u32), // Deterministic for other sections
            }
        } else {
            0 // No section has highest priority
        }
    }

    // Existing methods remain the same
    pub fn get_file_offset(&self) -> Option<u64> {
        if let Some(section) = &self.section {
            if self.virtual_offset - section.virtual_address > section.size_of_file {
                return None;
            }
            let virtual_offset = self.virtual_offset;
            let section_virtual_offset_start = section.virtual_address;
            let section_file_offset_start = section.file_offset;
            Some((virtual_offset - section_virtual_offset_start) + section_file_offset_start)
        } else {
            None
        }
    }

    pub fn get_virtual_address(&self) -> u64 {
        self.virtual_offset
    }

    pub(crate) fn get_section(&self) -> Option<Arc<Section>> {
        self.section.clone()
    }

    pub fn as_deterministic_string(&self) -> String {
        format!("{:016x}", self.virtual_offset)
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        // Two addresses are equal if they have the same section AND virtual address
        match (&self.section, &other.section) {
            (Some(s1), Some(s2)) => {
                s1.name == s2.name && self.virtual_offset == other.virtual_offset
            }
            (None, None) => self.virtual_offset == other.virtual_offset,
            _ => false,
        }
    }
}

impl Eq for Address {}

impl std::hash::Hash for Address {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash both section name and virtual address for uniqueness
        if let Some(ref section) = self.section {
            section.name.hash(state);
        } else {
            "none".hash(state);
        }
        self.virtual_offset.hash(state);
    }
}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Address {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // ENHANCED: Compare by section first, then by virtual address
        // This ensures deterministic ordering even when different sections
        // have the same virtual address

        // 1. Compare section priority
        match self.section_priority().cmp(&other.section_priority()) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        // 2. If same priority, compare section names for determinism
        match (&self.section, &other.section) {
            (Some(s1), Some(s2)) => match s1.name.cmp(&s2.name) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            },
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (None, None) => {}
        }

        // 3. Finally compare virtual addresses
        self.virtual_offset.cmp(&other.virtual_offset)
    }
}

// Arithmetic operations remain the same
impl std::ops::AddAssign<u64> for Address {
    fn add_assign(&mut self, rhs: u64) {
        self.virtual_offset += rhs;
    }
}

impl std::ops::Add<u64> for Address {
    type Output = Self;
    fn add(mut self, rhs: u64) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::Add<u64> for &Address {
    type Output = Address;
    fn add(self, rhs: u64) -> Self::Output {
        let mut new_address = self.clone();
        new_address += rhs;
        new_address
    }
}

impl std::ops::SubAssign<u64> for Address {
    fn sub_assign(&mut self, rhs: u64) {
        self.virtual_offset -= rhs;
    }
}

impl std::ops::Sub<u64> for Address {
    type Output = Self;
    fn sub(mut self, rhs: u64) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::Sub<u64> for &Address {
    type Output = Address;
    fn sub(self, rhs: u64) -> Self::Output {
        let mut new_address = self.clone();
        new_address -= rhs;
        new_address
    }
}

impl std::ops::Sub<&Address> for &Address {
    type Output = u64;
    fn sub(self, rhs: &Address) -> Self::Output {
        self.virtual_offset - rhs.virtual_offset
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(section) = &self.section {
            write!(f, "[{}] {:#X}", section.name, self.virtual_offset)
        } else {
            write!(f, "{:#X}", self.virtual_offset)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_ordering_with_sections() {
        // Create mock sections
        let text_section = Arc::new(Section {
            name: ".text".to_string(),
            virtual_address: 0x1000,
            file_offset: 0x400,
            size_of_file: 0x1000,
            size_of_memory: 0x1000,
            characteristics: 0,
        });

        let data_section = Arc::new(Section {
            name: ".data".to_string(),
            virtual_address: 0x1000, // Same virtual address!
            file_offset: 0x1400,
            size_of_file: 0x1000,
            size_of_memory: 0x1000,
            characteristics: 0,
        });

        let addr1 = Address {
            section: Some(text_section),
            virtual_offset: 0x1000,
        };

        let addr2 = Address {
            section: Some(data_section),
            virtual_offset: 0x1000,
        };

        // They should NOT be equal despite same virtual address
        assert_ne!(addr1, addr2);

        // .text should come before .data in ordering
        assert!(addr1 < addr2);

        // Deterministic IDs should be different
        assert_ne!(addr1.deterministic_id(), addr2.deterministic_id());
    }
}
