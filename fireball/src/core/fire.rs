//! Module defining the `Fire` struct responsible for common decompilation functionality

use crate::prelude::DecompileError;

/// This trait defines the necessary functions provided by this library when implementing a decompiler.
pub trait Fire {
    /// Returns the file path.
    ///
    /// ### Returns
    /// - `Option<String>` - the file path
    ///
    /// ### Note
    /// - This function performs no additional computation.
    fn get_path(&self) -> Option<String>;
    /// Returns the binary data.
    ///
    /// ### Returns
    /// - `&Vec<u8>` - the raw file binary data
    ///
    /// ### Note
    /// - This function performs no additional computation.
    fn get_binary(&self) -> &Vec<u8>;
    /// Decompiles the entire binary.
    ///     
    /// ### Returns
    /// - `Result<String, DecompileError>` - the decompiled code
    fn decompile_all(&self) -> Result<String, DecompileError>;
    /// Decompiles the binary starting from the entry point.
    ///
    /// ### Returns
    /// - `Result<String, DecompileError>` - the decompiled code
    fn decompile_from_entry(&self) -> Result<String, DecompileError>;
    /// Decompiles a block from a given file offset.
    ///
    /// ### Arguments
    /// - `address: u64` - the file offset to start decompilation
    ///
    /// ### Returns
    /// - `Result<String, DecompileError>` - the decompiled code
    fn decompile_from_file_offset(&self, address: u64) -> Result<String, DecompileError>;
    /// Decompiles a block from a given virtual address.
    ///
    /// ### Arguments
    /// - `address: u64` - the virtual address to start decompilation
    ///
    /// ### Returns
    /// - `Result<String, DecompileError>` - the decompiled code
    fn decompile_from_virtual_address(&self, address: u64) -> Result<String, DecompileError>;
}
