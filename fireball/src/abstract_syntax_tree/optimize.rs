mod call_argument_analyzation;
mod collapse_unused_variable;
mod constant_folding;
mod control_flow_cleanup;
mod copy_propagation;
mod expression_inlining;
mod ir_analyzation;
mod loop_analyzation;
mod parameter_analyzation;
pub mod pattern_matching;

use super::*;
use std::hash::Hash;

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

        if config.pattern_matching_enabled {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                pattern_matching::apply_patterns(
                    &mut ast,
                    function_id,
                    to_version,
                    &config.pattern_matching,
                    pattern_matching::AstPatternApplyPhase::BeforeIrAnalyzation,
                )?;
            }
        }

        if config.ir_analyzation {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                ir_analyzation::analyze_ir_function(&mut ast, function_id, to_version)?;
            }
            if config.pattern_matching_enabled {
                for (function_id, to_version) in versions.iter().copied() {
                    if !has_function_version(&ast, function_id, to_version) {
                        continue;
                    }
                    pattern_matching::apply_patterns(
                        &mut ast,
                        function_id,
                        to_version,
                        &config.pattern_matching,
                        pattern_matching::AstPatternApplyPhase::AfterIrAnalyzation,
                    )?;
                }
            }
        }
        if config.parameter_analyzation {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                parameter_analyzation::analyze_parameters(&mut ast, function_id, to_version)?;
            }
            if config.pattern_matching_enabled {
                for (function_id, to_version) in versions.iter().copied() {
                    if !has_function_version(&ast, function_id, to_version) {
                        continue;
                    }
                    pattern_matching::apply_patterns(
                        &mut ast,
                        function_id,
                        to_version,
                        &config.pattern_matching,
                        pattern_matching::AstPatternApplyPhase::AfterParameterAnalyzation,
                    )?;
                }
            }
        }
        if config.call_argument_analyzation {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                call_argument_analyzation::analyze_call_arguments(
                    &mut ast,
                    function_id,
                    to_version,
                )?;
            }

            // Call argument analyzation inlines callee bodies and creates new
            // split-tail functions, both of which can contain if(true) blocks
            // from IR analyzation. Run constant folding to eliminate dead branches.
            // This covers: (1) functions in `versions` whose bodies were modified
            // by inlining, (2) newly created split-tail functions, and (3)
            // functions that were removed (inlined) then re-created as split-tails
            // at a different version than tracked in `versions`.
            if config.constant_folding {
                let tracked: std::collections::HashMap<AstFunctionId, AstFunctionVersion> =
                    versions.iter().copied().collect();
                let all_funcs: Vec<(AstFunctionId, AstFunctionVersion)> = ast
                    .function_versions
                    .iter()
                    .map(|(&fid, &fver)| {
                        if let Some(&tracked_ver) = tracked.get(&fid) {
                            // Function existed before; use the tracked version if it
                            // still exists, otherwise fall back to the current version
                            // (the function was re-created as a split-tail).
                            if has_function_version(&ast, fid, tracked_ver) {
                                (fid, tracked_ver)
                            } else {
                                (fid, fver)
                            }
                        } else {
                            // Newly created function (split-tail).
                            (fid, fver)
                        }
                    })
                    .collect();
                for (fid, ver_to_fold) in all_funcs {
                    if has_function_version(&ast, fid, ver_to_fold) {
                        constant_folding::fold_constants(&mut ast, fid, ver_to_fold)?;
                    }
                }
            }

            if config.pattern_matching_enabled {
                for (function_id, to_version) in versions.iter().copied() {
                    if !has_function_version(&ast, function_id, to_version) {
                        continue;
                    }
                    pattern_matching::apply_patterns(
                        &mut ast,
                        function_id,
                        to_version,
                        &config.pattern_matching,
                        pattern_matching::AstPatternApplyPhase::AfterCallArgumentAnalyzation,
                    )?;
                }
            }
        }

        let run_iterative_passes = config.loop_analyzation
            || config.constant_folding
            || config.control_flow_cleanup
            || config.pattern_matching_enabled
            || config.collapse_unused_varaible
            || config.copy_propagation
            || config.expression_inlining;
        let max_pass_iterations = config.max_pass_iterations.max(1);
        if run_iterative_passes {
            for _ in 0..max_pass_iterations {
                let before = snapshot_optimized_functions(&ast, &versions);

                if config.constant_folding {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        constant_folding::fold_constants(&mut ast, function_id, to_version)?;
                    }
                }

                if config.copy_propagation {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        copy_propagation::propagate_copies(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                }

                if config.expression_inlining {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        expression_inlining::inline_expressions(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                }

                if config.loop_analyzation {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        loop_analyzation::analyze_loops(&mut ast, function_id, to_version)?;
                    }
                }

                if config.collapse_unused_varaible {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        collapse_unused_variable::collapse_unused_variables(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                }

                if config.control_flow_cleanup {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        control_flow_cleanup::cleanup_control_flow(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                }

                if config.pattern_matching_enabled {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::apply_patterns(
                            &mut ast,
                            function_id,
                            to_version,
                            &config.pattern_matching,
                            pattern_matching::AstPatternApplyPhase::AfterIteration,
                        )?;
                    }
                }

                let after = snapshot_optimized_functions(&ast, &versions);
                if before == after {
                    break;
                }
            }
        }

        if config.pattern_matching_enabled {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                pattern_matching::apply_patterns(
                    &mut ast,
                    function_id,
                    to_version,
                    &config.pattern_matching,
                    pattern_matching::AstPatternApplyPhase::AfterOptimization,
                )?;
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

fn has_function_version(
    ast: &Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> bool {
    ast.functions
        .read()
        .unwrap()
        .get(&function_id)
        .and_then(|version_map| version_map.get(&function_version))
        .is_some()
}
