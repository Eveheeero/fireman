use std::sync::Arc;

use crate::core::Section;

use goblin::Object;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) struct Address {
    section: Arc<Section>,
    virtual_offset: u64,
}

impl Address {
    /// 파일 오프셋을 기반으로 Address 객체를 생성한다.
    pub(crate) fn from_file_offset(offset: u64) -> Result<Self, ()> {
        // 오프셋에 해당하는 섹션 찾기
        let section = match Section::from_file_offset(offset) {
            Some(section) => section,
            None => return Err(()),
        };
        // 섹션정보를 기반으로 가상주소 연산
        let virtual_offset = offset - section.file_offset + section.virtual_address;

        Ok(Self {
            section,
            virtual_offset,
        })
    }

    /// 가상 주소를 기반으로 Address 객체를 생성한다.
    pub(crate) fn from_virtual_address(offset: u64) -> Result<Self, ()> {
        // 가상주소에 해당하는 섹션 찾기
        let section = match Section::from_virtual_address(offset) {
            Some(section) => section,
            None => return Err(()),
        };

        Ok(Self {
            section,
            virtual_offset: offset,
        })
    }

    pub(crate) fn get_file_offset(&self) -> u64 {
        let virtual_offset = self.virtual_offset;
        let section_virtual_offset_start = self.section.virtual_address;
        let section_file_offset_start = self.section.file_offset;
        (virtual_offset - section_virtual_offset_start) + section_file_offset_start
    }

    pub(crate) fn get_virtual_address(&self) -> u64 {
        self.virtual_offset
    }

    pub(crate) fn get_section(&self) -> Arc<Section> {
        self.section.clone()
    }
}
