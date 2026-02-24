mod call_argument_analyzation;
mod collapse_unused_variable;
mod constant_folding;
mod control_flow_cleanup;
mod ir_analyzation;
mod loop_analyzation;
mod parameter_analyzation;
pub mod pattern_matching;

use super::*;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

impl Ast {
    pub fn optimize(&self, config: Option<AstOptimizationConfig>) -> Result<Self, DecompileError> {
        let function_ids: Vec<_> = self.function_versions.keys().cloned().collect();
        self.optimize_functions(&function_ids, config)
    }

    pub fn optimize_function(
        &self,
        function_id: AstFunctionId,
        config: Option<AstOptimizationConfig>,
    ) -> Result<Self, DecompileError> {
        self.optimize_functions(&[function_id], config)
    }

    pub fn optimize_functions(
        &self,
        function_ids: &[AstFunctionId],
        config: Option<AstOptimizationConfig>,
    ) -> Result<Self, DecompileError> {
        let mut ast = self.clone();
        let config = config.unwrap_or_default();
        let mut ordered_function_ids = function_ids.to_vec();
        ordered_function_ids.sort_unstable();

        // Clone all target functions up front so later passes can query each other.
        let mut versions: Vec<(AstFunctionId, AstFunctionVersion)> = Vec::new();
        for function_id in ordered_function_ids.into_iter() {
            let from_version = *ast.function_versions.get(&function_id).unwrap();
            let to_version = ast.clone_function(&function_id, &from_version).unwrap();
            versions.push((function_id, to_version));
        }

        if config.ir_analyzation {
            for (function_id, to_version) in versions.iter().copied() {
                ir_analyzation::analyze_ir_function(&mut ast, function_id, to_version)?;
            }
        }
        if config.parameter_analyzation {
            for (function_id, to_version) in versions.iter().copied() {
                parameter_analyzation::analyze_parameters(&mut ast, function_id, to_version)?;
            }
        }
        if config.call_argument_analyzation {
            for (function_id, to_version) in versions.iter().copied() {
                call_argument_analyzation::analyze_call_arguments(
                    &mut ast,
                    function_id,
                    to_version,
                )?;
            }
        }

        let run_iterative_passes = config.loop_analyzation
            || config.constant_folding
            || config.control_flow_cleanup
            || config.pattern_matching_enabled
            || config.collapse_unused_varaible;
        let max_pass_iterations = config.max_pass_iterations.max(1);
        if run_iterative_passes {
            for _ in 0..max_pass_iterations {
                let before = snapshot_optimized_functions(&ast, &versions);
                let mut ran_ast_optimization = false;

                if config.constant_folding {
                    ran_ast_optimization = true;
                    for (function_id, to_version) in versions.iter().copied() {
                        constant_folding::fold_constants(&mut ast, function_id, to_version)?;
                    }
                    if config.pattern_matching_enabled {
                        for (function_id, to_version) in versions.iter().copied() {
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                &config.pattern_matching,
                            )?;
                        }
                    }
                }

                if config.loop_analyzation {
                    ran_ast_optimization = true;
                    for (function_id, to_version) in versions.iter().copied() {
                        loop_analyzation::analyze_loops(&mut ast, function_id, to_version)?;
                    }
                    if config.pattern_matching_enabled {
                        for (function_id, to_version) in versions.iter().copied() {
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                &config.pattern_matching,
                            )?;
                        }
                    }
                }

                if config.collapse_unused_varaible {
                    ran_ast_optimization = true;
                    for (function_id, to_version) in versions.iter().copied() {
                        collapse_unused_variable::collapse_unused_variables(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                    if config.pattern_matching_enabled {
                        for (function_id, to_version) in versions.iter().copied() {
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                &config.pattern_matching,
                            )?;
                        }
                    }
                }

                if config.control_flow_cleanup {
                    ran_ast_optimization = true;
                    for (function_id, to_version) in versions.iter().copied() {
                        control_flow_cleanup::cleanup_control_flow(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                    if config.pattern_matching_enabled {
                        for (function_id, to_version) in versions.iter().copied() {
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                &config.pattern_matching,
                            )?;
                        }
                    }
                }

                if config.pattern_matching_enabled && !ran_ast_optimization {
                    for (function_id, to_version) in versions.iter().copied() {
                        pattern_matching::apply_patterns(
                            &mut ast,
                            function_id,
                            to_version,
                            &config.pattern_matching,
                        )?;
                    }
                }

                let after = snapshot_optimized_functions(&ast, &versions);
                if before == after {
                    break;
                }
            }
        }

        Ok(ast)
    }
}

fn snapshot_optimized_functions(
    ast: &Ast,
    versions: &[(AstFunctionId, AstFunctionVersion)],
) -> u64 {
    let functions = ast.functions.read().unwrap();
    let mut hasher = DefaultHasher::new();
    for (function_id, function_version) in versions.iter().copied() {
        function_id.hash(&mut hasher);
        function_version.hash(&mut hasher);
        let function = functions
            .get(&function_id)
            .and_then(|version_map| version_map.get(&function_version))
            .unwrap();
        function.name.hash(&mut hasher);
        function.parameters.len().hash(&mut hasher);
        function.processed_optimizations.len().hash(&mut hasher);
        format!("{:?}", function.body).hash(&mut hasher);
    }
    hasher.finish()
}
