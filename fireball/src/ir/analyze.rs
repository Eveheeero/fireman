pub mod array_shape;
pub mod block_grouper;
pub mod data_access;
pub mod data_dependence;
pub mod datatype;
pub mod dominator;
pub mod escape;
pub mod field_alias;
pub mod function_summary;
pub mod global_recovery;
pub mod ir_function;
pub mod ir_to_ast;
pub mod points_to;
pub mod slicer;
pub mod ssa;
pub mod struct_recovery;
pub mod structuring;
pub mod taint;
pub mod value_set;
pub mod variables;

pub use array_shape::{ArrayShapeCandidate, analyze_array_shapes};
pub use block_grouper::{BlockGroup, BlockGrouper};
pub use data_access::analyze_data_access;
pub use data_dependence::{DataDependenceGraph, analyze_data_dependence};
pub use datatype::{DataType, KnownDataType, analyze_datatype};
pub use dominator::{
    ControlDependence, ControlFlowGraph, DominanceFrontier, DominatorTree,
    FunctionControlFlowAnalysis, LoopInfo, NaturalLoop, PostDominatorTree,
    analyze_control_dependence, analyze_dominators, analyze_function_control_flow, analyze_loops,
    analyze_postdominators, infer_entry_block_id,
};
pub use escape::{EscapeAnalysis, analyze_pointer_escape};
pub use field_alias::{
    FieldAliasAnalysis, FieldProjection, FieldProjectionKey, analyze_field_alias,
};
pub use function_summary::{FunctionSummary, augment_with_escape, summarize_function};
pub use ir_function::IrFunction;
pub use ir_to_ast::{generate_ast, generate_ast_with_pre_defined_symbols};
pub use points_to::{AbstractLocation, PointsToSet, analyze_points_to};
pub use slicer::{ProgramSlice, SliceCriterion, backward_slice, forward_slice};
pub use struct_recovery::{AggregateCandidate, FieldCandidate, recover_aggregates};
pub use structuring::{CfgInterval, StructuredRegion, discover_intervals, structure_function};
pub use taint::{TaintAnalysis, TaintLabel, analyze_taint};
pub use value_set::{Interval, ValueSetResult, analyze_value_set};
pub use variables::{IrVariable, analyze_variables};
