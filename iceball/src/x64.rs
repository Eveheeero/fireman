pub mod register;
pub mod statement;
mod statement_impl;

pub fn parse_argument(op: impl AsRef<str>) -> Result<crate::Argument, crate::DisassembleError> {
    let op = op.as_ref();

    if op.is_empty() {
        return Err(crate::DisassembleError::Unknown);
    }

    /* Constant */
    if op.chars().nth(0).unwrap()=='0' {
        let data = u64::from_str_radix(&op[2..], 16);
        if data.is_err() {
            panic!("Cannot parse {}", op);
        }
        return Ok(crate::Argument::Constant(data.unwrap()));
    }

    /* Register */
    if !op.contains(' ') {
        let data = op.parse();
        if data.is_err() {
            return Err(data.unwrap_err());
        }
        return Ok(crate::Argument::Register(crate::Register::X64(
            data.unwrap(),
        )));
    }

    /* Memory */
    todo!("{}", op)
}
