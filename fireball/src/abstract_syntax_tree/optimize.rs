mod collapse_unused_variable;
mod constant_folding;
mod ir_analyzation;
pub(crate) mod opt_utils;
mod parameter_analyzation;
pub mod pattern_matching;

use super::*;
use crate::pattern_matching::{AstPattern, AstPatternApplyPhase};
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

        apply_custom_patterns(
            &mut ast,
            &versions,
            &config.pattern_matching,
            AstPatternApplyPhase::BeforeIrAnalyzation,
        )?;

        if config.ir_analyzation {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                ir_analyzation::analyze_ir_function(&mut ast, function_id, to_version)?;
            }
            apply_custom_patterns(
                &mut ast,
                &versions,
                &config.pattern_matching,
                AstPatternApplyPhase::AfterIrAnalyzation,
            )?;
        }
        if config.parameter_analyzation {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                parameter_analyzation::analyze_parameters(&mut ast, function_id, to_version)?;
            }
            apply_custom_patterns(
                &mut ast,
                &versions,
                &config.pattern_matching,
                AstPatternApplyPhase::AfterParameterAnalyzation,
            )?;
        }

        let max_pass_iterations = config.max_pass_iterations.max(1);
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

            if config.collapse_unused_variable {
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
            apply_custom_patterns(
                &mut ast,
                &versions,
                &config.pattern_matching,
                AstPatternApplyPhase::AfterIteration,
            )?;

            let after = snapshot_optimized_functions(&ast, &versions);
            if before == after {
                break;
            }
        }

        apply_custom_patterns(
            &mut ast,
            &versions,
            &config.pattern_matching,
            AstPatternApplyPhase::AfterOptimization,
        )?;

        ast.shrink();
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

fn apply_custom_patterns(
    ast: &mut Ast,
    versions: &[(AstFunctionId, AstFunctionVersion)],
    patterns: &[AstPattern],
    phase: AstPatternApplyPhase,
) -> Result<(), DecompileError> {
    for (function_id, to_version) in versions.iter().copied() {
        if !has_function_version(&ast, function_id, to_version) {
            continue;
        }
        pattern_matching::apply_patterns(ast, function_id, to_version, patterns, phase)?
    }
    Ok(())
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
