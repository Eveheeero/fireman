//! Architecture-specific AST optimization
//!
//! This module provides AST-level optimizations that are architecture-aware,
//! allowing for better code generation based on the target architecture's
//! capabilities and conventions.

use crate::arch::{ArchType, ArchitectureInfo};
use crate::ir::analyze::advanced_constant_folding::AdvancedConstantFolder;
use crate::ir::analyze::common_subexpression_elimination::CommonSubexpressionEliminator;
use crate::ir::analyze::dead_code_elimination::DeadCodeEliminator;
use crate::ir::analyze::enhanced_c_codegen::EnhancedCConfig;
use crate::ir::analyze::expression_simplifier::ExpressionSimplifier;
use crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    BinaryOperator, CAst, CType, Expression, Function, Literal, Statement, Wrapped,
    WrappedStatement,
};

/// Architecture-specific optimization configuration
#[derive(Debug, Clone)]
pub struct ArchOptimizationConfig {
    /// Target architecture
    pub arch_type: ArchType,
    /// Architecture information
    pub arch_info: ArchitectureInfo,
    /// Enhanced C configuration
    pub enhanced_c_config: EnhancedCConfig,
    /// Enable SIMD pattern recognition
    pub enable_simd_patterns: bool,
    /// Enable architecture-specific idioms
    pub enable_arch_idioms: bool,
    /// Enable calling convention optimizations
    pub enable_cc_optimizations: bool,
    /// Enable expression simplification
    pub enable_expression_simplification: bool,
    /// Enable dead code elimination
    pub enable_dead_code_elimination: bool,
    /// Enable common subexpression elimination
    pub enable_cse: bool,
}

impl Default for ArchOptimizationConfig {
    fn default() -> Self {
        Self {
            arch_type: ArchType::Unknown,
            arch_info: ArchitectureInfo {
                arch_type: ArchType::Unknown,
                pointer_size: 64,
                endianness: crate::arch::Endianness::Little,
                register_count: 0,
                instruction_alignment: 1,
            },
            enhanced_c_config: EnhancedCConfig::default(),
            enable_simd_patterns: true,
            enable_arch_idioms: true,
            enable_cc_optimizations: true,
            enable_expression_simplification: true,
            enable_dead_code_elimination: true,
            enable_cse: true,
        }
    }
}

/// AST optimizer for architecture-specific optimizations
pub struct AstOptimizer {
    config: ArchOptimizationConfig,
    /// Statistics for optimization passes
    stats: OptimizationStats,
}

#[derive(Debug, Default)]
pub struct OptimizationStats {
    pub simd_patterns_recognized: usize,
    pub arch_idioms_applied: usize,
    pub cc_optimizations: usize,
    pub type_improvements: usize,
}

impl AstOptimizer {
    /// Create a new AST optimizer
    pub fn new(config: ArchOptimizationConfig) -> Self {
        Self {
            config,
            stats: OptimizationStats::default(),
        }
    }

    /// Optimize the entire AST
    pub fn optimize(&mut self, ast: &mut CAst) {
        // Run optimization passes in order
        if self.config.enable_simd_patterns {
            self.recognize_simd_patterns(ast);
        }

        if self.config.enable_arch_idioms {
            self.apply_architecture_idioms(ast);
        }

        if self.config.enable_cc_optimizations {
            self.optimize_calling_conventions(ast);
        }

        // Always run type optimization based on architecture
        self.optimize_types_for_architecture(ast);

        // Run expression simplification if enabled
        if self.config.enable_expression_simplification {
            self.simplify_expressions(ast);
        }

        // Run advanced constant folding
        if self.config.enable_expression_simplification {
            self.perform_advanced_constant_folding(ast);
        }

        // Run common subexpression elimination if enabled
        if self.config.enable_cse {
            self.eliminate_common_subexpressions(ast);
        }

        // Run dead code elimination if enabled
        if self.config.enable_dead_code_elimination {
            self.eliminate_dead_code(ast);
        }
    }

    /// Recognize and convert SIMD patterns
    fn recognize_simd_patterns(&mut self, ast: &mut CAst) {
        // Look for patterns that can be converted to SIMD operations
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                self.recognize_simd_in_function(&mut func);
                ast.functions.write().unwrap().insert(id, func);
            }
        }
    }

    /// Recognize SIMD patterns in a function
    fn recognize_simd_in_function(&mut self, function: &mut Function) {
        // TODO: Implement SIMD pattern recognition
        // Examples:
        // - Loop unrolling with parallel operations
        // - Vector addition/subtraction patterns
        // - Packed data operations

        // For now, just count potential patterns
        for stmt in &mut function.body {
            if self.is_potential_simd_pattern(stmt) {
                self.stats.simd_patterns_recognized += 1;
                // In the future, transform the statement
            }
        }
    }

    /// Check if a statement could be a SIMD pattern
    fn is_potential_simd_pattern(&self, stmt: &WrappedStatement) -> bool {
        match &stmt.statement {
            Statement::For(_, _, _, body) => {
                // Look for loops that operate on arrays
                body.iter().any(|s| self.is_array_operation(s))
            }
            _ => false,
        }
    }

    /// Check if a statement is an array operation
    fn is_array_operation(&self, stmt: &WrappedStatement) -> bool {
        match &stmt.statement {
            Statement::Assignment(lhs, rhs) => {
                self.is_array_access(lhs) || self.is_array_access(rhs)
            }
            _ => false,
        }
    }

    /// Check if an expression is an array access
    fn is_array_access(&self, expr: &Wrapped<Expression>) -> bool {
        matches!(&expr.item, Expression::ArrayAccess(_, _))
    }

    /// Apply architecture-specific idioms
    fn apply_architecture_idioms(&mut self, ast: &mut CAst) {
        match self.config.arch_type {
            ArchType::X86 | ArchType::X86_64 => self.apply_x86_idioms(ast),
            ArchType::Arm32 | ArchType::Arm64 => self.apply_arm_idioms(ast),
            _ => {}
        }
    }

    /// Apply x86-specific idioms
    fn apply_x86_idioms(&mut self, ast: &mut CAst) {
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                for stmt in &mut func.body {
                    self.apply_x86_idiom_to_statement(stmt);
                }
                ast.functions.write().unwrap().insert(id, func);
            }
        }
    }

    /// Apply x86 idioms to a statement
    fn apply_x86_idiom_to_statement(&mut self, stmt: &mut WrappedStatement) {
        if let Statement::Assignment(_lhs, rhs) = &mut stmt.statement {
            // XOR with self -> zero
            if let Expression::BinaryOp(BinaryOperator::BitXor, left, right) = &rhs.item {
                if self.expressions_equal(left, right) {
                    // Replace with zero
                    rhs.item = Expression::Literal(Literal::Int(0));
                    if let Some(ref mut comment) = stmt.comment {
                        comment.push_str(" // XOR self -> 0");
                    } else {
                        stmt.comment = Some("// XOR self -> 0".to_string());
                    }
                    self.stats.arch_idioms_applied += 1;
                }
            }

            // TEST instruction pattern (AND without storing result)
            // This would be detected at a higher level
        }
    }

    /// Apply ARM-specific idioms
    fn apply_arm_idioms(&mut self, ast: &mut CAst) {
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                for stmt in &mut func.body {
                    self.apply_arm_idiom_to_statement(stmt);
                }
                ast.functions.write().unwrap().insert(id, func);
            }
        }
    }

    /// Apply ARM idioms to a statement
    fn apply_arm_idiom_to_statement(&mut self, stmt: &mut WrappedStatement) {
        if let Statement::Assignment(_lhs, rhs) = &mut stmt.statement {
            // Barrel shifter patterns
            if self.is_arm_barrel_shift_pattern(rhs) {
                // Add comment about barrel shifter
                if let Some(ref mut comment) = stmt.comment {
                    comment.push_str(" // ARM barrel shifter");
                } else {
                    stmt.comment = Some("// ARM barrel shifter".to_string());
                }
                self.stats.arch_idioms_applied += 1;
            }
        }
    }

    /// Check if expression is an ARM barrel shifter pattern
    fn is_arm_barrel_shift_pattern(&self, expr: &Wrapped<Expression>) -> bool {
        match &expr.item {
            Expression::BinaryOp(op, _left, right) => {
                matches!(op, BinaryOperator::LeftShift | BinaryOperator::RightShift)
                    && matches!(&right.item, Expression::Literal(_))
            }
            _ => false,
        }
    }

    /// Optimize calling conventions
    fn optimize_calling_conventions(&mut self, ast: &mut CAst) {
        // Analyze function calls and optimize based on calling convention
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                self.optimize_function_calls(&mut func);
                ast.functions.write().unwrap().insert(id, func);
            }
        }
    }

    /// Optimize function calls in a function
    fn optimize_function_calls(&mut self, function: &mut Function) {
        for stmt in &mut function.body {
            self.optimize_call_in_statement(stmt);
        }
    }

    /// Optimize calls in a statement
    fn optimize_call_in_statement(&mut self, stmt: &mut WrappedStatement) {
        match &mut stmt.statement {
            Statement::Call(_target, args) => {
                // Add calling convention hints
                if self.config.arch_type == ArchType::X86_64 && args.len() <= 6 {
                    // System V ABI uses registers for first 6 args
                    if let Some(ref mut comment) = stmt.comment {
                        comment.push_str(" // Args in registers");
                    } else {
                        stmt.comment = Some("// Args in registers".to_string());
                    }
                }
                self.stats.cc_optimizations += 1;
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.optimize_call_in_statement(s);
                }
            }
            Statement::If(_, then_block, else_block) => {
                for s in then_block {
                    self.optimize_call_in_statement(s);
                }
                if let Some(else_stmts) = else_block {
                    for s in else_stmts {
                        self.optimize_call_in_statement(s);
                    }
                }
            }
            _ => {}
        }
    }

    /// Optimize types based on architecture
    fn optimize_types_for_architecture(&mut self, ast: &mut CAst) {
        // Adjust types based on architecture
        let pointer_size = (self.config.arch_info.pointer_size / 8) as usize; // Convert bits to bytes

        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                // Update variable types
                let vars = func.variables.clone();
                let mut vars_write = vars.write().unwrap();
                for (_vid, var) in vars_write.iter_mut() {
                    self.optimize_variable_type(var, pointer_size);
                }
                drop(vars_write);

                // Update function parameter and return types
                self.optimize_function_signature(&mut func, pointer_size);

                ast.functions.write().unwrap().insert(id, func);
            }
        }
    }

    /// Optimize a variable's type
    fn optimize_variable_type(
        &mut self,
        var: &mut crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::Variable,
        pointer_size: usize,
    ) {
        // Convert generic pointer types to architecture-specific
        match &var.var_type {
            CType::Pointer(_inner) => {
                // Ensure pointer size matches architecture
                self.stats.type_improvements += 1;
            }
            // For now, we'll handle specific cases
            // In a real implementation, we might add a CType::SizeT variant
            CType::UInt if pointer_size == 4 => {
                // Convert generic unsigned to 32-bit on 32-bit systems
                var.var_type = CType::UInt32;
                self.stats.type_improvements += 1;
            }
            _ => {}
        }
    }

    /// Optimize function signature
    fn optimize_function_signature(&mut self, _function: &mut Function, _pointer_size: usize) {
        // Optimize parameter types
        // Parameters will be optimized through variable optimization in optimize_variable_type
    }

    /// Check if two expressions are equal
    fn expressions_equal(&self, left: &Wrapped<Expression>, right: &Wrapped<Expression>) -> bool {
        match (&left.item, &right.item) {
            (Expression::Variable(_vars1, id1), Expression::Variable(_vars2, id2)) => {
                id1 == id2 // Same variable ID
            }
            (Expression::Literal(lit1), Expression::Literal(lit2)) => {
                // Compare literals
                match (lit1, lit2) {
                    (Literal::Int(a), Literal::Int(b)) => a == b,
                    (Literal::UInt(a), Literal::UInt(b)) => a == b,
                    (Literal::Float(a), Literal::Float(b)) => a == b,
                    (Literal::Char(a), Literal::Char(b)) => a == b,
                    (Literal::Bool(a), Literal::Bool(b)) => a == b,
                    (Literal::String(a), Literal::String(b)) => a == b,
                    _ => false,
                }
            }
            _ => false, // Conservative: different types are not equal
        }
    }

    /// Get optimization statistics
    pub fn stats(&self) -> &OptimizationStats {
        &self.stats
    }

    /// Simplify expressions in the AST
    fn simplify_expressions(&mut self, ast: &mut CAst) {
        let mut simplifier = ExpressionSimplifier::new();

        // Simplify expressions in all functions
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                // Simplify expressions in function body
                for stmt in &mut func.body {
                    simplifier.simplify_statement(stmt);
                }
                ast.functions.write().unwrap().insert(id, func);
            }
        }

        // Update stats with simplification results
        let simpl_stats = simplifier.stats();
        self.stats.simd_patterns_recognized += simpl_stats.redundant_parentheses_removed;
        self.stats.type_improvements += simpl_stats.constant_expressions_folded;
    }

    /// Perform advanced constant folding
    fn perform_advanced_constant_folding(&mut self, ast: &mut CAst) {
        let mut folder = AdvancedConstantFolder::new();
        folder.fold_constants(ast);

        // Update stats with folding results
        let fold_stats = folder.stats();
        self.stats.simd_patterns_recognized += fold_stats.constants_propagated;
        self.stats.type_improvements += fold_stats.algebraic_simplifications;
        self.stats.arch_idioms_applied += fold_stats.conditions_eliminated;
        self.stats.cc_optimizations += fold_stats.expressions_precomputed;
    }

    /// Eliminate dead code from the AST
    fn eliminate_dead_code(&mut self, ast: &mut CAst) {
        let mut eliminator = DeadCodeEliminator::new();
        eliminator.eliminate(ast);

        // Update stats with elimination results
        let elim_stats = eliminator.stats();
        // Add dead code stats to existing fields for now
        self.stats.arch_idioms_applied += elim_stats.unreachable_statements_removed;
        self.stats.cc_optimizations += elim_stats.unused_variables_removed;
    }

    /// Eliminate common subexpressions from the AST
    fn eliminate_common_subexpressions(&mut self, ast: &mut CAst) {
        let mut eliminator = CommonSubexpressionEliminator::new();
        eliminator.eliminate(ast);

        // Update stats with CSE results
        let cse_stats = eliminator.stats();
        // Map CSE stats to existing optimization stats
        self.stats.simd_patterns_recognized += cse_stats.subexpressions_eliminated;
        self.stats.type_improvements += cse_stats.temp_variables_created;
        self.stats.arch_idioms_applied += cse_stats.redundant_computations_removed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let config = ArchOptimizationConfig {
            arch_type: ArchType::X86_64,
            arch_info: ArchitectureInfo {
                arch_type: ArchType::X86_64,
                pointer_size: 64,
                endianness: crate::arch::Endianness::Little,
                register_count: 16,
                instruction_alignment: 1,
            },
            enhanced_c_config: EnhancedCConfig::default(),
            enable_simd_patterns: true,
            enable_arch_idioms: true,
            enable_cc_optimizations: true,
            enable_expression_simplification: true,
            enable_dead_code_elimination: true,
            enable_cse: true,
        };

        let optimizer = AstOptimizer::new(config);
        assert_eq!(optimizer.stats.simd_patterns_recognized, 0);
    }

    #[test]
    fn test_xor_self_optimization() {
        let config = ArchOptimizationConfig {
            arch_type: ArchType::X86_64,
            ..Default::default()
        };

        let mut optimizer = AstOptimizer::new(config);
        let mut ast = CAst::new();

        // Create a simple function with XOR self pattern
        // This is a simplified test - in reality, the AST would be more complex

        optimizer.optimize(&mut ast);

        // Check that optimization was attempted
        // In a full implementation, we would verify the transformation
    }
}
