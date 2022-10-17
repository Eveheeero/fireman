use fireball::core::Fire;
use fireball::pe::PE;

#[test]
fn hello_world() {
    let binary = include_bytes!("resources/hello_world.exe");
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    dbg!(pe);
}
