//! Convert High IR to C code using the existing C AST

use crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    self as cast, ArcFunctionMap, BinaryOperator, CAst, CType, Expression, Function, FunctionId,
    Literal, Statement, UnaryOperator, Variable, VariableId, Wrapped, WrappedStatement,
};
use crate::ir::high_ir as hir;
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

/// Convert High IR module to C code
pub struct HighIRToCConverter {
    /// Map from High IR variable names to C AST variable IDs
    var_map: HashMap<String, VariableId>,
    /// Next variable ID to assign
    next_var_id: u32,
}

impl HighIRToCConverter {
    pub fn new() -> Self {
        Self {
            var_map: HashMap::new(),
            next_var_id: 0,
        }
    }

    /// Convert High IR module to C AST
    pub fn convert(&mut self, module: &hir::Module) -> CAst {
        let mut c_ast = CAst::new();

        // Convert each source file
        for (_name, source_file) in &module.source_files {
            // Convert functions
            for func in &source_file.functions {
                self.convert_function(&mut c_ast, func);
            }
        }

        c_ast
    }

    /// Convert a High IR function to C AST
    fn convert_function(&mut self, c_ast: &mut CAst, func: &hir::Function) {
        // Create function ID from name (using a dummy address for now)
        let dummy_address = crate::core::Address::from_virtual_address(
            &std::sync::Arc::new(crate::core::Sections::default()),
            self.hash_name(&func.name),
        );
        let func_id = c_ast.generate_default_function(&dummy_address);

        // Convert parameters
        let mut parameters = Vec::new();
        for param in &func.parameters {
            let var_id = self.get_or_create_variable_id(&param.name, Some(func_id));
            parameters.push(Variable {
                name: param.name.clone(),
                id: var_id,
                var_type: self.convert_type(&param.ty),
                const_value: None,
            });
        }

        // Convert local variables
        let mut locals = HashMap::new();
        for local in &func.locals {
            let var_id = self.get_or_create_variable_id(&local.name, Some(func_id));
            locals.insert(
                var_id,
                Variable {
                    name: local.name.clone(),
                    id: var_id,
                    var_type: self.convert_type(&local.ty),
                    const_value: None,
                },
            );
        }

        // Convert function body
        let body = self.convert_block(&func.body, func_id, &c_ast.functions);

        // Create the C function
        let c_func = Function {
            name: func.name.clone(),
            id: func_id,
            return_type: self.convert_type(&func.return_type),
            parameters,
            variables: Arc::new(RwLock::new(locals)),
            body,
        };

        // Add to C AST
        c_ast.functions.write().unwrap().insert(func_id, c_func);
    }

    /// Convert a block of statements
    fn convert_block(
        &mut self,
        block: &hir::Block,
        func_id: FunctionId,
        functions: &ArcFunctionMap,
    ) -> Vec<WrappedStatement> {
        let mut statements = Vec::new();

        for stmt in &block.statements {
            if let Some(c_stmt) = self.convert_statement(stmt, func_id, functions) {
                statements.push(WrappedStatement {
                    statement: c_stmt,
                    from: None,
                    comment: None,
                });
            }
        }

        statements
    }

    /// Convert a High IR statement to C AST statement
    fn convert_statement(
        &mut self,
        stmt: &hir::Statement,
        func_id: FunctionId,
        functions: &ArcFunctionMap,
    ) -> Option<Statement> {
        match stmt {
            hir::Statement::Declaration { var, ty, init } => {
                let var_id = self.get_or_create_variable_id(var, Some(func_id));
                let c_var = Variable {
                    name: var.clone(),
                    id: var_id,
                    var_type: self.convert_type(ty),
                    const_value: None,
                };

                let init_expr = init.as_ref().map(|e| Wrapped {
                    item: self.convert_expression(e, func_id, functions),
                    origin_expr: None,
                    comment: None,
                });

                Some(Statement::Declaration(c_var, init_expr))
            }

            hir::Statement::Assignment { lvalue, rvalue } => {
                let lhs = self.convert_lvalue_to_expression(lvalue, func_id, functions);
                let rhs = self.convert_expression(rvalue, func_id, functions);

                Some(Statement::Assignment(
                    Wrapped {
                        item: lhs,
                        origin_expr: None,
                        comment: None,
                    },
                    Wrapped {
                        item: rhs,
                        origin_expr: None,
                        comment: None,
                    },
                ))
            }

            hir::Statement::Expression(expr) => {
                // Convert to a call statement if it's a function call
                match expr {
                    hir::Expression::Call {
                        function,
                        arguments,
                    } => {
                        let target = match &**function {
                            hir::Expression::Variable(name) => {
                                cast::JumpTarget::Unknown(name.clone())
                            }
                            _ => cast::JumpTarget::Unknown("unknown".to_string()),
                        };

                        let args: Vec<_> = arguments
                            .iter()
                            .map(|arg| Wrapped {
                                item: self.convert_expression(arg, func_id, functions),
                                origin_expr: None,
                                comment: None,
                            })
                            .collect();

                        Some(Statement::Call(target, args))
                    }
                    _ => {
                        // Other expressions become comments for now
                        Some(Statement::Comment(format!("Expression: {:?}", expr)))
                    }
                }
            }

            hir::Statement::Return(expr) => {
                let ret_expr = expr.as_ref().map(|e| Wrapped {
                    item: self.convert_expression(e, func_id, functions),
                    origin_expr: None,
                    comment: None,
                });
                Some(Statement::Return(ret_expr))
            }

            hir::Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = Wrapped {
                    item: self.convert_expression(condition, func_id, functions),
                    origin_expr: None,
                    comment: None,
                };

                let then_stmts = self.convert_statement_to_block(then_branch, func_id, functions);
                let else_stmts = else_branch
                    .as_ref()
                    .map(|stmt| self.convert_statement_to_block(stmt, func_id, functions));

                Some(Statement::If(cond, then_stmts, else_stmts))
            }

            hir::Statement::While { condition, body } => {
                let cond = Wrapped {
                    item: self.convert_expression(condition, func_id, functions),
                    origin_expr: None,
                    comment: None,
                };

                let body_stmts = self.convert_statement_to_block(body, func_id, functions);
                Some(Statement::While(cond, body_stmts))
            }

            hir::Statement::For {
                init,
                condition,
                update,
                body,
            } => {
                // Convert init statement
                let init_stmt = init.as_ref().and_then(|s| {
                    self.convert_statement(s, func_id, functions).map(|stmt| {
                        Box::new(WrappedStatement {
                            statement: stmt,
                            from: None,
                            comment: None,
                        })
                    })
                });

                // Convert condition
                let cond = condition
                    .as_ref()
                    .map(|c| self.convert_expression(c, func_id, functions))
                    .unwrap_or(Expression::Literal(Literal::Bool(true)));

                // Convert update as assignment
                let update_stmt = update.as_ref().and_then(|u| {
                    // Convert update expression to a statement
                    match u {
                        hir::Expression::Binary { op, left, right } => {
                            // Assume it's something like i = i + 1
                            if let hir::Expression::Variable(var_name) = &**left {
                                let lvalue = hir::LValue::Variable(var_name.clone());
                                let assign = hir::Statement::Assignment {
                                    lvalue,
                                    rvalue: hir::Expression::Binary {
                                        op: *op,
                                        left: left.clone(),
                                        right: right.clone(),
                                    },
                                };
                                self.convert_statement(&assign, func_id, functions)
                                    .map(|stmt| {
                                        Box::new(WrappedStatement {
                                            statement: stmt,
                                            from: None,
                                            comment: None,
                                        })
                                    })
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                });

                // Convert body
                let body_stmts = self.convert_statement_to_block(body, func_id, functions);

                // Handle missing init/update with empty statements
                let init_stmt = init_stmt.unwrap_or_else(|| {
                    Box::new(WrappedStatement {
                        statement: Statement::Empty,
                        from: None,
                        comment: None,
                    })
                });

                let update_stmt = update_stmt.unwrap_or_else(|| {
                    Box::new(WrappedStatement {
                        statement: Statement::Empty,
                        from: None,
                        comment: None,
                    })
                });

                Some(Statement::For(
                    init_stmt,
                    Wrapped {
                        item: cond,
                        origin_expr: None,
                        comment: None,
                    },
                    update_stmt,
                    body_stmts,
                ))
            }

            hir::Statement::Block(block) => {
                let stmts = self.convert_block(block, func_id, functions);
                Some(Statement::Block(stmts))
            }

            hir::Statement::Switch { value, cases } => {
                // C AST doesn't have direct switch support, convert to if-else chain
                Some(Statement::Comment(format!("Switch on {:?}", value)))
            }

            hir::Statement::Break => Some(Statement::Comment("break".to_string())),
            hir::Statement::Continue => Some(Statement::Comment("continue".to_string())),
            hir::Statement::Goto(label) => {
                Some(Statement::Goto(cast::JumpTarget::Unknown(label.clone())))
            }
            hir::Statement::Label(label) => Some(Statement::Label(label.clone())),
            hir::Statement::DoWhile { .. } => {
                Some(Statement::Comment("do-while not implemented".to_string()))
            }
        }
    }

    /// Convert a statement to a block of wrapped statements
    fn convert_statement_to_block(
        &mut self,
        stmt: &hir::Statement,
        func_id: FunctionId,
        functions: &ArcFunctionMap,
    ) -> Vec<WrappedStatement> {
        match stmt {
            hir::Statement::Block(block) => self.convert_block(block, func_id, functions),
            _ => {
                if let Some(c_stmt) = self.convert_statement(stmt, func_id, functions) {
                    vec![WrappedStatement {
                        statement: c_stmt,
                        from: None,
                        comment: None,
                    }]
                } else {
                    vec![]
                }
            }
        }
    }

    /// Convert High IR expression to C AST expression
    fn convert_expression(
        &mut self,
        expr: &hir::Expression,
        func_id: FunctionId,
        functions: &ArcFunctionMap,
    ) -> Expression {
        match expr {
            hir::Expression::Variable(name) => {
                let var_id = self.get_or_create_variable_id(name, Some(func_id));
                // Get the function's variable map
                if let Some(func) = functions.read().unwrap().get(&func_id) {
                    Expression::Variable(func.variables.clone(), var_id)
                } else {
                    Expression::Unknown
                }
            }

            hir::Expression::Literal(lit) => Expression::Literal(self.convert_literal(lit)),

            hir::Expression::Binary { op, left, right } => {
                let lhs = Box::new(Wrapped {
                    item: self.convert_expression(left, func_id, functions),
                    origin_expr: None,
                    comment: None,
                });
                let rhs = Box::new(Wrapped {
                    item: self.convert_expression(right, func_id, functions),
                    origin_expr: None,
                    comment: None,
                });
                Expression::BinaryOp(self.convert_binary_op(*op), lhs, rhs)
            }

            hir::Expression::Unary { op, operand } => {
                let operand = Box::new(Wrapped {
                    item: self.convert_expression(operand, func_id, functions),
                    origin_expr: None,
                    comment: None,
                });
                Expression::UnaryOp(self.convert_unary_op(*op), operand)
            }

            hir::Expression::Call {
                function,
                arguments,
            } => {
                let func_name = match &**function {
                    hir::Expression::Variable(name) => name.clone(),
                    _ => "unknown".to_string(),
                };

                let args: Vec<_> = arguments
                    .iter()
                    .map(|arg| Wrapped {
                        item: self.convert_expression(arg, func_id, functions),
                        origin_expr: None,
                        comment: None,
                    })
                    .collect();

                Expression::Call(func_name, args)
            }

            hir::Expression::Cast { ty, expr } => {
                let inner = Box::new(Wrapped {
                    item: self.convert_expression(expr, func_id, functions),
                    origin_expr: None,
                    comment: None,
                });
                Expression::Cast(self.convert_type(ty), inner)
            }

            hir::Expression::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                // C AST doesn't support ternary directly, generate a comment
                // TODO: Could convert to if-else statement
                Expression::Unknown
            }

            _ => Expression::Unknown, // TODO: Handle other expression types
        }
    }

    /// Convert L-value to expression
    fn convert_lvalue_to_expression(
        &mut self,
        lvalue: &hir::LValue,
        func_id: FunctionId,
        functions: &ArcFunctionMap,
    ) -> Expression {
        match lvalue {
            hir::LValue::Variable(name) => {
                let var_id = self.get_or_create_variable_id(name, Some(func_id));
                if let Some(func) = functions.read().unwrap().get(&func_id) {
                    Expression::Variable(func.variables.clone(), var_id)
                } else {
                    Expression::Unknown
                }
            }
            _ => Expression::Unknown, // TODO: Handle other l-value types
        }
    }

    /// Convert High IR type to C type
    fn convert_type(&self, ty: &hir::Type) -> CType {
        match ty {
            hir::Type::Void => CType::Void,
            hir::Type::Bool => CType::Bool,
            hir::Type::Char => CType::Char,
            hir::Type::Short => CType::Int16,
            hir::Type::Int => CType::Int32,
            hir::Type::Long => CType::Int64,
            hir::Type::LongLong => CType::Int64,
            hir::Type::Float => CType::Float,
            hir::Type::Double => CType::Double,
            hir::Type::UChar => CType::UInt8,
            hir::Type::UShort => CType::UInt16,
            hir::Type::UInt => CType::UInt32,
            hir::Type::ULong => CType::UInt64,
            hir::Type::ULongLong => CType::UInt64,
            hir::Type::Pointer(inner) => CType::Pointer(Box::new(self.convert_type(inner))),
            hir::Type::Array { element, size } => {
                let elem_type = self.convert_type(element);
                match size {
                    Some(s) => CType::Array(Box::new(elem_type), *s),
                    None => CType::Pointer(Box::new(elem_type)), // Unsized array as pointer
                }
            }
            _ => CType::Unknown, // TODO: Handle other types
        }
    }

    /// Convert High IR literal to C literal
    fn convert_literal(&self, lit: &hir::Literal) -> Literal {
        match lit {
            hir::Literal::Integer(i) => Literal::Int(*i),
            hir::Literal::Float(f) => {
                // Parse float string or use default
                Literal::Float(f.parse().unwrap_or(0.0))
            }
            hir::Literal::String(s) => Literal::String(s.clone()),
            hir::Literal::Char(c) => Literal::Char(*c),
            hir::Literal::Bool(b) => Literal::Bool(*b),
            hir::Literal::Null => Literal::Int(0), // NULL as 0
        }
    }

    /// Convert binary operator
    fn convert_binary_op(&self, op: hir::BinaryOp) -> cast::BinaryOperator {
        use cast::BinaryOperator;
        match op {
            hir::BinaryOp::Add => BinaryOperator::Add,
            hir::BinaryOp::Sub => BinaryOperator::Sub,
            hir::BinaryOp::Mul => BinaryOperator::Mul,
            hir::BinaryOp::Div => BinaryOperator::Div,
            hir::BinaryOp::Mod => BinaryOperator::Mod,
            hir::BinaryOp::Eq => BinaryOperator::Equal,
            hir::BinaryOp::Ne => BinaryOperator::NotEqual,
            hir::BinaryOp::Lt => BinaryOperator::Less,
            hir::BinaryOp::Le => BinaryOperator::LessEqual,
            hir::BinaryOp::Gt => BinaryOperator::Greater,
            hir::BinaryOp::Ge => BinaryOperator::GreaterEqual,
            hir::BinaryOp::And => BinaryOperator::LogicAnd,
            hir::BinaryOp::Or => BinaryOperator::LogicOr,
            hir::BinaryOp::BitAnd => BinaryOperator::BitAnd,
            hir::BinaryOp::BitOr => BinaryOperator::BitOr,
            hir::BinaryOp::BitXor => BinaryOperator::BitXor,
            hir::BinaryOp::Shl => BinaryOperator::LeftShift,
            hir::BinaryOp::Shr => BinaryOperator::RightShift,
        }
    }

    /// Convert unary operator
    fn convert_unary_op(&self, op: hir::UnaryOp) -> cast::UnaryOperator {
        use cast::UnaryOperator;
        match op {
            hir::UnaryOp::Neg => UnaryOperator::Negate,
            hir::UnaryOp::Not => UnaryOperator::Not,
            hir::UnaryOp::BitNot => UnaryOperator::BitNot,
            hir::UnaryOp::PreInc => UnaryOperator::PreInc,
            hir::UnaryOp::PreDec => UnaryOperator::PreDec,
            hir::UnaryOp::PostInc => UnaryOperator::PostInc,
            hir::UnaryOp::PostDec => UnaryOperator::PostDec,
        }
    }

    /// Get or create a variable ID
    fn get_or_create_variable_id(
        &mut self,
        name: &str,
        _func_id: Option<FunctionId>,
    ) -> VariableId {
        if let Some(&var_id) = self.var_map.get(name) {
            var_id
        } else {
            // This is a workaround since we can't directly create VariableId
            // We'll use a simple index-based approach that can be parsed later
            let var_id = unsafe { std::mem::transmute::<u64, VariableId>(self.next_var_id as u64) };
            self.next_var_id += 1;
            self.var_map.insert(name.to_string(), var_id);
            var_id
        }
    }

    /// Simple hash function for function names
    fn hash_name(&self, name: &str) -> u64 {
        let mut hash = 0u64;
        for byte in name.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }
}
