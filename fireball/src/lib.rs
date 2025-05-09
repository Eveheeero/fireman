#![allow(dead_code)]

pub mod arch;
pub mod core;
pub mod ir;
pub mod pe;
pub mod prelude;
#[cfg(test)]
mod tests;
pub mod utils;

use crate::core::Fire;
use crate::prelude::{trace, FireballError};

/// 모든 타입에 대한 파서를 저장하는 Enum
#[derive(Debug)]
pub enum Fireball {
    /// PE파일에 대한 파서
    PE(pe::Pe),
}

impl Fireball {
    /// 파일 경로를 통해 파서 객체를 생성한다.
    pub fn from_path(path: &str) -> Result<Self, FireballError> {
        trace!("파일 경로 {}로 로거 생성", path);
        Ok(Fireball::PE(pe::Pe::from_path(path)?))
    }

    /// 파서 객체를 반환한다.
    pub fn get_object(&self) -> &impl core::Fire {
        match self {
            Self::PE(pe) => pe,
        }
    }
}
