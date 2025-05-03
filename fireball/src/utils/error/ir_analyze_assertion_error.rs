#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IrAnalyzeAssertionFailure {
    AnalyzeNotPerformed(&'static str),
}
