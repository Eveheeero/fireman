use super::PE;
use crate::core::Fire;

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

    fn parse_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        self._parse_all()
    }

    fn parse_from_entry(&self) -> Result<(), Box<dyn std::error::Error>> {
        self._parse_from_entry()
    }

    fn parse_from_file_offset(&self, address: u64) -> Result<(), Box<dyn std::error::Error>> {
        self._parse_from_file_offset(address)
    }

    fn parse_from_virtual_address(&self, address: u64) -> Result<(), Box<dyn std::error::Error>> {
        self._parse_from_virtual_address(address)
    }
}

mod parse_all;
mod parse_from_entry;
mod parse_from_file_offset;
mod parse_from_virtual_address;
