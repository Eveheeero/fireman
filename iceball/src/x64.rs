pub mod register;
mod register_impl;
pub mod statement;
mod statement_impl;

pub fn parse_argument(op: impl AsRef<str>) -> Result<crate::Argument, crate::DisassembleError> {
    let op = op.as_ref();

    if op.is_empty() {
        return Err(crate::DisassembleError::Unknown);
    }

    /* Constant */
    if op.as_bytes()[0] == b'0' {
        let data = if op.len() == 1 {
            op.parse::<u64>().map_err(|_| crate::DisassembleError::Unknown)?
        } else {
            u64::from_str_radix(&op[2..], 16).map_err(|_| crate::DisassembleError::Unknown)?
        };
        return Ok(crate::Argument::Constant(data));
    }

    /* Register */
    if !op.contains(' ') {
        let data = op.parse()?;
        return Ok(crate::Argument::Register(crate::Register::X64(data)));
    }

    /* Memory */
    parse_memory(op)
}

// dword ptr [rbp - 4]
fn parse_memory(op: &str) -> Result<crate::Argument, crate::DisassembleError> {
    let mut result = Vec::<crate::RelativeAddressingArgument>::new();
    let mut inner = op.split(['[', ']']);
    let inner = inner.nth(1).ok_or(crate::DisassembleError::Unknown)?;
    let mut normalized = String::with_capacity(inner.len() * 2);
    for ch in inner.chars() {
        if ['+', '-', '*'].contains(&ch) {
            normalized.push(' ');
            normalized.push(ch);
            normalized.push(' ');
        } else {
            normalized.push(ch);
        }
    }
    let items = normalized.split_whitespace();
    for item in items {
        if item
            .as_bytes()
            .first()
            .is_some_and(|byte| byte.is_ascii_digit())
        {
            let num = if item.contains('x') {
                u64::from_str_radix(&item[2..], 16).map_err(|_| crate::DisassembleError::Unknown)?
            } else {
                item.parse().map_err(|_| crate::DisassembleError::Unknown)?
            };
            result.push(crate::RelativeAddressingArgument::Constant(num as i128));
            continue;
        }

        if ["+", "-", "*"].contains(&item) {
            result.push(crate::RelativeAddressingArgument::Operator(match item {
                "+" => crate::AddressingOperator::Add,
                "-" => crate::AddressingOperator::Sub,
                "*" => crate::AddressingOperator::Mul,
                _ => unreachable!(),
            }));
            continue;
        }

        let register = item.parse::<register::X64Register>()?;
        result.push(crate::RelativeAddressingArgument::Register(
            crate::Register::X64(register),
        ));
    }

    Ok(crate::Argument::Memory(crate::Memory::RelativeAddressing(
        result.into_boxed_slice(),
    )))
}
