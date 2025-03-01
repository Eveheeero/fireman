//! 코어 트레이트에 대한 구현이 담겨있는 모듈

mod decom_all;
mod decom_from_entry;
mod decom_from_file_offset;
mod decom_from_virtual_address;
mod decom_function;

use super::PE;
use crate::{
    core::{Fire, Sections},
    prelude::{DecompileError, IoError},
};
use std::sync::Arc;

impl Fire for PE {
    fn from_path(path: &str) -> Result<PE, IoError> {
        let binary = std::fs::read(path)?;
        Ok(PE::new(Some(path.to_owned()), binary))
    }

    fn from_binary(binary: Vec<u8>) -> Result<PE, IoError> {
        Ok(PE::new(None, binary))
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_binary(&self) -> &Vec<u8> {
        &self.binary
    }

    fn decom_all(&self) -> Result<(), DecompileError> {
        self._decom_all()
    }

    fn decom_from_entry(&self) -> Result<(), DecompileError> {
        self._decom_from_entry()
    }

    fn decom_from_file_offset(&self, address: u64) -> Result<(), DecompileError> {
        self._decom_from_file_offset(address)
    }

    fn decom_from_virtual_address(&self, address: u64) -> Result<(), DecompileError> {
        self._decom_from_virtual_address(address)
    }

    fn get_sections(&self) -> Arc<Sections> {
        self.sections.clone()
    }
}
