//! 프로그램 내부의 가상주소 혹은 파일 오프셋에 대한 정보를 담고있는 구조체인
//! "Address"를 정의하는 모듈
//!
//! # Note
//! - 주소에 대한 정보는 저장해 둘 필요가 없기 때문에 Addresses는 존재하지 않습니다.

use crate::core::{Section, Sections};
use std::sync::Arc;

/// 타겟 프로그램 내부에서 사용되는 주소에 대한 정보를 담고있는 구조체
///
/// 해당 구조체는 여러 섹션 정보로부터, 주어진 오프셋이 어느 위치에 있으며, 파일오프셋은 얼마인지 등을 계산해준다.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Address {
    /// Address객체가 어느 섹션에 속하는지를 나타내는 섹션 정보
    section: Option<Arc<Section>>,
    /// Address객체의 가상 주소
    ///
    /// ### Note
    /// - 파일 오프셋이 표현하지 못하는 범위를 가상주소가 표현할 수 있기 때문에, 가상주소를 사용한다.
    virtual_offset: u64,
}

impl Address {
    /// 파일 오프셋을 기반으로 Address 객체를 생성한다.
    ///
    /// ### Arguments
    /// - `sections: &Sections` - 섹션 정보를 담고있는 Sections 객체
    /// - `offset: u64` - 파일 오프셋
    ///
    /// ### Returns
    /// - `Self` - 파일 오프셋으로부터 생성된 Address 객체
    pub(crate) fn from_file_offset(sections: &Sections, offset: u64) -> Self {
        // 오프셋에 해당하는 섹션 찾기
        let section = sections.from_file_offset(offset);
        // 섹션정보를 기반으로 가상주소 연산
        // 파일 오프셋에는 항상 섹션이 존재하기때문에 unwrap()을 사용해도 무방하다.
        let virtual_offset = offset - section.as_ref().unwrap().file_offset
            + section.as_ref().unwrap().virtual_address;

        Self {
            section,
            virtual_offset,
        }
    }

    /// 가상 주소를 기반으로 Address 객체를 생성한다.
    ///
    /// ### Arguments
    /// - `sections: &Sections` - 섹션 정보를 담고있는 Sections 객체
    /// - `virtual_offset: u64` - 가상 주소
    ///
    /// ### Returns
    /// - `Self` - 가상 주소로부터 생성된 Address 객체
    pub(crate) fn from_virtual_address(sections: &Sections, offset: u64) -> Self {
        // 가상주소에 해당하는 섹션 찾기
        let section = sections.from_virtual_address(offset);

        Self {
            section,
            virtual_offset: offset,
        }
    }

    /// 파일 오프셋을 반환한다.
    ///
    /// ### Returns
    /// - `Option<u64>` - 파일 오프셋
    pub(crate) fn get_file_offset(&self) -> Option<u64> {
        if let Some(section) = &self.section {
            if self.virtual_offset - section.virtual_address > section.size_of_file {
                return None;
            }
            let virtual_offset = self.virtual_offset;
            let section_virtual_offset_start = section.virtual_address;
            let section_file_offset_start = section.file_offset;
            Some((virtual_offset - section_virtual_offset_start) + section_file_offset_start)
        } else {
            None
        }
    }

    /// 가상 주소를 반환한다.
    ///
    /// ### Returns
    /// - `u64` - 가상 주소
    pub(crate) fn get_virtual_address(&self) -> u64 {
        self.virtual_offset
    }

    /// 섹션 정보를 반환한다.
    ///
    /// ### Returns
    /// - `Option<Arc<Section>>` - 섹션 정보
    pub(crate) fn get_section(&self) -> Option<Arc<Section>> {
        self.section.clone()
    }
}

impl std::ops::AddAssign<u64> for Address {
    fn add_assign(&mut self, rhs: u64) {
        self.virtual_offset += rhs;
    }
}

impl std::ops::Add<u64> for Address {
    type Output = Self;
    fn add(mut self, rhs: u64) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::Add<u64> for &Address {
    type Output = Address;
    fn add(self, rhs: u64) -> Self::Output {
        let mut new_address = self.clone();
        new_address += rhs;
        new_address
    }
}

impl std::ops::SubAssign<u64> for Address {
    fn sub_assign(&mut self, rhs: u64) {
        self.virtual_offset -= rhs;
    }
}
impl std::ops::Sub<u64> for Address {
    type Output = Self;
    fn sub(mut self, rhs: u64) -> Self::Output {
        self -= rhs;
        self
    }
}
impl std::ops::Sub<u64> for &Address {
    type Output = Address;
    fn sub(self, rhs: u64) -> Self::Output {
        let mut new_address = self.clone();
        new_address -= rhs;
        new_address
    }
}
impl std::ops::Sub<&Address> for &Address {
    type Output = u64;
    fn sub(self, rhs: &Address) -> Self::Output {
        self.virtual_offset - rhs.virtual_offset
    }
}
impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        debug_assert_eq!(self.section, other.section);
        self.virtual_offset.partial_cmp(&other.virtual_offset)
    }
}
