mod arc_or_static;
pub mod error;

pub use arc_or_static::*;

pub fn ir_blocks_to_code(
    targets: impl IntoIterator<Item = std::sync::Arc<crate::core::Block>>,
) -> String {
    let mut cfg_analyzer = crate::ir::analyze::ControlFlowGraphAnalyzer::new();
    cfg_analyzer.add_targets(targets);
    let cfgs = cfg_analyzer.analyze();
    let mut result = String::new();
    for cfg in cfgs.into_iter() {
        let merged = crate::ir::analyze::ir_block_merger::merge_blocks(cfg.get_blocks());
        let ast = crate::ir::analyze::generate_c(&merged);
        let code = ast.to_c_code();
        result.push_str(&code);
        result.push_str(
            "--------------------------------------------------------------------------------\n",
        );
    }
    result
}
