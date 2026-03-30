//! Embedded Rust implementations of optimization passes.
//!
//! The on-disk layout mirrors `patterns/` by category and phase, while the
//! public re-exports below preserve the historical `embedded::<pass>` paths
//! used by `optimize.rs` and tests.

pub(crate) mod cleanup {
    pub(crate) mod after_iteration {
        pub(crate) mod control_flow_cleanup;
        pub(crate) mod flatten_blocks;
        pub(crate) mod loop_cleanup;
        pub(crate) mod prune_constant_conditions;
    }
    pub(crate) mod after_optimization {
        pub(crate) mod collapse_empty_blocks;
        pub(crate) mod prune_empty_else;
    }
}

pub(crate) mod optimization {
    pub(crate) mod after_iteration {
        pub(crate) mod assertion_recovery;
        pub(crate) mod boolean_recovery;
        pub(crate) mod cast_minimization;
        pub(crate) mod clamp_recovery;
        pub(crate) mod constant_folding;
        pub(crate) mod identity_simplification;
        pub(crate) mod operator_canonicalization;
    }

    pub(crate) mod after_optimization {
        pub(crate) mod early_return_normalization;
    }
}

pub(crate) mod recognition {
    pub(crate) mod after_iteration {
        pub(crate) mod bit_trick_recognition;
        pub(crate) mod magic_division_recovery;
    }
}

pub(crate) mod recovery {
    pub(crate) mod after_iteration {
        pub(crate) mod do_while_recovery;
        pub(crate) mod if_conversion_reversal;
        pub(crate) mod ternary_recovery;
    }
}

pub(crate) mod suppression {
    pub(crate) mod after_iteration {
        pub(crate) mod anti_debug_ast_suppression;
        pub(crate) mod logging_suppression;
        pub(crate) mod security_scaffold_suppression;
        pub(crate) mod static_guard_suppression;
    }

    pub(crate) mod before_ir_analyzation {
        pub(crate) mod alloca_suppression;
        pub(crate) mod anti_debug_suppression;
        pub(crate) mod atomic_fence_suppression;
        pub(crate) mod bare_metal_startup_suppression;
        pub(crate) mod loader_stub_suppression;
        pub(crate) mod plt_resolver_suppression;
        pub(crate) mod timing_check_suppression;
        pub(crate) mod tls_access_suppression;
        pub(crate) mod trampoline_suppression;
        pub(crate) mod veneer_suppression;
    }
}

pub(crate) mod test_utils;
