#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DecompileError {
    Unknown(Option<String>),
    HeaderParsingFailed,
    DisassembleFailed(super::disassemble_error::DisassembleError),
    EntryNotFound,
    /// The binary has no meaningful entry point (shared library, object file, or debug-info-only binary).
    NoEntryPoint,
    CASTGenerationFailed(Option<String>),
    /// RwLock was poisoned during an operation.
    LockPoisoned(String),
    /// Function with given ID and version was not found.
    FunctionNotFound(
        crate::abstract_syntax_tree::AstFunctionId,
        crate::abstract_syntax_tree::AstFunctionVersion,
    ),
    /// IR analysis assertion failure.
    IrAnalyzeAssertionFailure(super::ir_analyze_assertion_error::IrAnalyzeAssertionFailure),
}

impl Default for DecompileError {
    fn default() -> Self {
        Self::Unknown(None)
    }
}

impl std::fmt::Display for DecompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(msg) => {
                write!(f, "Unknown Error Occured! {}", msg.as_deref().unwrap_or(""))
            }
            Self::HeaderParsingFailed => write!(f, "Header Parsing Failed!"),
            Self::DisassembleFailed(err) => write!(f, "Fail to disassemble block! {}", err),
            Self::EntryNotFound => write!(f, "Entry Not Found!"),
            Self::NoEntryPoint => write!(
                f,
                "Binary has no entry point (shared library, object file, or debug-info-only binary)"
            ),
            Self::CASTGenerationFailed(msg) => write!(
                f,
                "C-AST Generation Failed! {}",
                msg.as_deref().unwrap_or("")
            ),
            Self::LockPoisoned(context) => write!(f, "Lock poisoned: {}", context),
            Self::FunctionNotFound(id, version) => {
                write!(f, "Function not found: id={:?}, version={:?}", id, version)
            }
            Self::IrAnalyzeAssertionFailure(err) => {
                write!(f, "IR analysis assertion failed: {:?}", err)
            }
        }
    }
}

impl From<goblin::error::Error> for DecompileError {
    fn from(_: goblin::error::Error) -> Self {
        Self::HeaderParsingFailed
    }
}

impl From<String> for DecompileError {
    fn from(msg: String) -> Self {
        Self::Unknown(Some(msg))
    }
}

impl From<&String> for DecompileError {
    fn from(msg: &String) -> Self {
        Self::Unknown(Some(msg.clone()))
    }
}

impl From<&str> for DecompileError {
    fn from(msg: &str) -> Self {
        Self::Unknown(Some(msg.to_string()))
    }
}

impl From<super::disassemble_error::DisassembleError> for DecompileError {
    fn from(err: super::disassemble_error::DisassembleError) -> Self {
        Self::DisassembleFailed(err)
    }
}

impl From<super::ir_analyze_assertion_error::IrAnalyzeAssertionFailure> for DecompileError {
    fn from(err: super::ir_analyze_assertion_error::IrAnalyzeAssertionFailure) -> Self {
        Self::IrAnalyzeAssertionFailure(err)
    }
}
