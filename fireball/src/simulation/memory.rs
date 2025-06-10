//! Memory management for simulation
//!
//! This module provides memory allocation and access functionality
//! for the simulation engine, including support for stack, heap,
//! and mapped memory regions.

use crate::simulation::{SimulationError, SimulationResult};
use std::collections::BTreeMap;

/// Memory page size (4KB)
const PAGE_SIZE: usize = 4096;

/// Memory page containing data
#[derive(Debug, Clone)]
struct MemoryPage {
    data: Vec<u8>,
}

impl MemoryPage {
    fn new() -> Self {
        Self {
            data: vec![0; PAGE_SIZE],
        }
    }
}

/// Memory permission flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl MemoryPermissions {
    /// Read-only memory
    pub const READ: Self = Self {
        read: true,
        write: false,
        execute: false,
    };

    /// Read-write memory
    pub const READ_WRITE: Self = Self {
        read: true,
        write: true,
        execute: false,
    };

    /// Read-execute memory
    pub const READ_EXECUTE: Self = Self {
        read: true,
        write: false,
        execute: true,
    };

    /// Read-write-execute memory
    pub const ALL: Self = Self {
        read: true,
        write: true,
        execute: true,
    };
}

/// Memory region with associated permissions
#[derive(Debug, Clone)]
struct MemoryRegion {
    start: u64,
    end: u64,
    permissions: MemoryPermissions,
    name: String,
}

/// Memory management for simulation
#[derive(Debug, Clone)]
pub struct Memory {
    /// Memory pages indexed by page number (address / PAGE_SIZE)
    pages: BTreeMap<u64, MemoryPage>,
    /// Memory regions with permissions
    regions: Vec<MemoryRegion>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Create a new memory instance
    pub fn new() -> Self {
        let mut memory = Self {
            pages: BTreeMap::new(),
            regions: Vec::new(),
        };

        // Initialize default memory regions
        memory.init_default_regions();

        memory
    }

    /// Initialize default memory regions for x86_64
    fn init_default_regions(&mut self) {
        // Stack region (8MB)
        self.add_region(
            0x7fff_f800_0000,
            0x7fff_ffff_ffff,
            MemoryPermissions::READ_WRITE,
            "stack".to_string(),
        );

        // Heap region (placeholder)
        self.add_region(
            0x0000_0040_0000,
            0x0000_0080_0000,
            MemoryPermissions::READ_WRITE,
            "heap".to_string(),
        );

        // Code region (placeholder)
        self.add_region(
            0x0000_0040_0000,
            0x0000_0050_0000,
            MemoryPermissions::READ_EXECUTE,
            "code".to_string(),
        );
    }

    /// Add a memory region
    pub fn add_region(
        &mut self,
        start: u64,
        end: u64,
        permissions: MemoryPermissions,
        name: String,
    ) {
        self.regions.push(MemoryRegion {
            start,
            end,
            permissions,
            name,
        });
    }

    /// Check if an address range has the required permissions
    fn check_permissions(
        &self,
        address: u64,
        size: usize,
        need_read: bool,
        need_write: bool,
        need_execute: bool,
    ) -> SimulationResult<()> {
        let end_address = address
            .checked_add(size as u64 - 1)
            .ok_or(SimulationError::MemoryAccessViolation { address, size })?;

        // Find the region containing this address range
        for region in &self.regions {
            if address >= region.start && end_address <= region.end {
                if (need_read && !region.permissions.read)
                    || (need_write && !region.permissions.write)
                    || (need_execute && !region.permissions.execute)
                {
                    return Err(SimulationError::MemoryAccessViolation { address, size });
                }
                return Ok(());
            }
        }

        // No region found - allow access for now (TODO: make this configurable)
        Ok(())
    }

    /// Read memory at the given address
    pub fn read(&self, address: u64, size: usize) -> SimulationResult<Vec<u8>> {
        self.check_permissions(address, size, true, false, false)?;

        let mut result = Vec::with_capacity(size);

        for i in 0..size {
            let byte_addr = address + i as u64;
            let page_num = byte_addr / PAGE_SIZE as u64;
            let page_offset = (byte_addr % PAGE_SIZE as u64) as usize;

            let byte = self
                .pages
                .get(&page_num)
                .map(|page| page.data[page_offset])
                .unwrap_or(0);

            result.push(byte);
        }

        Ok(result)
    }

    /// Write memory at the given address
    pub fn write(&mut self, address: u64, data: &[u8]) -> SimulationResult<()> {
        self.check_permissions(address, data.len(), false, true, false)?;

        for (i, &byte) in data.iter().enumerate() {
            let byte_addr = address + i as u64;
            let page_num = byte_addr / PAGE_SIZE as u64;
            let page_offset = (byte_addr % PAGE_SIZE as u64) as usize;

            let page = self.pages.entry(page_num).or_insert_with(MemoryPage::new);
            page.data[page_offset] = byte;
        }

        Ok(())
    }

    /// Read a u8 from memory
    pub fn read_u8(&self, address: u64) -> SimulationResult<u8> {
        let bytes = self.read(address, 1)?;
        Ok(bytes[0])
    }

    /// Read a u16 from memory (little-endian)
    pub fn read_u16(&self, address: u64) -> SimulationResult<u16> {
        let bytes = self.read(address, 2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    /// Read a u32 from memory (little-endian)
    pub fn read_u32(&self, address: u64) -> SimulationResult<u32> {
        let bytes = self.read(address, 4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Read a u64 from memory (little-endian)
    pub fn read_u64(&self, address: u64) -> SimulationResult<u64> {
        let bytes = self.read(address, 8)?;
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    /// Write a u8 to memory
    pub fn write_u8(&mut self, address: u64, value: u8) -> SimulationResult<()> {
        self.write(address, &[value])
    }

    /// Write a u16 to memory (little-endian)
    pub fn write_u16(&mut self, address: u64, value: u16) -> SimulationResult<()> {
        self.write(address, &value.to_le_bytes())
    }

    /// Write a u32 to memory (little-endian)
    pub fn write_u32(&mut self, address: u64, value: u32) -> SimulationResult<()> {
        self.write(address, &value.to_le_bytes())
    }

    /// Write a u64 to memory (little-endian)
    pub fn write_u64(&mut self, address: u64, value: u64) -> SimulationResult<()> {
        self.write(address, &value.to_le_bytes())
    }

    /// Allocate memory on the stack (decrements RSP)
    pub fn allocate_stack(&mut self, current_rsp: u64, size: u64) -> SimulationResult<u64> {
        let new_rsp = current_rsp
            .checked_sub(size)
            .ok_or(SimulationError::StackOverflow)?;

        // Ensure the stack memory is accessible
        self.check_permissions(new_rsp, size as usize, false, true, false)?;

        Ok(new_rsp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_read_write() {
        let mut memory = Memory::new();

        // Test writing and reading
        let address = 0x1000;
        memory.write_u32(address, 0x12345678).unwrap();
        assert_eq!(memory.read_u32(address).unwrap(), 0x12345678);

        // Test individual bytes
        assert_eq!(memory.read_u8(address).unwrap(), 0x78);
        assert_eq!(memory.read_u8(address + 1).unwrap(), 0x56);
        assert_eq!(memory.read_u8(address + 2).unwrap(), 0x34);
        assert_eq!(memory.read_u8(address + 3).unwrap(), 0x12);
    }

    #[test]
    fn test_memory_regions() {
        let mut memory = Memory::new();

        // Stack region should be writable
        let stack_addr = 0x7fff_ff00_0000;
        memory.write_u64(stack_addr, 0xdeadbeef).unwrap();
        assert_eq!(memory.read_u64(stack_addr).unwrap(), 0xdeadbeef);
    }

    #[test]
    fn test_cross_page_access() {
        let mut memory = Memory::new();

        // Write across page boundary
        let address = PAGE_SIZE as u64 - 2;
        memory.write_u32(address, 0x12345678).unwrap();
        assert_eq!(memory.read_u32(address).unwrap(), 0x12345678);
    }
}
