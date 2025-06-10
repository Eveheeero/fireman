//! Enhanced C printer extensions
//!
//! This module extends the existing C AST printer to support Enhanced C features
//! without modifying the core AST structure.

use super::c_abstract_syntax_tree::*;
use super::enhanced_c::*;
use std::fmt::Write;

/// Enhanced printer that wraps the standard printer
pub struct EnhancedPrinter<'a> {
    config: ExtendedPrintConfig,
    writer: &'a mut dyn Write,
}

impl<'a> EnhancedPrinter<'a> {
    pub fn new(writer: &'a mut dyn Write, config: ExtendedPrintConfig) -> Self {
        Self { config, writer }
    }

    /// Print a type with Enhanced C transformations
    pub fn print_type(&mut self, ty: &CType) -> std::fmt::Result {
        // Check for Enhanced C type mappings first
        if let Some(enhanced_type) = map_to_enhanced_type(ty, &self.config.enhanced) {
            write!(self.writer, "{}", enhanced_type)
        } else {
            // Fall back to standard type printing
            ty.print(self.writer, Some(self.config.base))
        }
    }

    /// Print a literal with Enhanced C transformations
    pub fn print_literal(&mut self, lit: &Literal) -> std::fmt::Result {
        // Check for nullptr transformation
        if let Some(nullptr_str) = transform_nullptr_literal(lit, &self.config.enhanced) {
            write!(self.writer, "{}", nullptr_str)
        } else {
            // Fall back to standard literal printing
            match lit {
                Literal::Int(i) => write!(self.writer, "{}", i),
                Literal::UInt(u) => write!(self.writer, "{}u", u),
                Literal::Float(f) => write!(self.writer, "{}", f),
                Literal::String(s) => write!(self.writer, "\"{}\"", s),
                Literal::Char(c) => write!(self.writer, "'{}'", c),
                Literal::Bool(b) => write!(self.writer, "{}", if *b { "true" } else { "false" }),
            }
        }
    }

    /// Print a variable declaration with potential auto type
    pub fn print_declaration(
        &mut self,
        var: &Variable,
        init: &Option<Wrapped<Expression>>,
    ) -> std::fmt::Result {
        let transformer = EnhancedCAstTransformer::new(self.config.enhanced);

        if transformer.can_use_auto(var, init) {
            write!(self.writer, "auto {} = ", var.name)?;
            if let Some(init_expr) = init {
                self.print_expression(&init_expr.item)?;
            }
        } else {
            self.print_type(&var.var_type)?;
            write!(self.writer, " {}", var.name)?;
            if let Some(init_expr) = init {
                write!(self.writer, " = ")?;
                self.print_expression(&init_expr.item)?;
            }
        }
        Ok(())
    }

    /// Print an expression with Enhanced C features
    pub fn print_expression(&mut self, expr: &Expression) -> std::fmt::Result {
        match expr {
            Expression::Literal(lit) => self.print_literal(lit),
            Expression::Cast(ty, inner) => {
                write!(self.writer, "(")?;
                self.print_type(ty)?;
                write!(self.writer, ")")?;
                self.print_expression(&inner.item)
            }
            // Delegate other cases to standard printer
            _ => expr.print(self.writer, Some(self.config.base)),
        }
    }

    /// Print enhanced headers if enabled
    pub fn print_headers(&mut self) -> std::fmt::Result {
        if self.config.enhanced.enabled {
            write!(
                self.writer,
                "{}",
                generate_enhanced_headers(&self.config.enhanced)
            )?;
        }
        Ok(())
    }
}

/// Extension trait for CAst to support Enhanced C printing
pub trait EnhancedCAstExt {
    fn to_enhanced_c_code(&self, config: ExtendedPrintConfig) -> String;
}

impl EnhancedCAstExt for CAst {
    fn to_enhanced_c_code(&self, config: ExtendedPrintConfig) -> String {
        let mut output = String::new();
        let mut printer = EnhancedPrinter::new(&mut output, config);

        // Print enhanced headers
        let _ = printer.print_headers();

        // Add standard headers
        output.push_str("#include <stdio.h>\n");
        output.push_str("#include <stdlib.h>\n");
        output.push_str("#include <stdbool.h>\n\n");

        // Use the standard to_c_code but with our extended config
        output.push_str(&self.to_c_code(Some(config.base)));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_printer_type_printing() {
        let config = ExtendedPrintConfig::default();
        let mut output = String::new();
        {
            let mut printer = EnhancedPrinter::new(&mut output, config);
            let _ = printer.print_type(&CType::Int32);
        }
        assert_eq!(output, "int32_t");
    }

    #[test]
    fn test_enhanced_printer_nullptr_printing() {
        let config = ExtendedPrintConfig::default();
        let mut output = String::new();
        {
            let mut printer = EnhancedPrinter::new(&mut output, config);
            let _ = printer.print_literal(&Literal::Int(0));
        }
        assert_eq!(output, "nullptr");
    }
}
