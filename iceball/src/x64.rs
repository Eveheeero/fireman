pub mod register;
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
            op.parse()
        } else {
            u64::from_str_radix(&op[2..], 16)
        };
        if data.is_err() {
            panic!("Cannot parse {}", op);
        }
        return Ok(crate::Argument::Constant(data.unwrap()));
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
    let inner = inner
        .nth(1)
        .unwrap_or_else(|| panic!("{}는 파싱 가능한 형태가 아닙니다.", op));
    let items = inner.split(' ');
    for item in items {
        if item.as_bytes()[0].is_ascii_digit() {
            let num = if item.contains('x') {
                u64::from_str_radix(&item[2..], 16).unwrap()
            } else {
                item.parse().unwrap()
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

        let register = item.parse::<register::X64Register>().unwrap();
        result.push(crate::RelativeAddressingArgument::Register(
            crate::Register::X64(register),
        ));
    }

    Ok(crate::Argument::Memory(crate::Memory::RelativeAddressing(
        result.into_boxed_slice(),
    )))
}
