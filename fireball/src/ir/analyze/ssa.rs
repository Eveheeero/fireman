//! SSA (Static Single Assignment) analysis.
//!
//! Phase 1 computes phi-placement sites from the dominance frontier.
//! Phase 2 performs a lightweight renaming pass that assigns SSA versions to
//! tracked registers and stack/frame slots without rewriting the IR yet.
//!
//! Reference: Cytron et al., "Efficiently Computing Static Single Assignment
//! Form and the Control Dependence Graph", TOPLAS 1991.

use crate::{
    core::Block,
    ir::{
        Ir, Register,
        analyze::{DominanceFrontier, DominatorTree},
        data::{IrData, IrDataOperation, IrIntrinsic},
        operator::IrUnaryOperator,
        statements::{IrStatement, IrStatementSpecial},
    },
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

/// Identifies the pre-SSA storage location tracked by the analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SsaBase {
    Register(Register),
    StackSlot(i64),
    FrameSlot(i64),
}

/// A versioned SSA variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SsaVar {
    base: SsaBase,
    version: usize,
}

impl SsaVar {
    fn new(base: SsaBase, version: usize) -> Self {
        Self { base, version }
    }

    pub fn base(&self) -> SsaBase {
        self.base
    }

    pub fn version(&self) -> usize {
        self.version
    }
}

/// Result of phi-site computation for each tracked variable.
#[derive(Debug, Clone)]
pub struct PhiSites {
    sites: HashMap<SsaBase, Vec<usize>>,
}

impl PhiSites {
    pub fn sites_for(&self, base: &SsaBase) -> &[usize] {
        self.sites.get(base).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn variables(&self) -> impl Iterator<Item = &SsaBase> {
        self.sites.keys()
    }

    pub fn total_phi_count(&self) -> usize {
        self.sites.values().map(|sites| sites.len()).sum()
    }
}

/// Summary of the SSA values observed while traversing one block.
#[derive(Debug, Clone, Default)]
pub struct BlockSsaState {
    defined: Vec<SsaVar>,
    used: Vec<SsaVar>,
}

impl BlockSsaState {
    pub fn defined(&self) -> &[SsaVar] {
        &self.defined
    }

    pub fn used(&self) -> &[SsaVar] {
        &self.used
    }
}

/// A materialized phi node with outgoing version and incoming versions.
#[derive(Debug, Clone)]
pub struct PhiNode {
    block_id: usize,
    output: SsaVar,
    inputs: Vec<(usize, SsaVar)>,
}

impl PhiNode {
    pub fn block_id(&self) -> usize {
        self.block_id
    }

    pub fn output(&self) -> SsaVar {
        self.output
    }

    pub fn inputs(&self) -> &[(usize, SsaVar)] {
        &self.inputs
    }
}

/// Additive SSA analysis result without mutating the IR.
#[derive(Debug, Clone)]
pub struct SsaRenameSummary {
    phi_sites: PhiSites,
    phi_nodes: Vec<PhiNode>,
    block_states: HashMap<usize, BlockSsaState>,
    version_counts: HashMap<SsaBase, usize>,
}

impl SsaRenameSummary {
    pub fn phi_sites(&self) -> &PhiSites {
        &self.phi_sites
    }

    pub fn phi_nodes(&self) -> &[PhiNode] {
        &self.phi_nodes
    }

    pub fn block_state(&self, block_id: usize) -> Option<&BlockSsaState> {
        self.block_states.get(&block_id)
    }

    pub fn tracked_variable_count(&self) -> usize {
        self.version_counts.len()
    }

    pub fn total_version_count(&self) -> usize {
        self.version_counts.values().sum()
    }
}

/// Compute phi-node placement sites using the iterated dominance frontier.
pub fn compute_phi_sites(
    blocks: &[Arc<Block>],
    dominator_tree: &DominatorTree,
    dominance_frontier: &DominanceFrontier,
) -> PhiSites {
    let defs_per_var = collect_variable_defs(blocks, dominator_tree.cfg().block_ids());
    let mut sites = HashMap::new();

    for (base, def_blocks) in &defs_per_var {
        if def_blocks.len() < 2 {
            continue;
        }

        let phi_blocks = iterated_dominance_frontier(def_blocks, dominance_frontier);
        if !phi_blocks.is_empty() {
            sites.insert(*base, phi_blocks);
        }
    }

    PhiSites { sites }
}

/// Build a lightweight SSA rename summary on top of phi-site placement.
pub fn build_ssa_rename_summary(
    blocks: &[Arc<Block>],
    dominator_tree: &DominatorTree,
) -> SsaRenameSummary {
    let dominance_frontier = DominanceFrontier::compute(dominator_tree);
    let phi_sites = compute_phi_sites(blocks, dominator_tree, &dominance_frontier);
    let mut phi_bases_per_block: HashMap<usize, Vec<SsaBase>> = HashMap::new();
    for base in phi_sites.variables().copied() {
        for &block_id in phi_sites.sites_for(&base) {
            phi_bases_per_block.entry(block_id).or_default().push(base);
        }
    }
    for bases in phi_bases_per_block.values_mut() {
        bases.sort_unstable_by_key(|base| ssa_base_sort_key(*base));
    }

    let mut block_map = HashMap::new();
    for block in blocks {
        block_map.insert(block.get_id(), block);
    }

    let mut dom_children: HashMap<usize, Vec<usize>> = HashMap::new();
    for &block_id in dominator_tree.cfg().block_ids() {
        dom_children.entry(block_id).or_default();
        if let Some(parent) = dominator_tree.immediate_dominator_of(block_id) {
            dom_children.entry(parent).or_default().push(block_id);
        }
    }
    for children in dom_children.values_mut() {
        children.sort_unstable();
    }

    let mut context = RenameContext {
        block_map,
        dom_children,
        phi_bases_per_block,
        current_versions: HashMap::new(),
        next_versions: HashMap::new(),
        phi_outputs: HashMap::new(),
        phi_inputs: HashMap::new(),
        block_states: HashMap::new(),
    };

    rename_block(
        dominator_tree.cfg().entry_block_id(),
        dominator_tree,
        &mut context,
    );

    let mut phi_nodes = Vec::new();
    for (&(block_id, base), &version) in &context.phi_outputs {
        let mut inputs = context
            .phi_inputs
            .get(&(block_id, base))
            .cloned()
            .unwrap_or_default();
        inputs.sort_unstable_by_key(|(pred, value_version)| (*pred, *value_version));
        phi_nodes.push(PhiNode {
            block_id,
            output: SsaVar::new(base, version),
            inputs: inputs
                .into_iter()
                .map(|(pred, input_version)| (pred, SsaVar::new(base, input_version)))
                .collect(),
        });
    }
    phi_nodes.sort_unstable_by_key(|node| {
        (
            node.block_id(),
            ssa_base_sort_key(node.output().base()),
            node.output().version(),
        )
    });

    SsaRenameSummary {
        phi_sites,
        phi_nodes,
        block_states: context.block_states,
        version_counts: context.next_versions,
    }
}

/// Run both SSA phases and log a concise summary.
pub fn log_ssa_analysis(blocks: &[Arc<Block>], dominator_tree: &DominatorTree) {
    let summary = build_ssa_rename_summary(blocks, dominator_tree);
    let phi_count = summary.phi_nodes().len();
    let tracked_count = summary.tracked_variable_count();
    let version_count = summary.total_version_count();

    if phi_count == 0 && tracked_count == 0 {
        debug!("SSA analysis: no tracked SSA variables discovered");
        return;
    }

    debug!(
        "SSA analysis: {} tracked variables, {} phi nodes, {} total versions",
        tracked_count, phi_count, version_count
    );

    for phi_node in summary.phi_nodes() {
        debug!(
            "  phi {:?}_{} at block {} <= {:?}",
            phi_node.output().base(),
            phi_node.output().version(),
            phi_node.block_id(),
            phi_node.inputs()
        );
    }

    let mut block_ids = summary.block_states.keys().copied().collect::<Vec<_>>();
    block_ids.sort_unstable();
    for block_id in block_ids {
        let Some(state) = summary.block_state(block_id) else {
            continue;
        };
        if state.defined().is_empty() && state.used().is_empty() {
            continue;
        }

        debug!(
            "  block {}: defs={}, uses={}",
            block_id,
            state.defined().len(),
            state.used().len()
        );
    }
}

struct RenameContext<'a> {
    block_map: HashMap<usize, &'a Arc<Block>>,
    dom_children: HashMap<usize, Vec<usize>>,
    phi_bases_per_block: HashMap<usize, Vec<SsaBase>>,
    current_versions: HashMap<SsaBase, Vec<usize>>,
    next_versions: HashMap<SsaBase, usize>,
    phi_outputs: HashMap<(usize, SsaBase), usize>,
    phi_inputs: HashMap<(usize, SsaBase), Vec<(usize, usize)>>,
    block_states: HashMap<usize, BlockSsaState>,
}

fn rename_block(block_id: usize, dominator_tree: &DominatorTree, context: &mut RenameContext<'_>) {
    let mut pushed_versions = Vec::new();

    if let Some(phi_bases) = context.phi_bases_per_block.get(&block_id).cloned() {
        for base in phi_bases {
            let version = allocate_version(base, block_id, context);
            context.phi_outputs.insert((block_id, base), version);
            pushed_versions.push(base);
        }
    }

    if let Some(block) = context.block_map.get(&block_id) {
        let ir_handle = block.get_ir();
        if let Some(ir_block) = ir_handle.as_ref() {
            for ir in ir_block.ir() {
                rename_ir(ir, block_id, context, &mut pushed_versions);
            }
        }
    }

    for &successor_id in dominator_tree.cfg().successors_of(block_id) {
        if let Some(phi_bases) = context.phi_bases_per_block.get(&successor_id) {
            for &base in phi_bases {
                if let Some(version) = current_version(base, context) {
                    context
                        .phi_inputs
                        .entry((successor_id, base))
                        .or_default()
                        .push((block_id, version));
                }
            }
        }
    }

    if let Some(children) = context.dom_children.get(&block_id).cloned() {
        for child_id in children {
            rename_block(child_id, dominator_tree, context);
        }
    }

    while let Some(base) = pushed_versions.pop() {
        if let Some(stack) = context.current_versions.get_mut(&base) {
            stack.pop();
            if stack.is_empty() {
                context.current_versions.remove(&base);
            }
        }
    }
}

fn rename_ir(
    ir: &Ir,
    block_id: usize,
    context: &mut RenameContext<'_>,
    pushed_versions: &mut Vec<SsaBase>,
) {
    let Some(statements) = ir.statements.as_ref() else {
        return;
    };

    for statement in statements.iter() {
        rename_statement(statement, block_id, context, pushed_versions);
    }
}

fn rename_statement(
    statement: &IrStatement,
    block_id: usize,
    context: &mut RenameContext<'_>,
    pushed_versions: &mut Vec<SsaBase>,
) {
    match statement {
        IrStatement::Assignment { from, to, .. } => {
            record_data_use(from.as_ref(), block_id, context);
            record_data_use_excluding_root(to.as_ref(), block_id, context);
            if let Some(base) = ir_data_to_ssa_base(to.as_ref()) {
                allocate_version(base, block_id, context);
                pushed_versions.push(base);
            }
        }
        IrStatement::Jump { target } | IrStatement::JumpByCall { target } => {
            record_data_use(target.as_ref(), block_id, context);
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            record_data_use(condition.as_ref(), block_id, context);
            for nested in true_branch.iter() {
                rename_statement(nested, block_id, context, pushed_versions);
            }
            for nested in false_branch.iter() {
                rename_statement(nested, block_id, context, pushed_versions);
            }
        }
        IrStatement::Special(special) => {
            rename_special_statement(special, block_id, context);
        }
        IrStatement::Undefined | IrStatement::Exception(_) | IrStatement::Halt => {}
    }
}

fn rename_special_statement(
    special: &IrStatementSpecial,
    block_id: usize,
    context: &mut RenameContext<'_>,
) {
    match special {
        IrStatementSpecial::TypeSpecified { location, .. } => {
            record_data_use(location.as_ref(), block_id, context);
        }
        IrStatementSpecial::CalcFlagsAutomatically {
            operation, flags, ..
        } => {
            record_data_use(operation.as_ref(), block_id, context);
            for flag in flags {
                record_data_use(flag.as_ref(), block_id, context);
            }
        }
        IrStatementSpecial::Assertion { condition } => {
            record_data_use(condition.as_ref(), block_id, context);
        }
    }
}

fn record_data_use(data: &IrData, block_id: usize, context: &mut RenameContext<'_>) {
    if let Some(base) = ir_data_to_ssa_base(data) {
        if let Some(version) = current_version(base, context) {
            context
                .block_states
                .entry(block_id)
                .or_default()
                .used
                .push(SsaVar::new(base, version));
        }
    }

    match data {
        IrData::Dereference(inner) => record_data_use(inner.as_ref(), block_id, context),
        IrData::Operation(IrDataOperation::Unary { arg, .. }) => {
            record_data_use(arg.as_ref(), block_id, context);
        }
        IrData::Operation(IrDataOperation::Binary { arg1, arg2, .. }) => {
            record_data_use(arg1.as_ref(), block_id, context);
            record_data_use(arg2.as_ref(), block_id, context);
        }
        IrData::Intrinsic(IrIntrinsic::ByteSizeOf(inner))
        | IrData::Intrinsic(IrIntrinsic::BitSizeOf(inner)) => {
            record_data_use(inner.as_ref(), block_id, context);
        }
        IrData::Intrinsic(IrIntrinsic::Sized(inner, _)) => {
            record_data_use(inner.as_ref(), block_id, context);
        }
        IrData::Constant(_) | IrData::Intrinsic(_) | IrData::Register(_) | IrData::Operand(_) => {}
    }
}

fn record_data_use_excluding_root(data: &IrData, block_id: usize, context: &mut RenameContext<'_>) {
    match data {
        IrData::Dereference(inner) => record_data_use(inner.as_ref(), block_id, context),
        IrData::Operation(IrDataOperation::Unary { arg, .. }) => {
            record_data_use(arg.as_ref(), block_id, context);
        }
        IrData::Operation(IrDataOperation::Binary { arg1, arg2, .. }) => {
            record_data_use(arg1.as_ref(), block_id, context);
            record_data_use(arg2.as_ref(), block_id, context);
        }
        IrData::Intrinsic(IrIntrinsic::ByteSizeOf(inner))
        | IrData::Intrinsic(IrIntrinsic::BitSizeOf(inner)) => {
            record_data_use(inner.as_ref(), block_id, context);
        }
        IrData::Intrinsic(IrIntrinsic::Sized(inner, _)) => {
            record_data_use(inner.as_ref(), block_id, context);
        }
        IrData::Constant(_) | IrData::Intrinsic(_) | IrData::Register(_) | IrData::Operand(_) => {}
    }
}

fn allocate_version(base: SsaBase, block_id: usize, context: &mut RenameContext<'_>) -> usize {
    let next_version = context.next_versions.entry(base).or_insert(0);
    let version = *next_version;
    *next_version += 1;
    context
        .current_versions
        .entry(base)
        .or_default()
        .push(version);
    context
        .block_states
        .entry(block_id)
        .or_default()
        .defined
        .push(SsaVar::new(base, version));
    version
}

fn current_version(base: SsaBase, context: &RenameContext<'_>) -> Option<usize> {
    context
        .current_versions
        .get(&base)
        .and_then(|stack| stack.last().copied())
}

fn collect_variable_defs(
    blocks: &[Arc<Block>],
    block_ids: &[usize],
) -> HashMap<SsaBase, HashSet<usize>> {
    let mut defs = HashMap::new();

    for &block_id in block_ids {
        let Some(block) = blocks.iter().find(|block| block.get_id() == block_id) else {
            continue;
        };
        let ir_handle = block.get_ir();
        let Some(ir_block) = ir_handle.as_ref() else {
            continue;
        };

        for ir in ir_block.ir() {
            collect_defs_from_ir(ir, block_id, &mut defs);
        }
    }

    defs
}

fn collect_defs_from_ir(ir: &Ir, block_id: usize, defs: &mut HashMap<SsaBase, HashSet<usize>>) {
    let Some(statements) = ir.statements.as_ref() else {
        return;
    };

    for statement in statements.iter() {
        collect_defs_from_statement(statement, block_id, defs);
    }
}

fn ssa_base_sort_key(base: SsaBase) -> String {
    format!("{base:?}")
}

fn collect_defs_from_statement(
    statement: &IrStatement,
    block_id: usize,
    defs: &mut HashMap<SsaBase, HashSet<usize>>,
) {
    match statement {
        IrStatement::Assignment { to, .. } => {
            if let Some(base) = ir_data_to_ssa_base(to.as_ref()) {
                defs.entry(base).or_default().insert(block_id);
            }
        }
        IrStatement::Condition {
            true_branch,
            false_branch,
            ..
        } => {
            for nested in true_branch.iter() {
                collect_defs_from_statement(nested, block_id, defs);
            }
            for nested in false_branch.iter() {
                collect_defs_from_statement(nested, block_id, defs);
            }
        }
        IrStatement::Undefined
        | IrStatement::Exception(_)
        | IrStatement::Jump { .. }
        | IrStatement::JumpByCall { .. }
        | IrStatement::Halt
        | IrStatement::Special(_) => {}
    }
}

fn ir_data_to_ssa_base(data: &IrData) -> Option<SsaBase> {
    match data {
        IrData::Register(register) => Some(SsaBase::Register(*register)),
        IrData::Dereference(inner) => stack_slot_base(inner.as_ref()),
        _ => stack_slot_base(data),
    }
}

fn stack_slot_base(data: &IrData) -> Option<SsaBase> {
    if let Some(offset) = data.get_offset_from_stack_pointer() {
        return signed_offset(offset.as_ref()).map(SsaBase::StackSlot);
    }
    if let Some(offset) = data.get_offset_from_base_pointer() {
        return signed_offset(offset.as_ref()).map(SsaBase::FrameSlot);
    }
    None
}

fn signed_offset(data: &IrData) -> Option<i64> {
    match data {
        IrData::Constant(value) => i64::try_from(*value).ok(),
        IrData::Operation(IrDataOperation::Unary {
            operator: IrUnaryOperator::Negation,
            arg,
        }) => signed_offset(arg.as_ref()).map(|value| -value),
        _ => None,
    }
}

fn iterated_dominance_frontier(
    initial_blocks: &HashSet<usize>,
    dominance_frontier: &DominanceFrontier,
) -> Vec<usize> {
    let mut result = HashSet::new();
    let mut worklist = initial_blocks.iter().copied().collect::<Vec<_>>();

    while let Some(block_id) = worklist.pop() {
        for &frontier_id in dominance_frontier.frontier_of(block_id) {
            if result.insert(frontier_id) {
                worklist.push(frontier_id);
            }
        }
    }

    let mut sorted = result.into_iter().collect::<Vec<_>>();
    sorted.sort_unstable();
    sorted
}
