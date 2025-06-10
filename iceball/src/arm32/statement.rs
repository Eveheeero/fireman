//! ARM32 instruction statements (mnemonics)

use crate::{DisassembleError, StatementInner};

/// ARM32 instruction statements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Arm32Statement {
    // Data processing
    And,
    Eor,
    Sub,
    Rsb,
    Add,
    Adc,
    Sbc,
    Rsc,
    Tst,
    Teq,
    Cmp,
    Cmn,
    Orr,
    Mov,
    Bic,
    Mvn,

    // Multiply
    Mul,
    Mla,
    Umull,
    Umlal,
    Smull,
    Smlal,

    // Divide (ARMv7+)
    Sdiv,
    Udiv,

    // Saturating arithmetic
    Qadd,
    Qsub,
    Qdadd,
    Qdsub,

    // Load/Store
    Ldr,
    Ldrb,
    Ldrh,
    Ldrsb,
    Ldrsh,
    Ldrd,
    Str,
    Strb,
    Strh,
    Strd,

    // Load/Store multiple
    Ldm,
    Stm,
    Push,
    Pop,

    // Load/Store exclusive
    Ldrex,
    Strex,
    Ldrexb,
    Strexb,
    Ldrexh,
    Strexh,
    Ldrexd,
    Strexd,

    // Branch
    B,
    Bl,
    Bx,
    Blx,

    // Compare and branch (Thumb-2)
    Cbz,
    Cbnz,

    // IT block (Thumb-2)
    It,

    // Shift and rotate
    Lsl,
    Lsr,
    Asr,
    Ror,
    Rrx,

    // Bit field
    Bfi,
    Bfc,
    Sbfx,
    Ubfx,

    // Packing/unpacking
    Pkhbt,
    Pkhtb,
    Sxtb,
    Sxth,
    Uxtb,
    Uxth,
    Sxtb16,
    Uxtb16,

    // Parallel arithmetic
    Sadd16,
    Sadd8,
    Ssub16,
    Ssub8,
    Uadd16,
    Uadd8,
    Usub16,
    Usub8,

    // Count leading zeros
    Clz,

    // Byte reverse
    Rev,
    Rev16,
    Revsh,

    // Exclusive monitor
    Clrex,

    // System
    Svc,
    Bkpt,
    Nop,
    Yield,
    Wfe,
    Wfi,
    Sev,
    Dmb,
    Dsb,
    Isb,

    // Coprocessor
    Mrc,
    Mcr,
    Mrc2,
    Mcr2,
    Mcrr,
    Mrrc,
    Mcrr2,
    Mrrc2,
    Cdp,
    Cdp2,
    Ldc,
    Stc,
    Ldc2,
    Stc2,

    // VFP/NEON instructions
    Vadd,
    Vsub,
    Vmul,
    Vdiv,
    Vmov,
    Vmrs,
    Vmsr,
    Vcmp,
    Vcmpe,
    Vcvt,
    Vabs,
    Vneg,
    Vsqrt,
    Vld1,
    Vld2,
    Vld3,
    Vld4,
    Vst1,
    Vst2,
    Vst3,
    Vst4,

    // Miscellaneous
    Adr,
    Movw,
    Movt,
    Mrs,
    Msr,
    Cps,
    Setend,

    // Hint instructions
    Pld,
    Pli,
    Pldw,

    // Security extensions
    Smc,

    // Unknown
    Unknown,
}

impl StatementInner for Arm32Statement {
    fn is_jcc(&self) -> bool {
        false // ARM32 uses conditional execution, not conditional jumps
    }

    fn is_jmp(&self) -> bool {
        matches!(self, Self::B | Self::Bx | Self::Cbz | Self::Cbnz)
    }

    fn is_call(&self) -> bool {
        matches!(self, Self::Bl | Self::Blx)
    }

    fn is_ret(&self) -> bool {
        false // ARM32 uses BX LR or POP {PC} for returns
    }
}

impl Arm32Statement {
    /// Parse mnemonic string to statement
    pub fn parse(mnemonic: impl AsRef<str>) -> Result<Arm32Statement, DisassembleError> {
        let mnemonic = mnemonic.as_ref().to_lowercase();

        // Remove condition code suffix if present
        let base_mnemonic = if mnemonic.len() > 2 {
            // Check if last 2 chars are a condition code
            let suffix = &mnemonic[mnemonic.len() - 2..];
            match suffix {
                "eq" | "ne" | "cs" | "cc" | "mi" | "pl" | "vs" | "vc" | "hi" | "ls" | "ge"
                | "lt" | "gt" | "le" | "al" => &mnemonic[..mnemonic.len() - 2],
                _ => mnemonic.as_str(),
            }
        } else {
            mnemonic.as_str()
        };

        // Also check for 'S' suffix (update flags)
        let base_mnemonic = base_mnemonic.trim_end_matches('s');

        let statement = match base_mnemonic {
            // Data processing
            "and" => Self::And,
            "eor" => Self::Eor,
            "sub" => Self::Sub,
            "rsb" => Self::Rsb,
            "add" => Self::Add,
            "adc" => Self::Adc,
            "sbc" => Self::Sbc,
            "rsc" => Self::Rsc,
            "tst" => Self::Tst,
            "teq" => Self::Teq,
            "cmp" => Self::Cmp,
            "cmn" => Self::Cmn,
            "orr" => Self::Orr,
            "mov" => Self::Mov,
            "bic" => Self::Bic,
            "mvn" => Self::Mvn,

            // Multiply
            "mul" => Self::Mul,
            "mla" => Self::Mla,
            "umull" => Self::Umull,
            "umlal" => Self::Umlal,
            "smull" => Self::Smull,
            "smlal" => Self::Smlal,

            // Divide
            "sdiv" => Self::Sdiv,
            "udiv" => Self::Udiv,

            // Saturating
            "qadd" => Self::Qadd,
            "qsub" => Self::Qsub,
            "qdadd" => Self::Qdadd,
            "qdsub" => Self::Qdsub,

            // Load/Store
            "ldr" => Self::Ldr,
            "ldrb" => Self::Ldrb,
            "ldrh" => Self::Ldrh,
            "ldrsb" => Self::Ldrsb,
            "ldrsh" => Self::Ldrsh,
            "ldrd" => Self::Ldrd,
            "str" => Self::Str,
            "strb" => Self::Strb,
            "strh" => Self::Strh,
            "strd" => Self::Strd,

            // Load/Store multiple
            "ldm" | "ldmia" | "ldmib" | "ldmda" | "ldmdb" | "ldmfd" | "ldmfa" | "ldmed"
            | "ldmea" => Self::Ldm,
            "stm" | "stmia" | "stmib" | "stmda" | "stmdb" | "stmfd" | "stmfa" | "stmed"
            | "stmea" => Self::Stm,
            "push" => Self::Push,
            "pop" => Self::Pop,

            // Load/Store exclusive
            "ldrex" => Self::Ldrex,
            "strex" => Self::Strex,
            "ldrexb" => Self::Ldrexb,
            "strexb" => Self::Strexb,
            "ldrexh" => Self::Ldrexh,
            "strexh" => Self::Strexh,
            "ldrexd" => Self::Ldrexd,
            "strexd" => Self::Strexd,

            // Branch
            "b" => Self::B,
            "bl" => Self::Bl,
            "bx" => Self::Bx,
            "blx" => Self::Blx,

            // Compare and branch
            "cbz" => Self::Cbz,
            "cbnz" => Self::Cbnz,

            // IT
            "it" | "itt" | "ite" | "ittt" | "itte" | "itet" | "itee" | "itttt" | "ittte"
            | "ittet" | "ittee" | "itett" | "itete" | "iteet" | "iteee" => Self::It,

            // Shift
            "lsl" => Self::Lsl,
            "lsr" => Self::Lsr,
            "asr" => Self::Asr,
            "ror" => Self::Ror,
            "rrx" => Self::Rrx,

            // Bit field
            "bfi" => Self::Bfi,
            "bfc" => Self::Bfc,
            "sbfx" => Self::Sbfx,
            "ubfx" => Self::Ubfx,

            // Packing
            "pkhbt" => Self::Pkhbt,
            "pkhtb" => Self::Pkhtb,
            "sxtb" => Self::Sxtb,
            "sxth" => Self::Sxth,
            "uxtb" => Self::Uxtb,
            "uxth" => Self::Uxth,
            "sxtb16" => Self::Sxtb16,
            "uxtb16" => Self::Uxtb16,

            // Parallel
            "sadd16" => Self::Sadd16,
            "sadd8" => Self::Sadd8,
            "ssub16" => Self::Ssub16,
            "ssub8" => Self::Ssub8,
            "uadd16" => Self::Uadd16,
            "uadd8" => Self::Uadd8,
            "usub16" => Self::Usub16,
            "usub8" => Self::Usub8,

            // Misc
            "clz" => Self::Clz,
            "rev" => Self::Rev,
            "rev16" => Self::Rev16,
            "revsh" => Self::Revsh,
            "clrex" => Self::Clrex,

            // System
            "svc" | "swi" => Self::Svc,
            "bkpt" => Self::Bkpt,
            "nop" => Self::Nop,
            "yield" => Self::Yield,
            "wfe" => Self::Wfe,
            "wfi" => Self::Wfi,
            "sev" => Self::Sev,
            "dmb" => Self::Dmb,
            "dsb" => Self::Dsb,
            "isb" => Self::Isb,

            // Coprocessor
            "mrc" => Self::Mrc,
            "mcr" => Self::Mcr,
            "mrc2" => Self::Mrc2,
            "mcr2" => Self::Mcr2,
            "mcrr" => Self::Mcrr,
            "mrrc" => Self::Mrrc,
            "mcrr2" => Self::Mcrr2,
            "mrrc2" => Self::Mrrc2,
            "cdp" => Self::Cdp,
            "cdp2" => Self::Cdp2,
            "ldc" => Self::Ldc,
            "stc" => Self::Stc,
            "ldc2" => Self::Ldc2,
            "stc2" => Self::Stc2,

            // VFP/NEON
            "vadd" => Self::Vadd,
            "vsub" => Self::Vsub,
            "vmul" => Self::Vmul,
            "vdiv" => Self::Vdiv,
            "vmov" => Self::Vmov,
            "vmrs" => Self::Vmrs,
            "vmsr" => Self::Vmsr,
            "vcmp" => Self::Vcmp,
            "vcmpe" => Self::Vcmpe,
            "vcvt" => Self::Vcvt,
            "vabs" => Self::Vabs,
            "vneg" => Self::Vneg,
            "vsqrt" => Self::Vsqrt,
            "vld1" => Self::Vld1,
            "vld2" => Self::Vld2,
            "vld3" => Self::Vld3,
            "vld4" => Self::Vld4,
            "vst1" => Self::Vst1,
            "vst2" => Self::Vst2,
            "vst3" => Self::Vst3,
            "vst4" => Self::Vst4,

            // Misc
            "adr" => Self::Adr,
            "movw" => Self::Movw,
            "movt" => Self::Movt,
            "mrs" => Self::Mrs,
            "msr" => Self::Msr,
            "cps" => Self::Cps,
            "setend" => Self::Setend,

            // Hint
            "pld" => Self::Pld,
            "pli" => Self::Pli,
            "pldw" => Self::Pldw,

            // Security
            "smc" => Self::Smc,

            _ => return Err(DisassembleError::UnknownStatement),
        };

        Ok(statement)
    }

    /// Check if instruction is a load
    pub fn is_load(&self) -> bool {
        matches!(
            self,
            Self::Ldr
                | Self::Ldrb
                | Self::Ldrh
                | Self::Ldrsb
                | Self::Ldrsh
                | Self::Ldrd
                | Self::Ldm
                | Self::Pop
                | Self::Ldrex
                | Self::Ldrexb
                | Self::Ldrexh
                | Self::Ldrexd
                | Self::Ldc
                | Self::Ldc2
                | Self::Vld1
                | Self::Vld2
                | Self::Vld3
                | Self::Vld4
        )
    }

    /// Check if instruction is a store
    pub fn is_store(&self) -> bool {
        matches!(
            self,
            Self::Str
                | Self::Strb
                | Self::Strh
                | Self::Strd
                | Self::Stm
                | Self::Push
                | Self::Strex
                | Self::Strexb
                | Self::Strexh
                | Self::Strexd
                | Self::Stc
                | Self::Stc2
                | Self::Vst1
                | Self::Vst2
                | Self::Vst3
                | Self::Vst4
        )
    }

    /// Check if instruction is arithmetic
    pub fn is_arithmetic(&self) -> bool {
        matches!(
            self,
            Self::Add
                | Self::Adc
                | Self::Sub
                | Self::Sbc
                | Self::Rsb
                | Self::Rsc
                | Self::Mul
                | Self::Mla
                | Self::Umull
                | Self::Umlal
                | Self::Smull
                | Self::Smlal
                | Self::Sdiv
                | Self::Udiv
                | Self::Qadd
                | Self::Qsub
                | Self::Qdadd
                | Self::Qdsub
                | Self::Vadd
                | Self::Vsub
                | Self::Vmul
                | Self::Vdiv
        )
    }

    /// Check if instruction is logical
    pub fn is_logical(&self) -> bool {
        matches!(
            self,
            Self::And | Self::Eor | Self::Orr | Self::Bic | Self::Mvn
        )
    }

    /// Check if instruction modifies flags
    pub fn modifies_flags(&self) -> bool {
        matches!(
            self,
            Self::Tst | Self::Teq | Self::Cmp | Self::Cmn | Self::Vcmp | Self::Vcmpe
        )
    }
}

impl std::fmt::Display for Arm32Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::And => "and",
            Self::Eor => "eor",
            Self::Sub => "sub",
            Self::Rsb => "rsb",
            Self::Add => "add",
            Self::Adc => "adc",
            Self::Sbc => "sbc",
            Self::Rsc => "rsc",
            Self::Tst => "tst",
            Self::Teq => "teq",
            Self::Cmp => "cmp",
            Self::Cmn => "cmn",
            Self::Orr => "orr",
            Self::Mov => "mov",
            Self::Bic => "bic",
            Self::Mvn => "mvn",
            Self::Mul => "mul",
            Self::Mla => "mla",
            Self::Umull => "umull",
            Self::Umlal => "umlal",
            Self::Smull => "smull",
            Self::Smlal => "smlal",
            Self::Sdiv => "sdiv",
            Self::Udiv => "udiv",
            Self::Qadd => "qadd",
            Self::Qsub => "qsub",
            Self::Qdadd => "qdadd",
            Self::Qdsub => "qdsub",
            Self::Ldr => "ldr",
            Self::Ldrb => "ldrb",
            Self::Ldrh => "ldrh",
            Self::Ldrsb => "ldrsb",
            Self::Ldrsh => "ldrsh",
            Self::Ldrd => "ldrd",
            Self::Str => "str",
            Self::Strb => "strb",
            Self::Strh => "strh",
            Self::Strd => "strd",
            Self::Ldm => "ldm",
            Self::Stm => "stm",
            Self::Push => "push",
            Self::Pop => "pop",
            Self::Ldrex => "ldrex",
            Self::Strex => "strex",
            Self::Ldrexb => "ldrexb",
            Self::Strexb => "strexb",
            Self::Ldrexh => "ldrexh",
            Self::Strexh => "strexh",
            Self::Ldrexd => "ldrexd",
            Self::Strexd => "strexd",
            Self::B => "b",
            Self::Bl => "bl",
            Self::Bx => "bx",
            Self::Blx => "blx",
            Self::Cbz => "cbz",
            Self::Cbnz => "cbnz",
            Self::It => "it",
            Self::Lsl => "lsl",
            Self::Lsr => "lsr",
            Self::Asr => "asr",
            Self::Ror => "ror",
            Self::Rrx => "rrx",
            Self::Bfi => "bfi",
            Self::Bfc => "bfc",
            Self::Sbfx => "sbfx",
            Self::Ubfx => "ubfx",
            Self::Pkhbt => "pkhbt",
            Self::Pkhtb => "pkhtb",
            Self::Sxtb => "sxtb",
            Self::Sxth => "sxth",
            Self::Uxtb => "uxtb",
            Self::Uxth => "uxth",
            Self::Sxtb16 => "sxtb16",
            Self::Uxtb16 => "uxtb16",
            Self::Sadd16 => "sadd16",
            Self::Sadd8 => "sadd8",
            Self::Ssub16 => "ssub16",
            Self::Ssub8 => "ssub8",
            Self::Uadd16 => "uadd16",
            Self::Uadd8 => "uadd8",
            Self::Usub16 => "usub16",
            Self::Usub8 => "usub8",
            Self::Clz => "clz",
            Self::Rev => "rev",
            Self::Rev16 => "rev16",
            Self::Revsh => "revsh",
            Self::Clrex => "clrex",
            Self::Svc => "svc",
            Self::Bkpt => "bkpt",
            Self::Nop => "nop",
            Self::Yield => "yield",
            Self::Wfe => "wfe",
            Self::Wfi => "wfi",
            Self::Sev => "sev",
            Self::Dmb => "dmb",
            Self::Dsb => "dsb",
            Self::Isb => "isb",
            Self::Mrc => "mrc",
            Self::Mcr => "mcr",
            Self::Mrc2 => "mrc2",
            Self::Mcr2 => "mcr2",
            Self::Mcrr => "mcrr",
            Self::Mrrc => "mrrc",
            Self::Mcrr2 => "mcrr2",
            Self::Mrrc2 => "mrrc2",
            Self::Cdp => "cdp",
            Self::Cdp2 => "cdp2",
            Self::Ldc => "ldc",
            Self::Stc => "stc",
            Self::Ldc2 => "ldc2",
            Self::Stc2 => "stc2",
            Self::Vadd => "vadd",
            Self::Vsub => "vsub",
            Self::Vmul => "vmul",
            Self::Vdiv => "vdiv",
            Self::Vmov => "vmov",
            Self::Vmrs => "vmrs",
            Self::Vmsr => "vmsr",
            Self::Vcmp => "vcmp",
            Self::Vcmpe => "vcmpe",
            Self::Vcvt => "vcvt",
            Self::Vabs => "vabs",
            Self::Vneg => "vneg",
            Self::Vsqrt => "vsqrt",
            Self::Vld1 => "vld1",
            Self::Vld2 => "vld2",
            Self::Vld3 => "vld3",
            Self::Vld4 => "vld4",
            Self::Vst1 => "vst1",
            Self::Vst2 => "vst2",
            Self::Vst3 => "vst3",
            Self::Vst4 => "vst4",
            Self::Adr => "adr",
            Self::Movw => "movw",
            Self::Movt => "movt",
            Self::Mrs => "mrs",
            Self::Msr => "msr",
            Self::Cps => "cps",
            Self::Setend => "setend",
            Self::Pld => "pld",
            Self::Pli => "pli",
            Self::Pldw => "pldw",
            Self::Smc => "smc",
            Self::Unknown => "unknown",
        };
        write!(f, "{}", s)
    }
}
