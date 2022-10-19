use crate::core::{Section, SECTIONS};

use goblin::Object;

pub(crate) struct Address {
    section: Section,
    virtual_offset: u64,
}

impl Address {
    pub(crate) fn from_file_offset(binary: &[u8], offset: u64) -> Self {
        let gl = Object::parse(binary).unwrap();
        let mut name: String = Default::default();
        let mut base_addr: u64 = Default::default();

        match gl {
            Object::PE(gl) => {
                let sections = gl.sections;
                for section in sections {
                    let section_start = section.pointer_to_raw_data as u64;
                    let section_end =
                        section.pointer_to_raw_data as u64 + section.size_of_raw_data as u64;
                    if offset >= section_start && offset < section_end {
                        name = section.name().unwrap().to_string();
                        base_addr = section.virtual_address as u64;
                        break;
                    }
                }
            }
            _ => todo!(),
        };

        Address {
            section: Section { name, base_addr },
            virtual_offset: offset,
        }
    }

    pub(crate) fn from_virtual_address(binary: &[u8], offset: u64) -> Self {
        let gl = Object::parse(binary).unwrap();
        let mut name: String = Default::default();
        let mut base_addr: u64 = Default::default();

        match gl {
            Object::PE(gl) => {
                let sections = gl.sections;
                for section in sections {
                    if section.virtual_address as u64 <= offset
                        && offset <= (section.virtual_address + section.virtual_size) as u64
                    {
                        name = section.name().unwrap().to_string();
                        base_addr = section.virtual_address as u64;
                        break;
                    }
                }
            }
            _ => todo!(),
        };

        Address {
            section: Section { name, base_addr },
            virtual_offset: offset,
        }
    }

    pub(crate) fn get_file_offset(&self) -> u64 {
        let virtual_offset = self.virtual_offset;
        let base_addr = self.section.base_addr;
        let offset = virtual_offset - base_addr;
        offset
    }

    pub(crate) fn get_virtual_address(&self) -> u64 {
        self.virtual_offset
    }

    pub(crate) fn get_section(&self) -> Section {
        self.section.clone()
    }
}
