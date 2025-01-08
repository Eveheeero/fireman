use crate::{
    core::{Address, Fire, RelationType},
    pe::PE,
    prelude::*,
};

fn get_binary() -> &'static [u8] {
    include_bytes!("../../tests/resources/hello_world.exe")
}

#[test]
fn pe_hello_world() {
    test_init();
    let binary = get_binary();
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    dbg!(pe);
}

#[test]
fn pe_hello_world_entry_parse() {
    test_init();
    let binary = get_binary();
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    let gl = goblin::pe::PE::parse(binary).unwrap();

    let sections = pe.get_sections();

    // 가상주소 기반 어셈블리 파싱 확인
    let entry_of_virtual_address = Address::from_virtual_address(&sections, gl.entry as u64);
    let insts_by_virtual_address = pe
        .parse_assem_range(&entry_of_virtual_address, 0x60)
        .unwrap();

    // 엔트리 포인트의 파일 오프셋 연산 (2022-10-19 기준 0x725)
    let mut entry_of_file_offset = 0;
    for section in gl.sections {
        if section.virtual_address as u64 <= gl.entry as u64
            && gl.entry as u64 <= (section.virtual_address + section.virtual_size) as u64
        {
            entry_of_file_offset = gl.entry as u64 - section.virtual_address as u64
                + section.pointer_to_raw_data as u64;
            break;
        }
    }

    // 파일 오프셋 기반 어셈블리 파싱 확인
    let entry_of_file_offset = Address::from_file_offset(&sections, entry_of_file_offset);
    let insts_by_file_offset = pe.parse_assem_range(&entry_of_file_offset, 0x60).unwrap();

    // 두 결과값이 일치하는지 확인
    for (left, right) in insts_by_virtual_address
        .iter()
        .zip(insts_by_file_offset.iter())
    {
        assert_eq!(left, right);
    }
}

#[test]
fn pe_hello_world_detect_block_entry() {
    test_init();
    let binary = get_binary();
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    let gl = goblin::pe::PE::parse(binary).unwrap();
    let sections = pe.get_sections();
    let entry = Address::from_virtual_address(&sections, gl.entry as u64);
    let block = pe.generate_block_from_address(&entry);

    assert_eq!(&block.get_section().unwrap().name, ".text");
    assert_eq!(*block.get_start_address(), entry);
    assert_ne!(*block.get_end_address().unwrap(), entry);
}

#[test]
fn pe_hello_world_detect_block_etc() {
    test_init();
    let binary = get_binary();
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    let gl = goblin::pe::PE::parse(binary).unwrap();
    let sections = pe.get_sections();
    let entry = Address::from_virtual_address(&sections, gl.entry as u64);
    for offset in std::iter::once(-6).chain(2..=7) {
        info!("{} 오프셋에 대한 파싱 진행", offset);
        let address;
        if offset < 0 {
            address = &entry - (-offset) as u64;
        } else {
            address = &entry + offset as u64;
        }
        let block = pe.generate_block_from_address(&address);
        assert_eq!(&block.get_section().unwrap().name, ".text");
        assert_eq!(*block.get_start_address(), address);
        assert_ne!(*block.get_end_address().unwrap(), address);
    }
}

#[test]
fn pe_hello_world_block_relation() {
    test_init();
    let binary = get_binary();
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    let gl = goblin::pe::PE::parse(binary).unwrap();
    let sections = pe.get_sections();

    /* 엔트리에 대한 블럭 파싱 및 relation 생성 확인 */
    let entry = Address::from_virtual_address(&sections, gl.entry as u64);
    pe.generate_block_from_address(&entry);
    let blocks = pe.inspect_blocks();
    let entry_block = blocks.get_by_start_address(&entry);
    assert!(entry_block.is_some());
    let entry_block = entry_block.unwrap();
    let entry_block_id = entry_block.get_id();
    let entry_connected_to = entry_block.get_connected_to();
    assert_eq!(entry_connected_to.len(), 1);
    assert_eq!(entry_connected_to[0].relation_type(), &RelationType::Call);

    /* 엔트리의 to에 대한 블럭 생성 확인 */
    let to_address = entry_connected_to[0].to().unwrap();
    pe.generate_block_from_address(&to_address);
    let blocks = pe.inspect_blocks();
    let to_block = blocks.get_by_start_address(&to_address);
    assert!(to_block.is_some());
    let to_block = to_block.unwrap();
    // check connected from
    let to_connected_from = to_block.get_connected_from();
    assert_eq!(to_connected_from.len(), 1);
    assert_eq!(to_connected_from[0].from(), entry_block_id);
    // check connected to
    let to_connected_to = to_block.get_connected_to();
    assert_eq!(to_connected_to.len(), 1);
    assert_eq!(
        to_connected_to[0].to().unwrap().get_virtual_address(),
        37216
    );
}
