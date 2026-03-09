mod bit_trick_recognition;
mod call_argument_analyzation;
pub(crate) mod call_graph;
mod collapse_unused_variable;
mod common_subexpression_elimination;
mod constant_folding;
mod control_flow_cleanup;
mod copy_propagation;
mod dead_store_elimination;
mod expression_inlining;
mod goto_containment;
mod induction_variable_analysis;
mod ir_analyzation;
mod lifetime_scoping;
mod loop_analyzation;
mod magic_division_recovery;
mod name_recovery;
mod operator_canonicalization;
mod opt_utils;
mod parameter_analyzation;
pub mod pattern_matching;
mod signedness_inference;
mod structured_region_lowering;
mod switch_reconstruction;
mod temporary_elimination;
mod variable_coalescing;

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
                structured_region_lowering::lower_structured_regions(
                    &mut ast,
                    function_id,
                    to_version,
                )?;
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

        // Signedness inference: refine Int → UInt based on usage context.
        if config.signedness_inference {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                signedness_inference::infer_signedness(&mut ast, function_id, to_version)?;
            }
        }

        let run_iterative_passes = config.loop_analyzation
            || config.constant_folding
            || config.control_flow_cleanup
            || config.pattern_matching_enabled
            || config.collapse_unused_varaible
            || config.dead_store_elimination
            || config.copy_propagation
            || config.expression_inlining
            || config.ternary_recovery
            || config.boolean_recovery;
        let max_pass_iterations = config.max_pass_iterations.max(1);
        if run_iterative_passes {
            for _ in 0..max_pass_iterations {
                let before = snapshot_optimized_functions(&ast, &versions);

                // Operator canonicalization.
                if config.use_embedded_passes {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::embedded::operator_canonicalization::canonicalize_operators(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                } else {
                    let op_canon_pat = pattern_matching::AstPattern::predefined_pattern(
                        "operator-canonicalization.fb",
                    )
                    .unwrap();
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::apply_patterns(
                            &mut ast,
                            function_id,
                            to_version,
                            std::slice::from_ref(&op_canon_pat),
                            pattern_matching::AstPatternApplyPhase::AfterIteration,
                        )?;
                    }
                }

                // Magic-constant division recovery: before constant folding so
                // the new Div expressions can be further simplified.
                if config.use_embedded_passes {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::embedded::magic_division_recovery::recover_magic_divisions(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                } else {
                    let magic_div_pat = pattern_matching::AstPattern::predefined_pattern(
                        "magic-division-recovery.fb",
                    )
                    .unwrap();
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::apply_patterns(
                            &mut ast,
                            function_id,
                            to_version,
                            std::slice::from_ref(&magic_div_pat),
                            pattern_matching::AstPatternApplyPhase::AfterIteration,
                        )?;
                    }
                }

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
                        copy_propagation::propagate_copies(&mut ast, function_id, to_version)?;
                    }
                }

                // Common subexpression elimination: after copy propagation so
                // copies are resolved, before expression inlining.
                for (function_id, to_version) in versions.iter().copied() {
                    if !has_function_version(&ast, function_id, to_version) {
                        continue;
                    }
                    common_subexpression_elimination::eliminate_common_subexpressions(
                        &mut ast,
                        function_id,
                        to_version,
                    )?;
                }

                if config.expression_inlining {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        expression_inlining::inline_expressions(&mut ast, function_id, to_version)?;
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

                if config.dead_store_elimination {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        dead_store_elimination::eliminate_dead_stores(
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

                    if !config.pattern_matching_enabled {
                        let flatten_blocks =
                            pattern_matching::AstPattern::predefined_pattern("flatten-blocks.fb")
                                .unwrap();
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                std::slice::from_ref(&flatten_blocks),
                                pattern_matching::AstPatternApplyPhase::AfterIteration,
                            )?;
                        }
                    }

                    // Tail-call merge and branch inversion: merge trailing
                    // Call();Return(None) into Return(Call()) and invert
                    // if(!cond) when the else branch is larger.
                    if config.use_embedded_passes {
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::embedded::control_flow_cleanup::cleanup_tail_calls_and_branches(
                                &mut ast,
                                function_id,
                                to_version,
                            )?;
                        }
                    } else {
                        let tail_call_pat =
                            pattern_matching::AstPattern::predefined_pattern("tail-call-merge.fb")
                                .unwrap();
                        let branch_inv_pat =
                            pattern_matching::AstPattern::predefined_pattern("branch-inversion.fb")
                                .unwrap();
                        let cleanup_patterns = vec![tail_call_pat, branch_inv_pat];
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                &cleanup_patterns,
                                pattern_matching::AstPatternApplyPhase::AfterIteration,
                            )?;
                        }
                    }

                    // Merge consecutive if-statements with the same pure condition:
                    // if(cond) { A } if(cond) { B } → if(cond) { A; B }
                    if config.use_embedded_passes {
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::embedded::control_flow_cleanup::merge_same_condition_ifs(
                                &mut ast,
                                function_id,
                                to_version,
                            )?;
                        }
                    } else {
                        let merge_pat = pattern_matching::AstPattern::predefined_pattern(
                            "merge-same-condition-ifs.fb",
                        )
                        .unwrap();
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                &[merge_pat.clone()],
                                pattern_matching::AstPatternApplyPhase::AfterIteration,
                            )?;
                        }
                    }
                }

                if config.boolean_recovery {
                    if config.use_embedded_passes {
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::embedded::boolean_recovery::recover_boolean(
                                &mut ast,
                                function_id,
                                to_version,
                            )?;
                        }
                    } else {
                        let boolean_pat =
                            pattern_matching::AstPattern::predefined_pattern("boolean-recovery.fb")
                                .unwrap();
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                std::slice::from_ref(&boolean_pat),
                                pattern_matching::AstPatternApplyPhase::AfterIteration,
                            )?;
                        }
                    }
                }

                if config.ternary_recovery {
                    if config.use_embedded_passes {
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::embedded::ternary_recovery::recover_ternary(
                                &mut ast,
                                function_id,
                                to_version,
                            )?;
                            pattern_matching::embedded::assertion_recovery::recover_assertions(
                                &mut ast,
                                function_id,
                                to_version,
                            )?;
                            pattern_matching::embedded::do_while_recovery::recover_do_while(
                                &mut ast,
                                function_id,
                                to_version,
                            )?;
                            pattern_matching::embedded::clamp_recovery::recover_clamp(
                                &mut ast,
                                function_id,
                                to_version,
                            )?;
                            pattern_matching::embedded::loop_cleanup::cleanup_loops(
                                &mut ast,
                                function_id,
                                to_version,
                            )?;
                        }
                    } else {
                        let ternary_pat =
                            pattern_matching::AstPattern::predefined_pattern("ternary-recovery.fb")
                                .unwrap();
                        let assertion_pat =
                            pattern_matching::AstPattern::predefined_pattern("assertion-recovery.fb")
                                .unwrap();
                        let do_while_pat =
                            pattern_matching::AstPattern::predefined_pattern("do-while-recovery.fb")
                                .unwrap();
                        let clamp_pat =
                            pattern_matching::AstPattern::predefined_pattern("clamp-recovery.fb")
                                .unwrap();
                        let loop_cleanup_pat =
                            pattern_matching::AstPattern::predefined_pattern("loop-cleanup.fb")
                                .unwrap();
                        let pats = vec![ternary_pat, assertion_pat, do_while_pat, clamp_pat, loop_cleanup_pat];
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                &pats,
                                pattern_matching::AstPatternApplyPhase::AfterIteration,
                            )?;
                        }
                    }
                }

                if config.use_embedded_passes {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::embedded::if_conversion_reversal::reverse_if_conversion(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                } else {
                    let if_conv_pat = pattern_matching::AstPattern::predefined_pattern(
                        "if-conversion-reversal.fb",
                    )
                    .unwrap();
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::apply_patterns(
                            &mut ast,
                            function_id,
                            to_version,
                            std::slice::from_ref(&if_conv_pat),
                            pattern_matching::AstPatternApplyPhase::AfterIteration,
                        )?;
                    }
                }

                // Dead branch elimination: if(true){A}→Block(A), if(false){_}→Empty.
                {
                    let dbe_pat = pattern_matching::AstPattern::predefined_pattern(
                        "dead-branch-elimination.fb",
                    )
                    .unwrap();
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::apply_patterns(
                            &mut ast,
                            function_id,
                            to_version,
                            std::slice::from_ref(&dbe_pat),
                            pattern_matching::AstPatternApplyPhase::AfterIteration,
                        )?;
                    }
                }

                // Identity simplification (x+0→x, x*1→x, etc.) via predefined pattern.
                // Runs before bit trick recognition so that simplified expressions
                // feed into rotation/strength-reduction detection.
                {
                    let id_simp_pat = pattern_matching::AstPattern::predefined_pattern(
                        "identity-simplification.fb",
                    )
                    .unwrap();
                    let id_simp_patterns = std::slice::from_ref(&id_simp_pat);
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::apply_patterns(
                            &mut ast,
                            function_id,
                            to_version,
                            id_simp_patterns,
                            pattern_matching::AstPatternApplyPhase::AfterIteration,
                        )?;
                    }
                }

                // Bit trick recognition runs unconditionally (no config toggle yet).
                // It is cheap and should run after constant folding has simplified
                // shift amounts into literals. Identity ops already extracted to .fb above.
                for (function_id, to_version) in versions.iter().copied() {
                    if !has_function_version(&ast, function_id, to_version) {
                        continue;
                    }
                    bit_trick_recognition::recognize_bit_tricks(&mut ast, function_id, to_version)?;
                }

                // Rotation recovery and strength reduction: rewrite shift/or
                // rotation idioms and shift/add strength-reduction back to
                // high-level rotate/multiply operations.
                if config.use_embedded_passes {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::embedded::bit_trick_recognition::recognize_rotation_and_strength_reduction(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                } else {
                    let rotation_pat =
                        pattern_matching::AstPattern::predefined_pattern("rotation-recovery.fb")
                            .unwrap();
                    let strength_pat =
                        pattern_matching::AstPattern::predefined_pattern("strength-reduction.fb")
                            .unwrap();
                    let recognition_patterns = vec![rotation_pat, strength_pat];
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::apply_patterns(
                            &mut ast,
                            function_id,
                            to_version,
                            &recognition_patterns,
                            pattern_matching::AstPatternApplyPhase::AfterIteration,
                        )?;
                    }
                }

                // Cast minimization runs unconditionally.
                if config.use_embedded_passes {
                    for (function_id, to_version) in versions.iter().copied() {
                        if !has_function_version(&ast, function_id, to_version) {
                            continue;
                        }
                        pattern_matching::embedded::cast_minimization::minimize_casts(
                            &mut ast,
                            function_id,
                            to_version,
                        )?;
                    }
                } else {
                    let cast_min_pattern =
                        pattern_matching::AstPattern::predefined_pattern("cast-minimization.fb");
                    if let Some(cast_min) = cast_min_pattern {
                        let cast_min_patterns = vec![cast_min];
                        for (function_id, to_version) in versions.iter().copied() {
                            if !has_function_version(&ast, function_id, to_version) {
                                continue;
                            }
                            pattern_matching::apply_patterns(
                                &mut ast,
                                function_id,
                                to_version,
                                &cast_min_patterns,
                                pattern_matching::AstPatternApplyPhase::AfterIteration,
                            )?;
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

        // Goto containment: convert forward goto/label pairs into structured if blocks.
        // Only run when control-flow cleanup is enabled (it restructures control flow).
        if config.control_flow_cleanup {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                goto_containment::contain_gotos(&mut ast, function_id, to_version)?;
            }
        }

        // Induction variable analysis: clean up for-loop conditions.
        // Only run when loop analysis is enabled.
        if config.loop_analyzation {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                induction_variable_analysis::analyze_induction_variables(
                    &mut ast,
                    function_id,
                    to_version,
                )?;
            }
        }

        if config.switch_reconstruction {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                switch_reconstruction::reconstruct_switches(&mut ast, function_id, to_version)?;
            }
        }

        // Early return normalization: convert if(cond){return} else{body} to guard clauses.
        if config.early_return_normalization {
            if config.use_embedded_passes {
                for (function_id, to_version) in versions.iter().copied() {
                    if !has_function_version(&ast, function_id, to_version) {
                        continue;
                    }
                    pattern_matching::embedded::early_return_normalization::normalize_early_returns(
                        &mut ast,
                        function_id,
                        to_version,
                    )?;
                }
            } else {
                let early_ret_pat = pattern_matching::AstPattern::predefined_pattern(
                    "early-return-normalization.fb",
                )
                .unwrap();
                for (function_id, to_version) in versions.iter().copied() {
                    if !has_function_version(&ast, function_id, to_version) {
                        continue;
                    }
                    pattern_matching::apply_patterns(
                        &mut ast,
                        function_id,
                        to_version,
                        std::slice::from_ref(&early_ret_pat),
                        pattern_matching::AstPatternApplyPhase::AfterOptimization,
                    )?;
                }
            }
        }

        // Temporary elimination: inline single-use non-pure temporaries.
        if config.expression_inlining {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                temporary_elimination::eliminate_temporaries(&mut ast, function_id, to_version)?;
            }
        }

        // Variable coalescing: merge non-interfering same-type variables.
        if config.collapse_unused_varaible {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                variable_coalescing::coalesce_variables(&mut ast, function_id, to_version)?;
            }
        }

        // Lifetime scoping: merge uninitialized declarations with first assignment.
        if config.lifetime_scoping {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                lifetime_scoping::narrow_lifetimes(&mut ast, function_id, to_version)?;
            }
        }

        // Name recovery: assign meaningful names to unnamed variables.
        if config.name_recovery {
            for (function_id, to_version) in versions.iter().copied() {
                if !has_function_version(&ast, function_id, to_version) {
                    continue;
                }
                name_recovery::recover_names(&mut ast, function_id, to_version)?;
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
