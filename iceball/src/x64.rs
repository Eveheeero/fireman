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
    let mut inner = op.split(['[', ']']);
    let inner = inner
        .nth(1)
        .unwrap_or_else(|| panic!("{}는 파싱 가능한 형태가 아닙니다.", op));

    let mut base: Option<crate::Register> = None;
    let mut index: Option<crate::Register> = None;
    let mut scale: u8 = 1;
    let mut displacement: i64 = 0;
    let mut size: Option<u8> = None;

    // Extract size hint from prefix (e.g., "dword ptr", "qword ptr")
    if let Some(size_pos) = op.find(" ptr") {
        let size_str = &op[..size_pos];
        size = match size_str {
            "byte" => Some(1),
            "word" => Some(2),
            "dword" => Some(4),
            "qword" => Some(8),
            _ => None,
        };
    }

    // Parse the memory operand expression
    let items: Vec<&str> = inner.split(' ').filter(|s| !s.is_empty()).collect();
    let mut i = 0;
    let mut pending_op = None;

    while i < items.len() {
        let item = items[i];

        // Handle operators
        if ["+", "-", "*"].contains(&item) {
            pending_op = Some(item);
            i += 1;
            continue;
        }

        // Handle numbers
        if item.as_bytes()[0].is_ascii_digit() {
            let num = if item.contains('x') {
                i64::from_str_radix(&item[2..], 16).unwrap()
            } else {
                item.parse().unwrap()
            };

            if let Some(op) = pending_op {
                match op {
                    "-" => displacement -= num,
                    "+" => displacement += num,
                    "*" => scale = num as u8,
                    _ => {}
                }
                pending_op = None;
            } else {
                displacement = num;
            }
            i += 1;
            continue;
        }

        // Handle registers
        if let Ok(reg) = item.parse::<register::X64Register>() {
            let reg = crate::Register::X64(reg);

            // Check if this is a scale operation
            if i + 2 < items.len() && items[i + 1] == "*" {
                index = Some(reg);
                i += 2; // Skip register and "*"
                if i < items.len() && items[i].as_bytes()[0].is_ascii_digit() {
                    scale = items[i].parse().unwrap_or(1);
                    i += 1;
                }
            } else if base.is_none() {
                base = Some(reg);
                i += 1;
            } else {
                index = Some(reg);
                i += 1;
            }
            continue;
        }

        i += 1;
    }

    Ok(crate::Argument::Memory(crate::Memory {
        base,
        index,
        scale,
        displacement,
        size,
    }))
}
