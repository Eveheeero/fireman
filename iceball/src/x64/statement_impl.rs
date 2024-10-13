use crate::{DisassembleError, Statement, StatementInner, X64Statement};

impl StatementInner for X64Statement {
    fn is_jcc(&self) -> bool {
        match *self {
            X64Statement::Jcc => true,
            X64Statement::Jmp => true,
            _ => false,
        }
    }

    fn is_call(&self) -> bool {
        self == &X64Statement::Call
    }

    fn is_ret(&self) -> bool {
        self == &X64Statement::Ret
    }
}

impl X64Statement {
    pub(crate) fn parse(op: impl AsRef<str>) -> Result<Statement, DisassembleError> {
        let op = op.as_ref();
        let op = op.to_ascii_uppercase();
        if let Ok(stmt) = op.parse() {
            Ok(stmt)
        } else {
            Err(DisassembleError::UnknownStatement)
        }
    }
}
