use crate::{DisassembleError, Statement, StatementInner, X64Statement};

impl StatementInner for X64Statement {
    fn is_jcc(&self) -> bool {
        matches!(*self, X64Statement::Jcc | X64Statement::Jmp)
    }

    fn is_call(&self) -> bool {
        self == &X64Statement::Call
    }

    fn is_ret(&self) -> bool {
        self == &X64Statement::Ret
    }
}

impl X64Statement {
    pub(crate) fn parse(mnemonic: impl AsRef<str>) -> Result<Statement, DisassembleError> {
        let mnemonic = mnemonic.as_ref();
        let mnemonic = mnemonic.to_ascii_uppercase();
        if let Ok(stmt) = mnemonic.parse::<X64Statement>() {
            Ok(Statement::X64(stmt))
        } else {
            Err(DisassembleError::UnknownStatement)
        }
    }
}
