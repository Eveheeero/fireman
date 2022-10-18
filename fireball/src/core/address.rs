use crate::core::{Section, SECTIONS};

use goblin::Object;

enum AddressType {
    FileOffset,
    VirtualAddress,
}

pub(crate) struct Address {
    address_type: AddressType,
    address_offset: usize,

    base_addr: Option<usize>,
    section: Section,
}

impl Address {
    pub(crate) fn from_file_offset(binary: &Vec<u8>, offset: usize) -> Self {
        let gl = Object::parse(binary).unwrap();
        match gl {
            Object::PE(gl) => todo!(),
            _ => todo!(),
        };
        todo!("Goblin을 이용해 섹션정보와 여러 주소를 파악함");
        Address {
            address_type: AddressType::FileOffset,
            address_offset: offset,
            base_addr: None,
            section: {
                Section {
                    name: Default::default(),
                    base_addr: Default::default(),
                }
            },
        }
    }

    pub(crate) fn from_virtual_address(binary: &Vec<u8>, offset: usize) -> Self {
        let gl = Object::parse(binary).unwrap();
        match gl {
            Object::PE(gl) => todo!(),
            _ => todo!(),
        };
        todo!("Goblin을 이용해 섹션정보와 여러 주소를 파악함");
        Address {
            address_type: AddressType::VirtualAddress,
            address_offset: offset,
            base_addr: todo!(),
            section: {
                Section {
                    name: Default::default(),
                    base_addr: Default::default(),
                }
            },
        }
    }

    pub(crate) fn get_file_offset(&self) -> usize {
        todo!()
    }

    pub(crate) fn get_virtual_address(&self) -> usize {
        todo!()
    }

    pub(crate) fn get_section(&self) -> Section {
        todo!()
    }
}
