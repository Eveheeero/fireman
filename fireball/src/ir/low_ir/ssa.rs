//! SSA (Static Single Assignment) construction for Low IR
//!
//! This module implements deterministic SSA construction with phi placement
//! following the algorithm from "Simple and Efficient Construction of Static Single Assignment Form"
//! by Matthias Braun et al.

use super::*;
use std::collections::{BTreeMap, BTreeSet};

/// Dominance information for SSA construction
#[derive(Debug)]
pub struct DominatorTree {
    /// Immediate dominator for each block
    idom: BTreeMap<BlockId, BlockId>,

    /// Dominance frontier for each block
    df: BTreeMap<BlockId, BTreeSet<BlockId>>,

    /// Children in dominator tree
    children: BTreeMap<BlockId, BTreeSet<BlockId>>,
}

/// SSA builder that converts Low IR to SSA form
pub struct SSABuilder {
    /// Current variable versions for each local
    versions: BTreeMap<LocalId, u32>,

    /// Stack of versions for each variable (for traversal)
    version_stacks: BTreeMap<&'static str, Vec<(LocalId, u32)>>,

    /// Phi functions to insert
    phis: BTreeMap<BlockId, Vec<PhiNode>>,

    /// Dominance information
    dom_tree: Option<DominatorTree>,
}

#[derive(Debug, Clone)]
pub struct PhiNode {
    pub dst: LocalId,
    pub incoming: BTreeMap<BlockId, LocalId>,
    pub ty: Type,
}

impl Default for SSABuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SSABuilder {
    pub fn new() -> Self {
        Self {
            versions: BTreeMap::new(),
            version_stacks: BTreeMap::new(),
            phis: BTreeMap::new(),
            dom_tree: None,
        }
    }

    /// Convert a function to SSA form
    pub fn build_ssa(&mut self, function: &mut Function) -> Result<(), String> {
        // Step 1: Compute dominance information
        self.compute_dominators(function)?;

        // Step 2: Find where to place phi functions
        self.place_phi_functions(function)?;

        // Step 3: Rename variables
        self.rename_variables(function)?;

        // Step 4: Insert phi instructions
        self.insert_phi_instructions(function)?;

        Ok(())
    }

    /// Compute dominator tree using lengauer-tarjan algorithm
    fn compute_dominators(&mut self, function: &Function) -> Result<(), String> {
        let entry = &function.entry;
        let mut idom = BTreeMap::new();
        let mut df = BTreeMap::new();
        let mut children = BTreeMap::new();

        // Get all blocks in reverse postorder
        let blocks = self.get_reverse_postorder(function, entry)?;

        // Initialize
        for block_id in &blocks {
            df.insert(block_id.clone(), BTreeSet::new());
            children.insert(block_id.clone(), BTreeSet::new());
        }

        // Compute immediate dominators
        // Simplified: entry dominates all blocks for now
        for block_id in &blocks {
            if block_id != entry {
                idom.insert(block_id.clone(), entry.clone());
                children.get_mut(entry).unwrap().insert(block_id.clone());
            }
        }

        // Compute dominance frontiers using the algorithm from the paper
        // DF[n] = {y | ∃x ∈ preds(y) : n dominates x and n does not strictly dominate y}
        for y in &blocks {
            let preds = self.get_predecessors(function, y);
            if preds.len() > 1 {
                // Only join points (multiple predecessors) can be in dominance frontiers
                for x in preds {
                    let mut runner = x;
                    // Walk up from predecessor x until we strictly dominate y
                    while runner != *y {
                        // Add y to runner's dominance frontier
                        df.get_mut(&runner).unwrap().insert(y.clone());

                        // If runner dominates y, we're done
                        if let Some(y_idom) = idom.get(y) {
                            if &runner == y_idom {
                                break;
                            }
                        }

                        // Move to immediate dominator
                        match idom.get(&runner) {
                            Some(dom) => runner = dom.clone(),
                            None => break, // Reached entry
                        }
                    }
                }
            }
        }

        self.dom_tree = Some(DominatorTree { idom, df, children });
        Ok(())
    }

    /// Get blocks in reverse postorder for deterministic iteration
    fn get_reverse_postorder(
        &self,
        function: &Function,
        entry: &BlockId,
    ) -> Result<Vec<BlockId>, String> {
        let mut visited = BTreeSet::new();
        let mut postorder = Vec::new();

        self.dfs_postorder(function, entry, &mut visited, &mut postorder)?;

        postorder.reverse();
        Ok(postorder)
    }

    /// DFS helper for postorder traversal
    fn dfs_postorder(
        &self,
        function: &Function,
        block_id: &BlockId,
        visited: &mut BTreeSet<BlockId>,
        postorder: &mut Vec<BlockId>,
    ) -> Result<(), String> {
        if !visited.insert(block_id.clone()) {
            return Ok(());
        }

        let block = function
            .blocks
            .get(block_id)
            .ok_or_else(|| format!("Block {:?} not found", block_id))?;

        // Visit successors in deterministic order
        let succs = self.get_successors(&block.terminator);
        for succ in succs {
            self.dfs_postorder(function, &succ, visited, postorder)?;
        }

        postorder.push(block_id.clone());
        Ok(())
    }

    /// Get predecessors of a block
    fn get_predecessors(&self, function: &Function, block_id: &BlockId) -> Vec<BlockId> {
        let mut preds = Vec::new();
        for (pred_id, pred_block) in &function.blocks {
            let succs = self.get_successors(&pred_block.terminator);
            if succs.contains(block_id) {
                preds.push(pred_id.clone());
            }
        }
        preds.sort(); // Ensure deterministic order
        preds
    }

    /// Check if a dominates b
    fn dominates(&self, idom: &BTreeMap<BlockId, BlockId>, a: &BlockId, b: &BlockId) -> bool {
        if a == b {
            return true;
        }
        let mut current = b.clone();
        while let Some(dom) = idom.get(&current) {
            if dom == a {
                return true;
            }
            current = dom.clone();
        }
        false
    }

    /// Get successors of a block from its terminator
    fn get_successors(&self, terminator: &Terminator) -> Vec<BlockId> {
        match terminator {
            Terminator::Return(_) => vec![],
            Terminator::Branch(target) => vec![target.clone()],
            Terminator::CondBranch {
                true_dest,
                false_dest,
                ..
            } => {
                vec![true_dest.clone(), false_dest.clone()]
            }
            Terminator::Switch { default, cases, .. } => {
                let mut succs = vec![default.clone()];
                succs.extend(cases.values().cloned());
                succs.sort(); // Ensure deterministic order
                succs.dedup();
                succs
            }
            Terminator::IndirectBranch { destinations, .. } => {
                let mut succs: Vec<_> = destinations.iter().cloned().collect();
                succs.sort(); // Already sorted as BTreeSet, but be explicit
                succs
            }
            Terminator::Unreachable => vec![],
        }
    }

    /// Place phi functions using dominance frontiers
    fn place_phi_functions(&mut self, function: &Function) -> Result<(), String> {
        let dom_tree = self
            .dom_tree
            .as_ref()
            .ok_or("Dominance tree not computed")?;

        // Find all variable definitions
        let mut var_defs: BTreeMap<&'static str, BTreeSet<BlockId>> = BTreeMap::new();

        for (block_id, block) in &function.blocks {
            for inst in &block.instructions {
                if let Some(dst) = self.get_instruction_def(inst) {
                    var_defs
                        .entry(dst.purpose)
                        .or_default()
                        .insert(block_id.clone());
                }
            }
        }

        // For each variable, place phis
        for (var_name, def_blocks) in var_defs {
            let mut work_list: Vec<_> = def_blocks.iter().cloned().collect();
            let mut has_phi = BTreeSet::new();

            while let Some(block_id) = work_list.pop() {
                if let Some(df_set) = dom_tree.df.get(&block_id) {
                    for df_block in df_set {
                        if has_phi.insert(df_block.clone()) {
                            // Create phi node
                            let phi = PhiNode {
                                dst: LocalId {
                                    source: Address::from_virtual_address(
                                        &std::sync::Arc::new(crate::core::Sections::default()),
                                        df_block.0,
                                    ),
                                    purpose: var_name,
                                    index: 0,
                                    version: 0, // Will be set during renaming
                                },
                                incoming: BTreeMap::new(),
                                ty: Type::I64, // Default, will be refined
                            };

                            self.phis.entry(df_block.clone()).or_default().push(phi);

                            work_list.push(df_block.clone());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get definition from instruction
    fn get_instruction_def<'a>(&self, inst: &'a Instruction) -> Option<&'a LocalId> {
        match inst {
            Instruction::BinOp { dst, .. }
            | Instruction::UnOp { dst, .. }
            | Instruction::Load { dst, .. }
            | Instruction::Cast { dst, .. }
            | Instruction::Phi { dst, .. }
            | Instruction::Select { dst, .. }
            | Instruction::Assign { dst, .. }
            | Instruction::FlagRead { dst, .. } => Some(dst),

            Instruction::CpuId { eax_out, .. } => Some(eax_out),

            Instruction::Call { dst, .. } => dst.as_ref(),

            Instruction::Store { .. } | Instruction::FlagWrite { .. } => None,
        }
    }

    /// Rename variables to SSA form
    fn rename_variables(&mut self, function: &mut Function) -> Result<(), String> {
        // Clone entry to avoid borrow issues
        let entry = function.entry.clone();

        // Initialize version counters
        self.versions.clear();
        self.version_stacks.clear();

        // Start renaming from entry block
        self.rename_block(function, &entry)?;

        Ok(())
    }

    /// Rename variables in a block and its dominated blocks
    fn rename_block(&mut self, function: &mut Function, block_id: &BlockId) -> Result<(), String> {
        // Process phi functions first
        if let Some(phis) = self.phis.get(block_id).cloned() {
            for phi in &phis {
                let new_version = self.get_next_version(&phi.dst);
                self.push_version(phi.dst.purpose, phi.dst.clone(), new_version);
            }
        }

        // Clone necessary data to avoid borrow conflicts
        let successors = {
            let block = function
                .blocks
                .get(block_id)
                .ok_or_else(|| format!("Block {:?} not found", block_id))?;
            self.get_successors(&block.terminator)
        };

        let children = self
            .dom_tree
            .as_ref()
            .and_then(|dt| dt.children.get(block_id))
            .cloned()
            .unwrap_or_default();

        // Now mutably borrow the block
        {
            let block = function
                .blocks
                .get_mut(block_id)
                .ok_or_else(|| format!("Block {:?} not found", block_id))?;

            // Process instructions
            for inst in &mut block.instructions {
                // Rename uses
                self.rename_instruction_uses(inst)?;

                // Update definition with new version
                self.update_instruction_def(inst)?;
            }

            // Rename uses in terminator
            self.rename_terminator_uses(&mut block.terminator)?;
        }

        // Fill in phi operands in successors
        for succ in &successors {
            // Collect the phi updates first to avoid borrow conflicts
            let mut phi_updates = Vec::new();

            if let Some(succ_phis) = self.phis.get(succ) {
                for (idx, phi) in succ_phis.iter().enumerate() {
                    if let Some((var, version)) = self.get_current_version(phi.dst.purpose) {
                        phi_updates.push((
                            idx,
                            LocalId {
                                source: var.source.clone(),
                                purpose: var.purpose,
                                index: var.index,
                                version,
                            },
                        ));
                    }
                }
            }

            // Now apply the updates
            if let Some(succ_phis) = self.phis.get_mut(succ) {
                for (idx, new_local) in phi_updates {
                    succ_phis[idx].incoming.insert(block_id.clone(), new_local);
                }
            }
        }

        // Recursively rename dominated blocks
        for child in &children {
            self.rename_block(function, child)?;
        }

        // Pop versions for this block
        let block = function.blocks.get(block_id).unwrap();
        self.pop_versions_for_block(block)?;

        Ok(())
    }

    /// Get next version number for a variable
    fn get_next_version(&mut self, var: &LocalId) -> u32 {
        let version = self.versions.entry(var.clone()).or_insert(0);
        *version += 1;
        *version
    }

    /// Push a version onto the stack
    fn push_version(&mut self, purpose: &'static str, var: LocalId, version: u32) {
        self.version_stacks
            .entry(purpose)
            .or_default()
            .push((var, version));
    }

    /// Get current version of a variable
    fn get_current_version(&self, purpose: &'static str) -> Option<(&LocalId, u32)> {
        self.version_stacks
            .get(purpose)
            .and_then(|stack| stack.last())
            .map(|(var, ver)| (var, *ver))
    }

    /// Update instruction definition with new version
    fn update_instruction_def(&mut self, inst: &mut Instruction) -> Result<(), String> {
        match inst {
            Instruction::BinOp { dst, .. }
            | Instruction::UnOp { dst, .. }
            | Instruction::Load { dst, .. }
            | Instruction::Cast { dst, .. }
            | Instruction::Phi { dst, .. }
            | Instruction::Select { dst, .. }
            | Instruction::Assign { dst, .. }
            | Instruction::FlagRead { dst, .. } => {
                let new_version = self.get_next_version(dst);
                self.push_version(dst.purpose, dst.clone(), new_version);
                dst.version = new_version;
            }

            Instruction::CpuId {
                eax_out,
                ebx_out,
                ecx_out,
                edx_out,
                ..
            } => {
                let new_ver = self.get_next_version(eax_out);
                self.push_version(eax_out.purpose, eax_out.clone(), new_ver);
                eax_out.version = new_ver;

                let new_ver = self.get_next_version(ebx_out);
                self.push_version(ebx_out.purpose, ebx_out.clone(), new_ver);
                ebx_out.version = new_ver;

                let new_ver = self.get_next_version(ecx_out);
                self.push_version(ecx_out.purpose, ecx_out.clone(), new_ver);
                ecx_out.version = new_ver;

                let new_ver = self.get_next_version(edx_out);
                self.push_version(edx_out.purpose, edx_out.clone(), new_ver);
                edx_out.version = new_ver;
            }

            Instruction::Call { dst: Some(dst), .. } => {
                let new_version = self.get_next_version(dst);
                self.push_version(dst.purpose, dst.clone(), new_version);
                dst.version = new_version;
            }

            Instruction::Store { .. }
            | Instruction::FlagWrite { .. }
            | Instruction::Call { dst: None, .. } => {}
        }
        Ok(())
    }

    /// Rename uses in an instruction
    fn rename_instruction_uses(&mut self, inst: &mut Instruction) -> Result<(), String> {
        match inst {
            Instruction::BinOp { lhs, rhs, .. } => {
                self.rename_value(lhs)?;
                self.rename_value(rhs)?;
            }
            Instruction::UnOp { src, .. } => {
                self.rename_value(src)?;
            }
            Instruction::Load { ptr, .. } => {
                self.rename_value(ptr)?;
            }
            Instruction::Store { val, ptr, .. } => {
                self.rename_value(val)?;
                self.rename_value(ptr)?;
            }
            Instruction::Cast { src, .. } => {
                self.rename_value(src)?;
            }
            Instruction::Call { func, args, .. } => {
                self.rename_value(func)?;
                for (arg, _) in args {
                    self.rename_value(arg)?;
                }
            }
            Instruction::Select {
                cond,
                true_val,
                false_val,
                ..
            } => {
                self.rename_value(cond)?;
                self.rename_value(true_val)?;
                self.rename_value(false_val)?;
            }
            Instruction::Assign { value, .. } => {
                self.rename_value(value)?;
            }
            Instruction::FlagWrite { value, .. } => {
                self.rename_value(value)?;
            }
            Instruction::CpuId { eax_in, ecx_in, .. } => {
                self.rename_value(eax_in)?;
                if let Some(ecx) = ecx_in {
                    self.rename_value(ecx)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Rename a value
    fn rename_value(&mut self, value: &mut Value) -> Result<(), String> {
        if let Value::Local(local) = value {
            if let Some((_, version)) = self.get_current_version(local.purpose) {
                local.version = version;
            }
        }
        Ok(())
    }

    /// Rename uses in terminator
    fn rename_terminator_uses(&mut self, term: &mut Terminator) -> Result<(), String> {
        match term {
            Terminator::Return(Some((val, _))) => {
                self.rename_value(val)?;
            }
            Terminator::CondBranch { cond, .. } => {
                self.rename_value(cond)?;
            }
            Terminator::Switch { value, .. } => {
                self.rename_value(value)?;
            }
            Terminator::IndirectBranch { addr, .. } => {
                self.rename_value(addr)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Pop versions added by this block
    fn pop_versions_for_block(&mut self, block: &BasicBlock) -> Result<(), String> {
        // Count definitions in this block
        let mut def_counts: BTreeMap<&'static str, usize> = BTreeMap::new();

        for inst in &block.instructions {
            if let Some(dst) = self.get_instruction_def(inst) {
                *def_counts.entry(dst.purpose).or_insert(0) += 1;
            }
        }

        // Pop the appropriate number of versions
        for (purpose, count) in def_counts {
            if let Some(stack) = self.version_stacks.get_mut(purpose) {
                for _ in 0..count {
                    stack.pop();
                }
            }
        }

        Ok(())
    }

    /// Insert phi instructions into blocks
    fn insert_phi_instructions(&mut self, function: &mut Function) -> Result<(), String> {
        for (block_id, phis) in &self.phis {
            if let Some(block) = function.blocks.get_mut(block_id) {
                // Convert PhiNodes to Instructions
                for phi in phis {
                    let phi_inst = Instruction::Phi {
                        dst: phi.dst.clone(),
                        incoming: phi
                            .incoming
                            .clone()
                            .into_iter()
                            .map(|(bid, lid)| (bid, Value::Local(lid)))
                            .collect(),
                        ty: phi.ty.clone(),
                    };
                    block.phis.push(phi_inst);
                }

                // Ensure phis are sorted for determinism
                block.sort_phis();
            }
        }

        Ok(())
    }
}
