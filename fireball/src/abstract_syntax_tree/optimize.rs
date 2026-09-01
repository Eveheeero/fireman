mod collapse_unused_variable;
mod constant_folding;
mod ir_analyzation;
pub(crate) mod opt_utils;
mod parameter_analyzation;
pub mod pattern_matching;

use super::*;
use crate::pattern_matching::AstPatternApplyPhase;
use std::hash::Hash;

impl Ast {
    pub fn optimize(
        &mut self,
        optimizations: Option<&[AstOptimizationKind]>,
    ) -> Result<(), DecompileError> {
        let function_ids: Vec<_> = self.function_versions.keys().cloned().collect();
        self.optimize_functions(&function_ids, optimizations)
    }

    pub fn optimize_function(
        &mut self,
        function_id: AstFunctionId,
        optimizations: Option<&[AstOptimizationKind]>,
    ) -> Result<(), DecompileError> {
        self.optimize_functions(&[function_id], optimizations)
    }

    pub fn optimize_functions(
        &mut self,
        function_ids: &[AstFunctionId],
        optimizations: Option<&[AstOptimizationKind]>,
    ) -> Result<(), DecompileError> {
        let default = AstOptimizationKind::all();
        let optimizations = optimizations.unwrap_or_else(|| default.as_slice());
        let mut ordered_function_ids = function_ids.to_vec();
        ordered_function_ids.sort_unstable();
        let target_functions = {
            ordered_function_ids
                .iter()
                .map(|function_id| {
                    let version = self.function_versions.get(function_id).unwrap();
                    (*function_id, *version)
                })
                .collect::<Vec<_>>()
        };
        fn optimize_function_inner(
            ast: &mut Ast,
            target_functions: &[(AstFunctionId, AstFunctionVersion)],
            optimizations: &[AstOptimizationKind],
        ) {
            for optimization in optimizations {
                match optimization {
                    AstOptimizationKind::IrAnalyzation => {
                        for target_function in target_functions {
                            ir_analyzation::analyze_ir_function(
                                ast,
                                target_function.0,
                                target_function.1,
                            )
                            .unwrap();
                        }
                    }
                    AstOptimizationKind::ParameterAnalyzation => {
                        for target_function in target_functions {
                            parameter_analyzation::analyze_parameters(
                                ast,
                                target_function.0,
                                target_function.1,
                            )
                            .unwrap();
                        }
                    }
                    AstOptimizationKind::ConstantFolding => {
                        for target_function in target_functions {
                            constant_folding::fold_constants(
                                ast,
                                target_function.0,
                                target_function.1,
                            )
                            .unwrap();
                        }
                    }
                    AstOptimizationKind::CollapseUnusedVariables => {
                        for target_function in target_functions {
                            collapse_unused_variable::collapse_unused_variables(
                                ast,
                                target_function.0,
                                target_function.1,
                            )
                            .unwrap();
                        }
                    }
                    AstOptimizationKind::OptimizationLoop(optimizations, loop_count) => {
                        let mut before_hash = snapshot_optimized_functions(ast, target_functions);
                        for _ in 0..*loop_count {
                            optimize_function_inner(ast, target_functions, optimizations);
                            let after_hash = snapshot_optimized_functions(ast, target_functions);
                            if before_hash == after_hash {
                                break;
                            }
                            before_hash = after_hash;
                        }
                    }
                    AstOptimizationKind::PatternMatching(pattern) => {
                        let pattern = *pattern.clone();
                        let pattern = &[pattern];
                        for target_function in target_functions {
                            pattern_matching::apply_patterns(
                                ast,
                                target_function.0,
                                target_function.1,
                                pattern,
                                AstPatternApplyPhase::AfterIrAnalyzation,
                            )
                            .unwrap();
                        }
                    }
                }
            }
        }
        optimize_function_inner(self, target_functions.as_slice(), optimizations);

        self.shrink();
        Ok(())
    }
}

fn snapshot_optimized_functions(
    ast: &Ast,
    versions: &[(AstFunctionId, AstFunctionVersion)],
) -> u64 {
    let functions = ast.functions.read().unwrap();
    let mut hasher = pattern_matching::Blake3StdHasher::new();
    for (function_id, function_version) in versions.iter().copied() {
        let Some(function) = functions
            .get(&function_id)
            .and_then(|version_map| version_map.get(&function_version))
        else {
            continue;
        };
        function_id.hash(&mut hasher);
        function_version.hash(&mut hasher);
        function.name.hash(&mut hasher);
        function.parameters.len().hash(&mut hasher);
        pattern_matching::hash_statement_list(&mut hasher, &function.body);
    }
    hasher.finish64()
}
