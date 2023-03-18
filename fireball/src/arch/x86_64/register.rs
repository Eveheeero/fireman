use std::fmt::{Display, Formatter};

pub enum Register {
    Ax(Ax),
    Bx(Bx),
    Cx(Cx),
    Dx(Dx),
    Si(Si),
    Di(Di),
    Sp(Sp),
    Bp(Bp),
    Ip(Ip),
    R8(R8),
    R9(R9),
    R10(R10),
    R11(R11),
    R12(R12),
    R13(R13),
    R14(R14),
    R15(R15),
}

impl Register {
    #[inline]
    pub fn str(&self) -> &'static str {
        match self {
            Register::Ax(ax) => match ax {
                Ax::Rax => "rax",
                Ax::Eax => "eax",
                Ax::Ax => "ax",
                Ax::Al => "al",
                Ax::Ah => "ah",
            },
            Register::Bx(bx) => match bx {
                Bx::Rbx => "rbx",
                Bx::Ebx => "ebx",
                Bx::Bx => "bx",
                Bx::Bl => "bl",
                Bx::Bh => "bh",
            },
            Register::Cx(cx) => match cx {
                Cx::Rcx => "rcx",
                Cx::Ecx => "ecx",
                Cx::Cx => "cx",
                Cx::Cl => "cl",
                Cx::Ch => "ch",
            },
            Register::Dx(dx) => match dx {
                Dx::Rdx => "rdx",
                Dx::Edx => "edx",
                Dx::Dx => "dx",
                Dx::Dl => "dl",
                Dx::Dh => "dh",
            },
            Register::Si(si) => match si {
                Si::Rsi => "rsi",
                Si::Esi => "esi",
                Si::Si => "si",
                Si::Sil => "sil",
            },
            Register::Di(di) => match di {
                Di::Rdi => "rdi",
                Di::Edi => "edi",
                Di::Di => "di",
                Di::Dil => "dil",
            },
            Register::Sp(sp) => match sp {
                Sp::Rsp => "rsp",
                Sp::Esp => "esp",
                Sp::Sp => "sp",
                Sp::Spl => "spl",
            },
            Register::Bp(bp) => match bp {
                Bp::Rbp => "rbp",
                Bp::Ebp => "ebp",
                Bp::Bp => "bp",
                Bp::Bpl => "bpl",
            },
            Register::Ip(ip) => match ip {
                Ip::Rip => "rip",
                Ip::Eip => "eip",
                Ip::Ip => "ip",
                Ip::Ipl => "ipl",
            },
            Register::R8(r8) => match r8 {
                R8::R8 => "r8",
                R8::R8d => "r8d",
                R8::R8w => "r8w",
                R8::R8b => "r8b",
            },
            Register::R9(r9) => match r9 {
                R9::R9 => "r9",
                R9::R9d => "r9d",
                R9::R9w => "r9w",
                R9::R9b => "r9b",
            },
            Register::R10(r10) => match r10 {
                R10::R10 => "r10",
                R10::R10d => "r10d",
                R10::R10w => "r10w",
                R10::R10b => "r10b",
            },
            Register::R11(r11) => match r11 {
                R11::R11 => "r11",
                R11::R11d => "r11d",
                R11::R11w => "r11w",
                R11::R11b => "r11b",
            },
            Register::R12(r12) => match r12 {
                R12::R12 => "r12",
                R12::R12d => "r12d",
                R12::R12w => "r12w",
                R12::R12b => "r12b",
            },
            Register::R13(r13) => match r13 {
                R13::R13 => "r13",
                R13::R13d => "r13d",
                R13::R13w => "r13w",
                R13::R13b => "r13b",
            },
            Register::R14(r14) => match r14 {
                R14::R14 => "r14",
                R14::R14d => "r14d",
                R14::R14w => "r14w",
                R14::R14b => "r14b",
            },
            Register::R15(r15) => match r15 {
                R15::R15 => "r15",
                R15::R15d => "r15d",
                R15::R15w => "r15w",
                R15::R15b => "r15b",
            },
        }
    }
}

impl Into<&str> for Register {
    fn into(self) -> &'static str {
        self.str()
    }
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value {
            "rax" => Register::Ax(Ax::Rax),
            "eax" => Register::Ax(Ax::Eax),
            "ax" => Register::Ax(Ax::Ax),
            "al" => Register::Ax(Ax::Al),
            "ah" => Register::Ax(Ax::Ah),

            "rbx" => Register::Bx(Bx::Rbx),
            "ebx" => Register::Bx(Bx::Ebx),
            "bx" => Register::Bx(Bx::Bx),
            "bl" => Register::Bx(Bx::Bl),
            "bh" => Register::Bx(Bx::Bh),

            "rcx" => Register::Cx(Cx::Rcx),
            "ecx" => Register::Cx(Cx::Ecx),
            "cx" => Register::Cx(Cx::Cx),
            "cl" => Register::Cx(Cx::Cl),
            "ch" => Register::Cx(Cx::Ch),

            "rdx" => Register::Dx(Dx::Rdx),
            "edx" => Register::Dx(Dx::Edx),
            "dx" => Register::Dx(Dx::Dx),
            "dl" => Register::Dx(Dx::Dl),
            "dh" => Register::Dx(Dx::Dh),

            "rsi" => Register::Si(Si::Rsi),
            "esi" => Register::Si(Si::Esi),
            "si" => Register::Si(Si::Si),
            "sil" => Register::Si(Si::Sil),

            "rdi" => Register::Di(Di::Rdi),
            "edi" => Register::Di(Di::Edi),
            "di" => Register::Di(Di::Di),
            "dil" => Register::Di(Di::Dil),

            "rsp" => Register::Sp(Sp::Rsp),
            "esp" => Register::Sp(Sp::Esp),
            "sp" => Register::Sp(Sp::Sp),
            "spl" => Register::Sp(Sp::Spl),

            "rbp" => Register::Bp(Bp::Rbp),
            "ebp" => Register::Bp(Bp::Ebp),
            "bp" => Register::Bp(Bp::Bp),
            "bpl" => Register::Bp(Bp::Bpl),

            "rip" => Register::Ip(Ip::Rip),
            "eip" => Register::Ip(Ip::Eip),
            "ip" => Register::Ip(Ip::Ip),
            "ipl" => Register::Ip(Ip::Ipl),

            "r8" => Register::R8(R8::R8),
            "r8d" => Register::R8(R8::R8d),
            "r8w" => Register::R8(R8::R8w),
            "r8b" => Register::R8(R8::R8b),

            "r9" => Register::R9(R9::R9),
            "r9d" => Register::R9(R9::R9d),
            "r9w" => Register::R9(R9::R9w),
            "r9b" => Register::R9(R9::R9b),

            "r10" => Register::R10(R10::R10),
            "r10d" => Register::R10(R10::R10d),
            "r10w" => Register::R10(R10::R10w),
            "r10b" => Register::R10(R10::R10b),

            "r11" => Register::R11(R11::R11),
            "r11d" => Register::R11(R11::R11d),
            "r11w" => Register::R11(R11::R11w),
            "r11b" => Register::R11(R11::R11b),

            "r12" => Register::R12(R12::R12),
            "r12d" => Register::R12(R12::R12d),
            "r12w" => Register::R12(R12::R12w),
            "r12b" => Register::R12(R12::R12b),

            "r13" => Register::R13(R13::R13),
            "r13d" => Register::R13(R13::R13d),
            "r13w" => Register::R13(R13::R13w),
            "r13b" => Register::R13(R13::R13b),

            "r14" => Register::R14(R14::R14),
            "r14d" => Register::R14(R14::R14d),
            "r14w" => Register::R14(R14::R14w),
            "r14b" => Register::R14(R14::R14b),

            "r15" => Register::R15(R15::R15),
            "r15d" => Register::R15(R15::R15d),
            "r15w" => Register::R15(R15::R15w),
            "r15b" => Register::R15(R15::R15b),

            _ => unreachable!("Invalid Register"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

pub enum Ax {
    Rax,
    Eax,
    Ax,
    Al,
    Ah,
}
pub enum Bx {
    Rbx,
    Ebx,
    Bx,
    Bl,
    Bh,
}

pub enum Cx {
    Rcx,
    Ecx,
    Cx,
    Cl,
    Ch,
}

pub enum Dx {
    Rdx,
    Edx,
    Dx,
    Dl,
    Dh,
}

pub enum Si {
    Rsi,
    Esi,
    Si,
    Sil,
}

pub enum Di {
    Rdi,
    Edi,
    Di,
    Dil,
}

pub enum Sp {
    Rsp,
    Esp,
    Sp,
    Spl,
}

pub enum Bp {
    Rbp,
    Ebp,
    Bp,
    Bpl,
}

pub enum Ip {
    Rip,
    Eip,
    Ip,
    Ipl,
}

pub enum R8 {
    R8,
    R8d,
    R8w,
    R8b,
}

pub enum R9 {
    R9,
    R9d,
    R9w,
    R9b,
}

pub enum R10 {
    R10,
    R10d,
    R10w,
    R10b,
}

pub enum R11 {
    R11,
    R11d,
    R11w,
    R11b,
}

pub enum R12 {
    R12,
    R12d,
    R12w,
    R12b,
}

pub enum R13 {
    R13,
    R13d,
    R13w,
    R13b,
}

pub enum R14 {
    R14,
    R14d,
    R14w,
    R14b,
}

pub enum R15 {
    R15,
    R15d,
    R15w,
    R15b,
}
