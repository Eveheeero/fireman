use crate::core::{Section, SECTIONS};

use goblin::Object;

pub(crate) struct Address {
    address_virtual_offset: usize,

    section: Section,
}

impl Address {
    pub(crate) fn from_file_offset(binary: &Vec<u8>, offset: usize) -> Self {
        let gl = Object::parse(binary).unwrap();
        let mut name: String = Default::default();
        let mut base_addr: usize = Default::default();

        match gl {
            Object::PE(gl) => {
                let sections = gl.sections;
                for section in sections {
                    let section_start = section.pointer_to_raw_data as usize;
                    let section_end =
                        section.pointer_to_raw_data as usize + section.size_of_raw_data as usize;
                    if offset >= section_start && offset < section_end {
                        // name = section.name.to_string();
                        name = String::new();
                        base_addr = section.virtual_address as usize;
                        break;
                    }
                }
            }
            _ => todo!(),
        };

        Address {
            address_virtual_offset: offset,
            section: Section { name, base_addr },
        }
    }

    pub(crate) fn from_virtual_address(binary: &Vec<u8>, offset: usize) -> Self {
        let gl = Object::parse(binary).unwrap();
        let mut name: String = Default::default();
        let mut base_addr: usize = Default::default();

        match gl {
            Object::PE(gl) => {
                let sections = gl.sections;
                for section in sections {
                    if section.virtual_address as usize <= offset
                        && offset <= (section.virtual_address + section.virtual_size) as usize
                    {
                        // name = section.name.to_owned();
                        name = String::new();
                        base_addr = section.virtual_address as usize;
                        break;
                    }
                }
            }
            _ => todo!(),
        };

        Address {
            address_virtual_offset: offset,
            section: Section { name, base_addr },
        }
    }

    pub(crate) fn get_file_offset(&self) -> usize {
        let virtual_offset = self.address_virtual_offset;
        let base_addr = self.section.base_addr;
        let offset = virtual_offset - base_addr;
        offset
    }

    pub(crate) fn get_virtual_address(&self) -> usize {
        self.address_virtual_offset
    }

    pub(crate) fn get_section(&self) -> Section {
        self.section.clone()
    }
}
