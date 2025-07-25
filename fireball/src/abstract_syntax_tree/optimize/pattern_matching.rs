#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstPattern {
    pub name: String,
    pub origin: AstPatternOrigin,
    pub arg: AstPatternArgType,
    pub pattern: String, // TODO
}
impl AstPattern {
    pub const ALL: Vec<Self> = vec![];
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstPatternOrigin {
    PreDefined,
    UserInput,
    File,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstPatternArgType {
    WithAssembly,
    WithIr,
    WithAst,
    WithOptimizedAst,
}
