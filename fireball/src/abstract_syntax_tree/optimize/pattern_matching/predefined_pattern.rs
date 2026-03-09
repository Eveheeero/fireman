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
        _ => None,
    }
}
