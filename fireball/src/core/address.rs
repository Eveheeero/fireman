use std::sync::Arc;

use crate::core::{Section, Sections};

use goblin::Object;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Address {
    section: Option<Arc<Section>>,
    virtual_offset: u64,
}

impl Address {
    /// 파일 오프셋을 기반으로 Address 객체를 생성한다.
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
    pub(crate) fn from_virtual_address(sections: &Sections, offset: u64) -> Self {
        // 가상주소에 해당하는 섹션 찾기
        let section = sections.from_virtual_address(offset);

        Self {
            section,
            virtual_offset: offset,
        }
    }

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

    pub(crate) fn get_virtual_address(&self) -> u64 {
        self.virtual_offset
    }

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
