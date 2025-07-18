mod collapse_unused_varaible;
mod ir_analyzation;
pub mod pattern_matching;

use super::*;

impl Ast {
    pub fn optimize(&self, config: Option<AstOptimizationConfig>) -> Result<Self, DecompileError> {
        let function_ids: Vec<_> = self.function_versions.keys().cloned().collect();
        self.optimize_functions(function_ids, config)
    }

    pub fn optimize_function(
        &self,
        function_id: AstFunctionId,
        config: Option<AstOptimizationConfig>,
    ) -> Result<Self, DecompileError> {
        self.optimize_functions([function_id].into(), config)
    }

    // TODO: Implement optimization passes:
    // 1. Dead code elimination
    // 2. Constant folding
    // 3. Common subexpression elimination
    // 4. Loop optimization
    // 5. Function inlining
    pub fn optimize_functions(
        &self,
        function_ids: Vec<AstFunctionId>,
        config: Option<AstOptimizationConfig>,
    ) -> Result<Self, DecompileError> {
        let mut ast = self.clone();
        let config = config.unwrap_or_default();

        for function_id in function_ids {
            let from_version = *ast.function_versions.get(&function_id).unwrap();
            let to_version = ast.clone_function(&function_id, &from_version).unwrap();

            if config.ir_analyzation {
                ir_analyzation::analyze_ir_function(&mut ast, function_id, to_version)?;
            }
            if config.collapse_unused_varaible {
                collapse_unused_varaible::collapse_unused_variables(
                    &mut ast,
                    function_id,
                    to_version,
                )?;
            }
        }

        Ok(ast)
    }
}
