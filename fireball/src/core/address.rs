enum AddressType {
    FileOffset,
    VirtualAddress,
}

pub(crate) struct Address {
    address_type: AddressType,
    address_offset: usize,

    base_addr: Option<usize>,
    section_name: String,
}

impl Address {
    pub(crate) fn from_file_offset(offset: usize) -> Self {
        Address {
            address_type: AddressType::FileOffset,
            address_offset: offset,
            base_addr: None,
            section_name: String::new(),
        }
    }

    pub(crate) fn from_virtual_address(base_addr: usize, offset: usize) -> Self {
        todo!("Goblin을 이용해 섹션 이름이나 베이스 주소를 가져와야함");
        Address {
            address_type: AddressType::VirtualAddress,
            address_offset: offset,
            base_addr: Some(base_addr),
            section_name: String::new(),
        }
    }
}
