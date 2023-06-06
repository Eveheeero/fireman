use fireball::core::Fire;
use fireball::pe::PE;

#[test]
#[ignore = "분석로직 개편의 이유로 비활성화"]
fn hello_world() {
    let _ = simplelog::SimpleLogger::init(log::LevelFilter::Trace, simplelog::Config::default());
    let binary = include_bytes!("resources/hello_world.exe");
    let pe = PE::from_binary(binary.to_vec()).unwrap();
    pe.decom_from_entry().unwrap();
    dbg!(pe);
}
