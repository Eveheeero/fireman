use crate::{core::Fire, pe::PE};

#[test]
fn pe_hello_world() {
    let binary = include_bytes!("../../tests/resources/hello_world.exe");
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    dbg!(pe);
}
