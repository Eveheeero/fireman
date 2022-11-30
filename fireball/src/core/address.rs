use std::sync::Arc;

use crate::core::{Section, Sections};

/// 타겟 프로그램 내부에서 사용되는 주소에 대한 정보를 담고있는 구조체
///
/// 해당 구조체는 여러 섹션 정보로부터, 주어진 오프셋이 어느 위치에 있으며, 파일오프셋은 얼마인지 등을 계산해준다.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Address {
    /// Address객체가 어느 섹션에 속하는지를 나타내는 섹션 정보
    section: Arc<Section>,
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
    /// - `Result<Self, ()>` - 섹션 정보를 찾을 수 없는 경우 에러를 반환한다.
    pub(crate) fn from_file_offset(sections: &Sections, offset: u64) -> Result<Self, ()> {
        /* 오프셋에 해당하는 섹션 찾기 */
        let section = match sections.from_file_offset(offset) {
            Some(section) => section,
            None => return Err(()),
        };

        /* 섹션정보를 기반으로 가상주소 연산 */
        let virtual_offset = offset - section.file_offset + section.virtual_address;

        /* 객체 생성 및 반환 */
        Ok(Self {
            section,
            virtual_offset,
        })
    }

    /// 가상 주소를 기반으로 Address 객체를 생성한다.
    ///
    /// ### Arguments
    /// - `sections: &Sections` - 섹션 정보를 담고있는 Sections 객체
    /// - `virtual_offset: u64` - 가상 주소
    ///
    /// ### Returns
    /// - `Result<Self, ()>` - 섹션 정보를 찾을 수 없는 경우 에러를 반환한다.
    ///
    /// ### Todo
    /// - 섹션 정보를 찾을 수 없는 경우 에러를 반환하는 것이 아닌, None을 반환하도록 수정해야 한다.
    pub(crate) fn from_virtual_address(sections: &Sections, offset: u64) -> Result<Self, ()> {
        /* 가상주소에 해당하는 섹션 찾기 */
        let section = match sections.from_virtual_address(offset) {
            Some(section) => section,
            None => return Err(()),
        };

        /* 객체 생성 및 값 반환 */
        Ok(Self {
            section,
            virtual_offset: offset,
        })
    }

    /// 파일 오프셋을 반환한다.
    ///
    /// ### Returns
    /// - `u64` - 파일 오프셋
    ///
    /// ### Todo
    /// - 파일 오프셋이 존재하지 않을 수 있으므로 파일 오프셋을 반환하는 것이 아닌, Option<u64>을 반환하도록 수정해야 한다.
    /// - 다음 섹션에 대한 범위 체크를 진행해야 한다. (기본 바이너리에 없는 가상 주소를 참조하는 경우, None을 반환해야 한다.)
    pub(crate) fn get_file_offset(&self) -> u64 {
        let virtual_offset = self.virtual_offset;
        let section_virtual_offset_start = self.section.virtual_address;
        let section_file_offset_start = self.section.file_offset;
        (virtual_offset - section_virtual_offset_start) + section_file_offset_start
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
    /// - `Arc<Section>` - 섹션 정보
    pub(crate) fn get_section(&self) -> Arc<Section> {
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
