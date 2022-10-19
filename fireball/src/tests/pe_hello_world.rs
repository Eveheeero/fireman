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
    let entry_of_raw = Address::from_file_offset(&binary.to_vec(), gl.entry as u64);
    let insts = pe.parse_assem_range(entry_of_raw, 0x60).unwrap();
    for inst in insts.iter() {
        println!("{line}", line = inst.to_string());
    }
}
