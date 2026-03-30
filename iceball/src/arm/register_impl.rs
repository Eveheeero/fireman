use crate::{ArmRegister, DisassembleError};
use std::str::FromStr;

impl FromStr for ArmRegister {
    type Err = DisassembleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim().to_ascii_lowercase();
        let value = value
            .split_once('.')
            .map_or(value.as_str(), |(register, _)| register);
        let value = value
            .split_once('[')
            .map_or(value, |(register, _)| register)
            .trim_end_matches('!');

        match value {
            "sp" => return Ok(Self::Sp),
            "wsp" => return Ok(Self::Wsp),
            "xzr" => return Ok(Self::Xzr),
            "wzr" => return Ok(Self::Wzr),
            "fp" => return Ok(Self::Fp),
            "lr" => return Ok(Self::Lr),
            "pc" => return Ok(Self::Pc),
            "ffr" => return Ok(Self::Ffr),
            _ => {}
        }

        let (prefix, number) = value.split_at(1);
        let number = number
            .parse::<u8>()
            .map_err(|_| DisassembleError::UnknownRegister)?;

        match prefix {
            "x" | "w" if number <= 30 => Ok(if prefix == "x" {
                Self::X(number)
            } else {
                Self::W(number)
            }),
            "v" | "q" | "d" | "s" | "h" | "b" | "z" if number <= 31 => Ok(match prefix {
                "v" => Self::V(number),
                "q" => Self::Q(number),
                "d" => Self::D(number),
                "s" => Self::S(number),
                "h" => Self::H(number),
                "b" => Self::B(number),
                "z" => Self::Z(number),
                _ => unreachable!(),
            }),
            "p" if number <= 15 => Ok(Self::P(number)),
            _ => Err(DisassembleError::UnknownRegister),
        }
    }
}

impl std::fmt::Display for ArmRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X(index) => write!(f, "x{index}"),
            Self::W(index) => write!(f, "w{index}"),
            Self::V(index) => write!(f, "v{index}"),
            Self::Q(index) => write!(f, "q{index}"),
            Self::D(index) => write!(f, "d{index}"),
            Self::S(index) => write!(f, "s{index}"),
            Self::H(index) => write!(f, "h{index}"),
            Self::B(index) => write!(f, "b{index}"),
            Self::Z(index) => write!(f, "z{index}"),
            Self::P(index) => write!(f, "p{index}"),
            Self::Sp => write!(f, "sp"),
            Self::Wsp => write!(f, "wsp"),
            Self::Xzr => write!(f, "xzr"),
            Self::Wzr => write!(f, "wzr"),
            Self::Fp => write!(f, "fp"),
            Self::Lr => write!(f, "lr"),
            Self::Pc => write!(f, "pc"),
            Self::Ffr => write!(f, "ffr"),
        }
    }
}
