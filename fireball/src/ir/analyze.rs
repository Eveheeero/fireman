pub mod control_flow_graph;
pub mod data_access;
pub mod datatype;
pub mod variables;

pub use control_flow_graph::{ControlFlowGraph, ControlFlowGraphAnalyzer};
pub use data_access::analyze_data_access;
pub use datatype::{analyze_datatype, DataType, KnownDataType};
pub use variables::{analyze_variables, IrVariable};
