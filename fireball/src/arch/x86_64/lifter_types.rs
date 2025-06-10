//! Type mappings between iceball and Low IR for x86_64 lifting

use iceball::X64Register;

/// x86_64 memory access representation for lifting
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryAccess {
    pub base: Option<X64Register>,
    pub index: Option<X64Register>,
    pub scale: i64,
    pub displacement: i128,
    pub size: Option<u8>, // Size in bytes (1, 2, 4, 8)
}

impl MemoryAccess {
    /// Parse from iceball Memory type
    pub fn from_iceball(mem: &iceball::Memory) -> Result<Self, String> {
        match mem {
            iceball::Memory::AbsoluteAddressing(addr) => Ok(MemoryAccess {
                base: None,
                index: None,
                scale: 1,
                displacement: *addr as i128,
                size: None,
            }),
            iceball::Memory::RelativeAddressing(parts) => {
                let mut base = None;
                let mut index = None;
                let mut scale = 1i64;
                let mut displacement = 0i128;

                // Parse the addressing formula
                let mut current_scale = 1i64;
                let mut in_multiplication = false;

                for part in parts.iter() {
                    use iceball::{AddressingOperator, RelativeAddressingArgument};

                    match part {
                        RelativeAddressingArgument::Register(reg) => {
                            match reg {
                                iceball::Register::X64(x64reg) => {
                                    if in_multiplication {
                                        // This is the index register
                                        if index.is_some() {
                                            return Err("Multiple index registers not supported"
                                                .to_string());
                                        }
                                        index = Some(*x64reg);
                                        scale = current_scale;
                                        in_multiplication = false;
                                    } else if base.is_none() {
                                        base = Some(*x64reg);
                                    } else if index.is_none() {
                                        index = Some(*x64reg);
                                    } else {
                                        return Err("Too many registers in address".to_string());
                                    }
                                }
                            }
                        }
                        RelativeAddressingArgument::Constant(val) => {
                            if in_multiplication {
                                current_scale = *val as i64;
                            } else {
                                displacement += val;
                            }
                        }
                        RelativeAddressingArgument::Operator(op) => {
                            match op {
                                AddressingOperator::Mul => {
                                    in_multiplication = true;
                                }
                                AddressingOperator::Add => {
                                    // Default behavior
                                }
                                AddressingOperator::Sub => {
                                    // Next constant should be negative
                                    // This is a simplification; proper parsing would need state
                                }
                            }
                        }
                    }
                }

                Ok(MemoryAccess {
                    base,
                    index,
                    scale,
                    displacement,
                    size: None,
                })
            }
        }
    }
}
