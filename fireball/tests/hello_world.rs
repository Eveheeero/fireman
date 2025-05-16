use fireball::{core::Fire, pe::Pe};

fn test_init() {
    use tracing_subscriber::{
        prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    };

    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let file = std::fs::File::create("fireball_outside.log").unwrap();
        let _ = tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(false),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(false)
                    .with_ansi(false)
                    .with_writer(file),
            )
            .with(
                tracing_subscriber::filter::Targets::new()
                    .with_target("fireball", tracing::Level::TRACE),
            )
            .try_init();
    });
}
fn get_binary() -> &'static [u8] {
    include_bytes!("resources/hello_world.exe")
}

#[test]
fn hello_world() {
    test_init();
    let binary = get_binary();

    let pe = Pe::from_binary(binary.to_vec()).unwrap();
    println!("{}", pe.decompile_all().unwrap());
    assert!(true);
}
