//! Module defining `PreDefinedOffset`, a struct holding information about
//! pre-defined address-to-name mappings inside the program.

use super::Address;

/// Struct that stores pre-defined address information inside the file,
/// e.g., mapping an address to a function start.
pub struct PreDefinedOffset {
    /// File offset inside the binary
    pub(crate) address: Address,
    /// The name associated with the address inside the binary
    pub(crate) name: String,
}
