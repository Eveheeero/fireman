use super::AstPattern;

const PREDEFINED_REMOVE_EMPTY_STATEMENTS_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-optimization/remove-empty-statements.fb");
const PREDEFINED_PRUNE_EMPTY_ELSE_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-optimization/prune-empty-else.fb");
const PREDEFINED_COLLAPSE_EMPTY_BLOCKS_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-optimization/collapse-empty-blocks.fb");
const PREDEFINED_FLATTEN_BLOCKS_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-iteration/flatten-blocks.fb");
const PREDEFINED_ERROR_CLEANUP_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-optimization/error-cleanup.fb");
const PREDEFINED_EXAMPLE_04_SCRIPT_AND_LOGS_FB: &str =
    include_str!("../../../../../patterns/examples/script_and_logs.fb");
const PREDEFINED_EXAMPLE_05_ALL_SYNTAX_FB: &str =
    include_str!("../../../../../patterns/examples/all_syntax.fb");
const PREDEFINED_TERNARY_RECOVERY_FB: &str =
    include_str!("../../../../../patterns/recovery/after-iteration/ternary-recovery.fb");
const PREDEFINED_IF_CONVERSION_REVERSAL_FB: &str =
    include_str!("../../../../../patterns/recovery/after-iteration/if-conversion-reversal.fb");
const PREDEFINED_BOOLEAN_RECOVERY_FB: &str =
    include_str!("../../../../../patterns/optimization/after-iteration/boolean-recovery.fb");
const PREDEFINED_CAST_MINIMIZATION_FB: &str =
    include_str!("../../../../../patterns/optimization/after-iteration/cast-minimization.fb");
const PREDEFINED_OPERATOR_CANONICALIZATION_FB: &str = include_str!(
    "../../../../../patterns/optimization/after-iteration/operator-canonicalization.fb"
);
const PREDEFINED_EARLY_RETURN_NORMALIZATION_FB: &str = include_str!(
    "../../../../../patterns/optimization/after-optimization/early-return-normalization.fb"
);
const PREDEFINED_IDENTITY_SIMPLIFICATION_FB: &str =
    include_str!("../../../../../patterns/optimization/after-iteration/identity-simplification.fb");
const PREDEFINED_DEAD_BRANCH_ELIMINATION_FB: &str =
    include_str!("../../../../../patterns/optimization/after-iteration/dead-branch-elimination.fb");
const PREDEFINED_CONSTANT_FOLDING_LITERALS_FB: &str = include_str!(
    "../../../../../patterns/optimization/after-iteration/constant-folding-literals.fb"
);
const PREDEFINED_CONSTANT_REASSOCIATION_FB: &str =
    include_str!("../../../../../patterns/optimization/after-iteration/constant-reassociation.fb");
const PREDEFINED_TAIL_CALL_MERGE_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-iteration/tail-call-merge.fb");
const PREDEFINED_BRANCH_INVERSION_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-iteration/branch-inversion.fb");
const PREDEFINED_ROTATION_RECOVERY_FB: &str =
    include_str!("../../../../../patterns/recognition/after-iteration/rotation-recovery.fb");
const PREDEFINED_STRENGTH_REDUCTION_FB: &str =
    include_str!("../../../../../patterns/recognition/after-iteration/strength-reduction.fb");
const PREDEFINED_MERGE_SAME_CONDITION_IFS_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-iteration/merge-same-condition-ifs.fb");
const PREDEFINED_MAGIC_DIVISION_RECOVERY_FB: &str =
    include_str!("../../../../../patterns/recognition/after-iteration/magic-division-recovery.fb");

// ── New suppression patterns (beforeIrAnalyzation) ──
const PREDEFINED_REGISTER_SPILL_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/register-spill-suppression.fb"
);
const PREDEFINED_SHADOW_SPACE_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/shadow-space-suppression.fb"
);
const PREDEFINED_RETPOLINE_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/retpoline-suppression.fb"
);
const PREDEFINED_SPECTRE_FENCE_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/spectre-fence-suppression.fb"
);
const PREDEFINED_SANITIZER_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/sanitizer-suppression.fb"
);
const PREDEFINED_COVERAGE_INSTRUMENTATION_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/coverage-instrumentation-suppression.fb"
);
const PREDEFINED_FUZZER_HOOK_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/fuzzer-hook-suppression.fb"
);
const PREDEFINED_SHADOW_CALLSTACK_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/shadow-callstack-suppression.fb"
);
const PREDEFINED_SAFESTACK_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/safestack-suppression.fb"
);
const PREDEFINED_STACK_CLASH_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/before-ir-analyzation/stack-clash-suppression.fb"
);
const PREDEFINED_SECURITY_SCAFFOLD_SUPPRESSION_FB: &str = include_str!(
    "../../../../../patterns/suppression/after-iteration/security-scaffold-suppression.fb"
);

// ── New cleanup patterns (afterOptimization) ──
const PREDEFINED_REDUNDANT_RETURN_ELIMINATION_FB: &str = include_str!(
    "../../../../../patterns/cleanup/after-optimization/redundant-return-elimination.fb"
);
const PREDEFINED_SINGLE_ARM_IF_CLEANUP_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-optimization/single-arm-if-cleanup.fb");
const PREDEFINED_REDUNDANT_BLOCK_UNWRAP_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-optimization/redundant-block-unwrap.fb");

// ── New optimization patterns (afterIteration) ──
const PREDEFINED_REDUNDANT_CAST_ELIMINATION_FB: &str = include_str!(
    "../../../../../patterns/optimization/after-iteration/redundant-cast-elimination.fb"
);
const PREDEFINED_NULL_CHECK_CANONICALIZATION_FB: &str = include_str!(
    "../../../../../patterns/optimization/after-iteration/null-check-canonicalization.fb"
);
const PREDEFINED_ASSERTION_RECOVERY_FB: &str =
    include_str!("../../../../../patterns/optimization/after-iteration/assertion-recovery.fb");
const PREDEFINED_TERNARY_TO_MINMAX_FB: &str =
    include_str!("../../../../../patterns/optimization/after-iteration/ternary-to-minmax.fb");

// ── New recognition patterns (afterIteration) ──
const PREDEFINED_DEREF_ADDRESSOF_CLEANUP_FB: &str =
    include_str!("../../../../../patterns/recognition/after-iteration/deref-addressof-cleanup.fb");
const PREDEFINED_DO_WHILE_RECOVERY_FB: &str =
    include_str!("../../../../../patterns/recovery/after-iteration/do-while-recovery.fb");
const PREDEFINED_CLAMP_RECOVERY_FB: &str =
    include_str!("../../../../../patterns/optimization/after-iteration/clamp-recovery.fb");
const PREDEFINED_LOOP_CLEANUP_FB: &str =
    include_str!("../../../../../patterns/cleanup/after-iteration/loop-cleanup.fb");

pub(super) fn predefined_patterns() -> Vec<AstPattern> {
    vec![
        AstPattern::from_predefined_include("flatten-blocks.fb", PREDEFINED_FLATTEN_BLOCKS_FB),
        AstPattern::from_predefined_include(
            "collapse-empty-blocks.fb",
            PREDEFINED_COLLAPSE_EMPTY_BLOCKS_FB,
        ),
        AstPattern::from_predefined_include("error-cleanup.fb", PREDEFINED_ERROR_CLEANUP_FB),
        AstPattern::from_predefined_include(
            "remove-empty-statements.fb",
            PREDEFINED_REMOVE_EMPTY_STATEMENTS_FB,
        ),
        AstPattern::from_predefined_include("prune-empty-else.fb", PREDEFINED_PRUNE_EMPTY_ELSE_FB),
        AstPattern::from_predefined_include("ternary-recovery.fb", PREDEFINED_TERNARY_RECOVERY_FB),
        AstPattern::from_predefined_include(
            "if-conversion-reversal.fb",
            PREDEFINED_IF_CONVERSION_REVERSAL_FB,
        ),
        AstPattern::from_predefined_include("boolean-recovery.fb", PREDEFINED_BOOLEAN_RECOVERY_FB),
        AstPattern::from_predefined_include(
            "cast-minimization.fb",
            PREDEFINED_CAST_MINIMIZATION_FB,
        ),
        AstPattern::from_predefined_include(
            "operator-canonicalization.fb",
            PREDEFINED_OPERATOR_CANONICALIZATION_FB,
        ),
        AstPattern::from_predefined_include(
            "early-return-normalization.fb",
            PREDEFINED_EARLY_RETURN_NORMALIZATION_FB,
        ),
        AstPattern::from_predefined_include(
            "identity-simplification.fb",
            PREDEFINED_IDENTITY_SIMPLIFICATION_FB,
        ),
        AstPattern::from_predefined_include(
            "dead-branch-elimination.fb",
            PREDEFINED_DEAD_BRANCH_ELIMINATION_FB,
        ),
        AstPattern::from_predefined_include(
            "constant-folding-literals.fb",
            PREDEFINED_CONSTANT_FOLDING_LITERALS_FB,
        ),
        AstPattern::from_predefined_include(
            "constant-reassociation.fb",
            PREDEFINED_CONSTANT_REASSOCIATION_FB,
        ),
        AstPattern::from_predefined_include("tail-call-merge.fb", PREDEFINED_TAIL_CALL_MERGE_FB),
        AstPattern::from_predefined_include("branch-inversion.fb", PREDEFINED_BRANCH_INVERSION_FB),
        AstPattern::from_predefined_include(
            "rotation-recovery.fb",
            PREDEFINED_ROTATION_RECOVERY_FB,
        ),
        AstPattern::from_predefined_include(
            "strength-reduction.fb",
            PREDEFINED_STRENGTH_REDUCTION_FB,
        ),
        AstPattern::from_predefined_include(
            "merge-same-condition-ifs.fb",
            PREDEFINED_MERGE_SAME_CONDITION_IFS_FB,
        ),
        AstPattern::from_predefined_include(
            "magic-division-recovery.fb",
            PREDEFINED_MAGIC_DIVISION_RECOVERY_FB,
        ),
        // ── New suppression patterns ──
        AstPattern::from_predefined_include(
            "register-spill-suppression.fb",
            PREDEFINED_REGISTER_SPILL_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "shadow-space-suppression.fb",
            PREDEFINED_SHADOW_SPACE_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "retpoline-suppression.fb",
            PREDEFINED_RETPOLINE_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "spectre-fence-suppression.fb",
            PREDEFINED_SPECTRE_FENCE_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "sanitizer-suppression.fb",
            PREDEFINED_SANITIZER_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "coverage-instrumentation-suppression.fb",
            PREDEFINED_COVERAGE_INSTRUMENTATION_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "fuzzer-hook-suppression.fb",
            PREDEFINED_FUZZER_HOOK_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "shadow-callstack-suppression.fb",
            PREDEFINED_SHADOW_CALLSTACK_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "safestack-suppression.fb",
            PREDEFINED_SAFESTACK_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "stack-clash-suppression.fb",
            PREDEFINED_STACK_CLASH_SUPPRESSION_FB,
        ),
        AstPattern::from_predefined_include(
            "security-scaffold-suppression.fb",
            PREDEFINED_SECURITY_SCAFFOLD_SUPPRESSION_FB,
        ),
        // ── New cleanup patterns ──
        AstPattern::from_predefined_include(
            "redundant-return-elimination.fb",
            PREDEFINED_REDUNDANT_RETURN_ELIMINATION_FB,
        ),
        AstPattern::from_predefined_include(
            "single-arm-if-cleanup.fb",
            PREDEFINED_SINGLE_ARM_IF_CLEANUP_FB,
        ),
        AstPattern::from_predefined_include(
            "redundant-block-unwrap.fb",
            PREDEFINED_REDUNDANT_BLOCK_UNWRAP_FB,
        ),
        // ── New optimization patterns ──
        AstPattern::from_predefined_include(
            "redundant-cast-elimination.fb",
            PREDEFINED_REDUNDANT_CAST_ELIMINATION_FB,
        ),
        AstPattern::from_predefined_include(
            "null-check-canonicalization.fb",
            PREDEFINED_NULL_CHECK_CANONICALIZATION_FB,
        ),
        AstPattern::from_predefined_include(
            "assertion-recovery.fb",
            PREDEFINED_ASSERTION_RECOVERY_FB,
        ),
        AstPattern::from_predefined_include(
            "ternary-to-minmax.fb",
            PREDEFINED_TERNARY_TO_MINMAX_FB,
        ),
        // ── New recognition patterns ──
        AstPattern::from_predefined_include(
            "deref-addressof-cleanup.fb",
            PREDEFINED_DEREF_ADDRESSOF_CLEANUP_FB,
        ),
        AstPattern::from_predefined_include("clamp-recovery.fb", PREDEFINED_CLAMP_RECOVERY_FB),
        AstPattern::from_predefined_include("loop-cleanup.fb", PREDEFINED_LOOP_CLEANUP_FB),
    ]
}

pub(super) fn predefined_pattern(name: &str) -> Option<AstPattern> {
    // Accept both full paths ("patterns/.../foo.fb") and bare names ("foo.fb").
    let short_name = name.rsplit('/').next().unwrap_or(name);
    match short_name {
        "flatten-blocks.fb" => Some(AstPattern::from_predefined_include(
            "flatten-blocks.fb",
            PREDEFINED_FLATTEN_BLOCKS_FB,
        )),
        "collapse-empty-blocks.fb" => Some(AstPattern::from_predefined_include(
            "collapse-empty-blocks.fb",
            PREDEFINED_COLLAPSE_EMPTY_BLOCKS_FB,
        )),
        "error-cleanup.fb" => Some(AstPattern::from_predefined_include(
            "error-cleanup.fb",
            PREDEFINED_ERROR_CLEANUP_FB,
        )),
        "remove-empty-statements.fb" => Some(AstPattern::from_predefined_include(
            "remove-empty-statements.fb",
            PREDEFINED_REMOVE_EMPTY_STATEMENTS_FB,
        )),
        "prune-empty-else.fb" => Some(AstPattern::from_predefined_include(
            "prune-empty-else.fb",
            PREDEFINED_PRUNE_EMPTY_ELSE_FB,
        )),
        "script_and_logs.fb" => Some(AstPattern::from_predefined_include(
            "script_and_logs.fb",
            PREDEFINED_EXAMPLE_04_SCRIPT_AND_LOGS_FB,
        )),
        "all_syntax.fb" => Some(AstPattern::from_predefined_include(
            "all_syntax.fb",
            PREDEFINED_EXAMPLE_05_ALL_SYNTAX_FB,
        )),
        "ternary-recovery.fb" => Some(AstPattern::from_predefined_include(
            "ternary-recovery.fb",
            PREDEFINED_TERNARY_RECOVERY_FB,
        )),
        "if-conversion-reversal.fb" => Some(AstPattern::from_predefined_include(
            "if-conversion-reversal.fb",
            PREDEFINED_IF_CONVERSION_REVERSAL_FB,
        )),
        "boolean-recovery.fb" => Some(AstPattern::from_predefined_include(
            "boolean-recovery.fb",
            PREDEFINED_BOOLEAN_RECOVERY_FB,
        )),
        "cast-minimization.fb" => Some(AstPattern::from_predefined_include(
            "cast-minimization.fb",
            PREDEFINED_CAST_MINIMIZATION_FB,
        )),
        "operator-canonicalization.fb" => Some(AstPattern::from_predefined_include(
            "operator-canonicalization.fb",
            PREDEFINED_OPERATOR_CANONICALIZATION_FB,
        )),
        "early-return-normalization.fb" => Some(AstPattern::from_predefined_include(
            "early-return-normalization.fb",
            PREDEFINED_EARLY_RETURN_NORMALIZATION_FB,
        )),
        "identity-simplification.fb" => Some(AstPattern::from_predefined_include(
            "identity-simplification.fb",
            PREDEFINED_IDENTITY_SIMPLIFICATION_FB,
        )),
        "dead-branch-elimination.fb" => Some(AstPattern::from_predefined_include(
            "dead-branch-elimination.fb",
            PREDEFINED_DEAD_BRANCH_ELIMINATION_FB,
        )),
        "constant-folding-literals.fb" => Some(AstPattern::from_predefined_include(
            "constant-folding-literals.fb",
            PREDEFINED_CONSTANT_FOLDING_LITERALS_FB,
        )),
        "constant-reassociation.fb" => Some(AstPattern::from_predefined_include(
            "constant-reassociation.fb",
            PREDEFINED_CONSTANT_REASSOCIATION_FB,
        )),
        "tail-call-merge.fb" => Some(AstPattern::from_predefined_include(
            "tail-call-merge.fb",
            PREDEFINED_TAIL_CALL_MERGE_FB,
        )),
        "branch-inversion.fb" => Some(AstPattern::from_predefined_include(
            "branch-inversion.fb",
            PREDEFINED_BRANCH_INVERSION_FB,
        )),
        "rotation-recovery.fb" => Some(AstPattern::from_predefined_include(
            "rotation-recovery.fb",
            PREDEFINED_ROTATION_RECOVERY_FB,
        )),
        "strength-reduction.fb" => Some(AstPattern::from_predefined_include(
            "strength-reduction.fb",
            PREDEFINED_STRENGTH_REDUCTION_FB,
        )),
        "merge-same-condition-ifs.fb" => Some(AstPattern::from_predefined_include(
            "merge-same-condition-ifs.fb",
            PREDEFINED_MERGE_SAME_CONDITION_IFS_FB,
        )),
        "magic-division-recovery.fb" => Some(AstPattern::from_predefined_include(
            "magic-division-recovery.fb",
            PREDEFINED_MAGIC_DIVISION_RECOVERY_FB,
        )),
        // ── New suppression patterns ──
        "register-spill-suppression.fb" => Some(AstPattern::from_predefined_include(
            "register-spill-suppression.fb",
            PREDEFINED_REGISTER_SPILL_SUPPRESSION_FB,
        )),
        "shadow-space-suppression.fb" => Some(AstPattern::from_predefined_include(
            "shadow-space-suppression.fb",
            PREDEFINED_SHADOW_SPACE_SUPPRESSION_FB,
        )),
        "retpoline-suppression.fb" => Some(AstPattern::from_predefined_include(
            "retpoline-suppression.fb",
            PREDEFINED_RETPOLINE_SUPPRESSION_FB,
        )),
        "spectre-fence-suppression.fb" => Some(AstPattern::from_predefined_include(
            "spectre-fence-suppression.fb",
            PREDEFINED_SPECTRE_FENCE_SUPPRESSION_FB,
        )),
        "sanitizer-suppression.fb" => Some(AstPattern::from_predefined_include(
            "sanitizer-suppression.fb",
            PREDEFINED_SANITIZER_SUPPRESSION_FB,
        )),
        "coverage-instrumentation-suppression.fb" => Some(AstPattern::from_predefined_include(
            "coverage-instrumentation-suppression.fb",
            PREDEFINED_COVERAGE_INSTRUMENTATION_SUPPRESSION_FB,
        )),
        "fuzzer-hook-suppression.fb" => Some(AstPattern::from_predefined_include(
            "fuzzer-hook-suppression.fb",
            PREDEFINED_FUZZER_HOOK_SUPPRESSION_FB,
        )),
        "shadow-callstack-suppression.fb" => Some(AstPattern::from_predefined_include(
            "shadow-callstack-suppression.fb",
            PREDEFINED_SHADOW_CALLSTACK_SUPPRESSION_FB,
        )),
        "safestack-suppression.fb" => Some(AstPattern::from_predefined_include(
            "safestack-suppression.fb",
            PREDEFINED_SAFESTACK_SUPPRESSION_FB,
        )),
        "stack-clash-suppression.fb" => Some(AstPattern::from_predefined_include(
            "stack-clash-suppression.fb",
            PREDEFINED_STACK_CLASH_SUPPRESSION_FB,
        )),
        "security-scaffold-suppression.fb" => Some(AstPattern::from_predefined_include(
            "security-scaffold-suppression.fb",
            PREDEFINED_SECURITY_SCAFFOLD_SUPPRESSION_FB,
        )),
        // ── New cleanup patterns ──
        "redundant-return-elimination.fb" => Some(AstPattern::from_predefined_include(
            "redundant-return-elimination.fb",
            PREDEFINED_REDUNDANT_RETURN_ELIMINATION_FB,
        )),
        "single-arm-if-cleanup.fb" => Some(AstPattern::from_predefined_include(
            "single-arm-if-cleanup.fb",
            PREDEFINED_SINGLE_ARM_IF_CLEANUP_FB,
        )),
        "redundant-block-unwrap.fb" => Some(AstPattern::from_predefined_include(
            "redundant-block-unwrap.fb",
            PREDEFINED_REDUNDANT_BLOCK_UNWRAP_FB,
        )),
        // ── New optimization patterns ──
        "redundant-cast-elimination.fb" => Some(AstPattern::from_predefined_include(
            "redundant-cast-elimination.fb",
            PREDEFINED_REDUNDANT_CAST_ELIMINATION_FB,
        )),
        "null-check-canonicalization.fb" => Some(AstPattern::from_predefined_include(
            "null-check-canonicalization.fb",
            PREDEFINED_NULL_CHECK_CANONICALIZATION_FB,
        )),
        "assertion-recovery.fb" => Some(AstPattern::from_predefined_include(
            "assertion-recovery.fb",
            PREDEFINED_ASSERTION_RECOVERY_FB,
        )),
        "ternary-to-minmax.fb" => Some(AstPattern::from_predefined_include(
            "ternary-to-minmax.fb",
            PREDEFINED_TERNARY_TO_MINMAX_FB,
        )),
        // ── New recognition patterns ──
        "deref-addressof-cleanup.fb" => Some(AstPattern::from_predefined_include(
            "deref-addressof-cleanup.fb",
            PREDEFINED_DEREF_ADDRESSOF_CLEANUP_FB,
        )),
        "do-while-recovery.fb" => Some(AstPattern::from_predefined_include(
            "do-while-recovery.fb",
            PREDEFINED_DO_WHILE_RECOVERY_FB,
        )),
        "clamp-recovery.fb" => Some(AstPattern::from_predefined_include(
            "clamp-recovery.fb",
            PREDEFINED_CLAMP_RECOVERY_FB,
        )),
        "loop-cleanup.fb" => Some(AstPattern::from_predefined_include(
            "loop-cleanup.fb",
            PREDEFINED_LOOP_CLEANUP_FB,
        )),
        _ => None,
    }
}
