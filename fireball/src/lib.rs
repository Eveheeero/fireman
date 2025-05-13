#![allow(dead_code)]

pub mod arch;
pub mod core;
pub mod ir;
pub mod ir_to_c;
pub mod pe;
pub mod prelude;
#[cfg(test)]
pub mod tests;
pub mod utils;

pub use crate::core::Fire;
use crate::prelude::{trace, FireballError};

/// 모든 타입에 대한 파서를 저장하는 Enum
#[derive(Debug)]
pub enum Fireball {
    /// PE파일에 대한 파서
    Pe(pe::Pe),
}

impl Fireball {
    /// 파일 경로를 통해 파서 객체를 생성한다.
    pub fn from_path(path: &str) -> Result<Self, FireballError> {
        trace!("파일 경로 {}로 로거 생성", path);
        Ok(Fireball::Pe(pe::Pe::from_path(path)?))
    }
    pub fn from_binary(binary: Vec<u8>) -> Result<Self, FireballError> {
        Ok(Fireball::Pe(pe::Pe::from_binary(binary)?))
    }

    /// 파서 객체를 반환한다.
    pub fn get_object(&self) -> &impl Fire {
        match self {
            Self::Pe(pe) => pe,
        }
    }
}

impl Fire for Fireball {
    fn get_path(&self) -> Option<String> {
        self.get_object().get_path()
    }

    fn get_binary(&self) -> &Vec<u8> {
        self.get_object().get_binary()
    }

    fn decom_all(&self) -> Result<Vec<std::sync::Arc<core::Block>>, prelude::DecompileError> {
        self.get_object().decom_all()
    }

    fn decom_from_entry(&self) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        self.get_object().decom_from_entry()
    }

    fn decom_from_file_offset(
        &self,
        address: u64,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        self.get_object().decom_from_file_offset(address)
    }

    fn decom_from_virtual_address(
        &self,
        address: u64,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        self.get_object().decom_from_virtual_address(address)
    }

    fn decom_block(
        &self,
        address: &core::Address,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        self.get_object().decom_block(address)
    }

    fn get_sections(&self) -> std::sync::Arc<core::Sections> {
        self.get_object().get_sections()
    }

    fn get_defined(&self) -> std::sync::Arc<core::PreDefinedOffsets> {
        self.get_object().get_defined()
    }

    fn get_blocks(&self) -> std::sync::Arc<core::Blocks> {
        self.get_object().get_blocks()
    }

    fn get_relations(&self) -> std::sync::Arc<core::Relations> {
        self.get_object().get_relations()
    }
}
