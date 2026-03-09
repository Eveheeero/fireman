//! Structural AST pattern matching with named captures, predicates, and emit.
//!
//! Parses patterns like `If($cond, [Assignment(Variable($_, $v1), $a)], Some([Assignment(Variable($_, $v2), $b)]))`
//! into a `PatTree`, matches them against `AstStatement` nodes to produce `Captures`,
//! evaluates `where` predicates (e.g. `eq($v1, $v2)`), and constructs replacement
//! statements via `emit`.

mod node_name;
mod parser;
mod types;

mod construct;
mod matcher;
mod predicate;
mod transform;

// Re-export public API
pub use construct::{
    construct_emit_after_list, construct_statement, inject_captures_into_rhai_scope,
};
pub use matcher::match_statement;
pub use parser::parse_pattern;
pub use predicate::{eval_where, parse_where};
pub use transform::{
    annotate_expressions_in_stmts, transform_expressions_in_stmts,
    transform_expressions_in_stmts_builtin,
};
pub use types::{Captures, PatTree, WherePredicate};
