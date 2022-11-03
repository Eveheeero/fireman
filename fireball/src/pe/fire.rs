use std::sync::Arc;

use super::PE;
use crate::core::{Address, Block, Fire, Relation, Sections};

impl Fire for PE {
    fn from_path(path: &str) -> Result<PE, Box<dyn std::error::Error>> {
        let binary = std::fs::read(path)?;
        Ok(PE::new(Some(path.to_owned()), binary))
    }

    fn from_binary(binary: Vec<u8>) -> Result<PE, Box<dyn std::error::Error>> {
        Ok(PE::new(None, binary))
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_binary(&self) -> &Vec<u8> {
        &self.binary
    }

    fn decom_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        self._decom_all()
    }

    fn decom_from_entry(&self) -> Result<(), Box<dyn std::error::Error>> {
        self._decom_from_entry()
    }

    fn decom_from_file_offset(&self, address: u64) -> Result<(), Box<dyn std::error::Error>> {
        self._decom_from_file_offset(address)
    }

    fn decom_from_virtual_address(&self, address: u64) -> Result<(), Box<dyn std::error::Error>> {
        self._decom_from_virtual_address(address)
    }

    fn get_sections(&self) -> Arc<Sections> {
        self.sections.clone()
    }

    fn parse_block(&self, address: Address) -> (Arc<Block>, Option<Arc<Relation>>) {
        self._parse_block(address)
    }
}

mod decom_all;
mod decom_from_entry;
mod decom_from_file_offset;
mod decom_from_virtual_address;
mod parse_block;
