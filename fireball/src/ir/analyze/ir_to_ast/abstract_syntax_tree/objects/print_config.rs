#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstPrintConfig {
    pub print_instruction: bool,
    pub print_ir: bool,
    pub print_empty_statement: bool,
}
impl AstPrintConfig {
    pub const DEFAULT: Self = Self {
        print_instruction: true,
        print_ir: true,
        print_empty_statement: false,
    };
    pub const ALL: Self = Self {
        print_instruction: true,
        print_ir: true,
        print_empty_statement: true,
    };
    pub const NONE: Self = Self {
        print_instruction: false,
        print_ir: false,
        print_empty_statement: false,
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
}
impl Default for AstPrintConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

pub trait PrintWithConfig {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String;
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result;
}
