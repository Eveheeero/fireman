use fireball::core::Fire;
use fireball::pe::PE;

#[test]
fn hello_world() {
    let _ = simplelog::SimpleLogger::init(log::LevelFilter::Trace, simplelog::Config::default());
    let binary = include_bytes!("resources/hello_world.exe");
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    pe.decom_from_entry().unwrap();
    dbg!(pe);
}
