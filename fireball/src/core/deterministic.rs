//! Deterministic infrastructure for the Fireman decompiler
//!
//! CRITICAL: This module ensures absolute determinism - same input ALWAYS produces same output
//! No HashMap, no time-based operations, no random values, no platform-specific behavior

use super::Address;
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};
// Use the existing Address type from core

/// Deterministic temporary variable naming
/// Format: purpose.address.counter
pub struct DeterministicNamer {
    /// Counters per (address, purpose) pair - using BTreeMap for determinism
    counters: BTreeMap<(Address, &'static str), u32>,
}

impl Default for DeterministicNamer {
    fn default() -> Self {
        Self::new()
    }
}

impl DeterministicNamer {
    pub fn new() -> Self {
        Self {
            counters: BTreeMap::new(),
        }
    }

    /// Generate a new temporary name
    /// Format: purpose.address.counter (e.g., "load.0000000000401000.0")
    pub fn new_temp(&mut self, addr: Address, purpose: &'static str) -> String {
        let counter = self.counters.entry((addr.clone(), purpose)).or_insert(0);
        let current = *counter;
        *counter += 1;

        // Fixed format: purpose.address.counter
        format!("{}.{}.{}", purpose, addr.as_deterministic_string(), current)
    }

    /// CRITICAL: Reset for each function to ensure isolation
    pub fn reset(&mut self) {
        self.counters.clear();
    }
}

/// Deterministic temporary allocator for IR generation
pub struct TempAllocator {
    counters: BTreeMap<(Address, &'static str), u32>,
}

impl Default for TempAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl TempAllocator {
    pub fn new() -> Self {
        Self {
            counters: BTreeMap::new(),
        }
    }

    /// Create a new LocalId with deterministic naming
    pub fn new_local(&mut self, addr: Address, purpose: &'static str) -> LocalId {
        let key = (addr.clone(), purpose);
        let index = self.counters.entry(key).or_insert(0);
        let current = *index;
        *index += 1;

        LocalId {
            source: addr,
            purpose,
            index: current,
            version: 0, // Will be set during SSA construction
        }
    }

    /// MUST reset between functions
    pub fn reset(&mut self) {
        self.counters.clear();
    }
}

/// Local SSA variable with deterministic naming
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalId {
    /// Source instruction address (primary key)
    pub source: Address,
    /// Purpose/type (e.g., "load", "addr", "result")
    pub purpose: &'static str,
    /// Index for same purpose at same address
    pub index: u32,
    /// SSA version (assigned during SSA construction)
    pub version: u32,
}

/// Deterministic ordering: by address, then purpose, then index, then version
impl Ord for LocalId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source
            .cmp(&other.source)
            .then_with(|| self.purpose.cmp(other.purpose))
            .then_with(|| self.index.cmp(&other.index))
            .then_with(|| self.version.cmp(&other.version))
    }
}

impl PartialOrd for LocalId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for LocalId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Deterministic format: %purpose_address_index.version
        if self.version == 0 {
            write!(
                f,
                "%{}_{}_{}",
                self.purpose,
                self.source.as_deterministic_string(),
                self.index
            )
        } else {
            write!(
                f,
                "%{}_{}_{}.{}",
                self.purpose,
                self.source.as_deterministic_string(),
                self.index,
                self.version
            )
        }
    }
}

/// Block identifier based on address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub Address);

impl Display for BlockId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "block_{}", self.0)
    }
}

/// Function identifier based on address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionId {
    pub address: Address,
}

impl Display for FunctionId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "func_{}", self.address)
    }
}

/// Global variable identifier
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalId {
    pub address: Address,
    pub name: Option<String>,
}

impl Display for GlobalId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "@{}", name)
        } else {
            write!(f, "@global_{}", self.address)
        }
    }
}

/// Canonicalize commutative operations for determinism
pub fn canonicalize_commutative<T: Ord>(op: BinaryOp, lhs: T, rhs: T) -> (T, T) {
    if op.is_commutative() && lhs > rhs {
        (rhs, lhs) // Swap to maintain canonical order
    } else {
        (lhs, rhs)
    }
}

/// Binary operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    // Arithmetic (canonical order for commutative ops)
    Add,
    Sub,
    Mul,
    SDiv,
    UDiv,
    SRem,
    URem,
    // Bitwise (canonical order)
    And,
    Or,
    Xor,
    Shl,
    LShr,
    AShr,
    // Comparison (not commutative)
    Eq,
    Ne,
    Slt,
    Sle,
    Sgt,
    Sge,
    Ult,
    Ule,
    Ugt,
    Uge,
}

impl BinaryOp {
    pub fn is_commutative(&self) -> bool {
        matches!(
            self,
            BinaryOp::Add
                | BinaryOp::Mul
                | BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::Xor
                | BinaryOp::Eq
                | BinaryOp::Ne
        )
    }
}

// Note: DeterminismValidator removed to avoid external dependencies
// TODO: Add determinism validation with serde feature flag

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Sections;

    // Helper to create a test address
    fn test_addr(va: u64) -> Address {
        // Create a dummy sections object for testing
        let sections = Sections::new();
        Address::from_virtual_address(&sections, va)
    }

    #[test]
    fn test_deterministic_namer() {
        let mut namer = DeterministicNamer::new();

        let addr1 = test_addr(0x401000);
        let addr2 = test_addr(0x401000);

        // Same address and purpose should increment
        assert_eq!(
            namer.new_temp(addr1.clone(), "load"),
            "load.0000000000401000.0"
        );
        assert_eq!(
            namer.new_temp(addr2.clone(), "load"),
            "load.0000000000401000.1"
        );
        assert_eq!(
            namer.new_temp(addr1.clone(), "store"),
            "store.0000000000401000.0"
        );

        // Reset should clear counters
        namer.reset();
        assert_eq!(namer.new_temp(addr1, "load"), "load.0000000000401000.0");
    }

    #[test]
    fn test_address_formatting() {
        assert_eq!(
            test_addr(0x401000).as_deterministic_string(),
            "0000000000401000"
        );
        assert_eq!(
            test_addr(0xFFFFFFFFFFFFFFFF).as_deterministic_string(),
            "ffffffffffffffff"
        );
        assert_eq!(test_addr(0).as_deterministic_string(), "0000000000000000");
    }

    #[test]
    fn test_local_id_ordering() {
        let id1 = LocalId {
            source: test_addr(0x401000),
            purpose: "load",
            index: 0,
            version: 0,
        };

        let id2 = LocalId {
            source: test_addr(0x401000),
            purpose: "load",
            index: 1,
            version: 0,
        };

        let id3 = LocalId {
            source: test_addr(0x401001),
            purpose: "load",
            index: 0,
            version: 0,
        };

        assert!(id1 < id2); // Same address/purpose, different index
        assert!(id1 < id3); // Different address
        assert!(id2 < id3); // Address takes precedence
    }

    #[test]
    fn test_canonicalize_commutative() {
        let a = 5;
        let b = 10;

        // Commutative operation should swap if needed
        let (x, y) = canonicalize_commutative(BinaryOp::Add, b, a);
        assert_eq!((x, y), (5, 10)); // Smaller first

        // Non-commutative should not swap
        let (x, y) = canonicalize_commutative(BinaryOp::Sub, b, a);
        assert_eq!((x, y), (10, 5)); // Original order
    }
}
