use crate::{
    core::{Address, Fire},
    pe::PE,
};

#[test]
fn pe_hello_world() {
    let binary = include_bytes!("../../tests/resources/hello_world.exe");
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    dbg!(pe);
}

#[test]
fn pe_hello_world_entry_parse() {
    let binary = include_bytes!("../../tests/resources/hello_world.exe");
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    let gl = goblin::pe::PE::parse(binary).unwrap();
    let mut entry_of_raw = 0;
    for section in gl.sections {
        let entry = gl.entry;
        let section_start = section.virtual_address as usize;
        let section_end = section.virtual_address as usize + section.virtual_size as usize;
        if entry >= section_start && entry < section_end {
            entry_of_raw = entry - section_start + section.pointer_to_raw_data as usize;
            break;
        }
    }
    let entry_of_raw = Address::from_file_offset(&binary.to_vec(), entry_of_raw);
    let insts = pe.parse_assem_range(entry_of_raw, 0x60).unwrap();
    for inst in insts.iter() {
        println!("{line}", line = inst.to_string());
    }
}
