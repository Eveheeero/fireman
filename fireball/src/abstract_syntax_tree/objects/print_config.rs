#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstPrintConfig {
    pub print_instruction: bool,
    pub print_ir: bool,
    pub print_empty_statement: bool,
    pub replace_constant: bool,
    pub parameter_usage_comment: bool,
    pub variable_usage_comment: bool,
}
impl AstPrintConfig {
    pub const DEFAULT: Self = Self {
        print_instruction: false,
        print_ir: false,
        print_empty_statement: false,
        replace_constant: true,
        parameter_usage_comment: true,
        variable_usage_comment: false,
    };
    pub const ALL: Self = Self {
        print_instruction: true,
        print_ir: true,
        print_empty_statement: true,
        replace_constant: true,
        parameter_usage_comment: true,
        variable_usage_comment: true,
    };
    pub const NONE: Self = Self {
        print_instruction: false,
        print_ir: false,
        print_empty_statement: false,
        replace_constant: false,
        parameter_usage_comment: false,
        variable_usage_comment: false,
    };

    pub fn print_instruction(mut self, value: bool) -> Self {
        self.print_instruction = value;
        self
    }
    pub fn print_ir(mut self, value: bool) -> Self {
        self.print_ir = value;
        self
    }
    pub fn print_empty_statement(mut self, value: bool) -> Self {
        self.print_empty_statement = value;
        self
    }
    pub fn replace_constant(mut self, value: bool) -> Self {
        self.replace_constant = value;
        self
    }
    pub fn parameter_usage_comment(mut self, value: bool) -> Self {
        self.parameter_usage_comment = value;
        self
    }
    pub fn variable_usage_comment(mut self, value: bool) -> Self {
        self.variable_usage_comment = value;
        self
    }
}
impl Default for AstPrintConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}
