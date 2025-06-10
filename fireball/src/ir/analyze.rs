pub mod control_flow_graph;
pub mod data_access;
pub mod datatype;
pub mod enhanced_c_codegen;
pub mod ir_block_merger;
pub mod ir_to_c;
pub mod loop_analysis;
pub mod loop_conversion;
pub mod struct_reconstruction;
pub mod type_recovery;
pub mod variable_naming;
pub mod variables;

pub use control_flow_graph::{ControlFlowGraph, ControlFlowGraphAnalyzer};
pub use data_access::analyze_data_access;
pub use datatype::{DataType, KnownDataType, analyze_datatype};

pub use ir_block_merger::MergedIr;
pub use ir_to_c::{generate_c_ast, generate_enhanced_c};
pub use loop_analysis::{AnalyzedLoop, ComplexLoopAnalyzer, LoopPattern};
pub use struct_reconstruction::{ReconstructedStruct, StructReconstructionEngine};
pub use type_recovery::{InferredType, TypeInfo, TypeRecoveryEngine};
pub use variable_naming::{VariableName, VariableNamingEngine};
pub use variables::{IrVariable, analyze_variables};
