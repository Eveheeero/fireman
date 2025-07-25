#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct AstVariableId {
    /// nth variable
    pub(crate) index: u32,
    pub(crate) parent: Option<AstFunctionId>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct AstFunctionId {
    pub(crate) address: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct AstFunctionVersion(pub usize);

impl AstVariableId {
    pub fn get_default_name(&self) -> String {
        if self.parent.is_some() {
            format!("v{}", self.index)
        } else {
            format!("g{}", self.index)
        }
    }
}
impl AstFunctionId {
    pub fn get_default_name(&self) -> String {
        format!("f{}", self.address)
    }
}
