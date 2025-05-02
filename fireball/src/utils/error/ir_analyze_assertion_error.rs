#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IrAnalyzeAssertionFailure {
    AnalyzeNotPerformed(&'static str),
    SpBasedLocationFound {
        ir_index: Option<usize>,
        sub_index: Option<usize>,
    },
}
