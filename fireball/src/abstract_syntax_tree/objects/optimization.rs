use crate::abstract_syntax_tree::pattern_matching::AstPattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstOptimizationKind {
    /// Turn Low IR([AstStatement::IR(...)]) to Middle IR for analyzation
    IrAnalyzation,
    ParameterAnalyzation,
    ConstantFolding,
    CollapseUnusedVariables,
    /// Loop for N times. if nothing changed during loops, stop
    OptimizationLoop(Vec<AstOptimizationKind>, u8),
    PatternMatching(Box<AstPattern>),
}

impl AstOptimizationKind {
    pub fn all() -> Vec<Self> {
        use AstOptimizationKind::*;
        [
            IrAnalyzation,
            ParameterAnalyzation,
            // rep 3 times
            OptimizationLoop([ConstantFolding, CollapseUnusedVariables].into(), 3),
        ]
        .into()
    }
}
