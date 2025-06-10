//! Alternative design: AddressKey for deterministic ordering
//!
//! This design keeps the original Address Ord implementation unchanged
//! but provides an AddressKey type for use in maps and sets where
//! section-aware ordering is needed.

use crate::core::Address;
use std::sync::Arc;

/// A key type that provides deterministic ordering for addresses
/// considering both section and virtual address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddressKey {
    /// Section name (or "none" if no section)
    section_name: String,
    /// Virtual address
    virtual_address: u64,
}

impl AddressKey {
    /// Create a key from an Address
    pub fn from_address(addr: &Address) -> Self {
        let section_name = addr
            .get_section()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "none".to_string());

        Self {
            section_name,
            virtual_address: addr.get_virtual_address(),
        }
    }

    /// Get a deterministic string representation
    pub fn as_string(&self) -> String {
        format!("{}:{:016x}", self.section_name, self.virtual_address)
    }
}

impl PartialOrd for AddressKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AddressKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare section names
        match self.section_name.cmp(&other.section_name) {
            std::cmp::Ordering::Equal => {
                // Then compare virtual addresses
                self.virtual_address.cmp(&other.virtual_address)
            }
            ord => ord,
        }
    }
}

/// Extension trait for Address to get deterministic keys
pub trait AddressDeterministic {
    fn to_key(&self) -> AddressKey;
}

impl AddressDeterministic for Address {
    fn to_key(&self) -> AddressKey {
        AddressKey::from_address(self)
    }
}

/// Wrapper for using Address in BTreeMap/BTreeSet with section-aware ordering
#[derive(Debug, Clone)]
pub struct DeterministicAddress(pub Address);

impl PartialEq for DeterministicAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_key() == other.0.to_key()
    }
}

impl Eq for DeterministicAddress {}

impl PartialOrd for DeterministicAddress {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DeterministicAddress {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.to_key().cmp(&other.0.to_key())
    }
}

impl std::hash::Hash for DeterministicAddress {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_key().hash(state);
    }
}

/// Usage example:
/// ```ignore
/// use std::collections::BTreeMap;
///
/// // When you need section-aware ordering:
/// let mut map: BTreeMap<AddressKey, Value> = BTreeMap::new();
/// map.insert(address.to_key(), value);
///
/// // Or use the wrapper:
/// let mut map: BTreeMap<DeterministicAddress, Value> = BTreeMap::new();
/// map.insert(DeterministicAddress(address), value);
/// ```

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Section;
    use std::collections::BTreeMap;

    #[test]
    fn test_address_key_ordering() {
        // Create test addresses with same virtual address but different sections
        let addr1 = Address {
            section: Some(Arc::new(Section {
                name: ".text".to_string(),
                virtual_address: 0x1000,
                file_offset: 0x400,
                size_of_file: 0x1000,
                size_of_memory: 0x1000,
                characteristics: 0,
            })),
            virtual_offset: 0x1000,
        };

        let addr2 = Address {
            section: Some(Arc::new(Section {
                name: ".data".to_string(),
                virtual_address: 0x1000,
                file_offset: 0x1400,
                size_of_file: 0x1000,
                size_of_memory: 0x1000,
                characteristics: 0,
            })),
            virtual_offset: 0x1000,
        };

        // Convert to keys
        let key1 = addr1.to_key();
        let key2 = addr2.to_key();

        // They should be different
        assert_ne!(key1, key2);

        // Can be used in BTreeMap
        let mut map: BTreeMap<AddressKey, &str> = BTreeMap::new();
        map.insert(key1, "text section");
        map.insert(key2, "data section");

        assert_eq!(map.len(), 2);
    }
}
