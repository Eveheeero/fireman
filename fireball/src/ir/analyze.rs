pub mod block_grouper;
pub mod data_access;
pub mod datatype;
pub mod dominator;
pub mod ir_function;
pub mod ir_to_ast;
pub mod variables;

pub use block_grouper::{BlockGroup, BlockGrouper};
pub use data_access::analyze_data_access;
pub use datatype::{DataType, KnownDataType, analyze_datatype};
pub use dominator::{
    ControlDependence, ControlFlowGraph, DominanceFrontier, DominatorTree,
    FunctionControlFlowAnalysis, LoopInfo, NaturalLoop, PostDominatorTree,
    analyze_control_dependence, analyze_dominators, analyze_function_control_flow, analyze_loops,
    analyze_postdominators, infer_entry_block_id,
};
pub use ir_function::IrFunction;
pub use ir_to_ast::{generate_ast, generate_ast_with_pre_defined_symbols};
pub use variables::{IrVariable, analyze_variables};
