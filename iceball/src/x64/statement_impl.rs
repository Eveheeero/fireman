use crate::{DisassembleError, Statement, StatementInner, X64Statement};

impl StatementInner for X64Statement {
    fn is_jcc(&self) -> bool {
        matches!(
            *self,
            X64Statement::Ja
                | X64Statement::Jae
                | X64Statement::Jb
                | X64Statement::Jbe
                | X64Statement::Jcxz
                | X64Statement::Jecxz
                | X64Statement::Jrcxz
                | X64Statement::Jz
                | X64Statement::Jg
                | X64Statement::Jge
                | X64Statement::Jl
                | X64Statement::Jle
                | X64Statement::Jnz
                | X64Statement::Jno
                | X64Statement::Jnp
                | X64Statement::Jns
                | X64Statement::Jo
                | X64Statement::Jp
                | X64Statement::Js
        )
    }
    fn is_jmp(&self) -> bool {
        matches!(*self, X64Statement::Jmp)
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
impl std::fmt::Display for X64Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().to_ascii_lowercase())
    }
}
