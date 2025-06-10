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
        // Extract x64 registers from the Memory struct
        let mut base = None;
        let mut index = None;

        if let Some(base_reg) = &mem.base {
            match base_reg {
                iceball::Register::X64(x64reg) => {
                    base = Some(*x64reg);
                }
                _ => {
                    return Err("Non-x86_64 base register in x86_64 addressing".to_string());
                }
            }
        }

        if let Some(index_reg) = &mem.index {
            match index_reg {
                iceball::Register::X64(x64reg) => {
                    index = Some(*x64reg);
                }
                _ => {
                    return Err("Non-x86_64 index register in x86_64 addressing".to_string());
                }
            }
        }

        Ok(MemoryAccess {
            base,
            index,
            scale: mem.scale as i64,
            displacement: mem.displacement as i128,
            size: mem.size,
        })
    }
}
