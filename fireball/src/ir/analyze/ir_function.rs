use crate::{
    core::{Block, Instruction},
    ir::{
        Ir, IrBlock,
        analyze::{
            DataType, DominanceFrontier, analyze_function_control_flow, infer_entry_block_id,
        },
        data::IrDataAccess,
        utils::IrStatementDescriptorMap,
    },
    prelude::*,
};
use iceball::{Statement, X64Statement};
use std::sync::Arc;

fn run_cfg_shape_analysis(blocks: &[Arc<Block>]) {
    let Some(entry_block_id) = infer_entry_block_id(blocks) else {
        debug!("Skip CFG shape analysis: failed to infer entry block");
        return;
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

    // L194: Multi-entry detection
    let multi_entries = analysis.cfg().multi_entry_blocks(blocks);
    if !multi_entries.is_empty() {
        debug!(
            "Multi-entry blocks detected (shared tails): {:?}",
            multi_entries
        );
    }

    // L192: Hot-cold chunk detection (gap > 4KB suggests split)
    let chunks = analysis.cfg().detect_address_gap_chunks(blocks, 4096);
    if chunks.len() > 1 {
        debug!(
            "Potential hot-cold split: {} address-separated chunks",
            chunks.len()
        );
    }
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

pub fn generate_ir_function(blocks: &[Arc<Block>]) -> IrFunction {
    info!("Generate IR function from {} blocks", blocks.len());
    run_cfg_shape_analysis(blocks);

    // Merge IR from all blocks in execution order
    let mut combined_ir = Vec::new();
    let mut instructions = Vec::new();
    for block in blocks {
        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };
        // TODO should we return err when ir not analyzed?
        // If block not analyzed, skip
        combined_ir.extend(ir_block.ir().iter().cloned());
        // if ir not sent, instruction must not be sent, it causes invalid ir analysis
        instructions.extend(block.get_instructions().iter().cloned());
    }

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
    IrFunction {
        instructions,
        ir: combined_ir,
        variables: merged_vars,
    }
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    instructions: Arc<[Instruction]>,
    ir: Vec<Ir>,
    variables: Vec<IrFunctionVariable>,
}

impl IrFunction {
    pub fn new(
        instructions: Arc<[Instruction]>,
        ir: Vec<Ir>,
        variables: Vec<IrFunctionVariable>,
    ) -> Self {
        Self {
            instructions,
            ir,
            variables,
        }
    }
    pub fn get_ir(&self) -> &Vec<Ir> {
        &self.ir
    }
    pub fn get_instructions(&self) -> &Arc<[Instruction]> {
        &self.instructions
    }
    pub fn get_variables(&self) -> &Vec<IrFunctionVariable> {
        &self.variables
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
