//! High-level IR (Source-like Representation)
//!
//! This module generates human-readable source-like code from Medium IR patterns,
//! performing variable naming, type inference, and high-level construct generation.

// pub mod to_c; // Disabled due to private field access issues with C AST
pub mod c_codegen;

use crate::ir::low_ir;
use crate::ir::medium_ir::{self, Pattern, PatternRef};
use std::collections::BTreeMap;

/// High-level IR module representing decompiled source
#[derive(Debug, Clone)]
pub struct Module {
    /// Target architecture info
    pub target: medium_ir::Module,

    /// Generated source files
    pub source_files: BTreeMap<String, SourceFile>,

    /// Global type definitions
    pub types: TypeDefinitions,

    /// Global variable declarations
    pub globals: Vec<GlobalVariable>,
}

/// A single source file
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// File name (e.g., "main.c")
    pub name: String,

    /// Include directives
    pub includes: Vec<String>,

    /// Type definitions in this file
    pub typedefs: Vec<TypeDef>,

    /// Function definitions
    pub functions: Vec<Function>,
}

/// High-level function representation
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name (human-readable)
    pub name: String,

    /// Return type
    pub return_type: Type,

    /// Parameters with names
    pub parameters: Vec<Parameter>,

    /// Function body as statements
    pub body: Block,

    /// Local variable declarations
    pub locals: Vec<LocalVariable>,

    /// Documentation comment
    pub doc_comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct LocalVariable {
    pub name: String,
    pub ty: Type,
    pub initial_value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct GlobalVariable {
    pub name: String,
    pub ty: Type,
    pub initial_value: Option<Expression>,
    pub is_static: bool,
}

/// High-level statement types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// Variable declaration: type name [= expr];
    Declaration {
        var: String,
        ty: Type,
        init: Option<Expression>,
    },

    /// Assignment: lvalue = expr;
    Assignment { lvalue: LValue, rvalue: Expression },

    /// Expression statement: expr;
    Expression(Expression),

    /// Return statement: return [expr];
    Return(Option<Expression>),

    /// If statement: if (cond) then_stmt [else else_stmt]
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },

    /// While loop: while (cond) body
    While {
        condition: Expression,
        body: Box<Statement>,
    },

    /// Do-while loop: do body while (cond);
    DoWhile {
        body: Box<Statement>,
        condition: Expression,
    },

    /// For loop: for (init; cond; update) body
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Box<Statement>,
    },

    /// Switch statement: switch (expr) { cases }
    Switch {
        value: Expression,
        cases: Vec<SwitchCase>,
    },

    /// Block statement: { statements }
    Block(Block),

    /// Break statement
    Break,

    /// Continue statement
    Continue,

    /// Goto statement (as last resort)
    Goto(String),

    /// Label for goto
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchCase {
    pub values: Vec<i64>, // Multiple values for case 1: case 2:
    pub is_default: bool,
    pub body: Vec<Statement>,
}

/// High-level expression types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    /// Variable reference
    Variable(String),

    /// Literal constant
    Literal(Literal),

    /// Binary operation: left op right
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Unary operation: op operand
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },

    /// Function call: func(args)
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },

    /// Array access: array[index]
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },

    /// Field access: struct.field or struct->field
    FieldAccess {
        object: Box<Expression>,
        field: String,
        is_pointer: bool,
    },

    /// Type cast: (type)expr
    Cast { ty: Type, expr: Box<Expression> },

    /// sizeof(type)
    SizeOf(Type),

    /// Ternary operator: cond ? then_expr : else_expr
    Ternary {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },

    /// Address-of: &expr
    AddressOf(Box<Expression>),

    /// Dereference: *expr
    Dereference(Box<Expression>),
}

/// L-value expressions (can appear on left side of assignment)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LValue {
    Variable(String),
    ArrayAccess {
        array: Box<LValue>,
        index: Expression,
    },
    FieldAccess {
        object: Box<LValue>,
        field: String,
        is_pointer: bool,
    },
    Dereference(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(i64),
    Float(String), // Store as string to preserve exact representation
    String(String),
    Char(char),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Logical
    And,
    Or,

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    // Arithmetic
    Neg, // -x

    // Logical
    Not, // !x

    // Bitwise
    BitNot, // ~x

    // Increment/Decrement
    PreInc,
    PreDec, // ++x, --x
    PostInc,
    PostDec, // x++, x--
}

/// Type representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    /// Primitive types
    Void,
    Bool,
    Char,
    Short,
    Int,
    Long,
    LongLong,
    Float,
    Double,

    /// Unsigned variants
    UChar,
    UShort,
    UInt,
    ULong,
    ULongLong,

    /// Pointer type
    Pointer(Box<Type>),

    /// Array type
    Array {
        element: Box<Type>,
        size: Option<usize>,
    },

    /// Function pointer type
    FunctionPointer {
        return_type: Box<Type>,
        parameters: Vec<Type>,
        variadic: bool,
    },

    /// Struct reference
    Struct(String),

    /// Union reference
    Union(String),

    /// Enum reference
    Enum(String),

    /// Typedef reference
    TypeDef(String),
}

/// Type definitions
#[derive(Debug, Clone)]
pub struct TypeDefinitions {
    pub structs: BTreeMap<String, StructDef>,
    pub unions: BTreeMap<String, UnionDef>,
    pub enums: BTreeMap<String, EnumDef>,
    pub typedefs: BTreeMap<String, TypeDef>,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
    pub is_packed: bool,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct UnionDef {
    pub name: String,
    pub fields: Vec<UnionField>,
}

#[derive(Debug, Clone)]
pub struct UnionField {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub values: Vec<EnumValue>,
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub ty: Type,
}

/// High IR generator
pub struct HighIRGenerator {
    /// Variable name generator
    name_gen: NameGenerator,

    /// Type inference engine
    type_inference: TypeInference,

    /// Pattern simplifier
    simplifier: PatternSimplifier,

    /// Track declared variables to avoid duplicate declarations
    declared_vars: std::collections::BTreeSet<String>,
}

impl HighIRGenerator {
    pub fn new() -> Self {
        Self {
            name_gen: NameGenerator::new(),
            type_inference: TypeInference::new(),
            simplifier: PatternSimplifier::new(),
            declared_vars: std::collections::BTreeSet::new(),
        }
    }

    /// Generate High IR from Medium IR
    pub fn generate(&mut self, medium_module: &medium_ir::Module) -> Module {
        let mut source_files = BTreeMap::new();
        let types = TypeDefinitions {
            structs: BTreeMap::new(),
            unions: BTreeMap::new(),
            enums: BTreeMap::new(),
            typedefs: BTreeMap::new(),
        };
        let globals = Vec::new();

        // Create main source file
        let mut main_file = SourceFile {
            name: "decompiled.c".to_string(),
            includes: vec![
                "stdio.h".to_string(),
                "stdlib.h".to_string(),
                "string.h".to_string(),
            ],
            typedefs: vec![],
            functions: vec![],
        };

        // Convert each function
        for medium_func in medium_module.functions.values() {
            let high_func = self.convert_function(medium_func);
            main_file.functions.push(high_func);
        }

        source_files.insert("decompiled.c".to_string(), main_file);

        Module {
            target: medium_module.clone(),
            source_files,
            types,
            globals,
        }
    }

    /// Convert a Medium IR function to High IR
    fn convert_function(&mut self, func: &medium_ir::Function) -> Function {
        // Clear declared variables for new function
        self.declared_vars.clear();

        // Generate function name
        let name = self.name_gen.generate_function_name(&func.id);

        // Infer return type
        let return_type = self.type_inference.infer_return_type(&func.signature);

        // Generate parameters
        let parameters = self.generate_parameters(&func.signature);

        // Mark parameters as already declared
        for param in &parameters {
            self.declared_vars.insert(param.name.clone());
        }

        // Convert function body
        let body_pattern = func.patterns.get(func.body).unwrap();
        let body = self.convert_pattern_to_block(body_pattern, &func.patterns);

        // Collect local variables
        let locals = self.collect_locals(&body);

        Function {
            name,
            return_type,
            parameters,
            body,
            locals,
            doc_comment: Some(format!("Function at 0x{:x}", func.id.0)),
        }
    }

    /// Generate parameter list
    fn generate_parameters(&mut self, sig: &medium_ir::FunctionSignature) -> Vec<Parameter> {
        sig.parameters
            .iter()
            .enumerate()
            .map(|(i, (hint, ty))| {
                // Generate better parameter names
                let name = match (i, hint.as_str()) {
                    (_, h) if !h.is_empty() && h != "param" => h.to_string(),
                    (0, _) => "a".to_string(),
                    (1, _) => "b".to_string(),
                    (2, _) => "c".to_string(),
                    (3, _) => "d".to_string(),
                    (4, _) => "e".to_string(),
                    _ => format!("arg{}", i),
                };
                Parameter {
                    name,
                    ty: self.type_inference.convert_type(ty),
                }
            })
            .collect()
    }

    /// Convert pattern to block of statements
    fn convert_pattern_to_block(
        &mut self,
        pattern: &Pattern,
        store: &medium_ir::PatternStore,
    ) -> Block {
        let statements = self.convert_pattern_to_statements(pattern, store);
        Block { statements }
    }

    /// Convert pattern to statements
    fn convert_pattern_to_statements(
        &mut self,
        pattern: &Pattern,
        store: &medium_ir::PatternStore,
    ) -> Vec<Statement> {
        match pattern {
            Pattern::ForLoop {
                init,
                condition,
                increment,
                body,
                ..
            } => {
                let init_stmt = init.map(|ref_| {
                    let init_pattern = store.get(ref_).unwrap();
                    let stmts = self.convert_pattern_to_statements(init_pattern, store);
                    Box::new(Statement::Block(Block { statements: stmts }))
                });

                let cond_expr = if let Some(cond_pattern) = store.get(*condition) {
                    self.pattern_to_expression(cond_pattern, store)
                } else {
                    Expression::Literal(Literal::Bool(true))
                };

                let update_expr = increment.and_then(|ref_| {
                    store
                        .get(ref_)
                        .map(|inc_pattern| self.pattern_to_expression(inc_pattern, store))
                });

                let body_stmt = if let Some(body_pattern) = store.get(*body) {
                    let body_stmts = self.convert_pattern_to_statements(body_pattern, store);
                    Box::new(Statement::Block(Block {
                        statements: body_stmts,
                    }))
                } else {
                    Box::new(Statement::Block(Block { statements: vec![] }))
                };

                vec![Statement::For {
                    init: init_stmt,
                    condition: Some(cond_expr),
                    update: update_expr,
                    body: body_stmt,
                }]
            }

            Pattern::WhileLoop {
                condition, body, ..
            } => {
                let cond_expr = if let Some(cond_pattern) = store.get(*condition) {
                    self.pattern_to_expression(cond_pattern, store)
                } else {
                    Expression::Literal(Literal::Bool(true))
                };

                let body_stmt = if let Some(body_pattern) = store.get(*body) {
                    let body_stmts = self.convert_pattern_to_statements(body_pattern, store);
                    Box::new(Statement::Block(Block {
                        statements: body_stmts,
                    }))
                } else {
                    Box::new(Statement::Block(Block { statements: vec![] }))
                };

                vec![Statement::While {
                    condition: cond_expr,
                    body: body_stmt,
                }]
            }

            Pattern::IfElse {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                let cond_expr = if let Some(cond_pattern) = store.get(*condition) {
                    self.pattern_to_expression(cond_pattern, store)
                } else {
                    Expression::Literal(Literal::Bool(true))
                };

                let then_stmt = if let Some(then_pattern) = store.get(*then_branch) {
                    let then_stmts = self.convert_pattern_to_statements(then_pattern, store);
                    Box::new(Statement::Block(Block {
                        statements: then_stmts,
                    }))
                } else {
                    Box::new(Statement::Block(Block { statements: vec![] }))
                };

                let else_stmt = else_branch.and_then(|ref_| {
                    store.get(ref_).map(|else_pattern| {
                        let else_stmts = self.convert_pattern_to_statements(else_pattern, store);
                        Box::new(Statement::Block(Block {
                            statements: else_stmts,
                        }))
                    })
                });

                vec![Statement::If {
                    condition: cond_expr,
                    then_branch: then_stmt,
                    else_branch: else_stmt,
                }]
            }

            Pattern::SwitchCase {
                value,
                cases,
                default,
                ..
            } => {
                let value_expr = if let Some(val_pattern) = store.get(*value) {
                    self.pattern_to_expression(val_pattern, store)
                } else {
                    Expression::Variable("unknown".to_string())
                };

                let mut switch_cases = vec![];

                // Add regular cases
                for (case_val, case_ref) in cases {
                    if let Some(case_pattern) = store.get(*case_ref) {
                        let case_stmts = self.convert_pattern_to_statements(case_pattern, store);
                        switch_cases.push(SwitchCase {
                            values: vec![*case_val],
                            is_default: false,
                            body: case_stmts,
                        });
                    }
                }

                // Add default case
                if let Some(default_ref) = default {
                    if let Some(default_pattern) = store.get(*default_ref) {
                        let default_stmts =
                            self.convert_pattern_to_statements(default_pattern, store);
                        switch_cases.push(SwitchCase {
                            values: vec![],
                            is_default: true,
                            body: default_stmts,
                        });
                    }
                }

                vec![Statement::Switch {
                    value: value_expr,
                    cases: switch_cases,
                }]
            }

            Pattern::FunctionCall {
                target,
                arguments,
                return_value,
                ..
            } => {
                let func_expr = match target {
                    medium_ir::FunctionRef::Library { name, .. } => {
                        Expression::Variable(name.clone())
                    }
                    medium_ir::FunctionRef::Address(addr) => {
                        Expression::Variable(format!("func_{:x}", addr.get_virtual_address()))
                    }
                    medium_ir::FunctionRef::Indirect(ref_) => {
                        if let Some(indirect_pattern) = store.get(*ref_) {
                            self.pattern_to_expression(indirect_pattern, store)
                        } else {
                            Expression::Variable("unknown_func".to_string())
                        }
                    }
                };

                let arg_exprs: Vec<_> = arguments
                    .iter()
                    .filter_map(|arg_ref| store.get(*arg_ref))
                    .map(|arg_pattern| self.pattern_to_expression(arg_pattern, store))
                    .collect();

                let call_expr = Expression::Call {
                    function: Box::new(func_expr),
                    arguments: arg_exprs,
                };

                // If there's a return value, generate assignment
                if return_value.is_some() {
                    vec![Statement::Assignment {
                        lvalue: LValue::Variable(self.name_gen.generate_temp_name()),
                        rvalue: call_expr,
                    }]
                } else {
                    vec![Statement::Expression(call_expr)]
                }
            }

            Pattern::Expression {
                operation,
                operands,
                ..
            } => {
                // Handle expression patterns
                let expr = self.convert_expression_pattern(operation, operands, store);
                vec![Statement::Expression(expr)]
            }

            Pattern::ArrayAccess {
                base,
                index,
                element_type,
                is_write,
                ..
            } => {
                // Generate array access expression
                let base_expr = if let Some(base_pattern) = store.get(*base) {
                    self.pattern_to_expression(base_pattern, store)
                } else {
                    Expression::Variable("array".to_string())
                };

                let index_expr = if let Some(index_pattern) = store.get(*index) {
                    self.pattern_to_expression(index_pattern, store)
                } else {
                    Expression::Literal(Literal::Integer(0))
                };

                let array_expr = Expression::ArrayAccess {
                    array: Box::new(base_expr.clone()),
                    index: Box::new(index_expr.clone()),
                };

                if *is_write {
                    // For writes, we need to generate an assignment
                    // The value being written should be in the surrounding context
                    vec![Statement::Assignment {
                        lvalue: LValue::ArrayAccess {
                            array: Box::new(LValue::Variable("array".to_string())), // TODO: Extract actual array lvalue
                            index: index_expr,
                        },
                        rvalue: Expression::Variable("value".to_string()), // TODO: Extract actual value
                    }]
                } else {
                    // For reads, generate a temporary assignment
                    let temp_name = self.name_gen.generate_temp_name();
                    vec![Statement::Declaration {
                        var: temp_name,
                        ty: self.type_inference.convert_type(element_type),
                        init: Some(array_expr),
                    }]
                }
            }

            Pattern::LowIR {
                instructions,
                terminator,
                ..
            } => {
                // Fallback: convert low IR instructions directly
                let mut stmts = self.convert_low_ir_instructions(instructions);

                // Handle terminator if present
                if let Some(term) = terminator {
                    if let Some(term_stmt) = self.convert_terminator(term) {
                        stmts.push(term_stmt);
                    }
                }

                stmts
            }

            _ => {
                // TODO: Handle other patterns
                vec![]
            }
        }
    }

    /// Convert pattern to expression
    fn pattern_to_expression(
        &mut self,
        pattern: &Pattern,
        store: &medium_ir::PatternStore,
    ) -> Expression {
        match pattern {
            Pattern::Expression {
                operation,
                operands,
                ..
            } => self.convert_expression_pattern(operation, operands, store),
            Pattern::LowIR { instructions, .. } => {
                // Extract expression from low IR instructions
                // Look for the last instruction that produces a value
                for inst in instructions.iter().rev() {
                    match inst {
                        low_ir::Instruction::BinOp { op, lhs, rhs, .. } => {
                            let left = self.convert_low_ir_value(lhs);
                            let right = self.convert_low_ir_value(rhs);
                            let binary_op = self.convert_low_ir_binop(op);

                            return Expression::Binary {
                                op: binary_op,
                                left: Box::new(left),
                                right: Box::new(right),
                            };
                        }
                        low_ir::Instruction::Assign { value, .. } => {
                            return self.convert_low_ir_value(value);
                        }
                        _ => {}
                    }
                }

                // Fallback if no suitable instruction found
                Expression::Variable("temp".to_string())
            }
            _ => Expression::Variable("unknown".to_string()),
        }
    }

    /// Convert expression pattern
    fn convert_expression_pattern(
        &mut self,
        operation: &medium_ir::ExpressionOp,
        operands: &[PatternRef],
        store: &medium_ir::PatternStore,
    ) -> Expression {
        use medium_ir::ExpressionOp::*;

        match (operation, operands.len()) {
            (Add, 2) => self.make_binary_expr(BinaryOp::Add, &operands[0], &operands[1], store),
            (Sub, 2) => self.make_binary_expr(BinaryOp::Sub, &operands[0], &operands[1], store),
            (Mul, 2) => self.make_binary_expr(BinaryOp::Mul, &operands[0], &operands[1], store),
            (Div, 2) => self.make_binary_expr(BinaryOp::Div, &operands[0], &operands[1], store),
            (Mod, 2) => self.make_binary_expr(BinaryOp::Mod, &operands[0], &operands[1], store),
            (And, 2) => self.make_binary_expr(BinaryOp::And, &operands[0], &operands[1], store),
            (Or, 2) => self.make_binary_expr(BinaryOp::Or, &operands[0], &operands[1], store),
            (Xor, 2) => self.make_binary_expr(BinaryOp::BitXor, &operands[0], &operands[1], store),
            (Shl, 2) => self.make_binary_expr(BinaryOp::Shl, &operands[0], &operands[1], store),
            (Shr, 2) => self.make_binary_expr(BinaryOp::Shr, &operands[0], &operands[1], store),
            (Eq, 2) => self.make_binary_expr(BinaryOp::Eq, &operands[0], &operands[1], store),
            (Ne, 2) => self.make_binary_expr(BinaryOp::Ne, &operands[0], &operands[1], store),
            (Lt, 2) => self.make_binary_expr(BinaryOp::Lt, &operands[0], &operands[1], store),
            (Le, 2) => self.make_binary_expr(BinaryOp::Le, &operands[0], &operands[1], store),
            (Gt, 2) => self.make_binary_expr(BinaryOp::Gt, &operands[0], &operands[1], store),
            (Ge, 2) => self.make_binary_expr(BinaryOp::Ge, &operands[0], &operands[1], store),
            (Not, 1) => self.make_unary_expr(UnaryOp::Not, &operands[0], store),
            // TODO: Handle negation - might be represented as 0 - x
            _ => Expression::Variable("unknown_expr".to_string()),
        }
    }

    /// Create binary expression
    fn make_binary_expr(
        &mut self,
        op: BinaryOp,
        left_ref: &PatternRef,
        right_ref: &PatternRef,
        store: &medium_ir::PatternStore,
    ) -> Expression {
        let left = store
            .get(*left_ref)
            .map(|p| self.pattern_to_expression(p, store))
            .unwrap_or_else(|| Expression::Variable("unknown".to_string()));
        let right = store
            .get(*right_ref)
            .map(|p| self.pattern_to_expression(p, store))
            .unwrap_or_else(|| Expression::Variable("unknown".to_string()));

        Expression::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create unary expression
    fn make_unary_expr(
        &mut self,
        op: UnaryOp,
        operand_ref: &PatternRef,
        store: &medium_ir::PatternStore,
    ) -> Expression {
        let operand = store
            .get(*operand_ref)
            .map(|p| self.pattern_to_expression(p, store))
            .unwrap_or_else(|| Expression::Variable("unknown".to_string()));

        Expression::Unary {
            op,
            operand: Box::new(operand),
        }
    }

    /// Convert low IR instructions to statements
    fn convert_low_ir_instructions(
        &mut self,
        instructions: &[low_ir::Instruction],
    ) -> Vec<Statement> {
        let mut statements = Vec::new();

        for inst in instructions {
            match inst {
                low_ir::Instruction::BinOp {
                    op,
                    dst,
                    lhs,
                    rhs,
                    ty,
                    ..
                } => {
                    // Convert binary operation to assignment
                    let left = self.convert_low_ir_value(lhs);
                    let right = self.convert_low_ir_value(rhs);
                    let binary_op = self.convert_low_ir_binop(op);

                    let expr = Expression::Binary {
                        op: binary_op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };

                    let var_name = self.convert_local_id_to_name(dst);

                    // Check if this is the first use of the variable
                    if !self.declared_vars.contains(&var_name) {
                        self.declared_vars.insert(var_name.clone());
                        // Generate a declaration
                        statements.push(Statement::Declaration {
                            var: var_name,
                            ty: self.convert_low_ir_type(ty),
                            init: Some(expr),
                        });
                    } else {
                        // Generate an assignment
                        let lvalue = LValue::Variable(var_name);
                        statements.push(Statement::Assignment {
                            lvalue,
                            rvalue: expr,
                        });
                    }
                }
                low_ir::Instruction::Assign { dst, value, .. } => {
                    let expr = self.convert_low_ir_value(value);
                    let var_name = self.convert_local_id_to_name(dst);

                    // For Assign, we need to infer the type from the value
                    let ty = self.infer_type_from_value(value);

                    // Check if this is the first use of the variable
                    if !self.declared_vars.contains(&var_name) {
                        self.declared_vars.insert(var_name.clone());
                        // Generate a declaration
                        statements.push(Statement::Declaration {
                            var: var_name,
                            ty,
                            init: Some(expr),
                        });
                    } else {
                        // Generate an assignment
                        let lvalue = LValue::Variable(var_name);
                        statements.push(Statement::Assignment {
                            lvalue,
                            rvalue: expr,
                        });
                    }
                }
                // TODO: Handle other instruction types
                _ => {}
            }
        }

        statements
    }

    /// Convert Low IR terminator to High IR statement
    fn convert_terminator(&mut self, terminator: &low_ir::Terminator) -> Option<Statement> {
        match terminator {
            low_ir::Terminator::Return(None) => Some(Statement::Return(None)),
            low_ir::Terminator::Return(Some((value, _ty))) => {
                let expr = self.convert_low_ir_value(value);
                Some(Statement::Return(Some(expr)))
            }
            low_ir::Terminator::Branch(target) => {
                Some(Statement::Goto(format!("L_{:x}", target.0)))
            }
            low_ir::Terminator::CondBranch {
                cond,
                true_dest,
                false_dest,
            } => {
                // For now, generate if-goto pattern
                // TODO: Optimize to structured control flow
                let cond_expr = self.convert_low_ir_value(cond);
                Some(Statement::If {
                    condition: cond_expr,
                    then_branch: Box::new(Statement::Goto(format!("L_{:x}", true_dest.0))),
                    else_branch: Some(Box::new(Statement::Goto(format!("L_{:x}", false_dest.0)))),
                })
            }
            _ => None, // Handle other terminators later
        }
    }

    /// Convert Low IR value to High IR expression
    fn convert_low_ir_value(&mut self, value: &low_ir::Value) -> Expression {
        match value {
            low_ir::Value::Local(local_id) => {
                Expression::Variable(self.convert_local_id_to_name(local_id))
            }
            low_ir::Value::Constant(constant) => {
                match constant {
                    low_ir::Constant::Int { value, .. } => {
                        Expression::Literal(Literal::Integer(*value as i64))
                    }
                    low_ir::Constant::Float { bits, .. } => {
                        // Convert bits to float string representation
                        Expression::Literal(Literal::Float(format!("0x{:x}", bits)))
                    }
                    _ => Expression::Variable("unknown_const".to_string()),
                }
            }
            _ => Expression::Variable("unknown_value".to_string()),
        }
    }

    /// Convert LocalId to variable name
    fn convert_local_id_to_name(&mut self, local_id: &low_ir::LocalId) -> String {
        // Use the purpose field directly for cleaner names
        if !local_id.purpose.is_empty() && local_id.purpose != "temp" {
            // For version 0, use the purpose directly (no suffix)
            if local_id.version == 0 {
                return local_id.purpose.to_string();
            } else {
                // For SSA versions, use underscore notation
                return format!("{}_{}", local_id.purpose, local_id.version);
            }
        }

        // Generate semantic names based on common patterns
        // For now, fallback to temp name
        self.name_gen.generate_temp_name()
    }

    /// Convert Low IR binary operation to High IR
    fn convert_low_ir_binop(&self, op: &low_ir::BinaryOp) -> BinaryOp {
        use low_ir::BinaryOp as LowOp;
        match op {
            LowOp::Add => BinaryOp::Add,
            LowOp::Sub => BinaryOp::Sub,
            LowOp::Mul => BinaryOp::Mul,
            LowOp::UDiv | LowOp::SDiv => BinaryOp::Div,
            LowOp::URem | LowOp::SRem => BinaryOp::Mod,
            LowOp::And => BinaryOp::BitAnd,
            LowOp::Or => BinaryOp::BitOr,
            LowOp::Xor => BinaryOp::BitXor,
            LowOp::Shl => BinaryOp::Shl,
            LowOp::LShr | LowOp::AShr => BinaryOp::Shr,
            LowOp::Eq => BinaryOp::Eq,
            LowOp::Ne => BinaryOp::Ne,
            LowOp::Ult | LowOp::Slt => BinaryOp::Lt,
            LowOp::Ule | LowOp::Sle => BinaryOp::Le,
            LowOp::Ugt | LowOp::Sgt => BinaryOp::Gt,
            LowOp::Uge | LowOp::Sge => BinaryOp::Ge,
        }
    }

    /// Convert Low IR type to High IR type
    #[allow(clippy::only_used_in_recursion)]
    fn convert_low_ir_type(&self, ty: &low_ir::Type) -> Type {
        match ty {
            low_ir::Type::Void => Type::Void,
            low_ir::Type::Bool => Type::Bool,
            low_ir::Type::I8 => Type::Char,
            low_ir::Type::I16 => Type::Short,
            low_ir::Type::I32 => Type::Int,
            low_ir::Type::I64 => Type::LongLong,
            low_ir::Type::F32 => Type::Float,
            low_ir::Type::F64 => Type::Double,
            low_ir::Type::Pointer(inner) => Type::Pointer(Box::new(
                inner
                    .as_ref()
                    .map(|t| self.convert_low_ir_type(t))
                    .unwrap_or(Type::Void),
            )),
            low_ir::Type::Array(elem, size) => Type::Array {
                element: Box::new(self.convert_low_ir_type(elem)),
                size: Some(*size),
            },
            _ => Type::Int, // Default for unknown types
        }
    }

    /// Infer type from Low IR value
    fn infer_type_from_value(&self, value: &low_ir::Value) -> Type {
        match value {
            low_ir::Value::Constant(constant) => match constant {
                low_ir::Constant::Int { ty, .. } => self.convert_low_ir_type(ty),
                low_ir::Constant::Float { ty, .. } => self.convert_low_ir_type(ty),
                _ => Type::Int,
            },
            _ => Type::Int, // Default for other cases
        }
    }

    /// Collect local variables from function body
    fn collect_locals(&self, _body: &Block) -> Vec<LocalVariable> {
        // TODO: Implement local variable collection
        vec![]
    }
}

/// Variable name generator
struct NameGenerator {
    temp_counter: usize,
    func_counter: usize,
}

impl NameGenerator {
    fn new() -> Self {
        Self {
            temp_counter: 0,
            func_counter: 0,
        }
    }

    fn generate_function_name(&mut self, id: &low_ir::FunctionId) -> String {
        // TODO: Use symbol information if available
        format!("sub_{:x}", id.0)
    }

    fn generate_temp_name(&mut self) -> String {
        let name = format!("var_{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }
}

/// Type inference engine
struct TypeInference;

impl TypeInference {
    fn new() -> Self {
        Self
    }

    fn infer_return_type(&self, sig: &medium_ir::FunctionSignature) -> Type {
        self.convert_type(&sig.return_type)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn convert_type(&self, ty: &medium_ir::TypeRef) -> Type {
        use medium_ir::{PrimitiveType, TypeRef};

        match ty {
            TypeRef::Primitive(prim) => match prim {
                PrimitiveType::Void => Type::Void,
                PrimitiveType::Bool => Type::Bool,
                PrimitiveType::I8 => Type::Char,
                PrimitiveType::U8 => Type::UChar,
                PrimitiveType::I16 => Type::Short,
                PrimitiveType::U16 => Type::UShort,
                PrimitiveType::I32 => Type::Int,
                PrimitiveType::U32 => Type::UInt,
                PrimitiveType::I64 => Type::LongLong,
                PrimitiveType::U64 => Type::ULongLong,
                PrimitiveType::F32 => Type::Float,
                PrimitiveType::F64 => Type::Double,
            },
            TypeRef::Pointer(pointee) => Type::Pointer(Box::new(self.convert_type(pointee))),
            TypeRef::Array { element, size } => Type::Array {
                element: Box::new(self.convert_type(element)),
                size: *size,
            },
            TypeRef::Struct { name, .. } => {
                Type::Struct(name.clone().unwrap_or_else(|| "unknown_struct".to_string()))
            }
            TypeRef::Unknown => Type::Int, // Default to int for unknown types
        }
    }
}

/// Pattern simplifier
struct PatternSimplifier;

impl PatternSimplifier {
    fn new() -> Self {
        Self
    }
}

impl Module {
    /// Create High IR from Medium IR
    pub fn from_medium_ir(medium: &medium_ir::Module) -> Self {
        let mut generator = HighIRGenerator::new();
        generator.generate(medium)
    }
}
