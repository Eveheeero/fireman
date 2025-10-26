use crate::{
    abstract_syntax_tree::{ArcAstVariableMap, AstValueType, AstVariableId},
    ir::data::IrData,
    utils::Aos,
};
use either::Either;

#[derive(Debug, Clone)]
pub struct AstParameter {
    pub location: AstParameterLocation,
    /// Either a related AST variable id (Left) or a temporary name when no related variable exists (Right)
    pub related_var_or_temp_name: Either<AstVariableId, String>,
}

impl AstParameter {
    /// Returns a display name for this parameter.
    /// If this parameter is linked to an AST variable, it looks up the actual variable and returns its name.
    /// Otherwise, it falls back to a deterministic name based on the parameter location.
    pub fn name(&self, variables: &ArcAstVariableMap) -> Result<String,()> {
        return match &self.related_var_or_temp_name {
            Either::Left(id) => {
                if let Some(var) = variables.read().unwrap().get(id) {
                    return Ok(var.name());
                }
                Err(())
            }
            Either::Right(temp) => {
                Ok(temp.clone())
            }
        }
    }

    /// Loads the parameter's type by looking up the related AST variable when available.
    /// Falls back to AstValueType::Unknown when no related variable exists or it cannot be found.
    pub fn read_type(&self, variables: &ArcAstVariableMap) -> Result<AstValueType,()> {
        match &self.related_var_or_temp_name {
            Either::Left(id) => {
                if let Some(var) = variables.read().unwrap().get(id) {
                    return Ok(var.var_type.clone());
                }
                Err(())
            }
            Either::Right(_) => Ok(AstValueType::Unknown),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AstParameterLocation {
    Register(Aos<IrData>),
    Stack(isize),
}
