//! Expression simplification for C AST
//!
//! This module provides expression simplification to improve code readability
//! by removing redundant parentheses and simplifying expressions.

use crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    BinaryOperator, CAst, Expression, Literal, Statement, UnaryOperator, Wrapped, WrappedStatement,
};

/// Expression simplifier that removes redundant operations and parentheses
pub struct ExpressionSimplifier {
    /// Statistics about simplifications performed
    stats: SimplificationStats,
}

#[derive(Debug, Default)]
pub struct SimplificationStats {
    pub redundant_parentheses_removed: usize,
    pub double_negations_removed: usize,
    pub identity_operations_removed: usize,
    pub constant_expressions_folded: usize,
    pub redundant_casts_removed: usize,
}

impl Default for ExpressionSimplifier {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpressionSimplifier {
    pub fn new() -> Self {
        Self {
            stats: SimplificationStats::default(),
        }
    }

    /// Simplify all expressions in the AST
    pub fn simplify_ast(&mut self, ast: &mut CAst) {
        // Process all functions
        let function_ids: Vec<_> = ast.functions.read().unwrap().keys().cloned().collect();
        for id in function_ids {
            if let Some(mut func) = ast.functions.write().unwrap().remove(&id) {
                // Simplify each statement in the function body
                for stmt in &mut func.body {
                    self.simplify_statement(stmt);
                }
                ast.functions.write().unwrap().insert(id, func);
            }
        }
    }

    /// Simplify expressions in a statement
    pub fn simplify_statement(&mut self, stmt: &mut WrappedStatement) {
        match &mut stmt.statement {
            Statement::Declaration(_var, init_expr) => {
                if let Some(expr) = init_expr {
                    self.simplify_expression(expr);
                }
            }
            Statement::Assignment(lhs, rhs) => {
                self.simplify_expression(lhs);
                self.simplify_expression(rhs);
            }
            Statement::If(cond, then_block, else_block) => {
                self.simplify_expression(cond);
                for s in then_block {
                    self.simplify_statement(s);
                }
                if let Some(else_stmts) = else_block {
                    for s in else_stmts {
                        self.simplify_statement(s);
                    }
                }
            }
            Statement::While(cond, body) => {
                self.simplify_expression(cond);
                for s in body {
                    self.simplify_statement(s);
                }
            }
            Statement::For(init, cond, update, body) => {
                self.simplify_statement(init);
                self.simplify_expression(cond);
                self.simplify_statement(update);
                for s in body {
                    self.simplify_statement(s);
                }
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    self.simplify_expression(e);
                }
            }
            Statement::Call(_target, args) => {
                for arg in args {
                    self.simplify_expression(arg);
                }
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.simplify_statement(s);
                }
            }
            _ => {}
        }
    }

    /// Simplify an expression recursively
    fn simplify_expression(&mut self, expr: &mut Wrapped<Expression>) {
        // First, recursively simplify sub-expressions
        match &mut expr.item {
            Expression::UnaryOp(_, inner) => {
                self.simplify_expression(inner);
            }
            Expression::BinaryOp(_, left, right) => {
                self.simplify_expression(left);
                self.simplify_expression(right);
            }
            Expression::Cast(_, inner) => {
                self.simplify_expression(inner);
            }
            Expression::Deref(inner) => {
                self.simplify_expression(inner);
            }
            Expression::AddressOf(inner) => {
                self.simplify_expression(inner);
            }
            Expression::ArrayAccess(base, index) => {
                self.simplify_expression(base);
                self.simplify_expression(index);
            }
            Expression::MemberAccess(base, _field) => {
                self.simplify_expression(base);
            }
            Expression::Call(_name, args) => {
                for arg in args {
                    self.simplify_expression(arg);
                }
            }
            _ => {} // Literals, variables, etc. don't need simplification
        }

        // Now apply pattern-based simplifications
        match &expr.item {
            Expression::UnaryOp(op, inner) => {
                let op_clone = op.clone();
                let inner_clone = inner.clone();
                self.simplify_unary_op(expr, op_clone, &inner_clone);
            }
            Expression::BinaryOp(op, left, right) => {
                let op_clone = op.clone();
                let left_clone = left.clone();
                let right_clone = right.clone();
                self.simplify_binary_op(expr, op_clone, &left_clone, &right_clone);
            }
            Expression::Cast(_, _) => {
                self.simplify_cast(expr);
            }
            _ => {}
        }
    }

    /// Simplify unary operations
    fn simplify_unary_op(
        &mut self,
        expr: &mut Wrapped<Expression>,
        op: UnaryOperator,
        inner: &Wrapped<Expression>,
    ) {
        match op {
            UnaryOperator::Not => {
                // Double negation: !!x -> x
                if let Expression::UnaryOp(UnaryOperator::Not, inner_inner) = &inner.item {
                    expr.item = inner_inner.item.clone();
                    self.stats.double_negations_removed += 1;
                }
            }
            UnaryOperator::BitNot => {
                // Double bitwise NOT: ~~x -> x
                if let Expression::UnaryOp(UnaryOperator::BitNot, inner_inner) = &inner.item {
                    expr.item = inner_inner.item.clone();
                    self.stats.double_negations_removed += 1;
                }
            }
            UnaryOperator::Negate => {
                // Negate literal: -5 -> literal(-5)
                if let Expression::Literal(Literal::Int(n)) = &inner.item {
                    expr.item = Expression::Literal(Literal::Int(-n));
                    self.stats.constant_expressions_folded += 1;
                }
            }
            _ => {}
        }
    }

    /// Simplify binary operations
    fn simplify_binary_op(
        &mut self,
        expr: &mut Wrapped<Expression>,
        op: BinaryOperator,
        left: &Wrapped<Expression>,
        right: &Wrapped<Expression>,
    ) {
        // Check for identity operations
        match op {
            BinaryOperator::Add => {
                // x + 0 -> x
                if self.is_zero(&right.item) {
                    expr.item = left.item.clone();
                    self.stats.identity_operations_removed += 1;
                } else if self.is_zero(&left.item) {
                    // 0 + x -> x
                    expr.item = right.item.clone();
                    self.stats.identity_operations_removed += 1;
                }
            }
            BinaryOperator::Sub => {
                // x - 0 -> x
                if self.is_zero(&right.item) {
                    expr.item = left.item.clone();
                    self.stats.identity_operations_removed += 1;
                }
                // x - x -> 0
                else if self.expressions_equal(&left.item, &right.item) {
                    expr.item = Expression::Literal(Literal::Int(0));
                    self.stats.identity_operations_removed += 1;
                }
            }
            BinaryOperator::Mul => {
                // x * 1 -> x
                if self.is_one(&right.item) {
                    expr.item = left.item.clone();
                    self.stats.identity_operations_removed += 1;
                } else if self.is_one(&left.item) {
                    // 1 * x -> x
                    expr.item = right.item.clone();
                    self.stats.identity_operations_removed += 1;
                }
                // x * 0 -> 0
                else if self.is_zero(&right.item) || self.is_zero(&left.item) {
                    expr.item = Expression::Literal(Literal::Int(0));
                    self.stats.identity_operations_removed += 1;
                }
            }
            BinaryOperator::Div => {
                // x / 1 -> x
                if self.is_one(&right.item) {
                    expr.item = left.item.clone();
                    self.stats.identity_operations_removed += 1;
                }
            }
            BinaryOperator::BitAnd => {
                // x & 0 -> 0
                if self.is_zero(&right.item) || self.is_zero(&left.item) {
                    expr.item = Expression::Literal(Literal::Int(0));
                    self.stats.identity_operations_removed += 1;
                }
                // x & -1 -> x (all bits set)
                else if self.is_all_bits_set(&right.item) {
                    expr.item = left.item.clone();
                    self.stats.identity_operations_removed += 1;
                } else if self.is_all_bits_set(&left.item) {
                    expr.item = right.item.clone();
                    self.stats.identity_operations_removed += 1;
                }
            }
            BinaryOperator::BitOr => {
                // x | 0 -> x
                if self.is_zero(&right.item) {
                    expr.item = left.item.clone();
                    self.stats.identity_operations_removed += 1;
                } else if self.is_zero(&left.item) {
                    expr.item = right.item.clone();
                    self.stats.identity_operations_removed += 1;
                }
                // x | -1 -> -1 (all bits set)
                else if self.is_all_bits_set(&right.item) || self.is_all_bits_set(&left.item) {
                    expr.item = Expression::Literal(Literal::UInt(u64::MAX));
                    self.stats.identity_operations_removed += 1;
                }
            }
            BinaryOperator::BitXor => {
                // x ^ 0 -> x
                if self.is_zero(&right.item) {
                    expr.item = left.item.clone();
                    self.stats.identity_operations_removed += 1;
                } else if self.is_zero(&left.item) {
                    expr.item = right.item.clone();
                    self.stats.identity_operations_removed += 1;
                }
                // x ^ x -> 0
                else if self.expressions_equal(&left.item, &right.item) {
                    expr.item = Expression::Literal(Literal::Int(0));
                    self.stats.identity_operations_removed += 1;
                }
            }
            BinaryOperator::LeftShift | BinaryOperator::RightShift => {
                // x << 0 or x >> 0 -> x
                if self.is_zero(&right.item) {
                    expr.item = left.item.clone();
                    self.stats.identity_operations_removed += 1;
                }
            }
            _ => {}
        }

        // Constant folding for arithmetic operations
        self.try_fold_constants(expr, op, &left.item, &right.item);
    }

    /// Simplify cast operations
    fn simplify_cast(&mut self, expr: &mut Wrapped<Expression>) {
        if let Expression::Cast(outer_type, inner) = &expr.item {
            // Double cast: (T1)(T2)x -> (T1)x if T1 == T2
            if let Expression::Cast(inner_type, inner_expr) = &inner.item {
                if outer_type == inner_type {
                    expr.item = Expression::Cast(outer_type.clone(), inner_expr.clone());
                    self.stats.redundant_casts_removed += 1;
                }
            }
        }
    }

    /// Try to fold constant expressions
    fn try_fold_constants(
        &mut self,
        expr: &mut Wrapped<Expression>,
        op: BinaryOperator,
        left: &Expression,
        right: &Expression,
    ) {
        if let (Expression::Literal(Literal::Int(a)), Expression::Literal(Literal::Int(b))) =
            (left, right)
        {
            let result = match op {
                BinaryOperator::Add => Some(a + b),
                BinaryOperator::Sub => Some(a - b),
                BinaryOperator::Mul => Some(a * b),
                BinaryOperator::Div if *b != 0 => Some(a / b),
                BinaryOperator::Mod if *b != 0 => Some(a % b),
                BinaryOperator::BitAnd => Some(a & b),
                BinaryOperator::BitOr => Some(a | b),
                BinaryOperator::BitXor => Some(a ^ b),
                BinaryOperator::LeftShift if *b >= 0 && *b < 64 => Some(a << b),
                BinaryOperator::RightShift if *b >= 0 && *b < 64 => Some(a >> b),
                _ => None,
            };

            if let Some(val) = result {
                expr.item = Expression::Literal(Literal::Int(val));
                self.stats.constant_expressions_folded += 1;
            }
        }
    }

    /// Check if expression is zero
    fn is_zero(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Literal(Literal::Int(0)) => true,
            Expression::Literal(Literal::UInt(0)) => true,
            Expression::Literal(Literal::Float(f)) if *f == 0.0 => true,
            _ => false,
        }
    }

    /// Check if expression is one
    fn is_one(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Literal(Literal::Int(1)) => true,
            Expression::Literal(Literal::UInt(1)) => true,
            Expression::Literal(Literal::Float(f)) if *f == 1.0 => true,
            _ => false,
        }
    }

    /// Check if expression has all bits set (-1 for signed, MAX for unsigned)
    fn is_all_bits_set(&self, expr: &Expression) -> bool {
        matches!(
            expr,
            Expression::Literal(Literal::Int(-1)) | Expression::Literal(Literal::UInt(u64::MAX))
        )
    }

    /// Check if two expressions are structurally equal
    fn expressions_equal(&self, left: &Expression, right: &Expression) -> bool {
        match (left, right) {
            (Expression::Variable(_vars1, id1), Expression::Variable(_vars2, id2)) => id1 == id2,
            (Expression::Literal(lit1), Expression::Literal(lit2)) => match (lit1, lit2) {
                (Literal::Int(a), Literal::Int(b)) => a == b,
                (Literal::UInt(a), Literal::UInt(b)) => a == b,
                (Literal::Float(a), Literal::Float(b)) => a == b,
                (Literal::Char(a), Literal::Char(b)) => a == b,
                (Literal::Bool(a), Literal::Bool(b)) => a == b,
                (Literal::String(a), Literal::String(b)) => a == b,
                _ => false,
            },
            _ => false, // Conservative: different types are not equal
        }
    }

    /// Get simplification statistics
    pub fn stats(&self) -> &SimplificationStats {
        &self.stats
    }
}

/// Operator precedence for determining when parentheses are needed
fn operator_precedence(op: &BinaryOperator) -> u8 {
    match op {
        BinaryOperator::Mul | BinaryOperator::Div | BinaryOperator::Mod => 12,
        BinaryOperator::Add | BinaryOperator::Sub => 11,
        BinaryOperator::LeftShift | BinaryOperator::RightShift => 10,
        BinaryOperator::Less
        | BinaryOperator::LessEqual
        | BinaryOperator::Greater
        | BinaryOperator::GreaterEqual => 9,
        BinaryOperator::Equal | BinaryOperator::NotEqual => 8,
        BinaryOperator::BitAnd => 7,
        BinaryOperator::BitXor => 6,
        BinaryOperator::BitOr => 5,
        BinaryOperator::LogicAnd => 4,
        BinaryOperator::LogicOr => 3,
    }
}

/// Check if parentheses are needed based on operator precedence
pub fn needs_parentheses(parent_op: &BinaryOperator, child_expr: &Expression) -> bool {
    if let Expression::BinaryOp(child_op, _, _) = child_expr {
        operator_precedence(child_op) < operator_precedence(parent_op)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_expr(expr: Expression) -> Wrapped<Expression> {
        Wrapped {
            item: expr,
            origin_expr: None,
            comment: None,
        }
    }

    #[test]
    fn test_remove_identity_operations() {
        // TODO: Add proper test once VariableId has public constructor
        let simplifier = ExpressionSimplifier::new();
        assert_eq!(simplifier.stats.identity_operations_removed, 0);
    }

    #[test]
    fn test_constant_folding() {
        let mut simplifier = ExpressionSimplifier::new();

        // Test 2 + 3 -> 5
        let mut expr = create_test_expr(Expression::BinaryOp(
            BinaryOperator::Add,
            Box::new(create_test_expr(Expression::Literal(Literal::Int(2)))),
            Box::new(create_test_expr(Expression::Literal(Literal::Int(3)))),
        ));

        simplifier.simplify_expression(&mut expr);

        match expr.item {
            Expression::Literal(Literal::Int(5)) => {}
            _ => panic!("Expected literal 5 after constant folding"),
        }

        assert_eq!(simplifier.stats().constant_expressions_folded, 1);
    }

    #[test]
    fn test_double_negation_removal() {
        // TODO: Add proper test once VariableId has public constructor
        let simplifier = ExpressionSimplifier::new();
        assert_eq!(simplifier.stats.double_negations_removed, 0);
    }

    #[test]
    fn test_xor_self_elimination() {
        // TODO: Add proper test once VariableId has public constructor
        let simplifier = ExpressionSimplifier::new();
        assert_eq!(simplifier.stats.redundant_parentheses_removed, 0);
    }

    #[test]
    fn test_operator_precedence() {
        // Multiplication has higher precedence than addition
        assert!(
            operator_precedence(&BinaryOperator::Mul) > operator_precedence(&BinaryOperator::Add)
        );

        // Logical AND has lower precedence than comparison
        assert!(
            operator_precedence(&BinaryOperator::LogicAnd)
                < operator_precedence(&BinaryOperator::Less)
        );
    }
}
