//! Module defining the `Address` struct, which holds information about
//! virtual addresses or file offsets inside the target program.
//!
//! # Note
//! - There is no `Addresses` type because there is no need to store address information.

use crate::core::{Section, Sections};
use std::sync::Arc;

/// A struct that holds address information used inside the target program.
///
/// This struct calculates which section a given offset belongs to and computes file offsets and other related values based on section data.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Address {
    /// Section information indicating which section the Address instance belongs to
    section: Option<Arc<Section>>,
    /// The virtual address of the Address instance
    ///
    /// ### Note
    /// - Virtual addresses are used because they can represent ranges that file offsets cannot.
    virtual_offset: u64,
}

impl Address {
    /// Creates an Address from a file offset.
    ///
    /// ### Arguments
    /// - `sections: &Sections` - a Sections object containing section information
    /// - `offset: u64` - file offset
    ///
    /// ### Returns
    /// - `Self` - the Address created from the file offset
    pub fn from_file_offset(sections: &Sections, offset: u64) -> Self {
        // Find the section corresponding to the offset
        let section = sections.from_file_offset(offset);
        // Compute the virtual address based on section info
        // It's safe to use unwrap() because a section always exists for the file offset.
        let virtual_offset = offset - section.as_ref().unwrap().file_offset
            + section.as_ref().unwrap().virtual_address;

        Self {
            section,
            virtual_offset,
        }
    }

    /// Creates an Address from a virtual address.
    ///
    /// ### Arguments
    /// - `sections: &Sections` - a Sections object containing section information
    /// - `virtual_offset: u64` - virtual address
    ///
    /// ### Returns
    /// - `Self` - the Address created from the virtual address
    pub fn from_virtual_address(sections: &Sections, offset: u64) -> Self {
        // Find the section corresponding to the virtual address
        let section = sections.from_virtual_address(offset);

        Self {
            section,
            virtual_offset: offset,
        }
    }

    /// Returns the file offset.
    ///
    /// ### Returns
    /// - `Option<u64>` - the file offset
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

    /// Returns the virtual address.
    ///
    /// ### Returns
    /// - `u64` - the virtual address
    pub fn get_virtual_address(&self) -> u64 {
        self.virtual_offset
    }

    /// Returns the section information.
    ///
    /// ### Returns
    /// - `Option<Arc<Section>>` - the section info
    pub(crate) fn get_section(&self) -> Option<Arc<Section>> {
        self.section.clone()
    }
}

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
impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        debug_assert_eq!(self.section, other.section);
        self.virtual_offset.partial_cmp(&other.virtual_offset)
    }
}
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(section) = &self.section {
            write!(f, "[{}]", section)?
        }
        write!(f, "{:#X}", self.virtual_offset)
    }
}
