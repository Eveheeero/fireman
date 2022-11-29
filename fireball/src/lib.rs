/// 코어 트레이트가 담겨있는 모듈
pub mod core;
use crate::core::Fire;

/// PE파일에 대한 구조체가 담겨있는 모듈
pub mod pe;

/// 공통적으로 사용되는 기능이 담겨있는 모듈
pub mod utils;

/// 기본적으로 사용되는 use문이 들어가는 모듈
pub(crate) mod prelude;
use crate::prelude::FireballError;

/// 모든 타입에 대한 파서를 저장하는 Enum
#[derive(Debug)]
pub enum Fireball {
    /// PE파일에 대한 파서
    PE(pe::PE),
}

impl Fireball {
    /// 파일 경로를 통해 파서 객체를 생성한다.
    pub fn from_path(path: &str) -> Result<Self, FireballError> {
        Ok(Fireball::PE(pe::PE::from_path(path)?))
    }

    /// 파서 객체를 반환한다.
    pub fn get_object(&self) -> &dyn core::Fire {
        match self {
            Fireball::PE(pe) => pe,
        }
    }
}

/// 테스트 모듈
#[cfg(test)]
mod tests;
