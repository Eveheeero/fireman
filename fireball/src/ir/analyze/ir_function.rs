use crate::{
    core::{Block, Instruction},
    ir::{
        Ir, IrBlock,
        analyze::{
            CfgInterval, DataType, DominanceFrontier, StructuredRegion,
            analyze_function_control_flow, discover_intervals, infer_entry_block_id,
            structure_function,
        },
        data::IrDataAccess,
        utils::IrStatementDescriptorMap,
    },
    prelude::*,
};
use iceball::{
    Argument, Memory, Register, RelativeAddressingArgument, Statement, X64Statement,
    x64::register::X64Register,
};
use std::sync::Arc;

struct CfgShapeArtifacts {
    ssa: super::ssa::SsaFunction,
    structured: StructuredRegion,
    intervals: Vec<CfgInterval>,
}

fn run_cfg_shape_analysis(blocks: &[Arc<Block>]) -> Option<CfgShapeArtifacts> {
    let Some(entry_block_id) = infer_entry_block_id(blocks) else {
        debug!("Skip CFG shape analysis: failed to infer entry block");
        return None;
    };

    let analysis = analyze_function_control_flow(blocks, entry_block_id);
    let control_dependent_blocks = analysis
        .cfg()
        .block_ids()
        .iter()
        .filter(|&&block_id| {
            !analysis
                .control_dependence()
                .controlling_predicates_of(block_id)
                .is_empty()
        })
        .count();

    let reducible = analysis.cfg().is_reducible(analysis.dominators());
    let dominance_frontier = DominanceFrontier::compute(analysis.dominators());
    let frontier_count = analysis
        .cfg()
        .block_ids()
        .iter()
        .filter(|&&id| dominance_frontier.has_frontier(id))
        .count();

    debug!(
        "CFG shape analysis: entry={}, blocks={}, exits={}, back_edges={}, natural_loops={}, control_dependent_blocks={}, reducible={}, df_nodes={}",
        entry_block_id,
        analysis.cfg().block_ids().len(),
        analysis.postdominators().cfg().exit_block_ids().len(),
        analysis.dominators().back_edges().len(),
        analysis.loops().loops().len(),
        control_dependent_blocks,
        reducible,
        frontier_count,
    );

    // Function boundary / compiler fingerprinting
    if let Some(entry_block) = blocks.iter().find(|b| b.get_id() == entry_block_id) {
        let prologue = detect_prologue_pattern(entry_block);
        let compiler = detect_compiler_hint(entry_block);
        if let Some(prologue) = prologue {
            debug!("Function prologue: {}", prologue);
        }
        if let Some(compiler) = compiler {
            debug!("Compiler hint: {}", compiler);
        }
    }

    // L74: Stack realignment detection (and rsp, -N)
    if let Some(entry_block) = blocks.iter().find(|b| b.get_id() == entry_block_id) {
        if let Some(alignment) = detect_stack_realignment(entry_block) {
            debug!("Stack realignment detected: aligned to {} bytes", alignment);
        }
    }

    // L72: FPO local inference — count SP-relative accesses for frame layout
    if let Some(entry_block) = blocks.iter().find(|b| b.get_id() == entry_block_id) {
        let prologue = detect_prologue_pattern(entry_block);
        if prologue.is_some_and(|p| p.contains("FPO")) {
            let sp_access_count = count_sp_relative_accesses(blocks);
            if sp_access_count > 0 {
                debug!(
                    "FPO function: {} SP-relative memory accesses (candidate locals)",
                    sp_access_count
                );
            }
        }
    }

    // L194: Multi-entry detection
    let multi_entries = analysis.cfg().multi_entry_blocks(blocks);
    if !multi_entries.is_empty() {
        debug!(
            "Multi-entry blocks detected (shared tails): {:?}",
            multi_entries
        );
    }

    // L37/L39: SSA summary and rename analysis spike.
    let ssa_function = super::ssa::construct_ssa(blocks, analysis.dominators());
    super::ssa::log_ssa_analysis(&ssa_function);

    // L60: CFG structuring using the existing region builder.
    let structured = structure_function(blocks, &analysis);
    let intervals = discover_intervals(analysis.cfg());
    let multi_block_intervals = intervals
        .iter()
        .filter(|interval| interval.block_count() > 1)
        .count();
    let largest_interval = intervals
        .iter()
        .map(CfgInterval::block_count)
        .max()
        .unwrap_or(0);
    debug!(
        "CFG structuring: {} blocks, {} constructs, {} SESE regions, {} intervals ({} multi-block, max {} blocks), {} gotos, {} labels, {} unresolved goto targets",
        structured.block_count(),
        structured.construct_count(),
        structured.sese_region_count(),
        intervals.len(),
        multi_block_intervals,
        largest_interval,
        structured.goto_count(),
        structured.label_count(),
        structured.unresolved_goto_target_count(),
    );

    // L192: Hot-cold chunk detection (gap > 4KB suggests split)
    let chunks = analysis.cfg().detect_address_gap_chunks(blocks, 4096);
    if chunks.len() > 1 {
        debug!(
            "Potential hot-cold split: {} address-separated chunks",
            chunks.len()
        );
    }

    Some(CfgShapeArtifacts {
        ssa: ssa_function,
        structured,
        intervals,
    })
}

/// L11: Detect function prologue patterns from the first few instructions.
fn detect_prologue_pattern(block: &Block) -> Option<&'static str> {
    let instructions = block.get_instructions();
    let stmts: Vec<X64Statement> = instructions
        .iter()
        .take(6)
        .filter_map(|inst| match inst.inner().statement {
            Ok(Statement::X64(s)) => Some(s),
            _ => None,
        })
        .collect();

    if stmts.len() < 2 {
        return None;
    }

    // push rbp; mov rbp, rsp — standard frame pointer prologue
    if stmts[0] == X64Statement::Push && stmts[1] == X64Statement::Mov {
        return Some("standard frame-pointer prologue (push rbp; mov rbp, rsp)");
    }

    // push rbp; push rbx; ... — callee-saved register preservation
    if stmts[0] == X64Statement::Push && stmts.get(1) == Some(&X64Statement::Push) {
        return Some("callee-saved register prologue (multiple push)");
    }

    // sub rsp, N — frame-pointer-omitted (FPO) prologue
    if stmts[0] == X64Statement::Sub {
        return Some("FPO prologue (sub rsp, imm)");
    }

    // endbr64; push rbp — CET-enabled prologue
    if stmts[0] == X64Statement::Endbr64 && stmts.get(1) == Some(&X64Statement::Push) {
        return Some("CET-enabled prologue (endbr64; push rbp)");
    }

    // endbr64; sub rsp — CET + FPO
    if stmts[0] == X64Statement::Endbr64 && stmts.get(1) == Some(&X64Statement::Sub) {
        return Some("CET + FPO prologue (endbr64; sub rsp)");
    }

    None
}

/// L19: Detect compiler family from instruction patterns and calling conventions.
fn detect_compiler_hint(block: &Block) -> Option<&'static str> {
    let instructions = block.get_instructions();
    let stmts: Vec<X64Statement> = instructions
        .iter()
        .take(10)
        .filter_map(|inst| match inst.inner().statement {
            Ok(Statement::X64(s)) => Some(s),
            _ => None,
        })
        .collect();

    // MSVC: endbr not used; often starts with mov [rsp+8], rcx (home space)
    // or sub rsp, imm; mov [rsp+XX], ...
    // GCC/Clang: endbr64 at function entry when CET is enabled
    if stmts.first() == Some(&X64Statement::Endbr64) {
        return Some("likely GCC/Clang (CET endbr64 at entry)");
    }

    // MSVC often uses `mov qword ptr [rsp+X], reg` as first instruction
    // for shadow space parameters. Check for mov as first instruction
    // without a preceding push rbp (MSVC x64 typically omits frame pointer).
    if stmts.first() == Some(&X64Statement::Mov) && stmts.get(1) == Some(&X64Statement::Mov) {
        return Some("likely MSVC x64 (shadow space parameter stores at entry)");
    }

    // Leaf function: no push/sub, starts directly with computation
    let has_frame_setup = stmts.iter().take(3).any(|s| {
        matches!(
            s,
            X64Statement::Push | X64Statement::Sub | X64Statement::Endbr64
        )
    });
    if !has_frame_setup && !stmts.is_empty() {
        return Some("leaf function (no frame setup)");
    }

    None
}

/// L74: Detect `and rsp, -N` stack realignment in the function prologue.
/// Returns the alignment value (e.g. 16, 32, 64) if found.
fn detect_stack_realignment(block: &Block) -> Option<u64> {
    let instructions = block.get_instructions();
    // Look for AND instruction in first 8 instructions
    for inst in instructions.iter().take(8) {
        let inner = inst.inner();
        let Ok(Statement::X64(X64Statement::And)) = inner.statement else {
            continue;
        };
        // Check that one operand is RSP/ESP and another is a constant mask
        let has_sp = inner.arguments.iter().any(|arg| {
            matches!(
                arg,
                Argument::Register(Register::X64(X64Register::Rsp | X64Register::Esp))
            )
        });
        if !has_sp {
            continue;
        }
        // Extract the immediate constant operand — it's a negative power of 2 mask
        // e.g., 0xfffffffffffffff0 = -16, 0xffffffffffffffe0 = -32
        for arg in inner.arguments.iter() {
            if let Argument::Constant(val) = arg {
                let alignment = (!val).wrapping_add(1);
                if alignment.is_power_of_two() && alignment >= 16 {
                    return Some(alignment);
                }
            }
        }
    }
    None
}

/// L72: Count SP-relative memory accesses across all blocks (for FPO functions).
/// Each unique [rsp+offset] pattern suggests a candidate local variable.
fn count_sp_relative_accesses(blocks: &[Arc<Block>]) -> usize {
    let mut count = 0usize;
    for block in blocks {
        for inst in block.get_instructions().iter() {
            let has_sp_mem = inst.inner().arguments.iter().any(|arg| {
                if let Argument::Memory(Memory::RelativeAddressing(parts)) = arg {
                    parts.iter().any(|p| {
                        matches!(
                            p,
                            RelativeAddressingArgument::Register(Register::X64(
                                X64Register::Rsp | X64Register::Esp
                            ))
                        )
                    })
                } else {
                    false
                }
            });
            if has_sp_mem {
                count += 1;
            }
        }
    }
    count
}

pub fn generate_ir_function(blocks: &[Arc<Block>]) -> IrFunction {
    info!("Generate IR function from {} blocks", blocks.len());
    let cfg_artifacts = run_cfg_shape_analysis(blocks);
    let points_to = super::points_to::analyze_points_to(blocks);
    super::points_to::log_points_to_analysis(&points_to);
    let data_dependence = super::data_dependence::analyze_data_dependence(blocks);
    super::data_dependence::log_data_dependence_analysis(&data_dependence);
    let aggregates = super::struct_recovery::recover_aggregates(blocks);
    super::struct_recovery::log_aggregate_recovery(&aggregates);
    let value_set = super::value_set::analyze_value_set(blocks);
    super::value_set::log_value_set_analysis(&value_set);
    let taint = super::taint::analyze_taint(blocks);
    super::taint::log_taint_analysis(&taint);
    let return_slice =
        super::slicer::backward_slice(blocks, super::slicer::SliceCriterion::ReturnValue);
    let parameter_forward_slice =
        super::slicer::forward_slice(blocks, super::slicer::SliceCriterion::Parameters);

    // Merge IR from all blocks in execution order
    let mut combined_ir = Vec::new();
    let mut ir_block_ids = Vec::new();
    let mut instructions = Vec::new();
    for block in blocks {
        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };
        // TODO should we return err when ir not analyzed?
        // If block not analyzed, skip
        ir_block_ids.extend(std::iter::repeat(block.get_id()).take(ir_block.ir().len()));
        combined_ir.extend(ir_block.ir().iter().cloned());
        // if ir not sent, instruction must not be sent, it causes invalid ir analysis
        instructions.extend(block.get_instructions().iter().cloned());
    }

    let total_slice_statements: usize = combined_ir
        .iter()
        .filter_map(|ir| ir.statements.as_ref().map(|statements| statements.len()))
        .sum();
    super::slicer::log_slice_analysis(&return_slice, total_slice_statements);
    super::slicer::log_slice_analysis(&parameter_forward_slice, total_slice_statements);

    debug!("IR Function size: {}", combined_ir.len());
    // Analyze IR function
    let mut ir_block = IrBlock::new(combined_ir.clone(), instructions.into());
    let instructions = ir_block.instructions().clone();
    ir_block.analyze_data_access();
    ir_block.analyze_datatypes();
    ir_block
        .analyze_variables()
        .expect("Variable analysis failed");

    // Collect merged variables
    let vars = ir_block.variables.unwrap();
    let merged_vars = vars
        .into_iter()
        .map(|v| IrFunctionVariable {
            data_type: v.data_type,
            data_accesses: v.into_data_accesses(),
        })
        .collect();

    info!("IrFunction generation completed");
    let (ssa, structured, cfg_intervals) = match cfg_artifacts {
        Some(artifacts) => (
            Some(artifacts.ssa),
            Some(artifacts.structured),
            Some(artifacts.intervals),
        ),
        None => (None, None, None),
    };
    IrFunction {
        instructions,
        ir: combined_ir,
        ir_block_ids: ir_block_ids.into(),
        variables: merged_vars,
        points_to: Some(points_to),
        data_dependence: Some(data_dependence),
        aggregates: Some(aggregates),
        value_set: Some(value_set),
        taint: Some(taint),
        return_slice: Some(return_slice),
        parameter_forward_slice: Some(parameter_forward_slice),
        ssa,
        structured,
        cfg_intervals,
    }
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    instructions: Arc<[Instruction]>,
    ir: Vec<Ir>,
    ir_block_ids: Arc<[usize]>,
    variables: Vec<IrFunctionVariable>,
    points_to: Option<super::points_to::PointsToSet>,
    data_dependence: Option<super::data_dependence::DataDependenceGraph>,
    aggregates: Option<Vec<super::struct_recovery::AggregateCandidate>>,
    value_set: Option<super::value_set::ValueSetResult>,
    taint: Option<super::taint::TaintAnalysis>,
    return_slice: Option<super::slicer::ProgramSlice>,
    parameter_forward_slice: Option<super::slicer::ProgramSlice>,
    ssa: Option<super::ssa::SsaFunction>,
    structured: Option<StructuredRegion>,
    cfg_intervals: Option<Vec<super::structuring::CfgInterval>>,
}

impl IrFunction {
    pub fn new(
        instructions: Arc<[Instruction]>,
        ir: Vec<Ir>,
        variables: Vec<IrFunctionVariable>,
    ) -> Self {
        Self {
            instructions,
            ir_block_ids: vec![0; ir.len()].into(),
            ir,
            variables,
            points_to: None,
            data_dependence: None,
            aggregates: None,
            value_set: None,
            taint: None,
            return_slice: None,
            parameter_forward_slice: None,
            ssa: None,
            structured: None,
            cfg_intervals: None,
        }
    }
    pub fn get_ir(&self) -> &Vec<Ir> {
        &self.ir
    }
    pub fn get_ir_block_ids(&self) -> &Arc<[usize]> {
        &self.ir_block_ids
    }
    pub fn get_instructions(&self) -> &Arc<[Instruction]> {
        &self.instructions
    }
    pub fn get_variables(&self) -> &Vec<IrFunctionVariable> {
        &self.variables
    }

    pub fn get_points_to(&self) -> Option<&super::points_to::PointsToSet> {
        self.points_to.as_ref()
    }

    pub fn get_data_dependence(&self) -> Option<&super::data_dependence::DataDependenceGraph> {
        self.data_dependence.as_ref()
    }

    pub fn get_aggregates(&self) -> Option<&[super::struct_recovery::AggregateCandidate]> {
        self.aggregates.as_deref()
    }

    pub fn get_value_set(&self) -> Option<&super::value_set::ValueSetResult> {
        self.value_set.as_ref()
    }

    pub fn get_taint(&self) -> Option<&super::taint::TaintAnalysis> {
        self.taint.as_ref()
    }

    pub fn get_return_slice(&self) -> Option<&super::slicer::ProgramSlice> {
        self.return_slice.as_ref()
    }

    pub fn get_parameter_forward_slice(&self) -> Option<&super::slicer::ProgramSlice> {
        self.parameter_forward_slice.as_ref()
    }

    pub fn get_ssa(&self) -> Option<&super::ssa::SsaFunction> {
        self.ssa.as_ref()
    }

    pub fn get_structured(&self) -> Option<&StructuredRegion> {
        self.structured.as_ref()
    }

    pub fn get_cfg_intervals(&self) -> Option<&[super::structuring::CfgInterval]> {
        self.cfg_intervals.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct IrFunctionVariable {
    data_accesses: IrStatementDescriptorMap<Vec<IrDataAccess>>,
    pub data_type: DataType,
}

impl IrFunctionVariable {
    pub fn get_data_accesses(&self) -> &IrStatementDescriptorMap<Vec<IrDataAccess>> {
        &self.data_accesses
    }
}
