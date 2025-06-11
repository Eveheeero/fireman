//! Pattern matching engine for Medium IR
//!
//! This module implements the core pattern matching logic that identifies
//! code patterns in the Low IR and converts them to Medium IR patterns.

use super::*;
use crate::ir::low_ir;
use std::collections::{BTreeMap, BTreeSet};

/// Pattern matching engine
pub struct PatternMatcher {
    /// Pattern database to match against
    pattern_db: PatternDatabase,

    /// Minimum confidence threshold for accepting matches
    min_confidence: Confidence,

    /// Cache of already matched patterns to avoid recomputation
    match_cache: BTreeMap<MatchKey, Option<MatchResult>>,
}

/// Key for caching match results
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MatchKey {
    /// Block or instruction range being matched
    location: MatchLocation,
    /// Pattern name being matched against
    pattern_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatchLocation {
    Block(low_ir::BlockId),
    InstructionRange(low_ir::BlockId, usize, usize),
    Function(low_ir::FunctionId),
}

/// Result of a pattern match
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// The pattern that was matched
    pub pattern: Pattern,
    /// Confidence level of the match
    pub confidence: Confidence,
    /// Bindings captured during matching
    pub bindings: PatternBindings,
    /// Location where the match occurred
    pub location: MatchLocation,
}

/// Bindings captured during pattern matching
#[derive(Debug, Clone, Default)]
pub struct PatternBindings {
    /// Register bindings
    pub registers: BTreeMap<String, low_ir::LocalId>,
    /// Value bindings
    pub values: BTreeMap<String, low_ir::Value>,
    /// Type bindings
    pub types: BTreeMap<String, low_ir::Type>,
    /// Immediate value bindings
    pub immediates: BTreeMap<String, i128>,
}

impl PatternMatcher {
    /// Create a new pattern matcher with the given database
    pub fn new(pattern_db: PatternDatabase) -> Self {
        Self {
            pattern_db,
            min_confidence: Confidence::MEDIUM,
            match_cache: BTreeMap::new(),
        }
    }

    /// Set the minimum confidence threshold
    pub fn set_min_confidence(&mut self, confidence: Confidence) {
        self.min_confidence = confidence;
    }

    /// Match patterns in a function
    pub fn match_function(&mut self, func: &low_ir::Function) -> Vec<MatchResult> {
        let mut matches = Vec::new();

        // Try to match function-level patterns (like function prologues/epilogues)
        matches.extend(self.match_function_patterns(func));

        // Match patterns in each block
        for (block_id, block) in &func.blocks {
            matches.extend(self.match_block_patterns(block_id, block, func));
        }

        // Match cross-block patterns (like loops)
        matches.extend(self.match_control_flow_patterns(func));

        // Filter out low-confidence matches
        matches.retain(|m| m.confidence >= self.min_confidence);

        // Sort by confidence (highest first) and location
        matches.sort_by(|a, b| {
            b.confidence
                .cmp(&a.confidence)
                .then_with(|| a.location.cmp(&b.location))
        });

        matches
    }

    /// Match function-level patterns
    fn match_function_patterns(&mut self, func: &low_ir::Function) -> Vec<MatchResult> {
        let mut matches = Vec::new();

        // Match against architecture-specific function patterns
        for arch_pattern in &self.pattern_db.arch_patterns {
            if let Some(result) = self.match_arch_pattern(arch_pattern, func) {
                matches.push(result);
            }
        }

        matches
    }

    /// Match patterns within a single block
    fn match_block_patterns(
        &mut self,
        block_id: &low_ir::BlockId,
        block: &low_ir::BasicBlock,
        func: &low_ir::Function,
    ) -> Vec<MatchResult> {
        let mut matches = Vec::new();

        // Try to match instruction sequences
        for i in 0..block.instructions.len() {
            // Match library function calls
            if let Some(result) = self.match_library_call(&block.instructions[i], block_id, i) {
                matches.push(result);
            }

            // Match idiom patterns starting at this instruction
            // Clone idioms to avoid borrowing issues
            let idioms = self.pattern_db.idioms.clone();
            for idiom in &idioms {
                if let Some(result) =
                    self.match_idiom_at(idiom, &block.instructions[i..], block_id, i)
                {
                    matches.push(result);
                }
            }

            // Match array access patterns
            if let Some(result) = self.match_array_access(&block.instructions, i, block_id) {
                matches.push(result);
            }
        }

        // Match terminator patterns
        if let Some(result) = self.match_terminator_pattern(&block.terminator, block_id, func) {
            matches.push(result);
        }

        matches
    }

    /// Match control flow patterns across blocks
    fn match_control_flow_patterns(&mut self, func: &low_ir::Function) -> Vec<MatchResult> {
        let mut matches = Vec::new();

        // Detect loops
        matches.extend(self.detect_loops(func));

        // Detect if-else patterns
        matches.extend(self.detect_if_else_patterns(func));

        // Detect switch patterns
        matches.extend(self.detect_switch_patterns(func));

        matches
    }

    /// Match a library function call
    fn match_library_call(
        &mut self,
        inst: &low_ir::Instruction,
        block_id: &low_ir::BlockId,
        inst_index: usize,
    ) -> Option<MatchResult> {
        if let low_ir::Instruction::Call {
            func,
            args,
            dst,
            conv,
        } = inst
        {
            // Try to resolve the function being called
            if let low_ir::Value::Function(func_id) = func {
                // Look up in import table or symbol table
                if let Some(lib_pattern) = self.resolve_library_function(func_id) {
                    // Verify argument count and types match
                    if self.verify_function_signature(lib_pattern, args, conv) {
                        let pattern = Pattern::FunctionCall {
                            target: FunctionRef::Library {
                                name: lib_pattern.name.clone(),
                                library: lib_pattern.library.clone(),
                            },
                            arguments: args
                                .iter()
                                .map(|(val, _)| {
                                    // Create pattern refs for arguments
                                    PatternRef(0) // TODO: Proper pattern refs
                                })
                                .collect(),
                            return_value: dst.as_ref().map(|_| PatternRef(0)),
                            confidence: Confidence::HIGH,
                        };

                        return Some(MatchResult {
                            pattern,
                            confidence: Confidence::HIGH,
                            bindings: PatternBindings::default(),
                            location: MatchLocation::InstructionRange(
                                block_id.clone(),
                                inst_index,
                                inst_index + 1,
                            ),
                        });
                    }
                }
            }
        }

        None
    }

    /// Match an idiom pattern at a specific location
    fn match_idiom_at(
        &mut self,
        idiom: &IdiomPattern,
        instructions: &[low_ir::Instruction],
        block_id: &low_ir::BlockId,
        start_index: usize,
    ) -> Option<MatchResult> {
        let key = MatchKey {
            location: MatchLocation::InstructionRange(
                block_id.clone(),
                start_index,
                start_index + instructions.len(),
            ),
            pattern_name: idiom.name.clone(),
        };

        // Check cache
        if let Some(cached) = self.match_cache.get(&key) {
            return cached.clone();
        }

        // Try to match the idiom's pattern matcher
        let result = match &idiom.matcher {
            super::PatternMatcher::InstructionSequence(seq) => {
                self.match_instruction_sequence(seq, instructions, block_id, start_index)
            }
            super::PatternMatcher::ControlFlow(_cf_matcher) => {
                // Control flow patterns are handled separately
                None
            }
            super::PatternMatcher::DataFlow(_df_matcher) => {
                // Data flow patterns need more context
                None
            }
            _ => None,
        };

        // Apply confidence boost if matched
        let result = result.map(|mut r| {
            let new_confidence =
                std::cmp::min(100, r.confidence.0 as i16 + idiom.confidence_boost as i16) as u8;
            r.confidence = Confidence(new_confidence);
            r
        });

        // Cache the result
        self.match_cache.insert(key, result.clone());

        result
    }

    /// Match an instruction sequence pattern
    fn match_instruction_sequence(
        &self,
        sequence: &[InstructionMatcher],
        instructions: &[low_ir::Instruction],
        block_id: &low_ir::BlockId,
        start_index: usize,
    ) -> Option<MatchResult> {
        if sequence.is_empty() || instructions.is_empty() {
            return None;
        }

        let mut bindings = PatternBindings::default();
        let mut matched_count = 0;
        let mut inst_idx = 0;

        for matcher in sequence {
            if inst_idx >= instructions.len() {
                return None;
            }

            if self.match_single_instruction(matcher, &instructions[inst_idx], &mut bindings) {
                matched_count += 1;
                inst_idx += 1;
            } else if !self.is_optional_instruction(matcher) {
                return None;
            }
        }

        if matched_count == 0 {
            return None;
        }

        // Calculate confidence based on how well the sequence matched
        let confidence = Confidence((matched_count * 100 / sequence.len()) as u8);

        Some(MatchResult {
            pattern: Pattern::LowIR {
                instructions: instructions[..inst_idx].to_vec(),
                terminator: None,
                source_block: block_id.clone(),
                confidence,
            },
            confidence,
            bindings,
            location: MatchLocation::InstructionRange(
                block_id.clone(),
                start_index,
                start_index + inst_idx,
            ),
        })
    }

    /// Match a single instruction against a matcher
    fn match_single_instruction(
        &self,
        matcher: &InstructionMatcher,
        inst: &low_ir::Instruction,
        bindings: &mut PatternBindings,
    ) -> bool {
        match (matcher, inst) {
            (InstructionMatcher::Any, _) => true,

            // Note: Push/Pop/Ret don't exist in Low IR - they're assembly level
            // We match against the IR equivalents instead
            (
                InstructionMatcher::MovReg(dst_pat, src_pat),
                low_ir::Instruction::Assign { dst, value, .. },
            ) => {
                bindings.registers.insert(dst_pat.to_string(), dst.clone());
                self.match_value_pattern(src_pat, value, bindings)
            }

            (
                InstructionMatcher::SubImm(reg_pat, imm_matcher),
                low_ir::Instruction::BinOp {
                    op: low_ir::BinaryOp::Sub,
                    dst,
                    lhs,
                    rhs,
                    ..
                },
            ) => {
                bindings.registers.insert(reg_pat.to_string(), dst.clone());
                if let low_ir::Value::Local(local) = lhs {
                    if local == dst {
                        // Check if rhs matches the immediate pattern
                        if let InstructionMatcher::Any = imm_matcher.as_ref() {
                            if let low_ir::Value::Constant(low_ir::Constant::Int { value, ty: _ }) =
                                rhs
                            {
                                bindings.immediates.insert("frame_size".to_string(), *value);
                                return true;
                            }
                        }
                    }
                }
                false
            }

            // For other assembly-level matchers, we can't match them in Low IR
            (InstructionMatcher::Push(_), _) => false,
            (InstructionMatcher::Pop(_), _) => false,
            (InstructionMatcher::Ret, _) => false,
            (InstructionMatcher::Leave, _) => false,
            (InstructionMatcher::Lea(_, _), _) => false,
            (InstructionMatcher::AddReg(_, _, _), _) => false,

            _ => false,
        }
    }

    /// Check if an instruction matcher is optional
    fn is_optional_instruction(&self, _matcher: &InstructionMatcher) -> bool {
        // For now, no instructions are optional
        // This could be extended to support optional patterns
        false
    }

    /// Match a register pattern against a value
    fn match_register_pattern(
        &self,
        pattern: &str,
        value: &low_ir::Value,
        bindings: &mut PatternBindings,
    ) -> bool {
        match value {
            low_ir::Value::Local(local) => {
                bindings
                    .registers
                    .insert(pattern.to_string(), local.clone());
                true
            }
            _ => false,
        }
    }

    /// Match a value pattern
    fn match_value_pattern(
        &self,
        pattern: &str,
        value: &low_ir::Value,
        bindings: &mut PatternBindings,
    ) -> bool {
        bindings.values.insert(pattern.to_string(), value.clone());
        true
    }

    /// Resolve a function ID to a library pattern
    fn resolve_library_function(&self, func_id: &low_ir::FunctionId) -> Option<&LibraryPattern> {
        // TODO: Implement proper symbol resolution
        // This would involve:
        // 1. Looking up the function in the import table
        // 2. Checking against known library function addresses
        // 3. Using debug symbols if available

        // For now, just check if we have a pattern with a matching name
        // This is a placeholder implementation
        None
    }

    /// Verify that a function call matches a library pattern's signature
    fn verify_function_signature(
        &self,
        pattern: &LibraryPattern,
        args: &[(low_ir::Value, low_ir::Type)],
        conv: &low_ir::CallConv,
    ) -> bool {
        // Check calling convention
        if pattern.signature.convention != *conv {
            return false;
        }

        // Check argument count (considering variadic functions)
        if !pattern.signature.variadic && args.len() != pattern.signature.parameters.len() {
            return false;
        }

        if pattern.signature.variadic && args.len() < pattern.signature.parameters.len() {
            return false;
        }

        // TODO: Check argument types

        true
    }

    /// Match array access patterns
    fn match_array_access(
        &self,
        instructions: &[low_ir::Instruction],
        index: usize,
        block_id: &low_ir::BlockId,
    ) -> Option<MatchResult> {
        if index >= instructions.len() {
            return None;
        }

        match &instructions[index] {
            low_ir::Instruction::Load {
                dst: _, ptr, ty: _, ..
            } => {
                // Look for pointer arithmetic pattern: base + (index * scale)
                if let Some((base, idx, scale)) =
                    self.analyze_pointer_arithmetic(ptr, instructions, index)
                {
                    let element_type = self.infer_element_type_from_scale(scale);

                    let pattern = Pattern::ArrayAccess {
                        base: PatternRef(0), // TODO: Proper pattern ref
                        index: PatternRef(0),
                        element_type,
                        is_write: matches!(instructions[index], low_ir::Instruction::Store { .. }),
                        confidence: Confidence::MEDIUM,
                    };

                    return Some(MatchResult {
                        pattern,
                        confidence: Confidence::MEDIUM,
                        bindings: PatternBindings::default(),
                        location: MatchLocation::InstructionRange(
                            block_id.clone(),
                            index,
                            index + 1,
                        ),
                    });
                }
            }
            low_ir::Instruction::Store {
                val: _, ptr, ty: _, ..
            } => {
                // Similar to Load but for writes
                if let Some((base, idx, scale)) =
                    self.analyze_pointer_arithmetic(ptr, instructions, index)
                {
                    let element_type = self.infer_element_type_from_scale(scale);

                    let pattern = Pattern::ArrayAccess {
                        base: PatternRef(0), // TODO: Proper pattern ref
                        index: PatternRef(0),
                        element_type,
                        is_write: true,
                        confidence: Confidence::MEDIUM,
                    };

                    return Some(MatchResult {
                        pattern,
                        confidence: Confidence::MEDIUM,
                        bindings: PatternBindings::default(),
                        location: MatchLocation::InstructionRange(
                            block_id.clone(),
                            index,
                            index + 1,
                        ),
                    });
                }
            }
            _ => {}
        }

        None
    }

    /// Analyze pointer arithmetic to detect array access
    fn analyze_pointer_arithmetic(
        &self,
        ptr: &low_ir::Value,
        instructions: &[low_ir::Instruction],
        current_index: usize,
    ) -> Option<(low_ir::Value, low_ir::Value, usize)> {
        // Look backwards for pointer computation
        if let low_ir::Value::Local(ptr_local) = ptr {
            for i in (0..current_index).rev() {
                if let low_ir::Instruction::BinOp {
                    dst,
                    op: low_ir::BinaryOp::Add,
                    lhs,
                    rhs,
                    ..
                } = &instructions[i]
                {
                    if dst == ptr_local {
                        // Check if rhs is index * scale
                        if let Some((index, scale)) =
                            self.extract_scaled_index(rhs, instructions, i)
                        {
                            return Some((lhs.clone(), index, scale));
                        }
                    }
                }
            }
        }

        None
    }

    /// Extract scaled index from a value
    fn extract_scaled_index(
        &self,
        value: &low_ir::Value,
        instructions: &[low_ir::Instruction],
        current_index: usize,
    ) -> Option<(low_ir::Value, usize)> {
        if let low_ir::Value::Local(local) = value {
            for i in (0..current_index).rev() {
                match &instructions[i] {
                    low_ir::Instruction::BinOp {
                        dst,
                        op: low_ir::BinaryOp::Mul,
                        lhs,
                        rhs: low_ir::Value::Constant(low_ir::Constant::Int { value, .. }),
                        ..
                    } if dst == local => {
                        return Some((lhs.clone(), *value as usize));
                    }
                    low_ir::Instruction::BinOp {
                        dst,
                        op: low_ir::BinaryOp::Shl,
                        lhs,
                        rhs: low_ir::Value::Constant(low_ir::Constant::Int { value, .. }),
                        ..
                    } if dst == local => {
                        let scale = 1usize << (*value as usize);
                        return Some((lhs.clone(), scale));
                    }
                    _ => {}
                }
            }
        }

        None
    }

    /// Infer element type from scale factor
    fn infer_element_type_from_scale(&self, scale: usize) -> TypeRef {
        match scale {
            1 => TypeRef::Primitive(PrimitiveType::U8),
            2 => TypeRef::Primitive(PrimitiveType::U16),
            4 => TypeRef::Primitive(PrimitiveType::U32),
            8 => TypeRef::Primitive(PrimitiveType::U64),
            _ => TypeRef::Unknown,
        }
    }

    /// Match terminator patterns
    fn match_terminator_pattern(
        &self,
        terminator: &low_ir::Terminator,
        block_id: &low_ir::BlockId,
        func: &low_ir::Function,
    ) -> Option<MatchResult> {
        match terminator {
            low_ir::Terminator::CondBranch {
                cond,
                true_dest,
                false_dest,
            } => {
                // Simple if-else pattern
                let pattern = Pattern::IfElse {
                    condition: PatternRef(0),   // TODO: Extract condition pattern
                    then_branch: PatternRef(0), // TODO: Extract then branch
                    else_branch: if true_dest != false_dest {
                        Some(PatternRef(0))
                    } else {
                        None
                    },
                    confidence: Confidence::MEDIUM,
                };

                Some(MatchResult {
                    pattern,
                    confidence: Confidence::MEDIUM,
                    bindings: PatternBindings::default(),
                    location: MatchLocation::Block(block_id.clone()),
                })
            }
            _ => None,
        }
    }

    /// Detect loop patterns in the function
    fn detect_loops(&mut self, func: &low_ir::Function) -> Vec<MatchResult> {
        let mut loops = Vec::new();

        // Find natural loops using dominator analysis
        let dominators = self.compute_dominators(func);
        let back_edges = self.find_back_edges(func, &dominators);

        for (header, tail) in back_edges {
            if let Some(loop_match) = self.analyze_loop(&header, &tail, func, &dominators) {
                loops.push(loop_match);
            }
        }

        loops
    }

    /// Compute dominator tree for the function
    fn compute_dominators(
        &self,
        func: &low_ir::Function,
    ) -> BTreeMap<low_ir::BlockId, BTreeSet<low_ir::BlockId>> {
        let mut dominators = BTreeMap::new();

        // Entry block dominates only itself
        let entry = &func.entry;
        let mut entry_doms = BTreeSet::new();
        entry_doms.insert(entry.clone());
        dominators.insert(entry.clone(), entry_doms);

        // All other blocks are initially dominated by all blocks
        let all_blocks: BTreeSet<_> = func.blocks.keys().cloned().collect();
        for block_id in func.blocks.keys() {
            if block_id != entry {
                dominators.insert(block_id.clone(), all_blocks.clone());
            }
        }

        // Iterate until fixpoint
        let mut changed = true;
        while changed {
            changed = false;

            for (block_id, block) in &func.blocks {
                if block_id == entry {
                    continue;
                }

                // Find predecessors
                let predecessors = self.find_predecessors(block_id, func);

                if !predecessors.is_empty() {
                    // New dominators = intersection of predecessor dominators + self
                    let mut new_doms = all_blocks.clone();

                    for pred in &predecessors {
                        if let Some(pred_doms) = dominators.get(pred) {
                            new_doms = new_doms.intersection(pred_doms).cloned().collect();
                        }
                    }

                    new_doms.insert(block_id.clone());

                    if dominators.get(block_id) != Some(&new_doms) {
                        dominators.insert(block_id.clone(), new_doms);
                        changed = true;
                    }
                }
            }
        }

        dominators
    }

    /// Find predecessors of a block
    fn find_predecessors(
        &self,
        target: &low_ir::BlockId,
        func: &low_ir::Function,
    ) -> Vec<low_ir::BlockId> {
        let mut predecessors = Vec::new();

        for (block_id, block) in &func.blocks {
            let successors = self.get_successors(&block.terminator);
            if successors.contains(target) {
                predecessors.push(block_id.clone());
            }
        }

        predecessors
    }

    /// Get successors of a terminator
    fn get_successors(&self, terminator: &low_ir::Terminator) -> Vec<low_ir::BlockId> {
        match terminator {
            low_ir::Terminator::Branch(target) => vec![target.clone()],
            low_ir::Terminator::CondBranch {
                true_dest,
                false_dest,
                ..
            } => {
                vec![true_dest.clone(), false_dest.clone()]
            }
            low_ir::Terminator::Switch { cases, default, .. } => {
                let mut targets: Vec<_> = cases.values().cloned().collect();
                targets.push(default.clone());
                targets
            }
            _ => vec![],
        }
    }

    /// Find back edges in the control flow graph
    fn find_back_edges(
        &self,
        func: &low_ir::Function,
        dominators: &BTreeMap<low_ir::BlockId, BTreeSet<low_ir::BlockId>>,
    ) -> Vec<(low_ir::BlockId, low_ir::BlockId)> {
        let mut back_edges = Vec::new();

        for (block_id, block) in &func.blocks {
            let successors = self.get_successors(&block.terminator);

            for successor in successors {
                // A back edge is where the target dominates the source
                if let Some(successor_doms) = dominators.get(&successor) {
                    if successor_doms.contains(block_id) {
                        back_edges.push((successor, block_id.clone()));
                    }
                }
            }
        }

        back_edges
    }

    /// Analyze a loop given its header and tail
    fn analyze_loop(
        &self,
        header: &low_ir::BlockId,
        tail: &low_ir::BlockId,
        func: &low_ir::Function,
        dominators: &BTreeMap<low_ir::BlockId, BTreeSet<low_ir::BlockId>>,
    ) -> Option<MatchResult> {
        // Collect all blocks in the loop
        let loop_blocks = self.collect_loop_blocks(header, tail, func);

        // Determine loop type (for, while, do-while)
        let loop_type = self.determine_loop_type(header, &loop_blocks, func);

        let pattern = match loop_type {
            LoopType::For { init, increment } => Pattern::ForLoop {
                init: Some(PatternRef(0)),      // TODO: Extract init pattern
                condition: PatternRef(0),       // TODO: Extract condition
                increment: Some(PatternRef(0)), // TODO: Extract increment
                body: PatternRef(0),            // TODO: Extract body
                confidence: Confidence::HIGH,
            },
            LoopType::While => Pattern::WhileLoop {
                condition: PatternRef(0), // TODO: Extract condition
                body: PatternRef(0),      // TODO: Extract body
                confidence: Confidence::HIGH,
            },
            LoopType::DoWhile => Pattern::DoWhileLoop {
                body: PatternRef(0),      // TODO: Extract body
                condition: PatternRef(0), // TODO: Extract condition
                confidence: Confidence::HIGH,
            },
        };

        Some(MatchResult {
            pattern,
            confidence: Confidence::HIGH,
            bindings: PatternBindings::default(),
            location: MatchLocation::Block(header.clone()),
        })
    }

    /// Collect all blocks that are part of a loop
    fn collect_loop_blocks(
        &self,
        header: &low_ir::BlockId,
        tail: &low_ir::BlockId,
        func: &low_ir::Function,
    ) -> BTreeSet<low_ir::BlockId> {
        let mut loop_blocks = BTreeSet::new();
        let mut worklist = vec![tail.clone()];

        loop_blocks.insert(header.clone());

        while let Some(block_id) = worklist.pop() {
            if loop_blocks.insert(block_id.clone()) {
                // Add predecessors to worklist
                let predecessors = self.find_predecessors(&block_id, func);
                for pred in predecessors {
                    if pred != *header && !loop_blocks.contains(&pred) {
                        worklist.push(pred);
                    }
                }
            }
        }

        loop_blocks
    }

    /// Determine the type of loop
    fn determine_loop_type(
        &self,
        header: &low_ir::BlockId,
        loop_blocks: &BTreeSet<low_ir::BlockId>,
        func: &low_ir::Function,
    ) -> LoopType {
        // Check for initialization before the loop
        let has_init = self.has_loop_initialization(header, loop_blocks, func);

        // Check for increment in the loop
        let has_increment = self.has_loop_increment(loop_blocks, func);

        // Check if condition is at the beginning or end
        let condition_at_start = self.is_condition_at_start(header, func);

        if has_init && has_increment && condition_at_start {
            LoopType::For {
                init: true,
                increment: true,
            }
        } else if condition_at_start {
            LoopType::While
        } else {
            LoopType::DoWhile
        }
    }

    /// Check if loop has initialization
    fn has_loop_initialization(
        &self,
        header: &low_ir::BlockId,
        loop_blocks: &BTreeSet<low_ir::BlockId>,
        func: &low_ir::Function,
    ) -> bool {
        // Look for assignments in predecessors that aren't part of the loop
        let predecessors = self.find_predecessors(header, func);

        for pred_id in predecessors {
            if !loop_blocks.contains(&pred_id) {
                if let Some(block) = func.blocks.get(&pred_id) {
                    if block
                        .instructions
                        .iter()
                        .any(|inst| matches!(inst, low_ir::Instruction::Assign { .. }))
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check if loop has increment
    fn has_loop_increment(
        &self,
        loop_blocks: &BTreeSet<low_ir::BlockId>,
        func: &low_ir::Function,
    ) -> bool {
        for block_id in loop_blocks {
            if let Some(block) = func.blocks.get(block_id) {
                if block.instructions.iter().any(|inst| {
                    matches!(
                        inst,
                        low_ir::Instruction::BinOp {
                            op: low_ir::BinaryOp::Add | low_ir::BinaryOp::Sub,
                            ..
                        }
                    )
                }) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if condition is at the start of the loop
    fn is_condition_at_start(&self, header: &low_ir::BlockId, func: &low_ir::Function) -> bool {
        if let Some(block) = func.blocks.get(header) {
            matches!(block.terminator, low_ir::Terminator::CondBranch { .. })
        } else {
            false
        }
    }

    /// Detect if-else patterns
    fn detect_if_else_patterns(&self, func: &low_ir::Function) -> Vec<MatchResult> {
        let mut patterns = Vec::new();

        for (block_id, block) in &func.blocks {
            if let low_ir::Terminator::CondBranch {
                cond,
                true_dest,
                false_dest,
            } = &block.terminator
            {
                // Skip loop headers
                if self.is_loop_header(block_id, func) {
                    continue;
                }

                let pattern = Pattern::IfElse {
                    condition: PatternRef(0),   // TODO: Extract condition
                    then_branch: PatternRef(0), // TODO: Extract then branch
                    else_branch: if true_dest != false_dest {
                        Some(PatternRef(0))
                    } else {
                        None
                    },
                    confidence: Confidence::HIGH,
                };

                patterns.push(MatchResult {
                    pattern,
                    confidence: Confidence::HIGH,
                    bindings: PatternBindings::default(),
                    location: MatchLocation::Block(block_id.clone()),
                });
            }
        }

        patterns
    }

    /// Check if a block is a loop header
    fn is_loop_header(&self, block_id: &low_ir::BlockId, func: &low_ir::Function) -> bool {
        // A block is a loop header if it has a back edge
        for (other_id, other_block) in &func.blocks {
            if other_id > block_id {
                let successors = self.get_successors(&other_block.terminator);
                if successors.contains(block_id) {
                    return true;
                }
            }
        }

        false
    }

    /// Detect switch patterns
    fn detect_switch_patterns(&self, func: &low_ir::Function) -> Vec<MatchResult> {
        let mut patterns = Vec::new();

        for (block_id, block) in &func.blocks {
            if let low_ir::Terminator::Switch {
                value,
                cases,
                default,
            } = &block.terminator
            {
                let mut case_patterns = BTreeMap::new();

                for (constant, _target) in cases {
                    if let low_ir::Constant::Int { value, .. } = constant {
                        let case_value = (*value).clamp(i64::MIN as i128, i64::MAX as i128) as i64;
                        case_patterns.insert(case_value, PatternRef(0)); // TODO: Extract case body
                    }
                }

                let pattern = Pattern::SwitchCase {
                    value: PatternRef(0), // TODO: Extract switch value
                    cases: case_patterns,
                    default: Some(PatternRef(0)), // TODO: Extract default case
                    confidence: Confidence::HIGH,
                };

                patterns.push(MatchResult {
                    pattern,
                    confidence: Confidence::HIGH,
                    bindings: PatternBindings::default(),
                    location: MatchLocation::Block(block_id.clone()),
                });
            }
        }

        patterns
    }

    /// Match an architecture-specific pattern
    fn match_arch_pattern(
        &self,
        arch_pattern: &ArchPattern,
        func: &low_ir::Function,
    ) -> Option<MatchResult> {
        // For now, only match instruction sequences
        if let super::PatternMatcher::InstructionSequence(seq) = &arch_pattern.matcher {
            // Try to match at the beginning of the function (for prologues)
            if let Some(entry_block) = func.blocks.get(&func.entry) {
                if let Some(result) =
                    self.match_instruction_sequence(seq, &entry_block.instructions, &func.entry, 0)
                {
                    return Some(result);
                }
            }

            // Try to match at the end of blocks (for epilogues)
            for (block_id, block) in &func.blocks {
                if matches!(block.terminator, low_ir::Terminator::Return { .. }) {
                    // Look at the last few instructions
                    let inst_count = block.instructions.len();
                    if inst_count >= seq.len() {
                        let start = inst_count - seq.len();
                        if let Some(result) = self.match_instruction_sequence(
                            seq,
                            &block.instructions[start..],
                            block_id,
                            start,
                        ) {
                            return Some(result);
                        }
                    }
                }
            }
        }

        None
    }
}

/// Types of loops
#[derive(Debug, Clone, Copy)]
enum LoopType {
    For { init: bool, increment: bool },
    While,
    DoWhile,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matcher_creation() {
        let db = PatternDatabase::default();
        let matcher = PatternMatcher::new(db);

        assert_eq!(matcher.min_confidence, Confidence::MEDIUM);
    }

    #[test]
    fn test_instruction_matching() {
        let db = PatternDatabase::default();
        let matcher = PatternMatcher::new(db);

        // Test with an actual Low IR instruction
        let sections = std::sync::Arc::new(crate::core::Sections::default());
        let addr = crate::core::Address::from_virtual_address(&sections, 0x1000);
        let dst = low_ir::LocalId {
            source: addr.clone(),
            purpose: "test",
            index: 0,
            version: 0,
        };

        let inst = low_ir::Instruction::Assign {
            dst: dst.clone(),
            value: low_ir::Value::Constant(low_ir::Constant::Int {
                value: 42,
                ty: low_ir::Type::I32,
            }),
            source_addr: addr,
        };

        let mut bindings = PatternBindings::default();

        // Test matching with Any matcher
        assert!(matcher.match_single_instruction(&InstructionMatcher::Any, &inst, &mut bindings));

        // Test non-matching patterns (assembly-level instructions don't match)
        assert!(!matcher.match_single_instruction(
            &InstructionMatcher::Push("rbp"),
            &inst,
            &mut bindings
        ));
    }
}
