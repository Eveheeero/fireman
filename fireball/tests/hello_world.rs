use fireball::{core::Fire, pe::Pe};

#[test]
fn hello_world() {
    tracing_subscriber::fmt().init();
    let binary = include_bytes!("resources/hello_world.exe");
    let pe = Pe::from_binary(binary.to_vec()).unwrap();
    pe.decom_from_entry().unwrap();
    dbg!(pe);
}
