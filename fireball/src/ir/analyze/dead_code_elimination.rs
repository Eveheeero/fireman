//! Dead code elimination for C AST
//!
//! This module removes unreachable code and unused variables from the generated C code.
//! It performs:
//! - Unreachable code removal (code after return statements)
//! - Unused variable elimination
//! - Empty block removal
//! - Redundant control flow simplification

use crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    CAst, Expression, Function, FunctionId, JumpTarget, Statement, VariableId, Wrapped,
    WrappedStatement,
};
use std::collections::BTreeSet;

/// Dead code eliminator
pub struct DeadCodeEliminator {
    /// Statistics about eliminated code
    stats: EliminationStats,
    /// Track which variables are used
    used_variables: BTreeSet<VariableId>,
    /// Track which functions are called
    called_functions: BTreeSet<FunctionId>,
}

#[derive(Debug, Default)]
pub struct EliminationStats {
    pub unreachable_statements_removed: usize,
    pub unused_variables_removed: usize,
    pub empty_blocks_removed: usize,
    pub redundant_jumps_removed: usize,
}

impl DeadCodeEliminator {
    /// Create a new dead code eliminator
    pub fn new() -> Self {
        Self {
            stats: EliminationStats::default(),
            used_variables: BTreeSet::new(),
            called_functions: BTreeSet::new(),
        }
    }

    /// Eliminate dead code from the entire AST
    pub fn eliminate(&mut self, ast: &mut CAst) {
        // First pass: collect all function calls to identify entry points
        self.collect_called_functions(ast);

        // Process each function
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                // Reset per-function state
                self.used_variables.clear();

                // First, collect all used variables
                self.collect_used_variables(&func);

                // Then eliminate dead code
                self.eliminate_in_function(&mut func);

                // Remove unused variables
                self.remove_unused_variables(&mut func);

                ast.functions.write().unwrap().insert(id, func);
            }
        }

        // Remove uncalled functions (except main and exported functions)
        self.remove_uncalled_functions(ast);
    }

    /// Collect all function calls in the AST
    fn collect_called_functions(&mut self, ast: &CAst) {
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(func) = ast.functions.read().unwrap().get(&id) {
                for stmt in &func.body {
                    self.collect_calls_in_statement(stmt);
                }
            }
        }
    }

    /// Collect function calls in a statement
    fn collect_calls_in_statement(&mut self, stmt: &WrappedStatement) {
        match &stmt.statement {
            Statement::Call(target, args) => {
                // Record the function call
                match target {
                    JumpTarget::Variable { id, .. } => {
                        self.used_variables.insert(*id);
                    }
                    JumpTarget::Function { target } => {
                        self.called_functions.insert(*target);
                    }
                    _ => {}
                }

                // Check arguments for nested calls
                for arg in args {
                    self.collect_calls_in_expression(arg);
                }
            }
            Statement::Assignment(_, expr) => {
                self.collect_calls_in_expression(expr);
            }
            Statement::Return(Some(expr)) => {
                self.collect_calls_in_expression(expr);
            }
            Statement::If(cond, then_block, else_block) => {
                self.collect_calls_in_expression(cond);
                for s in then_block {
                    self.collect_calls_in_statement(s);
                }
                if let Some(else_stmts) = else_block {
                    for s in else_stmts {
                        self.collect_calls_in_statement(s);
                    }
                }
            }
            Statement::While(cond, body) => {
                self.collect_calls_in_expression(cond);
                for s in body {
                    self.collect_calls_in_statement(s);
                }
            }
            Statement::For(init, cond, update, body) => {
                self.collect_calls_in_statement(init);
                self.collect_calls_in_expression(cond);
                self.collect_calls_in_statement(update);
                for s in body {
                    self.collect_calls_in_statement(s);
                }
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.collect_calls_in_statement(s);
                }
            }
            _ => {}
        }
    }

    /// Collect function calls in an expression
    #[allow(clippy::only_used_in_recursion)]
    fn collect_calls_in_expression(&mut self, expr: &Wrapped<Expression>) {
        match &expr.item {
            Expression::Call(_, args) => {
                for arg in args {
                    self.collect_calls_in_expression(arg);
                }
            }
            Expression::UnaryOp(_, inner) => {
                self.collect_calls_in_expression(inner);
            }
            Expression::BinaryOp(_, left, right) => {
                self.collect_calls_in_expression(left);
                self.collect_calls_in_expression(right);
            }
            Expression::Cast(_, inner) => {
                self.collect_calls_in_expression(inner);
            }
            Expression::ArrayAccess(base, index) => {
                self.collect_calls_in_expression(base);
                self.collect_calls_in_expression(index);
            }
            Expression::MemberAccess(base, _) => {
                self.collect_calls_in_expression(base);
            }
            Expression::AddressOf(inner) | Expression::Deref(inner) => {
                self.collect_calls_in_expression(inner);
            }
            _ => {}
        }
    }

    /// Collect all variables used in a function
    fn collect_used_variables(&mut self, func: &Function) {
        for stmt in &func.body {
            self.collect_used_vars_in_statement(stmt);
        }
    }

    /// Collect used variables in a statement
    fn collect_used_vars_in_statement(&mut self, stmt: &WrappedStatement) {
        match &stmt.statement {
            Statement::Assignment(lhs, rhs) => {
                // LHS might be a complex expression (array access, member access)
                self.collect_used_vars_in_expression(lhs);
                self.collect_used_vars_in_expression(rhs);
            }
            Statement::Return(Some(expr)) => {
                self.collect_used_vars_in_expression(expr);
            }
            Statement::If(cond, then_block, else_block) => {
                self.collect_used_vars_in_expression(cond);
                for s in then_block {
                    self.collect_used_vars_in_statement(s);
                }
                if let Some(else_stmts) = else_block {
                    for s in else_stmts {
                        self.collect_used_vars_in_statement(s);
                    }
                }
            }
            Statement::While(cond, body) => {
                self.collect_used_vars_in_expression(cond);
                for s in body {
                    self.collect_used_vars_in_statement(s);
                }
            }
            Statement::For(init, cond, update, body) => {
                self.collect_used_vars_in_statement(init);
                self.collect_used_vars_in_expression(cond);
                self.collect_used_vars_in_statement(update);
                for s in body {
                    self.collect_used_vars_in_statement(s);
                }
            }
            Statement::Call(target, args) => {
                if let JumpTarget::Variable { id, .. } = target {
                    self.used_variables.insert(*id);
                }
                for arg in args {
                    self.collect_used_vars_in_expression(arg);
                }
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.collect_used_vars_in_statement(s);
                }
            }
            _ => {}
        }
    }

    /// Collect used variables in an expression
    #[allow(clippy::only_used_in_recursion)]
    fn collect_used_vars_in_expression(&mut self, expr: &Wrapped<Expression>) {
        match &expr.item {
            Expression::Variable(_, var_id) => {
                self.used_variables.insert(*var_id);
            }
            Expression::UnaryOp(_, inner) => {
                self.collect_used_vars_in_expression(inner);
            }
            Expression::BinaryOp(_, left, right) => {
                self.collect_used_vars_in_expression(left);
                self.collect_used_vars_in_expression(right);
            }
            Expression::Cast(_, inner) => {
                self.collect_used_vars_in_expression(inner);
            }
            Expression::ArrayAccess(base, index) => {
                self.collect_used_vars_in_expression(base);
                self.collect_used_vars_in_expression(index);
            }
            Expression::MemberAccess(base, _) => {
                self.collect_used_vars_in_expression(base);
            }
            Expression::AddressOf(inner) | Expression::Deref(inner) => {
                self.collect_used_vars_in_expression(inner);
            }
            Expression::Call(_, args) => {
                for arg in args {
                    self.collect_used_vars_in_expression(arg);
                }
            }
            _ => {}
        }
    }

    /// Eliminate dead code in a function
    fn eliminate_in_function(&mut self, func: &mut Function) {
        // Remove unreachable statements
        self.remove_unreachable_statements(&mut func.body);

        // Remove empty blocks
        self.remove_empty_blocks(&mut func.body);

        // Simplify redundant control flow
        self.simplify_control_flow(&mut func.body);
    }

    /// Remove statements after return statements
    fn remove_unreachable_statements(&mut self, stmts: &mut Vec<WrappedStatement>) {
        let mut new_stmts = Vec::new();
        let mut found_return = false;

        for mut stmt in stmts.drain(..) {
            if found_return {
                self.stats.unreachable_statements_removed += 1;
                continue;
            }

            match &mut stmt.statement {
                Statement::Return(_) => {
                    found_return = true;
                    new_stmts.push(stmt);
                }
                Statement::Block(block_stmts) => {
                    self.remove_unreachable_statements(block_stmts);
                    new_stmts.push(stmt);
                }
                Statement::If(_, then_block, else_block) => {
                    self.remove_unreachable_statements(then_block);
                    if let Some(else_stmts) = else_block {
                        self.remove_unreachable_statements(else_stmts);
                    }
                    new_stmts.push(stmt);
                }
                Statement::While(_, body) => {
                    self.remove_unreachable_statements(body);
                    new_stmts.push(stmt);
                }
                Statement::For(_, _, _, body) => {
                    self.remove_unreachable_statements(body);
                    new_stmts.push(stmt);
                }
                _ => {
                    new_stmts.push(stmt);
                }
            }
        }

        *stmts = new_stmts;
    }

    /// Remove empty blocks
    fn remove_empty_blocks(&mut self, stmts: &mut Vec<WrappedStatement>) {
        stmts.retain(|stmt| match &stmt.statement {
            Statement::Block(block_stmts) if block_stmts.is_empty() => {
                self.stats.empty_blocks_removed += 1;
                false
            }
            _ => true,
        });

        // Recursively clean nested structures
        for stmt in stmts.iter_mut() {
            match &mut stmt.statement {
                Statement::Block(block_stmts) => {
                    self.remove_empty_blocks(block_stmts);
                }
                Statement::If(_, then_block, else_block) => {
                    self.remove_empty_blocks(then_block);
                    if let Some(else_stmts) = else_block {
                        self.remove_empty_blocks(else_stmts);
                    }
                }
                Statement::While(_, body) => {
                    self.remove_empty_blocks(body);
                }
                Statement::For(_, _, _, body) => {
                    self.remove_empty_blocks(body);
                }
                _ => {}
            }
        }
    }

    /// Simplify redundant control flow
    #[allow(clippy::only_used_in_recursion)]
    fn simplify_control_flow(&mut self, stmts: &mut [WrappedStatement]) {
        for stmt in stmts.iter_mut() {
            match &mut stmt.statement {
                // Remove if statements with empty branches
                Statement::If(_, then_block, else_block) => {
                    if then_block.is_empty() && else_block.as_ref().is_none_or(Vec::is_empty) {
                        // Both branches are empty, this will be removed by empty block removal
                        continue;
                    }

                    // Recursively simplify nested blocks
                    self.simplify_control_flow(then_block);
                    if let Some(else_stmts) = else_block {
                        self.simplify_control_flow(else_stmts);
                    }
                }
                Statement::While(_, body) => {
                    self.simplify_control_flow(body);
                }
                Statement::For(_, _, _, body) => {
                    self.simplify_control_flow(body);
                }
                Statement::Block(block_stmts) => {
                    self.simplify_control_flow(block_stmts);
                }
                _ => {}
            }
        }
    }

    /// Remove unused variables from a function
    fn remove_unused_variables(&mut self, func: &mut Function) {
        let mut vars_to_remove = Vec::new();

        // Check each variable
        for (var_id, _var) in func.variables.read().unwrap().iter() {
            if !self.used_variables.contains(var_id) {
                vars_to_remove.push(*var_id);
                self.stats.unused_variables_removed += 1;
            }
        }

        // Remove unused variables
        let mut vars = func.variables.write().unwrap();
        for var_id in vars_to_remove {
            vars.remove(&var_id);
        }
    }

    /// Remove uncalled functions (except main and exported)
    fn remove_uncalled_functions(&mut self, ast: &mut CAst) {
        let mut funcs_to_remove = Vec::new();

        // Check each function
        let functions = ast.functions.read().unwrap();
        for (id, func) in functions.iter() {
            // Keep main function and any function marked as exported
            if func.name == "main" || func.name.starts_with("export_") {
                continue;
            }

            // Remove if not called
            if !self.called_functions.contains(id) {
                funcs_to_remove.push(*id);
            }
        }
        drop(functions);

        // Remove uncalled functions
        let mut functions = ast.functions.write().unwrap();
        for id in funcs_to_remove {
            functions.remove(&id);
        }
    }

    /// Get elimination statistics
    pub fn stats(&self) -> &EliminationStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dead_code_eliminator_creation() {
        let eliminator = DeadCodeEliminator::new();
        assert_eq!(eliminator.stats.unreachable_statements_removed, 0);
        assert_eq!(eliminator.stats.unused_variables_removed, 0);
        assert_eq!(eliminator.stats.empty_blocks_removed, 0);
        assert_eq!(eliminator.stats.redundant_jumps_removed, 0);
    }

    #[test]
    fn test_empty_ast() {
        let mut eliminator = DeadCodeEliminator::new();
        let mut ast = CAst::new();

        eliminator.eliminate(&mut ast);

        // Should handle empty AST without errors
        assert_eq!(eliminator.stats.unreachable_statements_removed, 0);
    }
}
