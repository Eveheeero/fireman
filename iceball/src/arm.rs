pub mod register;
mod register_impl;
pub mod statements;

use crate::{
    AddressingOperator, Argument, DisassembleError, Memory, Register, RelativeAddressingArgument,
    Statement, StatementInner,
};
pub use register::ArmRegister;
pub use statements::ArmStatement;

impl StatementInner for ArmStatement {
    fn is_jcc(&self) -> bool {
        matches!(
            *self,
            ArmStatement::B
                | ArmStatement::Cbnz
                | ArmStatement::Cbz
                | ArmStatement::Tbnz
                | ArmStatement::Tbz
        )
    }

    fn is_jmp(&self) -> bool {
        matches!(*self, ArmStatement::Br | ArmStatement::Braa)
    }

    fn is_call(&self) -> bool {
        matches!(*self, ArmStatement::Bl | ArmStatement::Blr)
    }

    fn is_ret(&self) -> bool {
        matches!(
            *self,
            ArmStatement::Ret
                | ArmStatement::Retaa
                | ArmStatement::Eret
                | ArmStatement::Eretaa
                | ArmStatement::Drps
        )
    }
}

impl ArmStatement {
    pub(crate) fn parse(mnemonic: impl AsRef<str>) -> Result<Statement, DisassembleError> {
        let mnemonic = mnemonic.as_ref().to_ascii_uppercase();
        if let Ok(stmt) = mnemonic.parse::<ArmStatement>() {
            Ok(Statement::Arm(stmt))
        } else {
            Err(DisassembleError::UnknownStatement)
        }
    }
}

pub fn parse_argument(op: impl AsRef<str>) -> Result<Argument, DisassembleError> {
    let op = op.as_ref().trim();
    if op.is_empty() {
        return Err(DisassembleError::Unknown);
    }

    if op.contains('[') {
        return parse_memory(op);
    }

    if let Some(constant) = parse_immediate_u64(op) {
        return Ok(Argument::Constant(constant?));
    }

    parse_register(op).map(Argument::Register)
}

fn parse_register(op: &str) -> Result<Register, DisassembleError> {
    let register = op.parse::<ArmRegister>()?;
    Ok(Register::Arm(register))
}

fn parse_memory(op: &str) -> Result<Argument, DisassembleError> {
    let start = op.find('[').ok_or(DisassembleError::Unknown)?;
    let end = op[start..]
        .find(']')
        .map(|offset| start + offset)
        .ok_or(DisassembleError::Unknown)?;
    let inner = &op[start + 1..end];

    let mut items = Vec::<RelativeAddressingArgument>::new();
    let mut terms = inner
        .split(',')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .peekable();

    while let Some(segment) = terms.next() {
        let normalized = segment.trim_end_matches('!');
        if let Some(register) = parse_memory_register(normalized) {
            push_term(
                &mut items,
                RelativeAddressingArgument::Register(Register::Arm(register)),
            );
            continue;
        }

        if let Some(constant) = parse_immediate_i128(normalized) {
            push_term(&mut items, RelativeAddressingArgument::Constant(constant?));
            continue;
        }

        if let Some(shift_operand) = normalized.strip_prefix("lsl ") {
            let shift = parse_shift_amount(shift_operand)?;
            items.push(RelativeAddressingArgument::Operator(
                AddressingOperator::Mul,
            ));
            items.push(RelativeAddressingArgument::Constant(i128::from(
                1_u64
                    .checked_shl(u32::from(shift))
                    .ok_or(DisassembleError::Unknown)?,
            )));
            continue;
        }

        if normalized.eq_ignore_ascii_case("lsl") {
            let shift = terms
                .next()
                .ok_or(DisassembleError::Unknown)
                .and_then(parse_shift_amount)?;
            items.push(RelativeAddressingArgument::Operator(
                AddressingOperator::Mul,
            ));
            items.push(RelativeAddressingArgument::Constant(i128::from(
                1_u64
                    .checked_shl(u32::from(shift))
                    .ok_or(DisassembleError::Unknown)?,
            )));
            continue;
        }

        return Err(DisassembleError::Unknown);
    }

    Ok(Argument::Memory(Memory::RelativeAddressing(
        items.into_boxed_slice(),
    )))
}

fn push_term(items: &mut Vec<RelativeAddressingArgument>, term: RelativeAddressingArgument) {
    if matches!(
        items.last(),
        Some(RelativeAddressingArgument::Register(_))
            | Some(RelativeAddressingArgument::Constant(_))
    ) {
        items.push(RelativeAddressingArgument::Operator(
            AddressingOperator::Add,
        ));
    }
    items.push(term);
}

fn parse_memory_register(value: &str) -> Option<ArmRegister> {
    value
        .split_once('.')
        .map_or(value, |(register, _)| register)
        .parse::<ArmRegister>()
        .ok()
}

fn parse_immediate_u64(value: &str) -> Option<Result<u64, DisassembleError>> {
    let value = value.strip_prefix('#').unwrap_or(value).trim();
    if value.is_empty() {
        return None;
    }

    if let Some(hex) = value.strip_prefix("0x") {
        return Some(u64::from_str_radix(hex, 16).map_err(|_| DisassembleError::Unknown));
    }

    if value.starts_with('-') {
        return None;
    }

    value
        .chars()
        .all(|ch| ch.is_ascii_digit())
        .then(|| value.parse::<u64>().map_err(|_| DisassembleError::Unknown))
}

fn parse_immediate_i128(value: &str) -> Option<Result<i128, DisassembleError>> {
    let value = value.strip_prefix('#').unwrap_or(value).trim();
    if value.is_empty() {
        return None;
    }

    if let Some(hex) = value.strip_prefix("0x") {
        return Some(i128::from_str_radix(hex, 16).map_err(|_| DisassembleError::Unknown));
    }

    if let Some(hex) = value.strip_prefix("-0x") {
        return Some(
            i128::from_str_radix(hex, 16)
                .map(|parsed| -parsed)
                .map_err(|_| DisassembleError::Unknown),
        );
    }

    let signed = value.starts_with('-') || value.starts_with('+');
    let digits_only = value
        .chars()
        .all(|ch| ch.is_ascii_digit() || matches!(ch, '-' | '+'));
    (signed || digits_only).then(|| value.parse::<i128>().map_err(|_| DisassembleError::Unknown))
}

fn parse_shift_amount(value: &str) -> Result<u8, DisassembleError> {
    let shift = parse_immediate_u64(value).ok_or(DisassembleError::Unknown)??;
    u8::try_from(shift).map_err(|_| DisassembleError::Unknown)
}

impl std::fmt::Display for ArmStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().to_ascii_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::parse_argument;
    use crate::{
        AddressingOperator, Argument, ArmRegister, Memory, Register, RelativeAddressingArgument,
    };

    #[test]
    fn parses_arm_register_operand() {
        let parsed = parse_argument("x3").unwrap();
        assert_eq!(parsed, Argument::Register(Register::Arm(ArmRegister::X(3))));
    }

    #[test]
    fn parses_arm_vector_register_operand_with_arrangement() {
        let parsed = parse_argument("v31.16b").unwrap();
        assert_eq!(
            parsed,
            Argument::Register(Register::Arm(ArmRegister::V(31)))
        );
    }

    #[test]
    fn parses_arm_memory_with_immediate_offset() {
        let parsed = parse_argument("[x0, #0x10]").unwrap();
        assert_eq!(
            parsed,
            Argument::Memory(Memory::RelativeAddressing(
                [
                    RelativeAddressingArgument::Register(Register::Arm(ArmRegister::X(0))),
                    RelativeAddressingArgument::Operator(AddressingOperator::Add),
                    RelativeAddressingArgument::Constant(16),
                ]
                .into()
            ))
        );
    }

    #[test]
    fn parses_arm_memory_with_scaled_index() {
        let parsed = parse_argument("[x0, x1, lsl #2]").unwrap();
        assert_eq!(
            parsed,
            Argument::Memory(Memory::RelativeAddressing(
                [
                    RelativeAddressingArgument::Register(Register::Arm(ArmRegister::X(0))),
                    RelativeAddressingArgument::Operator(AddressingOperator::Add),
                    RelativeAddressingArgument::Register(Register::Arm(ArmRegister::X(1))),
                    RelativeAddressingArgument::Operator(AddressingOperator::Mul),
                    RelativeAddressingArgument::Constant(4),
                ]
                .into()
            ))
        );
    }
}
