//! Common subexpression elimination (CSE) optimization
//!
//! This module identifies and eliminates redundant computations by storing
//! the results of expressions and reusing them when the same expression
//! appears again within the same scope.
//!
//! Due to limitations in creating new VariableIds, this implementation
//! currently only tracks and reports potential CSE opportunities without
//! actually performing the transformation.

use crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    BinaryOperator, CAst, Expression, Function, Literal, Statement, UnaryOperator, VariableId,
    WrappedStatement,
};
use std::collections::{BTreeMap, BTreeSet};

/// Common subexpression eliminator
pub struct CommonSubexpressionEliminator {
    /// Map of expression hashes to their locations
    expression_cache: BTreeMap<ExpressionHash, Vec<ExpressionLocation>>,
    /// Statistics
    stats: CSEStats,
    /// Variables that have been modified (invalidates expressions using them)
    modified_vars: BTreeSet<VariableId>,
}

#[derive(Debug, Default)]
pub struct CSEStats {
    pub subexpressions_eliminated: usize,
    pub temp_variables_created: usize,
    pub redundant_computations_removed: usize,
}

/// Location where an expression was found
#[derive(Debug, Clone)]
struct ExpressionLocation {
    statement_index: usize,
    is_rhs: bool,
}

/// Hash representation of an expression for comparison
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ExpressionHash {
    Literal(LiteralHash),
    Variable(VariableId),
    UnaryOp(UnaryOperator, Box<ExpressionHash>),
    BinaryOp(BinaryOperator, Box<ExpressionHash>, Box<ExpressionHash>),
    // We don't cache function calls or memory accesses as they may have side effects
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LiteralHash {
    Int(i64),
    UInt(u64),
    Bool(bool),
    Char(char),
    String(String),
    // Float comparison is tricky, so we convert to bits
    Float(u64), // Float bits representation
}

impl CommonSubexpressionEliminator {
    pub fn new() -> Self {
        Self {
            expression_cache: BTreeMap::new(),
            stats: CSEStats::default(),
            modified_vars: BTreeSet::new(),
        }
    }

    /// Perform common subexpression elimination on the entire AST
    pub fn eliminate(&mut self, ast: &mut CAst) {
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                self.analyze_function(&mut func);
                ast.functions.write().unwrap().insert(id, func);
            }
        }
    }

    /// Analyze a function for common subexpressions
    fn analyze_function(&mut self, func: &mut Function) {
        // Clear state for each function
        self.expression_cache.clear();
        self.modified_vars.clear();

        // First pass: collect all expressions
        for (idx, stmt) in func.body.iter().enumerate() {
            self.collect_expressions_in_statement(stmt, idx);
        }

        // Count redundant computations
        for locations in self.expression_cache.values() {
            if locations.len() > 1 {
                self.stats.redundant_computations_removed += locations.len() - 1;
                self.stats.subexpressions_eliminated += 1;
            }
        }

        // Add comments to identify CSE opportunities
        for locations in self.expression_cache.values() {
            if locations.len() > 1 {
                // Add comments to the statements where CSE could be applied
                for (i, loc) in locations.iter().enumerate() {
                    if let Some(stmt) = func.body.get_mut(loc.statement_index) {
                        let comment = if i == 0 {
                            "CSE opportunity: first occurrence of expression".to_string()
                        } else {
                            format!(
                                "CSE opportunity: could reuse expression from statement {}",
                                locations[0].statement_index
                            )
                        };

                        if let Some(existing) = &mut stmt.comment {
                            existing.push_str(&format!("; {}", comment));
                        } else {
                            stmt.comment = Some(comment);
                        }
                    }
                }
            }
        }
    }

    /// Collect expressions in a statement
    fn collect_expressions_in_statement(&mut self, stmt: &WrappedStatement, stmt_idx: usize) {
        match &stmt.statement {
            Statement::Assignment(lhs, rhs) => {
                // Check if we're modifying a variable
                if let Expression::Variable(_, vid) = &lhs.item {
                    self.modified_vars.insert(*vid);
                    // Invalidate cached expressions that use this variable
                    self.invalidate_expressions_using_var(*vid);
                }

                // Collect RHS expression
                if self.is_cacheable_expression(&rhs.item) {
                    if let Some(hash) = self.compute_expression_hash(&rhs.item) {
                        self.expression_cache
                            .entry(hash)
                            .or_default()
                            .push(ExpressionLocation {
                                statement_index: stmt_idx,
                                is_rhs: true,
                            });
                    }
                }
            }
            Statement::If(cond, _then_block, _else_block) => {
                // Collect condition expression
                if self.is_cacheable_expression(&cond.item) {
                    if let Some(hash) = self.compute_expression_hash(&cond.item) {
                        self.expression_cache
                            .entry(hash)
                            .or_default()
                            .push(ExpressionLocation {
                                statement_index: stmt_idx,
                                is_rhs: false,
                            });
                    }
                }
            }
            Statement::While(cond, _body) => {
                // Collect condition expression
                if self.is_cacheable_expression(&cond.item) {
                    if let Some(hash) = self.compute_expression_hash(&cond.item) {
                        self.expression_cache
                            .entry(hash)
                            .or_default()
                            .push(ExpressionLocation {
                                statement_index: stmt_idx,
                                is_rhs: false,
                            });
                    }
                }
            }
            Statement::Return(Some(expr)) => {
                if self.is_cacheable_expression(&expr.item) {
                    if let Some(hash) = self.compute_expression_hash(&expr.item) {
                        self.expression_cache
                            .entry(hash)
                            .or_default()
                            .push(ExpressionLocation {
                                statement_index: stmt_idx,
                                is_rhs: false,
                            });
                    }
                }
            }
            _ => {}
        }
    }

    /// Check if an expression is worth caching
    fn is_cacheable_expression(&self, expr: &Expression) -> bool {
        match expr {
            // Don't cache simple values
            Expression::Literal(_) | Expression::Variable(_, _) => false,
            // Don't cache expressions with side effects
            Expression::Call(_, _) => false,
            Expression::Deref(_) => false, // Memory access might change
            Expression::ArrayAccess(_, _) => false, // Array element might change
            // These are worth caching
            Expression::BinaryOp(_, _, _) => true,
            Expression::UnaryOp(_, _) => true,
            Expression::Cast(_, _) => true,
            _ => false,
        }
    }

    /// Compute a hash representation of an expression
    #[allow(clippy::only_used_in_recursion)]
    fn compute_expression_hash(&self, expr: &Expression) -> Option<ExpressionHash> {
        match expr {
            Expression::Literal(lit) => Some(ExpressionHash::Literal(match lit {
                Literal::Int(n) => LiteralHash::Int(*n),
                Literal::UInt(n) => LiteralHash::UInt(*n),
                Literal::Float(f) => LiteralHash::Float(f.to_bits()),
                Literal::Bool(b) => LiteralHash::Bool(*b),
                Literal::Char(c) => LiteralHash::Char(*c),
                Literal::String(s) => LiteralHash::String(s.clone()),
            })),
            Expression::Variable(_, vid) => Some(ExpressionHash::Variable(*vid)),
            Expression::UnaryOp(op, inner) => {
                let inner_hash = self.compute_expression_hash(&inner.item)?;
                Some(ExpressionHash::UnaryOp(op.clone(), Box::new(inner_hash)))
            }
            Expression::BinaryOp(op, left, right) => {
                let left_hash = self.compute_expression_hash(&left.item)?;
                let right_hash = self.compute_expression_hash(&right.item)?;

                // For commutative operations, normalize the order
                let (left_hash, right_hash) = match op {
                    BinaryOperator::Add
                    | BinaryOperator::Mul
                    | BinaryOperator::BitAnd
                    | BinaryOperator::BitOr
                    | BinaryOperator::BitXor
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual => {
                        if left_hash > right_hash {
                            (right_hash, left_hash)
                        } else {
                            (left_hash, right_hash)
                        }
                    }
                    _ => (left_hash, right_hash),
                };

                Some(ExpressionHash::BinaryOp(
                    op.clone(),
                    Box::new(left_hash),
                    Box::new(right_hash),
                ))
            }
            _ => None, // Don't hash complex expressions
        }
    }

    /// Invalidate cached expressions that use a modified variable
    fn invalidate_expressions_using_var(&mut self, var_id: VariableId) {
        let to_remove: Vec<_> = self
            .expression_cache
            .iter()
            .filter(|(hash, _)| self.expression_uses_var(hash, var_id))
            .map(|(hash, _)| hash.clone())
            .collect();

        for hash in to_remove {
            self.expression_cache.remove(&hash);
        }
    }

    /// Check if an expression hash uses a specific variable
    #[allow(clippy::only_used_in_recursion)]
    fn expression_uses_var(&self, expr_hash: &ExpressionHash, var_id: VariableId) -> bool {
        match expr_hash {
            ExpressionHash::Variable(vid) => *vid == var_id,
            ExpressionHash::UnaryOp(_, inner) => self.expression_uses_var(inner, var_id),
            ExpressionHash::BinaryOp(_, left, right) => {
                self.expression_uses_var(left, var_id) || self.expression_uses_var(right, var_id)
            }
            _ => false,
        }
    }

    /// Get statistics
    pub fn stats(&self) -> &CSEStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cse_basic() {
        let eliminator = CommonSubexpressionEliminator::new();

        // Test basic functionality
        assert_eq!(eliminator.stats().subexpressions_eliminated, 0);
        assert_eq!(eliminator.stats().temp_variables_created, 0);
        assert_eq!(eliminator.stats().redundant_computations_removed, 0);
    }
}
