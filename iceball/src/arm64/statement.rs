//! ARM64 (AArch64) instruction statements

use crate::DisassembleError;
use std::str::FromStr;

/// ARM64 instruction mnemonics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Arm64Statement {
    // Data processing - immediate
    Add,
    Adds,
    Sub,
    Subs,
    Cmp,
    Cmn,
    Mov,
    Movz,
    Movn,
    Movk,

    // Logical - immediate
    And,
    Ands,
    Orr,
    Eor,
    Tst,

    // Data processing - register
    Adc,
    Adcs,
    Sbc,
    Sbcs,

    // Logical - shifted register
    Bic,
    Bics,
    Orn,
    Eon,
    Mvn,

    // Shift and rotate
    Lsl,
    Lsr,
    Asr,
    Ror,

    // Bit manipulation
    Sbfm,
    Bfm,
    Ubfm,
    Extr,

    // Conditional select
    Csel,
    Csinc,
    Csinv,
    Csneg,
    Cset,
    Csetm,

    // Multiply and divide
    Mul,
    Madd,
    Msub,
    Mneg,
    Smull,
    Smulh,
    Umull,
    Umulh,
    Udiv,
    Sdiv,

    // Load/Store
    Ldr,
    Ldrb,
    Ldrh,
    Ldrsb,
    Ldrsh,
    Ldrsw,
    Str,
    Strb,
    Strh,

    // Load/Store pair
    Ldp,
    Stp,
    Ldpsw,

    // Load/Store exclusive
    Ldxr,
    Ldxrb,
    Ldxrh,
    Stxr,
    Stxrb,
    Stxrh,
    Ldaxr,
    Ldaxrb,
    Ldaxrh,
    Stlxr,
    Stlxrb,
    Stlxrh,

    // Load-Acquire/Store-Release
    Ldar,
    Ldarb,
    Ldarh,
    Stlr,
    Stlrb,
    Stlrh,

    // Branch
    B,
    Bl,
    Br,
    Blr,
    Ret,

    // Conditional branch
    Beq,
    Bne,
    Bcs, // B.HS
    Bcc, // B.LO
    Bmi,
    Bpl,
    Bvs,
    Bvc,
    Bhi,
    Bls,
    Bge,
    Blt,
    Bgt,
    Ble,

    // Compare and branch
    Cbz,
    Cbnz,
    Tbz,
    Tbnz,

    // System
    Nop,
    Svc,
    Hvc,
    Smc,
    Brk,
    Hlt,
    Dcps1,
    Dcps2,
    Dcps3,

    // System register
    Mrs,
    Msr,

    // Barriers
    Dmb,
    Dsb,
    Isb,

    // Cache maintenance
    Dc,
    Ic,

    // Address generation
    Adr,
    Adrp,

    // SIMD/FP data processing
    Fadd,
    Fsub,
    Fmul,
    Fdiv,
    Fneg,
    Fabs,
    Fsqrt,
    Fcmp,
    Fcmpe,
    Fmov,
    Fcvt,
    Fcvtzs,
    Fcvtzu,
    Scvtf,
    Ucvtf,

    // SIMD load/store
    Ld1,
    Ld2,
    Ld3,
    Ld4,
    St1,
    St2,
    St3,
    St4,

    // Advanced SIMD
    Dup,
    Ins,
    Umov,
    Smov,

    // Crypto extensions
    Aese,
    Aesd,
    Aesmc,
    Aesimc,
}

impl Arm64Statement {
    /// Parse ARM64 statement from string
    pub fn parse(s: &str) -> Result<Self, DisassembleError> {
        Self::from_str(s)
    }

    /// Convert to string
    pub fn to_string(&self) -> &'static str {
        match self {
            // Data processing - immediate
            Self::Add => "add",
            Self::Adds => "adds",
            Self::Sub => "sub",
            Self::Subs => "subs",
            Self::Cmp => "cmp",
            Self::Cmn => "cmn",
            Self::Mov => "mov",
            Self::Movz => "movz",
            Self::Movn => "movn",
            Self::Movk => "movk",

            // Logical - immediate
            Self::And => "and",
            Self::Ands => "ands",
            Self::Orr => "orr",
            Self::Eor => "eor",
            Self::Tst => "tst",

            // Data processing - register
            Self::Adc => "adc",
            Self::Adcs => "adcs",
            Self::Sbc => "sbc",
            Self::Sbcs => "sbcs",

            // Logical - shifted register
            Self::Bic => "bic",
            Self::Bics => "bics",
            Self::Orn => "orn",
            Self::Eon => "eon",
            Self::Mvn => "mvn",

            // Shift and rotate
            Self::Lsl => "lsl",
            Self::Lsr => "lsr",
            Self::Asr => "asr",
            Self::Ror => "ror",

            // Bit manipulation
            Self::Sbfm => "sbfm",
            Self::Bfm => "bfm",
            Self::Ubfm => "ubfm",
            Self::Extr => "extr",

            // Conditional select
            Self::Csel => "csel",
            Self::Csinc => "csinc",
            Self::Csinv => "csinv",
            Self::Csneg => "csneg",
            Self::Cset => "cset",
            Self::Csetm => "csetm",

            // Multiply and divide
            Self::Mul => "mul",
            Self::Madd => "madd",
            Self::Msub => "msub",
            Self::Mneg => "mneg",
            Self::Smull => "smull",
            Self::Smulh => "smulh",
            Self::Umull => "umull",
            Self::Umulh => "umulh",
            Self::Udiv => "udiv",
            Self::Sdiv => "sdiv",

            // Load/Store
            Self::Ldr => "ldr",
            Self::Ldrb => "ldrb",
            Self::Ldrh => "ldrh",
            Self::Ldrsb => "ldrsb",
            Self::Ldrsh => "ldrsh",
            Self::Ldrsw => "ldrsw",
            Self::Str => "str",
            Self::Strb => "strb",
            Self::Strh => "strh",

            // Load/Store pair
            Self::Ldp => "ldp",
            Self::Stp => "stp",
            Self::Ldpsw => "ldpsw",

            // Load/Store exclusive
            Self::Ldxr => "ldxr",
            Self::Ldxrb => "ldxrb",
            Self::Ldxrh => "ldxrh",
            Self::Stxr => "stxr",
            Self::Stxrb => "stxrb",
            Self::Stxrh => "stxrh",
            Self::Ldaxr => "ldaxr",
            Self::Ldaxrb => "ldaxrb",
            Self::Ldaxrh => "ldaxrh",
            Self::Stlxr => "stlxr",
            Self::Stlxrb => "stlxrb",
            Self::Stlxrh => "stlxrh",

            // Load-Acquire/Store-Release
            Self::Ldar => "ldar",
            Self::Ldarb => "ldarb",
            Self::Ldarh => "ldarh",
            Self::Stlr => "stlr",
            Self::Stlrb => "stlrb",
            Self::Stlrh => "stlrh",

            // Branch
            Self::B => "b",
            Self::Bl => "bl",
            Self::Br => "br",
            Self::Blr => "blr",
            Self::Ret => "ret",

            // Conditional branch
            Self::Beq => "b.eq",
            Self::Bne => "b.ne",
            Self::Bcs => "b.cs",
            Self::Bcc => "b.cc",
            Self::Bmi => "b.mi",
            Self::Bpl => "b.pl",
            Self::Bvs => "b.vs",
            Self::Bvc => "b.vc",
            Self::Bhi => "b.hi",
            Self::Bls => "b.ls",
            Self::Bge => "b.ge",
            Self::Blt => "b.lt",
            Self::Bgt => "b.gt",
            Self::Ble => "b.le",

            // Compare and branch
            Self::Cbz => "cbz",
            Self::Cbnz => "cbnz",
            Self::Tbz => "tbz",
            Self::Tbnz => "tbnz",

            // System
            Self::Nop => "nop",
            Self::Svc => "svc",
            Self::Hvc => "hvc",
            Self::Smc => "smc",
            Self::Brk => "brk",
            Self::Hlt => "hlt",
            Self::Dcps1 => "dcps1",
            Self::Dcps2 => "dcps2",
            Self::Dcps3 => "dcps3",

            // System register
            Self::Mrs => "mrs",
            Self::Msr => "msr",

            // Barriers
            Self::Dmb => "dmb",
            Self::Dsb => "dsb",
            Self::Isb => "isb",

            // Cache maintenance
            Self::Dc => "dc",
            Self::Ic => "ic",

            // Address generation
            Self::Adr => "adr",
            Self::Adrp => "adrp",

            // SIMD/FP data processing
            Self::Fadd => "fadd",
            Self::Fsub => "fsub",
            Self::Fmul => "fmul",
            Self::Fdiv => "fdiv",
            Self::Fneg => "fneg",
            Self::Fabs => "fabs",
            Self::Fsqrt => "fsqrt",
            Self::Fcmp => "fcmp",
            Self::Fcmpe => "fcmpe",
            Self::Fmov => "fmov",
            Self::Fcvt => "fcvt",
            Self::Fcvtzs => "fcvtzs",
            Self::Fcvtzu => "fcvtzu",
            Self::Scvtf => "scvtf",
            Self::Ucvtf => "ucvtf",

            // SIMD load/store
            Self::Ld1 => "ld1",
            Self::Ld2 => "ld2",
            Self::Ld3 => "ld3",
            Self::Ld4 => "ld4",
            Self::St1 => "st1",
            Self::St2 => "st2",
            Self::St3 => "st3",
            Self::St4 => "st4",

            // Advanced SIMD
            Self::Dup => "dup",
            Self::Ins => "ins",
            Self::Umov => "umov",
            Self::Smov => "smov",

            // Crypto extensions
            Self::Aese => "aese",
            Self::Aesd => "aesd",
            Self::Aesmc => "aesmc",
            Self::Aesimc => "aesimc",
        }
    }
}

impl FromStr for Arm64Statement {
    type Err = DisassembleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();

        match s.as_str() {
            // Data processing - immediate
            "add" => Ok(Self::Add),
            "adds" => Ok(Self::Adds),
            "sub" => Ok(Self::Sub),
            "subs" => Ok(Self::Subs),
            "cmp" => Ok(Self::Cmp),
            "cmn" => Ok(Self::Cmn),
            "mov" => Ok(Self::Mov),
            "movz" => Ok(Self::Movz),
            "movn" => Ok(Self::Movn),
            "movk" => Ok(Self::Movk),

            // Logical - immediate
            "and" => Ok(Self::And),
            "ands" => Ok(Self::Ands),
            "orr" => Ok(Self::Orr),
            "eor" => Ok(Self::Eor),
            "tst" => Ok(Self::Tst),

            // Data processing - register
            "adc" => Ok(Self::Adc),
            "adcs" => Ok(Self::Adcs),
            "sbc" => Ok(Self::Sbc),
            "sbcs" => Ok(Self::Sbcs),

            // Logical - shifted register
            "bic" => Ok(Self::Bic),
            "bics" => Ok(Self::Bics),
            "orn" => Ok(Self::Orn),
            "eon" => Ok(Self::Eon),
            "mvn" => Ok(Self::Mvn),

            // Shift and rotate
            "lsl" => Ok(Self::Lsl),
            "lsr" => Ok(Self::Lsr),
            "asr" => Ok(Self::Asr),
            "ror" => Ok(Self::Ror),

            // Bit manipulation
            "sbfm" => Ok(Self::Sbfm),
            "bfm" => Ok(Self::Bfm),
            "ubfm" => Ok(Self::Ubfm),
            "extr" => Ok(Self::Extr),

            // Conditional select
            "csel" => Ok(Self::Csel),
            "csinc" => Ok(Self::Csinc),
            "csinv" => Ok(Self::Csinv),
            "csneg" => Ok(Self::Csneg),
            "cset" => Ok(Self::Cset),
            "csetm" => Ok(Self::Csetm),

            // Multiply and divide
            "mul" => Ok(Self::Mul),
            "madd" => Ok(Self::Madd),
            "msub" => Ok(Self::Msub),
            "mneg" => Ok(Self::Mneg),
            "smull" => Ok(Self::Smull),
            "smulh" => Ok(Self::Smulh),
            "umull" => Ok(Self::Umull),
            "umulh" => Ok(Self::Umulh),
            "udiv" => Ok(Self::Udiv),
            "sdiv" => Ok(Self::Sdiv),

            // Load/Store
            "ldr" => Ok(Self::Ldr),
            "ldrb" => Ok(Self::Ldrb),
            "ldrh" => Ok(Self::Ldrh),
            "ldrsb" => Ok(Self::Ldrsb),
            "ldrsh" => Ok(Self::Ldrsh),
            "ldrsw" => Ok(Self::Ldrsw),
            "str" => Ok(Self::Str),
            "strb" => Ok(Self::Strb),
            "strh" => Ok(Self::Strh),

            // Load/Store pair
            "ldp" => Ok(Self::Ldp),
            "stp" => Ok(Self::Stp),
            "ldpsw" => Ok(Self::Ldpsw),

            // Load/Store exclusive
            "ldxr" => Ok(Self::Ldxr),
            "ldxrb" => Ok(Self::Ldxrb),
            "ldxrh" => Ok(Self::Ldxrh),
            "stxr" => Ok(Self::Stxr),
            "stxrb" => Ok(Self::Stxrb),
            "stxrh" => Ok(Self::Stxrh),
            "ldaxr" => Ok(Self::Ldaxr),
            "ldaxrb" => Ok(Self::Ldaxrb),
            "ldaxrh" => Ok(Self::Ldaxrh),
            "stlxr" => Ok(Self::Stlxr),
            "stlxrb" => Ok(Self::Stlxrb),
            "stlxrh" => Ok(Self::Stlxrh),

            // Load-Acquire/Store-Release
            "ldar" => Ok(Self::Ldar),
            "ldarb" => Ok(Self::Ldarb),
            "ldarh" => Ok(Self::Ldarh),
            "stlr" => Ok(Self::Stlr),
            "stlrb" => Ok(Self::Stlrb),
            "stlrh" => Ok(Self::Stlrh),

            // Branch
            "b" => Ok(Self::B),
            "bl" => Ok(Self::Bl),
            "br" => Ok(Self::Br),
            "blr" => Ok(Self::Blr),
            "ret" => Ok(Self::Ret),

            // Conditional branch
            "b.eq" | "beq" => Ok(Self::Beq),
            "b.ne" | "bne" => Ok(Self::Bne),
            "b.cs" | "b.hs" | "bcs" => Ok(Self::Bcs),
            "b.cc" | "b.lo" | "bcc" => Ok(Self::Bcc),
            "b.mi" | "bmi" => Ok(Self::Bmi),
            "b.pl" | "bpl" => Ok(Self::Bpl),
            "b.vs" | "bvs" => Ok(Self::Bvs),
            "b.vc" | "bvc" => Ok(Self::Bvc),
            "b.hi" | "bhi" => Ok(Self::Bhi),
            "b.ls" | "bls" => Ok(Self::Bls),
            "b.ge" | "bge" => Ok(Self::Bge),
            "b.lt" | "blt" => Ok(Self::Blt),
            "b.gt" | "bgt" => Ok(Self::Bgt),
            "b.le" | "ble" => Ok(Self::Ble),

            // Compare and branch
            "cbz" => Ok(Self::Cbz),
            "cbnz" => Ok(Self::Cbnz),
            "tbz" => Ok(Self::Tbz),
            "tbnz" => Ok(Self::Tbnz),

            // System
            "nop" => Ok(Self::Nop),
            "svc" => Ok(Self::Svc),
            "hvc" => Ok(Self::Hvc),
            "smc" => Ok(Self::Smc),
            "brk" => Ok(Self::Brk),
            "hlt" => Ok(Self::Hlt),
            "dcps1" => Ok(Self::Dcps1),
            "dcps2" => Ok(Self::Dcps2),
            "dcps3" => Ok(Self::Dcps3),

            // System register
            "mrs" => Ok(Self::Mrs),
            "msr" => Ok(Self::Msr),

            // Barriers
            "dmb" => Ok(Self::Dmb),
            "dsb" => Ok(Self::Dsb),
            "isb" => Ok(Self::Isb),

            // Cache maintenance
            "dc" => Ok(Self::Dc),
            "ic" => Ok(Self::Ic),

            // Address generation
            "adr" => Ok(Self::Adr),
            "adrp" => Ok(Self::Adrp),

            // SIMD/FP data processing
            "fadd" => Ok(Self::Fadd),
            "fsub" => Ok(Self::Fsub),
            "fmul" => Ok(Self::Fmul),
            "fdiv" => Ok(Self::Fdiv),
            "fneg" => Ok(Self::Fneg),
            "fabs" => Ok(Self::Fabs),
            "fsqrt" => Ok(Self::Fsqrt),
            "fcmp" => Ok(Self::Fcmp),
            "fcmpe" => Ok(Self::Fcmpe),
            "fmov" => Ok(Self::Fmov),
            "fcvt" => Ok(Self::Fcvt),
            "fcvtzs" => Ok(Self::Fcvtzs),
            "fcvtzu" => Ok(Self::Fcvtzu),
            "scvtf" => Ok(Self::Scvtf),
            "ucvtf" => Ok(Self::Ucvtf),

            // SIMD load/store
            "ld1" => Ok(Self::Ld1),
            "ld2" => Ok(Self::Ld2),
            "ld3" => Ok(Self::Ld3),
            "ld4" => Ok(Self::Ld4),
            "st1" => Ok(Self::St1),
            "st2" => Ok(Self::St2),
            "st3" => Ok(Self::St3),
            "st4" => Ok(Self::St4),

            // Advanced SIMD
            "dup" => Ok(Self::Dup),
            "ins" => Ok(Self::Ins),
            "umov" => Ok(Self::Umov),
            "smov" => Ok(Self::Smov),

            // Crypto extensions
            "aese" => Ok(Self::Aese),
            "aesd" => Ok(Self::Aesd),
            "aesmc" => Ok(Self::Aesmc),
            "aesimc" => Ok(Self::Aesimc),

            _ => Err(DisassembleError::UnknownStatement),
        }
    }
}

impl crate::StatementInner for Arm64Statement {
    fn is_jcc(&self) -> bool {
        matches!(
            self,
            Self::Beq
                | Self::Bne
                | Self::Bcs
                | Self::Bcc
                | Self::Bmi
                | Self::Bpl
                | Self::Bvs
                | Self::Bvc
                | Self::Bhi
                | Self::Bls
                | Self::Bge
                | Self::Blt
                | Self::Bgt
                | Self::Ble
                | Self::Cbz
                | Self::Cbnz
                | Self::Tbz
                | Self::Tbnz
        )
    }

    fn is_jmp(&self) -> bool {
        matches!(self, Self::B | Self::Br)
    }

    fn is_call(&self) -> bool {
        matches!(self, Self::Bl | Self::Blr)
    }

    fn is_ret(&self) -> bool {
        matches!(self, Self::Ret)
    }
}
