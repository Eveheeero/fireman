pub mod control_flow_graph;
pub mod data_access;
pub mod datatype;
pub mod ir_function;
pub mod ir_to_ast;
pub mod variables;

pub use control_flow_graph::{ControlFlowGraph, ControlFlowGraphAnalyzer};
pub use data_access::analyze_data_access;
pub use datatype::{DataType, KnownDataType, analyze_datatype};
pub use ir_function::IrFunction;
pub use ir_to_ast::generate_ast;
pub use variables::{IrVariable, analyze_variables};
