//! Simple C code generator for High IR
//!
//! This module provides a straightforward conversion from High IR to C code strings
//! without the complexity of the full C AST.

use crate::ir::high_ir as hir;

/// Simple C code generator
pub struct CCodeGenerator {
    /// Current indentation level
    indent_level: usize,
    /// Output buffer
    output: String,
}

impl CCodeGenerator {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            output: String::new(),
        }
    }

    /// Generate C code from High IR module
    pub fn generate(&mut self, module: &hir::Module) -> String {
        self.output.clear();

        // Add standard includes
        self.writeln("#include <stdio.h>");
        self.writeln("#include <stdlib.h>");
        self.writeln("#include <stdint.h>");
        self.writeln("#include <stdbool.h>");
        self.writeln("");

        // Generate code for each source file
        for source_file in module.source_files.values() {
            for func in &source_file.functions {
                self.generate_function(func);
                self.writeln("");
            }
        }

        self.output.clone()
    }

    /// Generate a function
    fn generate_function(&mut self, func: &hir::Function) {
        // Generate function signature
        self.write(&format!(
            "{} {}(",
            self.type_to_c(&func.return_type),
            func.name
        ));

        // Parameters
        for (i, param) in func.parameters.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.write(&format!("{} {}", self.type_to_c(&param.ty), param.name));
        }

        self.writeln(") {");
        self.indent_level += 1;

        // Local variables
        for local in &func.locals {
            self.write_indent();
            self.write(&format!("{} {}", self.type_to_c(&local.ty), local.name));
            if let Some(init) = &local.initial_value {
                self.write(" = ");
                self.generate_expression(init);
            }
            self.writeln(";");
        }

        if !func.locals.is_empty() {
            self.writeln("");
        }

        // Function body
        self.generate_block(&func.body);

        self.indent_level -= 1;
        self.writeln("}");
    }

    /// Generate a block of statements
    fn generate_block(&mut self, block: &hir::Block) {
        for stmt in &block.statements {
            self.generate_statement(stmt);
        }
    }

    /// Generate a statement
    fn generate_statement(&mut self, stmt: &hir::Statement) {
        match stmt {
            hir::Statement::Declaration { var, ty, init } => {
                self.write_indent();
                self.write(&format!("{} {}", self.type_to_c(ty), var));
                if let Some(expr) = init {
                    self.write(" = ");
                    self.generate_expression(expr);
                }
                self.writeln(";");
            }

            hir::Statement::Assignment { lvalue, rvalue } => {
                self.write_indent();
                self.generate_lvalue(lvalue);
                self.write(" = ");
                self.generate_expression(rvalue);
                self.writeln(";");
            }

            hir::Statement::Expression(expr) => {
                self.write_indent();
                self.generate_expression(expr);
                self.writeln(";");
            }

            hir::Statement::Return(expr) => {
                self.write_indent();
                self.write("return");
                if let Some(e) = expr {
                    self.write(" ");
                    self.generate_expression(e);
                }
                self.writeln(";");
            }

            hir::Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.write_indent();
                self.write("if (");
                self.generate_expression(condition);
                self.writeln(") {");

                self.indent_level += 1;
                self.generate_statement(then_branch);
                self.indent_level -= 1;

                if let Some(else_stmt) = else_branch {
                    self.write_indent();
                    self.writeln("} else {");
                    self.indent_level += 1;
                    self.generate_statement(else_stmt);
                    self.indent_level -= 1;
                }

                self.write_indent();
                self.writeln("}");
            }

            hir::Statement::While { condition, body } => {
                self.write_indent();
                self.write("while (");
                self.generate_expression(condition);
                self.writeln(") {");

                self.indent_level += 1;
                self.generate_statement(body);
                self.indent_level -= 1;

                self.write_indent();
                self.writeln("}");
            }

            hir::Statement::DoWhile { body, condition } => {
                self.write_indent();
                self.writeln("do {");

                self.indent_level += 1;
                self.generate_statement(body);
                self.indent_level -= 1;

                self.write_indent();
                self.write("} while (");
                self.generate_expression(condition);
                self.writeln(");");
            }

            hir::Statement::For {
                init,
                condition,
                update,
                body,
            } => {
                self.write_indent();
                self.write("for (");

                // Init
                if let Some(init_stmt) = init {
                    // Generate init without newline
                    match &**init_stmt {
                        hir::Statement::Declaration { var, ty, init } => {
                            self.write(&format!("{} {}", self.type_to_c(ty), var));
                            if let Some(expr) = init {
                                self.write(" = ");
                                self.generate_expression(expr);
                            }
                        }
                        hir::Statement::Assignment { lvalue, rvalue } => {
                            self.generate_lvalue(lvalue);
                            self.write(" = ");
                            self.generate_expression(rvalue);
                        }
                        _ => {}
                    }
                }
                self.write("; ");

                // Condition
                if let Some(cond) = condition {
                    self.generate_expression(cond);
                }
                self.write("; ");

                // Update
                if let Some(upd) = update {
                    self.generate_expression(upd);
                }

                self.writeln(") {");

                self.indent_level += 1;
                self.generate_statement(body);
                self.indent_level -= 1;

                self.write_indent();
                self.writeln("}");
            }

            hir::Statement::Switch { value, cases } => {
                self.write_indent();
                self.write("switch (");
                self.generate_expression(value);
                self.writeln(") {");

                self.indent_level += 1;
                for case in cases {
                    if case.is_default {
                        self.write_indent();
                        self.writeln("default:");
                    } else {
                        for val in &case.values {
                            self.write_indent();
                            self.writeln(&format!("case {}:", val));
                        }
                    }

                    self.indent_level += 1;
                    for stmt in &case.body {
                        self.generate_statement(stmt);
                    }

                    // Add break if not already present
                    if !case.body.iter().any(|s| matches!(s, hir::Statement::Break)) {
                        self.write_indent();
                        self.writeln("break;");
                    }

                    self.indent_level -= 1;
                }
                self.indent_level -= 1;

                self.write_indent();
                self.writeln("}");
            }

            hir::Statement::Block(block) => {
                self.write_indent();
                self.writeln("{");
                self.indent_level += 1;
                self.generate_block(block);
                self.indent_level -= 1;
                self.write_indent();
                self.writeln("}");
            }

            hir::Statement::Break => {
                self.write_indent();
                self.writeln("break;");
            }

            hir::Statement::Continue => {
                self.write_indent();
                self.writeln("continue;");
            }

            hir::Statement::Goto(label) => {
                self.write_indent();
                self.writeln(&format!("goto {};", label));
            }

            hir::Statement::Label(label) => {
                // Labels are not indented
                self.writeln(&format!("{}:", label));
            }
        }
    }

    /// Generate an expression
    fn generate_expression(&mut self, expr: &hir::Expression) {
        match expr {
            hir::Expression::Variable(name) => {
                self.write(name);
            }

            hir::Expression::Literal(lit) => {
                self.generate_literal(lit);
            }

            hir::Expression::Binary { op, left, right } => {
                self.write("(");
                self.generate_expression(left);
                self.write(&format!(" {} ", self.binary_op_to_c(*op)));
                self.generate_expression(right);
                self.write(")");
            }

            hir::Expression::Unary { op, operand } => match op {
                hir::UnaryOp::PostInc | hir::UnaryOp::PostDec => {
                    self.generate_expression(operand);
                    self.write(self.unary_op_to_c(*op));
                }
                _ => {
                    self.write(self.unary_op_to_c(*op));
                    self.generate_expression(operand);
                }
            },

            hir::Expression::Call {
                function,
                arguments,
            } => {
                self.generate_expression(function);
                self.write("(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }

            hir::Expression::ArrayAccess { array, index } => {
                self.generate_expression(array);
                self.write("[");
                self.generate_expression(index);
                self.write("]");
            }

            hir::Expression::FieldAccess {
                object,
                field,
                is_pointer,
            } => {
                self.generate_expression(object);
                self.write(if *is_pointer { "->" } else { "." });
                self.write(field);
            }

            hir::Expression::Cast { ty, expr } => {
                self.write("(");
                self.write(&self.type_to_c(ty));
                self.write(")");
                self.generate_expression(expr);
            }

            hir::Expression::SizeOf(ty) => {
                self.write("sizeof(");
                self.write(&self.type_to_c(ty));
                self.write(")");
            }

            hir::Expression::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                self.write("(");
                self.generate_expression(condition);
                self.write(" ? ");
                self.generate_expression(then_expr);
                self.write(" : ");
                self.generate_expression(else_expr);
                self.write(")");
            }

            hir::Expression::AddressOf(expr) => {
                self.write("&");
                self.generate_expression(expr);
            }

            hir::Expression::Dereference(expr) => {
                self.write("*");
                self.generate_expression(expr);
            }
        }
    }

    /// Generate an l-value
    fn generate_lvalue(&mut self, lvalue: &hir::LValue) {
        match lvalue {
            hir::LValue::Variable(name) => {
                self.write(name);
            }
            hir::LValue::ArrayAccess { array, index } => {
                self.generate_lvalue(array);
                self.write("[");
                self.generate_expression(index);
                self.write("]");
            }
            hir::LValue::FieldAccess {
                object,
                field,
                is_pointer,
            } => {
                self.generate_lvalue(object);
                self.write(if *is_pointer { "->" } else { "." });
                self.write(field);
            }
            hir::LValue::Dereference(expr) => {
                self.write("*");
                self.generate_expression(expr);
            }
        }
    }

    /// Generate a literal
    fn generate_literal(&mut self, lit: &hir::Literal) {
        match lit {
            hir::Literal::Integer(i) => {
                self.write(&i.to_string());
            }
            hir::Literal::Float(f) => {
                self.write(f);
            }
            hir::Literal::String(s) => {
                self.write("\"");
                for c in s.chars() {
                    match c {
                        '\n' => self.write("\\n"),
                        '\r' => self.write("\\r"),
                        '\t' => self.write("\\t"),
                        '\\' => self.write("\\\\"),
                        '"' => self.write("\\\""),
                        _ => self.write(&c.to_string()),
                    }
                }
                self.write("\"");
            }
            hir::Literal::Char(c) => {
                self.write("'");
                match c {
                    '\n' => self.write("\\n"),
                    '\r' => self.write("\\r"),
                    '\t' => self.write("\\t"),
                    '\\' => self.write("\\\\"),
                    '\'' => self.write("\\'"),
                    _ => self.write(&c.to_string()),
                }
                self.write("'");
            }
            hir::Literal::Bool(b) => {
                self.write(if *b { "true" } else { "false" });
            }
            hir::Literal::Null => {
                self.write("NULL");
            }
        }
    }

    /// Convert type to C string
    #[allow(clippy::only_used_in_recursion)]
    fn type_to_c(&self, ty: &hir::Type) -> String {
        match ty {
            hir::Type::Void => "void".to_string(),
            hir::Type::Bool => "bool".to_string(),
            hir::Type::Char => "char".to_string(),
            hir::Type::Short => "short".to_string(),
            hir::Type::Int => "int".to_string(),
            hir::Type::Long => "long".to_string(),
            hir::Type::LongLong => "long long".to_string(),
            hir::Type::Float => "float".to_string(),
            hir::Type::Double => "double".to_string(),
            hir::Type::UChar => "unsigned char".to_string(),
            hir::Type::UShort => "unsigned short".to_string(),
            hir::Type::UInt => "unsigned int".to_string(),
            hir::Type::ULong => "unsigned long".to_string(),
            hir::Type::ULongLong => "unsigned long long".to_string(),
            hir::Type::Pointer(inner) => format!("{}*", self.type_to_c(inner)),
            hir::Type::Array { element, size } => match size {
                Some(s) => format!("{}[{}]", self.type_to_c(element), s),
                None => format!("{}[]", self.type_to_c(element)),
            },
            hir::Type::FunctionPointer {
                return_type,
                parameters,
                variadic,
            } => {
                let params = parameters
                    .iter()
                    .map(|p| self.type_to_c(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                let params = if *variadic {
                    if params.is_empty() {
                        "..."
                    } else {
                        &format!("{}, ...", params)
                    }
                } else {
                    &params
                };
                format!("{}(*)({})", self.type_to_c(return_type), params)
            }
            hir::Type::Struct(name) => format!("struct {}", name),
            hir::Type::Union(name) => format!("union {}", name),
            hir::Type::Enum(name) => format!("enum {}", name),
            hir::Type::TypeDef(name) => name.clone(),
        }
    }

    /// Convert binary operator to C string
    fn binary_op_to_c(&self, op: hir::BinaryOp) -> &'static str {
        match op {
            hir::BinaryOp::Add => "+",
            hir::BinaryOp::Sub => "-",
            hir::BinaryOp::Mul => "*",
            hir::BinaryOp::Div => "/",
            hir::BinaryOp::Mod => "%",
            hir::BinaryOp::Eq => "==",
            hir::BinaryOp::Ne => "!=",
            hir::BinaryOp::Lt => "<",
            hir::BinaryOp::Le => "<=",
            hir::BinaryOp::Gt => ">",
            hir::BinaryOp::Ge => ">=",
            hir::BinaryOp::And => "&&",
            hir::BinaryOp::Or => "||",
            hir::BinaryOp::BitAnd => "&",
            hir::BinaryOp::BitOr => "|",
            hir::BinaryOp::BitXor => "^",
            hir::BinaryOp::Shl => "<<",
            hir::BinaryOp::Shr => ">>",
        }
    }

    /// Convert unary operator to C string
    fn unary_op_to_c(&self, op: hir::UnaryOp) -> &'static str {
        match op {
            hir::UnaryOp::Neg => "-",
            hir::UnaryOp::Not => "!",
            hir::UnaryOp::BitNot => "~",
            hir::UnaryOp::PreInc => "++",
            hir::UnaryOp::PreDec => "--",
            hir::UnaryOp::PostInc => "++",
            hir::UnaryOp::PostDec => "--",
        }
    }

    /// Write with indentation
    fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("    ");
        }
    }

    /// Write string to output
    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    /// Write string with newline
    fn writeln(&mut self, s: &str) {
        self.output.push_str(s);
        self.output.push('\n');
    }
}
