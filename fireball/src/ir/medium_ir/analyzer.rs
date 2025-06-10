//! Medium IR analyzer for pattern recognition

use super::*;
use crate::ir::low_ir::{self, Module as LowModule};
use crate::ir::medium_ir::pattern_database::PatternDatabaseBuilder;
use std::collections::BTreeMap;
use std::path::Path;

/// Analyzer that converts Low IR to Medium IR with pattern recognition
pub struct MediumIRAnalyzer {
    /// Pattern database for matching
    pattern_db: PatternDatabase,

    /// Confidence threshold for pattern acceptance
    confidence_threshold: Confidence,
}

impl MediumIRAnalyzer {
    pub fn new() -> Self {
        Self {
            pattern_db: PatternDatabase::default(),
            confidence_threshold: Confidence::LOW,
        }
    }

    /// Create analyzer with custom pattern database
    pub fn with_pattern_database(pattern_db: PatternDatabase) -> Self {
        Self {
            pattern_db,
            confidence_threshold: Confidence::LOW,
        }
    }

    /// Load patterns from directory
    pub fn load_patterns_from_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let mut builder = PatternDatabaseBuilder::new();

        // Load pattern files
        builder.load_from_directory(path)?;

        // Build the database
        self.pattern_db = builder.build();

        Ok(())
    }

    /// Analyze Low IR module and produce Medium IR
    pub fn analyze(&self, low_module: &LowModule) -> Module {
        let mut module = Module {
            target: low_module.target.clone(),
            functions: BTreeMap::new(),
            global_patterns: self.pattern_db.clone(),
        };

        // Analyze each function
        for (func_id, low_func) in &low_module.functions {
            let analyzed = self.analyze_function(low_func);
            module.functions.insert(func_id.clone(), analyzed);
        }

        module
    }

    /// Analyze a single function
    fn analyze_function(&self, low_func: &low_ir::Function) -> Function {
        let mut pattern_store = PatternStore::new();

        // First pass: identify basic patterns
        let basic_patterns = self.identify_basic_patterns(low_func, &mut pattern_store);

        // Second pass: identify higher-level patterns
        let body = self.identify_composite_patterns(&basic_patterns, &mut pattern_store);

        // Analyze function signature
        let signature = self.analyze_signature(low_func);

        Function {
            id: low_func.id.clone(),
            signature,
            patterns: pattern_store,
            body,
            confidence: Confidence::MEDIUM, // TODO: Calculate actual confidence
        }
    }

    /// Identify basic patterns in the function
    fn identify_basic_patterns(
        &self,
        low_func: &low_ir::Function,
        store: &mut PatternStore,
    ) -> BTreeMap<low_ir::BlockId, Vec<PatternRef>> {
        let mut basic_patterns = BTreeMap::new();

        for (block_id, block) in &low_func.blocks {
            let patterns = self.analyze_block(block, low_func, store);
            basic_patterns.insert(block_id.clone(), patterns);
        }

        basic_patterns
    }

    /// Analyze a basic block for patterns
    fn analyze_block(
        &self,
        block: &low_ir::BasicBlock,
        func: &low_ir::Function,
        store: &mut PatternStore,
    ) -> Vec<PatternRef> {
        let mut patterns = Vec::new();

        // Check for loop patterns
        if let Some(loop_pattern) = self.detect_loop_pattern(block, func, store) {
            patterns.push(loop_pattern);
        }

        // Check for conditional patterns
        if let Some(cond_pattern) = self.detect_conditional_pattern(
            &block.terminator,
            store,
            &block.instructions,
            func,
            &block.id,
        ) {
            patterns.push(cond_pattern);
        }

        // Check for function call patterns
        for inst in &block.instructions {
            if let Some(call_pattern) = self.detect_call_pattern(inst, store) {
                patterns.push(call_pattern);
            }
        }

        // Check for array access patterns
        for (i, inst) in block.instructions.iter().enumerate() {
            if let Some(array_pattern) =
                self.detect_array_access_pattern(inst, &block.instructions, i, store)
            {
                patterns.push(array_pattern);
            }
        }

        // Check for string operation patterns
        for (i, inst) in block.instructions.iter().enumerate() {
            if let Some(string_pattern) =
                self.detect_string_operation(inst, &block.instructions, i, store)
            {
                patterns.push(string_pattern);
            }
        }

        // Check for memory allocation patterns
        for inst in &block.instructions {
            if let Some(alloc_pattern) = self.detect_memory_allocation_pattern(inst, store) {
                patterns.push(alloc_pattern);
            }
        }

        // If no patterns found, wrap instructions as LowIR pattern
        if patterns.is_empty() {
            let low_ir_pattern = Pattern::LowIR {
                instructions: block.instructions.clone(),
                terminator: Some(block.terminator.clone()),
                source_block: block.id.clone(),
                confidence: Confidence::CERTAIN,
            };
            patterns.push(store.insert(low_ir_pattern));
        }

        patterns
    }

    /// Detect loop patterns in a block
    fn detect_loop_pattern(
        &self,
        block: &low_ir::BasicBlock,
        func: &low_ir::Function,
        store: &mut PatternStore,
    ) -> Option<PatternRef> {
        // Check if this block is a loop header
        if self.is_loop_header(block, func) {
            // For now, create a simple while loop pattern
            // Extract condition from the block's terminator
            let condition_pattern = match &block.terminator {
                low_ir::Terminator::CondBranch { cond, .. } => {
                    // Find the instruction that produces this condition value
                    let mut cond_instructions = vec![];

                    // Look for the instruction that generates the condition
                    for inst in &block.instructions {
                        match inst {
                            low_ir::Instruction::BinOp { dst, .. } => {
                                // Check if this instruction produces the condition value
                                if let low_ir::Value::Local(cond_local) = cond {
                                    if dst == cond_local {
                                        cond_instructions.push(inst.clone());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    let cond_pattern = Pattern::LowIR {
                        instructions: cond_instructions.clone(),
                        terminator: None,
                        source_block: block.id.clone(),
                        confidence: if cond_instructions.is_empty() {
                            Confidence::LOW
                        } else {
                            Confidence::HIGH
                        },
                    };
                    store.insert(cond_pattern)
                }
                _ => return None,
            };

            // Collect body instructions from blocks that are part of the loop
            let mut body_instructions = vec![];
            let mut body_blocks = vec![];

            // Find blocks that are part of the loop body
            for (other_id, other_block) in &func.blocks {
                // A block is part of the loop body if:
                // 1. It has higher address than header (typical for loop body)
                // 2. It eventually branches back to the header
                if other_id > &block.id {
                    let branches_to_header = match &other_block.terminator {
                        low_ir::Terminator::Branch(target) => target == &block.id,
                        low_ir::Terminator::CondBranch {
                            true_dest,
                            false_dest,
                            ..
                        } => true_dest == &block.id || false_dest == &block.id,
                        _ => false,
                    };

                    if branches_to_header {
                        body_blocks.push((other_id, other_block));
                        body_instructions.extend(other_block.instructions.clone());
                    }
                }
            }

            let body = Pattern::LowIR {
                instructions: body_instructions,
                terminator: None,
                source_block: block.id.clone(),
                confidence: if body_blocks.is_empty() {
                    Confidence::LOW
                } else {
                    Confidence::MEDIUM
                },
            };
            let body_ref = store.insert(body);

            // Check if this looks like a for loop (has initialization and increment)
            let mut has_init = false;
            let mut has_increment = false;
            let mut init_ref = None;
            let mut increment_ref = None;

            // Look for initialization in direct predecessor blocks (not just entry)
            // Find predecessors of the loop header
            let mut predecessors = Vec::new();
            for (pred_id, pred_block) in &func.blocks {
                match &pred_block.terminator {
                    low_ir::Terminator::Branch(target) if target == &block.id => {
                        predecessors.push((pred_id, pred_block));
                    }
                    low_ir::Terminator::CondBranch {
                        true_dest,
                        false_dest,
                        ..
                    } => {
                        if true_dest == &block.id || false_dest == &block.id {
                            predecessors.push((pred_id, pred_block));
                        }
                    }
                    _ => {}
                }
            }

            // Check if any predecessor (that's not part of the loop) has initialization
            for (pred_id, pred_block) in predecessors {
                // If predecessor has lower address, it's likely not part of the loop
                if pred_id < &block.id {
                    if pred_block
                        .instructions
                        .iter()
                        .any(|inst| matches!(inst, low_ir::Instruction::Assign { .. }))
                    {
                        has_init = true;
                        let init = Pattern::LowIR {
                            instructions: pred_block.instructions.clone(),
                            terminator: None,
                            source_block: pred_id.clone(),
                            confidence: Confidence::MEDIUM,
                        };
                        init_ref = Some(store.insert(init));
                        break;
                    }
                }
            }

            // Look for increment in the loop body
            // A loop body typically has higher address than the header
            for (other_id, other_block) in &func.blocks {
                if other_id > &block.id && other_id < &low_ir::BlockId(block.id.0 + 0x100) {
                    // Check if this block eventually branches back to the header
                    let branches_to_header = match &other_block.terminator {
                        low_ir::Terminator::Branch(target) => target == &block.id,
                        low_ir::Terminator::CondBranch {
                            true_dest,
                            false_dest,
                            ..
                        } => true_dest == &block.id || false_dest == &block.id,
                        _ => false,
                    };

                    if branches_to_header {
                        for inst in &other_block.instructions {
                            if matches!(
                                inst,
                                low_ir::Instruction::BinOp {
                                    op: low_ir::BinaryOp::Add,
                                    ..
                                } | low_ir::Instruction::BinOp {
                                    op: low_ir::BinaryOp::Sub,
                                    ..
                                }
                            ) {
                                has_increment = true;
                                let inc = Pattern::LowIR {
                                    instructions: vec![inst.clone()],
                                    terminator: None,
                                    source_block: other_id.clone(),
                                    confidence: Confidence::MEDIUM,
                                };
                                increment_ref = Some(store.insert(inc));
                                break;
                            }
                        }
                    }
                }
            }

            // A for loop typically has both init and increment
            // A while loop might have increment but no init
            let is_for_loop = has_init && has_increment;

            // Create the appropriate loop pattern
            let pattern = if is_for_loop {
                Pattern::ForLoop {
                    init: init_ref,
                    condition: condition_pattern,
                    increment: increment_ref,
                    body: body_ref,
                    confidence: Confidence::MEDIUM,
                }
            } else {
                Pattern::WhileLoop {
                    condition: condition_pattern,
                    body: body_ref,
                    confidence: Confidence::MEDIUM,
                }
            };

            Some(store.insert(pattern))
        } else {
            None
        }
    }

    /// Check if a block is a loop header
    fn is_loop_header(&self, block: &low_ir::BasicBlock, func: &low_ir::Function) -> bool {
        // A block is a loop header if it has a back edge
        // (i.e., a predecessor with a higher address)
        for (other_id, other_block) in &func.blocks {
            match &other_block.terminator {
                low_ir::Terminator::Branch(target) => {
                    if target == &block.id && other_id > &block.id {
                        return true;
                    }
                }
                low_ir::Terminator::CondBranch {
                    true_dest,
                    false_dest,
                    ..
                } => {
                    if (true_dest == &block.id || false_dest == &block.id) && other_id > &block.id {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// Detect conditional patterns in terminator
    fn detect_conditional_pattern(
        &self,
        terminator: &low_ir::Terminator,
        store: &mut PatternStore,
        block_instructions: &[low_ir::Instruction],
        func: &low_ir::Function,
        block_id: &low_ir::BlockId,
    ) -> Option<PatternRef> {
        match terminator {
            low_ir::Terminator::CondBranch {
                cond,
                true_dest,
                false_dest,
            } => {
                // Extract condition computation instructions
                let mut cond_instructions = vec![];
                for inst in block_instructions {
                    match inst {
                        low_ir::Instruction::BinOp { dst, .. } => {
                            if let low_ir::Value::Local(cond_local) = cond {
                                if dst == cond_local {
                                    cond_instructions.push(inst.clone());
                                }
                            }
                        }
                        _ => {}
                    }
                }

                let condition = Pattern::LowIR {
                    instructions: cond_instructions,
                    terminator: None,
                    source_block: block_id.clone(),
                    confidence: Confidence::HIGH,
                };
                let cond_ref = store.insert(condition);

                // Extract then branch instructions
                let then_instructions = if let Some(then_block) = func.blocks.get(true_dest) {
                    then_block.instructions.clone()
                } else {
                    vec![]
                };

                let then_terminator = if let Some(then_block) = func.blocks.get(true_dest) {
                    Some(then_block.terminator.clone())
                } else {
                    None
                };

                let then_pattern = Pattern::LowIR {
                    instructions: then_instructions,
                    terminator: then_terminator,
                    source_block: true_dest.clone(),
                    confidence: Confidence::HIGH,
                };
                let then_ref = store.insert(then_pattern);

                // Extract else branch instructions if it exists
                let else_branch = if let Some(else_block) = func.blocks.get(false_dest) {
                    let else_pattern = Pattern::LowIR {
                        instructions: else_block.instructions.clone(),
                        terminator: Some(else_block.terminator.clone()),
                        source_block: false_dest.clone(),
                        confidence: Confidence::HIGH,
                    };
                    Some(store.insert(else_pattern))
                } else {
                    None
                };

                // Create if-else pattern
                let if_else = Pattern::IfElse {
                    condition: cond_ref,
                    then_branch: then_ref,
                    else_branch,
                    confidence: Confidence::MEDIUM,
                };

                Some(store.insert(if_else))
            }
            low_ir::Terminator::Switch {
                value: _, cases, ..
            } => {
                // Create switch pattern
                let value_pattern = Pattern::LowIR {
                    instructions: vec![], // TODO: Extract value computation
                    terminator: None,
                    source_block: block_id.clone(),
                    confidence: Confidence::HIGH,
                };
                let value_ref = store.insert(value_pattern);

                let mut case_patterns = BTreeMap::new();
                for (constant, _block_id) in cases {
                    if let low_ir::Constant::Int { value, .. } = constant {
                        let case_pattern = Pattern::LowIR {
                            instructions: vec![], // TODO: Extract case body
                            terminator: None,
                            source_block: block_id.clone(),
                            confidence: Confidence::HIGH,
                        };
                        // Convert i128 to i64, clamping if necessary
                        let case_value = (*value).clamp(i64::MIN as i128, i64::MAX as i128) as i64;
                        case_patterns.insert(case_value, store.insert(case_pattern));
                    }
                }

                let switch = Pattern::SwitchCase {
                    value: value_ref,
                    cases: case_patterns,
                    default: None, // TODO: Handle default case
                    confidence: Confidence::MEDIUM,
                };

                Some(store.insert(switch))
            }
            _ => None,
        }
    }

    /// Detect function call patterns
    fn detect_call_pattern(
        &self,
        inst: &low_ir::Instruction,
        store: &mut PatternStore,
    ) -> Option<PatternRef> {
        match inst {
            low_ir::Instruction::Call {
                func, args, dst, ..
            } => {
                // Determine function reference
                let func_ref = match func {
                    low_ir::Value::Function(id) => {
                        let addr = Address::from_virtual_address(
                            &std::sync::Arc::new(crate::core::Sections::default()),
                            id.0,
                        );

                        // Check if this matches a known library function
                        if let Some(lib_pattern) = self.match_library_function(id.0) {
                            FunctionRef::Library {
                                name: lib_pattern.name.clone(),
                                library: lib_pattern.library.clone(),
                            }
                        } else {
                            FunctionRef::Address(addr)
                        }
                    }
                    _ => {
                        let indirect = Pattern::LowIR {
                            instructions: vec![], // TODO: Extract indirect target
                            terminator: None,
                            source_block: low_ir::BlockId(0), // TODO: Get proper block ID
                            confidence: Confidence::LOW,
                        };
                        FunctionRef::Indirect(store.insert(indirect))
                    }
                };

                // Convert arguments to patterns
                let arg_patterns: Vec<_> = args
                    .iter()
                    .map(|(_val, _ty)| {
                        let arg = Pattern::LowIR {
                            instructions: vec![], // TODO: Extract argument computation
                            terminator: None,
                            source_block: low_ir::BlockId(0), // TODO: Get proper block ID
                            confidence: Confidence::HIGH,
                        };
                        store.insert(arg)
                    })
                    .collect();

                // Create return value pattern if present
                let return_pattern = dst.as_ref().map(|_| {
                    let ret = Pattern::LowIR {
                        instructions: vec![], // TODO: Extract return value handling
                        terminator: None,
                        source_block: low_ir::BlockId(0), // TODO: Get proper block ID
                        confidence: Confidence::HIGH,
                    };
                    store.insert(ret)
                });

                let call = Pattern::FunctionCall {
                    target: func_ref,
                    arguments: arg_patterns,
                    return_value: return_pattern,
                    confidence: Confidence::HIGH,
                };

                Some(store.insert(call))
            }
            _ => None,
        }
    }

    /// Identify composite patterns from basic patterns
    fn identify_composite_patterns(
        &self,
        basic_patterns: &BTreeMap<low_ir::BlockId, Vec<PatternRef>>,
        store: &mut PatternStore,
    ) -> PatternRef {
        // Look for high-level patterns like loops first
        for (_block_id, patterns) in basic_patterns {
            for pattern_ref in patterns {
                if let Some(pattern) = store.get(*pattern_ref) {
                    // If we found a loop or conditional pattern, make it the top-level pattern
                    match pattern {
                        Pattern::ForLoop { .. }
                        | Pattern::WhileLoop { .. }
                        | Pattern::DoWhileLoop { .. }
                        | Pattern::IfElse { .. }
                        | Pattern::SwitchCase { .. } => {
                            return *pattern_ref;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Otherwise, collect all patterns
        let all_patterns: Vec<_> = basic_patterns
            .values()
            .flat_map(|patterns| patterns.iter().cloned())
            .collect();

        if all_patterns.is_empty() {
            let empty = Pattern::LowIR {
                instructions: vec![],
                terminator: None,
                source_block: low_ir::BlockId(0), // Empty pattern, no specific block
                confidence: Confidence::LOW,
            };
            store.insert(empty)
        } else if all_patterns.len() == 1 {
            all_patterns[0]
        } else {
            // Create a compound pattern using Expression with And
            let seq = Pattern::Expression {
                operation: ExpressionOp::And,
                operands: all_patterns,
                confidence: Confidence::MEDIUM,
            };
            store.insert(seq)
        }
    }

    /// Analyze function signature
    fn analyze_signature(&self, low_func: &low_ir::Function) -> FunctionSignature {
        // Extract signature from Low IR function type
        match &low_func.signature {
            low_ir::Type::Function {
                ret,
                params,
                varargs,
            } => {
                FunctionSignature {
                    return_type: self.convert_type(ret),
                    parameters: params
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            // Use simple parameter names for better readability
                            let name = match i {
                                0 => "a".to_string(),
                                1 => "b".to_string(),
                                2 => "c".to_string(),
                                3 => "d".to_string(),
                                4 => "e".to_string(),
                                _ => format!("param_{}", i),
                            };
                            (name, self.convert_type(ty))
                        })
                        .collect(),
                    convention: low_ir::CallConv::C, // TODO: Detect actual convention
                    variadic: *varargs,
                }
            }
            _ => {
                // Fallback signature
                FunctionSignature {
                    return_type: TypeRef::Unknown,
                    parameters: vec![],
                    convention: low_ir::CallConv::C,
                    variadic: false,
                }
            }
        }
    }

    /// Convert Low IR type to Medium IR type
    fn convert_type(&self, low_type: &low_ir::Type) -> TypeRef {
        match low_type {
            low_ir::Type::Void => TypeRef::Primitive(PrimitiveType::Void),
            low_ir::Type::Bool => TypeRef::Primitive(PrimitiveType::Bool),
            low_ir::Type::I8 => TypeRef::Primitive(PrimitiveType::I8),
            low_ir::Type::I16 => TypeRef::Primitive(PrimitiveType::I16),
            low_ir::Type::I32 => TypeRef::Primitive(PrimitiveType::I32),
            low_ir::Type::I64 => TypeRef::Primitive(PrimitiveType::I64),
            low_ir::Type::F32 => TypeRef::Primitive(PrimitiveType::F32),
            low_ir::Type::F64 => TypeRef::Primitive(PrimitiveType::F64),
            low_ir::Type::Pointer(pointee) => TypeRef::Pointer(Box::new(
                pointee
                    .as_ref()
                    .map(|t| self.convert_type(t))
                    .unwrap_or(TypeRef::Unknown),
            )),
            low_ir::Type::Array(elem, size) => TypeRef::Array {
                element: Box::new(self.convert_type(elem)),
                size: Some(*size),
            },
            low_ir::Type::Struct(fields) => {
                let size = fields.iter().filter_map(|f| f.size()).sum();
                TypeRef::Struct { name: None, size }
            }
            _ => TypeRef::Unknown,
        }
    }

    /// Detect array access patterns in instructions
    fn detect_array_access_pattern(
        &self,
        inst: &low_ir::Instruction,
        instructions: &[low_ir::Instruction],
        inst_index: usize,
        store: &mut PatternStore,
    ) -> Option<PatternRef> {
        match inst {
            low_ir::Instruction::Load {
                dst: _, ptr, ty: _, ..
            } => {
                // Check if the pointer is computed as base + index * scale
                if let Some((base_val, index_val, element_ty)) =
                    self.analyze_array_pointer(ptr, instructions, inst_index)
                {
                    // Create pattern for base value
                    let base_pattern =
                        self.create_value_pattern(&base_val, instructions, inst_index);
                    let base_ref = store.insert(base_pattern);

                    // Create pattern for index value
                    let index_pattern =
                        self.create_value_pattern(&index_val, instructions, inst_index);
                    let index_ref = store.insert(index_pattern);

                    let array_access = Pattern::ArrayAccess {
                        base: base_ref,
                        index: index_ref,
                        element_type: element_ty,
                        is_write: false,
                        confidence: Confidence::MEDIUM,
                    };

                    Some(store.insert(array_access))
                } else {
                    None
                }
            }
            low_ir::Instruction::Store {
                val: _, ptr, ty: _, ..
            } => {
                // Similar to Load but for writes
                if let Some((base_val, index_val, element_ty)) =
                    self.analyze_array_pointer(ptr, instructions, inst_index)
                {
                    // Create pattern for base value
                    let base_pattern =
                        self.create_value_pattern(&base_val, instructions, inst_index);
                    let base_ref = store.insert(base_pattern);

                    // Create pattern for index value
                    let index_pattern =
                        self.create_value_pattern(&index_val, instructions, inst_index);
                    let index_ref = store.insert(index_pattern);

                    let array_access = Pattern::ArrayAccess {
                        base: base_ref,
                        index: index_ref,
                        element_type: element_ty,
                        is_write: true,
                        confidence: Confidence::MEDIUM,
                    };

                    Some(store.insert(array_access))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Analyze a pointer to determine if it's an array access
    fn analyze_array_pointer(
        &self,
        ptr: &low_ir::Value,
        instructions: &[low_ir::Instruction],
        current_index: usize,
    ) -> Option<(low_ir::Value, low_ir::Value, TypeRef)> {
        // Look for patterns like:
        // %ptr = %base + (%index * scale)
        // %ptr = %base + (%index << shift)

        if let low_ir::Value::Local(ptr_local) = ptr {
            // Search backwards for the instruction that defines this pointer
            for i in (0..current_index).rev() {
                match &instructions[i] {
                    low_ir::Instruction::BinOp {
                        dst,
                        op: low_ir::BinaryOp::Add,
                        lhs,
                        rhs,
                        ..
                    } if dst == ptr_local => {
                        // Found an add instruction that produces our pointer
                        // Check if rhs is a multiplication (index * scale)
                        if let Some((index, scale)) =
                            self.find_scale_operation(rhs, instructions, i)
                        {
                            // Determine element type from scale
                            let element_ty = match scale {
                                1 => TypeRef::Primitive(PrimitiveType::I8),
                                2 => TypeRef::Primitive(PrimitiveType::I16),
                                4 => TypeRef::Primitive(PrimitiveType::I32),
                                8 => TypeRef::Primitive(PrimitiveType::I64),
                                _ => TypeRef::Unknown,
                            };
                            return Some((lhs.clone(), index, element_ty));
                        }
                    }
                    _ => {}
                }
            }
        }

        None
    }

    /// Find a scale operation (multiplication or shift)
    fn find_scale_operation(
        &self,
        value: &low_ir::Value,
        instructions: &[low_ir::Instruction],
        current_index: usize,
    ) -> Option<(low_ir::Value, usize)> {
        if let low_ir::Value::Local(local) = value {
            // Search for the instruction that defines this value
            for i in (0..current_index).rev() {
                match &instructions[i] {
                    low_ir::Instruction::BinOp {
                        dst,
                        op: low_ir::BinaryOp::Mul,
                        lhs,
                        rhs,
                        ..
                    } if dst == local => {
                        // Found multiplication
                        if let low_ir::Value::Constant(low_ir::Constant::Int { value, .. }) = rhs {
                            return Some((lhs.clone(), *value as usize));
                        }
                    }
                    low_ir::Instruction::BinOp {
                        dst,
                        op: low_ir::BinaryOp::Shl,
                        lhs,
                        rhs,
                        ..
                    } if dst == local => {
                        // Found shift left (equivalent to multiplication by power of 2)
                        if let low_ir::Value::Constant(low_ir::Constant::Int { value, .. }) = rhs {
                            let scale = 1usize << (*value as usize);
                            return Some((lhs.clone(), scale));
                        }
                    }
                    _ => {}
                }
            }
        }

        None
    }

    /// Create a pattern from a Low IR value
    fn create_value_pattern(
        &self,
        value: &low_ir::Value,
        instructions: &[low_ir::Instruction],
        current_index: usize,
    ) -> Pattern {
        match value {
            low_ir::Value::Local(local) => {
                // Find the instruction that defines this local
                let mut defining_instructions = vec![];

                for i in (0..current_index).rev() {
                    match &instructions[i] {
                        low_ir::Instruction::BinOp { dst, .. } if dst == local => {
                            defining_instructions.push(instructions[i].clone());
                            break;
                        }
                        low_ir::Instruction::Assign { dst, .. } if dst == local => {
                            defining_instructions.push(instructions[i].clone());
                            break;
                        }
                        low_ir::Instruction::Load { dst, .. } if dst == local => {
                            defining_instructions.push(instructions[i].clone());
                            break;
                        }
                        _ => {}
                    }
                }

                Pattern::LowIR {
                    instructions: defining_instructions,
                    terminator: None,
                    source_block: low_ir::BlockId(0), // TODO: Get actual block ID
                    confidence: Confidence::HIGH,
                }
            }
            low_ir::Value::Constant(_) => {
                // For constants, create an empty pattern that represents the constant
                Pattern::LowIR {
                    instructions: vec![],
                    terminator: None,
                    source_block: low_ir::BlockId(0),
                    confidence: Confidence::CERTAIN,
                }
            }
            _ => {
                // For other values, create a low confidence pattern
                Pattern::LowIR {
                    instructions: vec![],
                    terminator: None,
                    source_block: low_ir::BlockId(0),
                    confidence: Confidence::LOW,
                }
            }
        }
    }
}

impl Default for PatternDatabase {
    fn default() -> Self {
        Self {
            library_functions: Self::init_library_functions(),
            idioms: Self::init_idioms(),
            arch_patterns: Self::init_arch_patterns(),
        }
    }
}

impl PatternDatabase {
    /// Initialize known library function patterns
    fn init_library_functions() -> BTreeMap<String, LibraryPattern> {
        let mut funcs = BTreeMap::new();

        // malloc pattern
        funcs.insert(
            "malloc".to_string(),
            LibraryPattern {
                name: "malloc".to_string(),
                library: "libc".to_string(),
                signature: FunctionSignature {
                    return_type: TypeRef::Pointer(Box::new(TypeRef::Primitive(
                        PrimitiveType::Void,
                    ))),
                    parameters: vec![("size".to_string(), TypeRef::Primitive(PrimitiveType::U64))],
                    convention: low_ir::CallConv::C,
                    variadic: false,
                },
                behavior: PatternBehavior::ModifiesMemory {
                    regions: vec![MemoryRegion::Heap],
                },
            },
        );

        // free pattern
        funcs.insert(
            "free".to_string(),
            LibraryPattern {
                name: "free".to_string(),
                library: "libc".to_string(),
                signature: FunctionSignature {
                    return_type: TypeRef::Primitive(PrimitiveType::Void),
                    parameters: vec![(
                        "ptr".to_string(),
                        TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::Void))),
                    )],
                    convention: low_ir::CallConv::C,
                    variadic: false,
                },
                behavior: PatternBehavior::ModifiesMemory {
                    regions: vec![MemoryRegion::Heap],
                },
            },
        );

        // TODO: Add more library functions

        funcs
    }

    /// Initialize common code idioms
    fn init_idioms() -> Vec<IdiomPattern> {
        vec![
            // TODO: Add common idiom patterns
        ]
    }

    /// Initialize architecture-specific patterns
    fn init_arch_patterns() -> Vec<ArchPattern> {
        vec![
            // TODO: Add architecture-specific patterns
        ]
    }
}

impl MediumIRAnalyzer {
    /// Try to match a function address to a known library function
    fn match_library_function(&self, _address: u64) -> Option<&LibraryPattern> {
        // TODO: Implement proper symbol resolution
        // For now, check if address matches known library function addresses
        // This would typically involve:
        // 1. Checking import tables in PE/ELF
        // 2. Matching against known library addresses
        // 3. Using debug symbols if available

        // Simple heuristic: check if function name exists in pattern database
        // In a real implementation, this would use the actual symbol table
        None
    }

    /// Detect string operations like strlen, strcpy
    fn detect_string_operation(
        &self,
        inst: &low_ir::Instruction,
        instructions: &[low_ir::Instruction],
        inst_index: usize,
        store: &mut PatternStore,
    ) -> Option<PatternRef> {
        // Look for patterns like:
        // 1. Loop that reads bytes until null terminator (strlen)
        // 2. Loop that copies bytes until null terminator (strcpy)
        // 3. Loop that compares bytes (strcmp)

        // This is a simplified detection - real implementation would be more sophisticated
        match inst {
            low_ir::Instruction::Load { ptr, ty, .. } => {
                // Check if this is loading a byte
                if matches!(ty, low_ir::Type::I8) {
                    // Look ahead for a comparison with zero (null terminator check)
                    for i in inst_index + 1..instructions.len() {
                        if let low_ir::Instruction::BinOp {
                            op: low_ir::BinaryOp::Eq,
                            rhs: low_ir::Value::Constant(low_ir::Constant::Int { value: 0, .. }),
                            ..
                        } = &instructions[i]
                        {
                            // This might be a string operation
                            let str_op = Pattern::StringOperation {
                                operation: StringOp::Length,
                                operands: vec![self.create_ptr_pattern(
                                    ptr,
                                    instructions,
                                    inst_index,
                                    store,
                                )],
                                confidence: Confidence::MEDIUM,
                            };
                            return Some(store.insert(str_op));
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }

    /// Create a pattern for a pointer value
    fn create_ptr_pattern(
        &self,
        ptr: &low_ir::Value,
        instructions: &[low_ir::Instruction],
        current_index: usize,
        store: &mut PatternStore,
    ) -> PatternRef {
        let pattern = self.create_value_pattern(ptr, instructions, current_index);
        store.insert(pattern)
    }

    /// Detect memory allocation patterns
    fn detect_memory_allocation_pattern(
        &self,
        inst: &low_ir::Instruction,
        store: &mut PatternStore,
    ) -> Option<PatternRef> {
        match inst {
            low_ir::Instruction::Call { func, args, .. } => {
                // Check if this is a call to a known allocator
                if let low_ir::Value::Function(id) = func {
                    if let Some(lib_pattern) = self.match_library_function(id.0) {
                        match lib_pattern.name.as_str() {
                            "malloc" | "calloc" => {
                                // Extract size argument
                                if let Some((_size_val, _)) = args.first() {
                                    let size_pattern = Pattern::LowIR {
                                        instructions: vec![], // TODO: Extract size computation
                                        terminator: None,
                                        source_block: low_ir::BlockId(0),
                                        confidence: Confidence::HIGH,
                                    };
                                    let size_ref = store.insert(size_pattern);

                                    let alloc = Pattern::MemoryAllocation {
                                        size: size_ref,
                                        allocator: match lib_pattern.name.as_str() {
                                            "malloc" => AllocatorType::Malloc,
                                            "calloc" => AllocatorType::Calloc,
                                            _ => AllocatorType::Malloc,
                                        },
                                        confidence: Confidence::HIGH,
                                    };
                                    return Some(store.insert(alloc));
                                }
                            }
                            "free" => {
                                // Extract pointer argument
                                if let Some((_ptr_val, _)) = args.first() {
                                    let ptr_pattern = Pattern::LowIR {
                                        instructions: vec![], // TODO: Extract pointer
                                        terminator: None,
                                        source_block: low_ir::BlockId(0),
                                        confidence: Confidence::HIGH,
                                    };
                                    let ptr_ref = store.insert(ptr_pattern);

                                    let dealloc = Pattern::MemoryDeallocation {
                                        pointer: ptr_ref,
                                        deallocator: DeallocatorType::Free,
                                        confidence: Confidence::HIGH,
                                    };
                                    return Some(store.insert(dealloc));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }
}
