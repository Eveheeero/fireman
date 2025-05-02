#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IrAnalyzeAssertionFailure {
    AnalyzeNotPerformed(&'static str),
    SpBasedLocationFound { ir_index: usize, sub_index: usize },
}
