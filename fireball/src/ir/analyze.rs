pub mod control_flow_graph;
pub mod data_access;
pub mod datatype;
pub mod ir_block_merger;
pub mod ir_to_c;
pub mod variables;

pub use control_flow_graph::{ControlFlowGraph, ControlFlowGraphAnalyzer};
pub use data_access::analyze_data_access;
pub use datatype::{DataType, KnownDataType, analyze_datatype};
pub use ir_block_merger::MergedIr;
pub use ir_to_c::generate_c_ast;
pub use variables::{IrVariable, analyze_variables};
