use fireball::{core::Fire, pe::Pe, utils::test_log_subscriber_with_file};
use std::path::Path;
use tracing::Dispatch;

fn get_binary() -> &'static [u8] {
    include_bytes!("../../../fireball/tests/resources/hello_world.exe")
}

#[test]
fn hello_world() {
    let subscriber = test_log_subscriber_with_file("logs/fireman_hello_world.log");

    tracing::dispatcher::with_default(&Dispatch::new(subscriber), || {
        let binary = get_binary();

        let pe = Pe::from_binary(binary.to_vec()).unwrap();
        let result = pe.decompile_all().unwrap();
        println!("{}", &result);
        let file = Path::new("logs/hello_world_result.log");
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(file, &result).unwrap();
        assert!(true);
    });
}
