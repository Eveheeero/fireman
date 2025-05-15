use fireball::{
    core::Fire,
    ir::analyze::{generate_c, ir_block_merger::merge_blocks, ControlFlowGraphAnalyzer},
    pe::Pe,
};

fn test_init() {
    use tracing_subscriber::{
        prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    };

    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(false),
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
    let targets = pe.analyze_all().unwrap();
    let mut cfg_analyzer = ControlFlowGraphAnalyzer::new();
    cfg_analyzer.add_targets(targets);
    let cfgs = cfg_analyzer.analyze();
    for cfg in cfgs.iter() {
        let merged = merge_blocks(cfg.get_blocks());
        let result = generate_c(&merged);

        println!("{}", result.to_c_code());
        println!(
            "--------------------------------------------------------------------------------"
        );
    }
}
