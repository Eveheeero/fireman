mod get_related_variables;
mod print_with_config;

use super::*;

pub trait PrintWithConfig {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String;
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub enum AstVariableAccessType {
    Read,
    Write,
}
pub trait GetRelatedVariables {
    fn get_related_variables(&self) -> Vec<(AstVariableAccessType, AstVariableId)>;
}
