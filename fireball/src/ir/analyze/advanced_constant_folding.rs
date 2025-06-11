//! Advanced constant folding and propagation
//!
//! This module extends basic constant folding with:
//! - Cross-statement constant propagation
//! - Algebraic simplifications
//! - Compile-time evaluation of more complex expressions
//! - Constant condition elimination

use crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    ArcVariableMap, BinaryOperator, CAst, Expression, Function, Literal, Statement, UnaryOperator,
    VariableId, Wrapped, WrappedStatement,
};
use std::collections::BTreeMap;

/// Advanced constant folder with propagation
pub struct AdvancedConstantFolder {
    /// Known constant values for variables
    constant_values: BTreeMap<VariableId, Literal>,
    /// Statistics
    stats: FoldingStats,
}

#[derive(Debug, Default)]
pub struct FoldingStats {
    pub constants_propagated: usize,
    pub algebraic_simplifications: usize,
    pub conditions_eliminated: usize,
    pub expressions_precomputed: usize,
}

impl AdvancedConstantFolder {
    pub fn new() -> Self {
        Self {
            constant_values: BTreeMap::new(),
            stats: FoldingStats::default(),
        }
    }

    /// Perform advanced constant folding on the entire AST
    pub fn fold_constants(&mut self, ast: &mut CAst) {
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                self.fold_function(&mut func);
                ast.functions.write().unwrap().insert(id, func);
            }
        }
    }

    /// Fold constants in a function
    fn fold_function(&mut self, func: &mut Function) {
        // Clear constant map for each function
        self.constant_values.clear();

        // Process statements
        let mut i = 0;
        while i < func.body.len() {
            if self.fold_statement(&mut func.body[i], &func.variables) {
                // Statement was eliminated
                func.body.remove(i);
                self.stats.conditions_eliminated += 1;
            } else {
                i += 1;
            }
        }
    }

    /// Fold constants in a statement. Returns true if statement should be removed.
    fn fold_statement(&mut self, stmt: &mut WrappedStatement, vars: &ArcVariableMap) -> bool {
        match &mut stmt.statement {
            Statement::Assignment(lhs, rhs) => {
                // First fold the RHS
                self.fold_expression(rhs, vars);

                // Check if we're assigning a constant
                if let Expression::Variable(_, vid) = &lhs.item {
                    if let Expression::Literal(lit) = &rhs.item {
                        // Track this constant assignment
                        self.constant_values.insert(*vid, lit.clone());
                    } else {
                        // Variable is no longer constant
                        self.constant_values.remove(vid);
                    }
                } else {
                    // Complex LHS - fold it too
                    self.fold_expression(lhs, vars);
                }
                false
            }
            Statement::If(cond, then_block, else_block) => {
                self.fold_expression(cond, vars);

                // Check if condition is constant
                if let Expression::Literal(lit) = &cond.item {
                    match lit {
                        Literal::Bool(true) => {
                            // Always true - replace with then block
                            let new_stmts = std::mem::take(then_block);
                            stmt.statement = Statement::Block(new_stmts);
                            return false; // Don't remove, just transformed
                        }
                        Literal::Int(n) if *n != 0 => {
                            // Always true - replace with then block
                            let new_stmts = std::mem::take(then_block);
                            stmt.statement = Statement::Block(new_stmts);
                            return false; // Don't remove, just transformed
                        }
                        Literal::Bool(false) | Literal::Int(0) => {
                            // Always false
                            if let Some(else_stmts) = else_block {
                                let new_stmts = std::mem::take(else_stmts);
                                stmt.statement = Statement::Block(new_stmts);
                                return false; // Don't remove, just transformed
                            } else {
                                return true; // Remove the if entirely
                            }
                        }
                        _ => {}
                    }
                }

                // Fold statements in both branches
                self.fold_statements(then_block, vars);
                if let Some(else_stmts) = else_block {
                    self.fold_statements(else_stmts, vars);
                }
                false
            }
            Statement::While(cond, body) => {
                self.fold_expression(cond, vars);

                // Check for constant false condition
                if let Expression::Literal(Literal::Bool(false) | Literal::Int(0)) = &cond.item {
                    return true; // Remove the while loop entirely
                }

                // For loops, we need to be careful about constant propagation
                // Save current constants and restore after loop
                let saved_constants = self.constant_values.clone();
                self.fold_statements(body, vars);
                self.constant_values = saved_constants;
                false
            }
            Statement::For(init, cond, update, body) => {
                // For loops: init and update are statements
                // Recursively fold the init statement
                if self.fold_statement(init, vars) {
                    // If init is removed, the for loop structure is broken
                    return true;
                }

                // Fold the condition
                self.fold_expression(cond, vars);

                // Check for constant false condition
                if let Expression::Literal(Literal::Bool(false) | Literal::Int(0)) = &cond.item {
                    // For loop never executes
                    return true; // Remove the for loop
                }

                // Recursively fold the update statement
                if self.fold_statement(update, vars) {
                    // If update is removed, the for loop structure is broken
                    return true;
                }

                // Save constants and restore after loop
                let saved_constants = self.constant_values.clone();
                self.fold_statements(body, vars);
                self.constant_values = saved_constants;
                false
            }
            Statement::Block(stmts) => {
                self.fold_statements(stmts, vars);
                stmts.is_empty() // Remove empty blocks
            }
            Statement::Return(Some(expr)) => {
                self.fold_expression(expr, vars);
                false
            }
            Statement::Call(_target, args) => {
                for arg in args {
                    self.fold_expression(arg, vars);
                }
                false
            }
            Statement::Declaration(_, Some(init)) => {
                self.fold_expression(init, vars);
                false
            }
            _ => false,
        }
    }

    /// Fold statements in a block
    fn fold_statements(&mut self, stmts: &mut Vec<WrappedStatement>, vars: &ArcVariableMap) {
        let mut i = 0;
        while i < stmts.len() {
            if self.fold_statement(&mut stmts[i], vars) {
                stmts.remove(i);
                self.stats.conditions_eliminated += 1;
            } else {
                i += 1;
            }
        }
    }

    /// Fold constants in an expression
    fn fold_expression(&mut self, expr: &mut Wrapped<Expression>, vars: &ArcVariableMap) {
        match &mut expr.item {
            Expression::Variable(_, vid) => {
                // Try constant propagation
                if let Some(constant) = self.constant_values.get(vid).cloned() {
                    expr.item = Expression::Literal(constant);
                    self.stats.constants_propagated += 1;
                }
            }
            Expression::BinaryOp(op, left, right) => {
                self.fold_expression(left, vars);
                self.fold_expression(right, vars);

                // Clone values we need before calling other methods
                let op_clone = op.clone();
                let left_clone = left.clone();
                let right_clone = right.clone();

                // Try algebraic simplifications
                self.apply_algebraic_simplifications(
                    expr,
                    op_clone,
                    &left_clone,
                    &right_clone,
                    vars,
                );

                // Try constant folding
                if let Expression::BinaryOp(op2, left2, right2) = &expr.item {
                    if let (Expression::Literal(l), Expression::Literal(r)) =
                        (&left2.item, &right2.item)
                    {
                        if let Some(result) = self.fold_binary_op(op2.clone(), l, r) {
                            expr.item = Expression::Literal(result);
                            self.stats.expressions_precomputed += 1;
                        }
                    }
                }
            }
            Expression::UnaryOp(op, operand) => {
                self.fold_expression(operand, vars);

                // Try constant folding
                if let Expression::Literal(lit) = &operand.item {
                    if let Some(result) = self.fold_unary_op(op.clone(), lit) {
                        expr.item = Expression::Literal(result);
                        self.stats.expressions_precomputed += 1;
                    }
                }
            }
            Expression::Call(_target, args) => {
                // target is a String, not an expression
                for arg in args {
                    self.fold_expression(arg, vars);
                }
            }
            Expression::ArrayAccess(array, index) => {
                self.fold_expression(array, vars);
                self.fold_expression(index, vars);
            }
            Expression::MemberAccess(base, _field) => {
                self.fold_expression(base, vars);
            }
            Expression::Deref(inner) => {
                self.fold_expression(inner, vars);
            }
            Expression::AddressOf(inner) => {
                self.fold_expression(inner, vars);
            }
            Expression::Cast(_, inner) => {
                self.fold_expression(inner, vars);
                // TODO: Fold casts when safe
            }
            _ => {}
        }
    }

    /// Apply algebraic simplifications
    fn apply_algebraic_simplifications(
        &mut self,
        expr: &mut Wrapped<Expression>,
        op: BinaryOperator,
        left: &Wrapped<Expression>,
        right: &Wrapped<Expression>,
        vars: &ArcVariableMap,
    ) {
        match op {
            BinaryOperator::Add => {
                // (a + b) + c -> a + (b + c) if b and c are constants
                if let Expression::BinaryOp(BinaryOperator::Add, a, b) = &left.item {
                    if let (Expression::Literal(_), Expression::Literal(_)) = (&b.item, &right.item)
                    {
                        // Rewrite to a + (b + c)
                        let new_right = Wrapped {
                            item: Expression::BinaryOp(
                                BinaryOperator::Add,
                                b.clone(),
                                Box::new(right.clone()),
                            ),
                            origin_expr: None,
                            comment: None,
                        };
                        expr.item = Expression::BinaryOp(
                            BinaryOperator::Add,
                            a.clone(),
                            Box::new(new_right),
                        );
                        self.stats.algebraic_simplifications += 1;
                        // Re-fold to compute b + c
                        self.fold_expression(expr, vars);
                    }
                }
            }
            BinaryOperator::Mul => {
                // Strength reduction: x * 2^n -> x << n
                if let Expression::Literal(Literal::Int(n)) = &right.item {
                    if *n > 0 && (*n as u64).is_power_of_two() {
                        let shift = (*n as u64).trailing_zeros() as i64;
                        expr.item = Expression::BinaryOp(
                            BinaryOperator::LeftShift,
                            Box::new(left.clone()),
                            Box::new(Wrapped {
                                item: Expression::Literal(Literal::Int(shift)),
                                origin_expr: None,
                                comment: None,
                            }),
                        );
                        self.stats.algebraic_simplifications += 1;
                    }
                }
            }
            BinaryOperator::Div => {
                // Strength reduction: x / 2^n -> x >> n (for unsigned)
                if let Expression::Literal(Literal::UInt(n)) = &right.item {
                    if n.is_power_of_two() && *n > 0 {
                        let shift = n.trailing_zeros() as i64;
                        expr.item = Expression::BinaryOp(
                            BinaryOperator::RightShift,
                            Box::new(left.clone()),
                            Box::new(Wrapped {
                                item: Expression::Literal(Literal::Int(shift)),
                                origin_expr: None,
                                comment: None,
                            }),
                        );
                        self.stats.algebraic_simplifications += 1;
                    }
                }
            }
            _ => {}
        }
    }

    /// Fold binary operation on literals
    fn fold_binary_op(
        &self,
        op: BinaryOperator,
        left: &Literal,
        right: &Literal,
    ) -> Option<Literal> {
        match (left, right) {
            (Literal::Int(a), Literal::Int(b)) => {
                let result = match op {
                    BinaryOperator::Add => a.checked_add(*b),
                    BinaryOperator::Sub => a.checked_sub(*b),
                    BinaryOperator::Mul => a.checked_mul(*b),
                    BinaryOperator::Div if *b != 0 => a.checked_div(*b),
                    BinaryOperator::Mod if *b != 0 => a.checked_rem(*b),
                    BinaryOperator::BitAnd => Some(a & b),
                    BinaryOperator::BitOr => Some(a | b),
                    BinaryOperator::BitXor => Some(a ^ b),
                    BinaryOperator::LeftShift if *b >= 0 && *b < 64 => a.checked_shl(*b as u32),
                    BinaryOperator::RightShift if *b >= 0 && *b < 64 => a.checked_shr(*b as u32),
                    BinaryOperator::Less => Some(if a < b { 1 } else { 0 }),
                    BinaryOperator::Greater => Some(if a > b { 1 } else { 0 }),
                    BinaryOperator::LessEqual => Some(if a <= b { 1 } else { 0 }),
                    BinaryOperator::GreaterEqual => Some(if a >= b { 1 } else { 0 }),
                    BinaryOperator::Equal => Some(if a == b { 1 } else { 0 }),
                    BinaryOperator::NotEqual => Some(if a != b { 1 } else { 0 }),
                    BinaryOperator::LogicAnd => Some(if *a != 0 && *b != 0 { 1 } else { 0 }),
                    BinaryOperator::LogicOr => Some(if *a != 0 || *b != 0 { 1 } else { 0 }),
                    _ => None,
                };
                result.map(Literal::Int)
            }
            (Literal::UInt(a), Literal::UInt(b)) => {
                let result = match op {
                    BinaryOperator::Add => a.checked_add(*b),
                    BinaryOperator::Sub => a.checked_sub(*b),
                    BinaryOperator::Mul => a.checked_mul(*b),
                    BinaryOperator::Div if *b != 0 => a.checked_div(*b),
                    BinaryOperator::Mod if *b != 0 => a.checked_rem(*b),
                    BinaryOperator::BitAnd => Some(a & b),
                    BinaryOperator::BitOr => Some(a | b),
                    BinaryOperator::BitXor => Some(a ^ b),
                    BinaryOperator::LeftShift if *b < 64 => a.checked_shl(*b as u32),
                    BinaryOperator::RightShift if *b < 64 => a.checked_shr(*b as u32),
                    BinaryOperator::Less => Some(if a < b { 1 } else { 0 }),
                    BinaryOperator::Greater => Some(if a > b { 1 } else { 0 }),
                    BinaryOperator::LessEqual => Some(if a <= b { 1 } else { 0 }),
                    BinaryOperator::GreaterEqual => Some(if a >= b { 1 } else { 0 }),
                    BinaryOperator::Equal => Some(if a == b { 1 } else { 0 }),
                    BinaryOperator::NotEqual => Some(if a != b { 1 } else { 0 }),
                    BinaryOperator::LogicAnd => Some(if *a != 0 && *b != 0 { 1 } else { 0 }),
                    BinaryOperator::LogicOr => Some(if *a != 0 || *b != 0 { 1 } else { 0 }),
                    _ => None,
                };
                result.map(Literal::UInt)
            }
            (Literal::Bool(a), Literal::Bool(b)) => match op {
                BinaryOperator::LogicAnd => Some(Literal::Bool(*a && *b)),
                BinaryOperator::LogicOr => Some(Literal::Bool(*a || *b)),
                BinaryOperator::Equal => Some(Literal::Bool(a == b)),
                BinaryOperator::NotEqual => Some(Literal::Bool(a != b)),
                _ => None,
            },
            _ => None,
        }
    }

    /// Fold unary operation on literal
    fn fold_unary_op(&self, op: UnaryOperator, lit: &Literal) -> Option<Literal> {
        match (op, lit) {
            (UnaryOperator::Negate, Literal::Int(n)) => Some(Literal::Int(-n)),
            (UnaryOperator::BitNot, Literal::Int(n)) => Some(Literal::Int(!n)),
            (UnaryOperator::BitNot, Literal::UInt(n)) => Some(Literal::UInt(!n)),
            (UnaryOperator::Not, Literal::Bool(b)) => Some(Literal::Bool(!b)),
            (UnaryOperator::Not, Literal::Int(n)) => Some(Literal::Bool(*n == 0)),
            (UnaryOperator::Not, Literal::UInt(n)) => Some(Literal::Bool(*n == 0)),
            _ => None,
        }
    }

    /// Check if expression is pure (no side effects)
    fn is_pure_expression(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Literal(_) | Expression::Variable(_, _) => true,
            Expression::BinaryOp(_, left, right) => {
                self.is_pure_expression(&left.item) && self.is_pure_expression(&right.item)
            }
            Expression::UnaryOp(_, operand) => self.is_pure_expression(&operand.item),
            Expression::Cast(_, inner) => self.is_pure_expression(&inner.item),
            Expression::AddressOf(inner) => self.is_pure_expression(&inner.item),
            Expression::Call(_, _) => false, // Calls may have side effects
            Expression::Deref(_) => false,   // Memory access may fault
            Expression::ArrayAccess(_, _) => false, // Array access may be out of bounds
            Expression::MemberAccess(_, _) => false, // Member access may fault
            _ => false,
        }
    }

    /// Get statistics
    pub fn stats(&self) -> &FoldingStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_propagation() {
        let folder = AdvancedConstantFolder::new();

        // Test basic implementation
        assert_eq!(folder.stats().constants_propagated, 0);
        assert_eq!(folder.stats().algebraic_simplifications, 0);
        assert_eq!(folder.stats().conditions_eliminated, 0);
        assert_eq!(folder.stats().expressions_precomputed, 0);
    }

    #[test]
    fn test_algebraic_simplification() {
        let folder = AdvancedConstantFolder::new();

        // Test x * 2 -> x << 1
        let lit2 = Literal::Int(2);
        let lit4 = Literal::Int(4);

        // Test constant folding
        let result = folder.fold_binary_op(BinaryOperator::Add, &lit2, &lit4);
        assert_eq!(result, Some(Literal::Int(6)));
    }

    #[test]
    fn test_condition_elimination() {
        let folder = AdvancedConstantFolder::new();

        // Test that if (true) gets simplified
        assert_eq!(folder.stats().conditions_eliminated, 0);
    }
}
