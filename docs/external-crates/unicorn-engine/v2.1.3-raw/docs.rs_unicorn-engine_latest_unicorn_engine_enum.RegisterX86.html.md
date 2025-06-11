---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html"
title: "RegisterX86 in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.RegisterX86.html "Get a link to this specific version")
- [Docs.rs crate page](https://docs.rs/crate/unicorn-engine/latest "See unicorn-engine in docs.rs")
- GPL-2.0

- Links
- [Documentation](https://github.com/unicorn-engine/unicorn/wiki "Canonical documentation")
- [Repository](https://github.com/unicorn-engine/unicorn)
- [crates.io](https://crates.io/crates/unicorn-engine "See unicorn-engine in crates.io")
- [Source](https://docs.rs/crate/unicorn-engine/latest/source/ "Browse source of unicorn-engine-2.1.3")

- Owners
- [wtdcode](https://crates.io/users/wtdcode)

- Dependencies
- - [bitflags ^2.3.3\\
     \\
     _normal_](https://docs.rs/bitflags/^2.3.3)
- [libc ^0.2\\
\\
_normal_](https://docs.rs/libc/^0.2)
- [cc ^1.0\\
\\
_build_](https://docs.rs/cc/^1.0)
- [cmake ^0.1\\
\\
_build_](https://docs.rs/cmake/^0.1)
- [pkg-config ^0.3\\
\\
_build_](https://docs.rs/pkg-config/^0.3)

- Versions

- [**6.05%**\\
of the crate is documented](https://docs.rs/crate/unicorn-engine/latest)

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.RegisterX86.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum RegisterX86Copy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#7-248)

```

#[repr(C)]pub enum RegisterX86 {
Show 240 variants    INVALID = 0,
    AH = 1,
    AL = 2,
    AX = 3,
    BH = 4,
    BL = 5,
    BP = 6,
    BPL = 7,
    BX = 8,
    CH = 9,
    CL = 10,
    CS = 11,
    CX = 12,
    DH = 13,
    DI = 14,
    DIL = 15,
    DL = 16,
    DS = 17,
    DX = 18,
    EAX = 19,
    EBP = 20,
    EBX = 21,
    ECX = 22,
    EDI = 23,
    EDX = 24,
    EFLAGS = 25,
    EIP = 26,
    ES = 28,
    ESI = 29,
    ESP = 30,
    FPSW = 31,
    FS = 32,
    GS = 33,
    IP = 34,
    RAX = 35,
    RBP = 36,
    RBX = 37,
    RCX = 38,
    RDI = 39,
    RDX = 40,
    RIP = 41,
    RSI = 43,
    RSP = 44,
    SI = 45,
    SIL = 46,
    SP = 47,
    SPL = 48,
    SS = 49,
    CR0 = 50,
    CR1 = 51,
    CR2 = 52,
    CR3 = 53,
    CR4 = 54,
    CR8 = 58,
    DR0 = 66,
    DR1 = 67,
    DR2 = 68,
    DR3 = 69,
    DR4 = 70,
    DR5 = 71,
    DR6 = 72,
    DR7 = 73,
    FP0 = 82,
    FP1 = 83,
    FP2 = 84,
    FP3 = 85,
    FP4 = 86,
    FP5 = 87,
    FP6 = 88,
    FP7 = 89,
    K0 = 90,
    K1 = 91,
    K2 = 92,
    K3 = 93,
    K4 = 94,
    K5 = 95,
    K6 = 96,
    K7 = 97,
    MM0 = 98,
    MM1 = 99,
    MM2 = 100,
    MM3 = 101,
    MM4 = 102,
    MM5 = 103,
    MM6 = 104,
    MM7 = 105,
    R8 = 106,
    R9 = 107,
    R10 = 108,
    R11 = 109,
    R12 = 110,
    R13 = 111,
    R14 = 112,
    R15 = 113,
    ST0 = 114,
    ST1 = 115,
    ST2 = 116,
    ST3 = 117,
    ST4 = 118,
    ST5 = 119,
    ST6 = 120,
    ST7 = 121,
    XMM0 = 122,
    XMM1 = 123,
    XMM2 = 124,
    XMM3 = 125,
    XMM4 = 126,
    XMM5 = 127,
    XMM6 = 128,
    XMM7 = 129,
    XMM8 = 130,
    XMM9 = 131,
    XMM10 = 132,
    XMM11 = 133,
    XMM12 = 134,
    XMM13 = 135,
    XMM14 = 136,
    XMM15 = 137,
    XMM16 = 138,
    XMM17 = 139,
    XMM18 = 140,
    XMM19 = 141,
    XMM20 = 142,
    XMM21 = 143,
    XMM22 = 144,
    XMM23 = 145,
    XMM24 = 146,
    XMM25 = 147,
    XMM26 = 148,
    XMM27 = 149,
    XMM28 = 150,
    XMM29 = 151,
    XMM30 = 152,
    XMM31 = 153,
    YMM0 = 154,
    YMM1 = 155,
    YMM2 = 156,
    YMM3 = 157,
    YMM4 = 158,
    YMM5 = 159,
    YMM6 = 160,
    YMM7 = 161,
    YMM8 = 162,
    YMM9 = 163,
    YMM10 = 164,
    YMM11 = 165,
    YMM12 = 166,
    YMM13 = 167,
    YMM14 = 168,
    YMM15 = 169,
    YMM16 = 170,
    YMM17 = 171,
    YMM18 = 172,
    YMM19 = 173,
    YMM20 = 174,
    YMM21 = 175,
    YMM22 = 176,
    YMM23 = 177,
    YMM24 = 178,
    YMM25 = 179,
    YMM26 = 180,
    YMM27 = 181,
    YMM28 = 182,
    YMM29 = 183,
    YMM30 = 184,
    YMM31 = 185,
    ZMM0 = 186,
    ZMM1 = 187,
    ZMM2 = 188,
    ZMM3 = 189,
    ZMM4 = 190,
    ZMM5 = 191,
    ZMM6 = 192,
    ZMM7 = 193,
    ZMM8 = 194,
    ZMM9 = 195,
    ZMM10 = 196,
    ZMM11 = 197,
    ZMM12 = 198,
    ZMM13 = 199,
    ZMM14 = 200,
    ZMM15 = 201,
    ZMM16 = 202,
    ZMM17 = 203,
    ZMM18 = 204,
    ZMM19 = 205,
    ZMM20 = 206,
    ZMM21 = 207,
    ZMM22 = 208,
    ZMM23 = 209,
    ZMM24 = 210,
    ZMM25 = 211,
    ZMM26 = 212,
    ZMM27 = 213,
    ZMM28 = 214,
    ZMM29 = 215,
    ZMM30 = 216,
    ZMM31 = 217,
    R8B = 218,
    R9B = 219,
    R10B = 220,
    R11B = 221,
    R12B = 222,
    R13B = 223,
    R14B = 224,
    R15B = 225,
    R8D = 226,
    R9D = 227,
    R10D = 228,
    R11D = 229,
    R12D = 230,
    R13D = 231,
    R14D = 232,
    R15D = 233,
    R8W = 234,
    R9W = 235,
    R10W = 236,
    R11W = 237,
    R12W = 238,
    R13W = 239,
    R14W = 240,
    R15W = 241,
    IDTR = 242,
    GDTR = 243,
    LDTR = 244,
    TR = 245,
    FPCW = 246,
    FPTAG = 247,
    MSR = 248,
    MXCSR = 249,
    FS_BASE = 250,
    GS_BASE = 251,
    FLAGS = 252,
    RFLAGS = 253,
    FIP = 254,
    FCS = 255,
    FDP = 256,
    FDS = 257,
    FOP = 258,
    ENDING = 259,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.INVALID)

### INVALID = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.AH)

### AH = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.AL)

### AL = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.AX)

### AX = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.BH)

### BH = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.BL)

### BL = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.BP)

### BP = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.BPL)

### BPL = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.BX)

### BX = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CH)

### CH = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CL)

### CL = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CS)

### CS = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CX)

### CX = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DH)

### DH = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DI)

### DI = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DIL)

### DIL = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DL)

### DL = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DS)

### DS = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DX)

### DX = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.EAX)

### EAX = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.EBP)

### EBP = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.EBX)

### EBX = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ECX)

### ECX = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.EDI)

### EDI = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.EDX)

### EDX = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.EFLAGS)

### EFLAGS = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.EIP)

### EIP = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ES)

### ES = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ESI)

### ESI = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ESP)

### ESP = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FPSW)

### FPSW = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FS)

### FS = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.GS)

### GS = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.IP)

### IP = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RAX)

### RAX = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RBP)

### RBP = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RBX)

### RBX = 37

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RCX)

### RCX = 38

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RDI)

### RDI = 39

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RDX)

### RDX = 40

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RIP)

### RIP = 41

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RSI)

### RSI = 43

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RSP)

### RSP = 44

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.SI)

### SI = 45

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.SIL)

### SIL = 46

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.SP)

### SP = 47

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.SPL)

### SPL = 48

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.SS)

### SS = 49

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CR0)

### CR0 = 50

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CR1)

### CR1 = 51

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CR2)

### CR2 = 52

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CR3)

### CR3 = 53

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CR4)

### CR4 = 54

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.CR8)

### CR8 = 58

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DR0)

### DR0 = 66

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DR1)

### DR1 = 67

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DR2)

### DR2 = 68

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DR3)

### DR3 = 69

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DR4)

### DR4 = 70

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DR5)

### DR5 = 71

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DR6)

### DR6 = 72

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.DR7)

### DR7 = 73

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FP0)

### FP0 = 82

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FP1)

### FP1 = 83

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FP2)

### FP2 = 84

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FP3)

### FP3 = 85

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FP4)

### FP4 = 86

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FP5)

### FP5 = 87

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FP6)

### FP6 = 88

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FP7)

### FP7 = 89

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.K0)

### K0 = 90

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.K1)

### K1 = 91

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.K2)

### K2 = 92

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.K3)

### K3 = 93

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.K4)

### K4 = 94

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.K5)

### K5 = 95

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.K6)

### K6 = 96

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.K7)

### K7 = 97

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MM0)

### MM0 = 98

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MM1)

### MM1 = 99

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MM2)

### MM2 = 100

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MM3)

### MM3 = 101

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MM4)

### MM4 = 102

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MM5)

### MM5 = 103

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MM6)

### MM6 = 104

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MM7)

### MM7 = 105

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R8)

### R8 = 106

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R9)

### R9 = 107

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R10)

### R10 = 108

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R11)

### R11 = 109

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R12)

### R12 = 110

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R13)

### R13 = 111

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R14)

### R14 = 112

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R15)

### R15 = 113

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ST0)

### ST0 = 114

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ST1)

### ST1 = 115

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ST2)

### ST2 = 116

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ST3)

### ST3 = 117

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ST4)

### ST4 = 118

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ST5)

### ST5 = 119

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ST6)

### ST6 = 120

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ST7)

### ST7 = 121

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM0)

### XMM0 = 122

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM1)

### XMM1 = 123

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM2)

### XMM2 = 124

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM3)

### XMM3 = 125

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM4)

### XMM4 = 126

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM5)

### XMM5 = 127

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM6)

### XMM6 = 128

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM7)

### XMM7 = 129

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM8)

### XMM8 = 130

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM9)

### XMM9 = 131

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM10)

### XMM10 = 132

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM11)

### XMM11 = 133

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM12)

### XMM12 = 134

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM13)

### XMM13 = 135

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM14)

### XMM14 = 136

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM15)

### XMM15 = 137

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM16)

### XMM16 = 138

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM17)

### XMM17 = 139

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM18)

### XMM18 = 140

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM19)

### XMM19 = 141

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM20)

### XMM20 = 142

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM21)

### XMM21 = 143

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM22)

### XMM22 = 144

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM23)

### XMM23 = 145

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM24)

### XMM24 = 146

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM25)

### XMM25 = 147

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM26)

### XMM26 = 148

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM27)

### XMM27 = 149

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM28)

### XMM28 = 150

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM29)

### XMM29 = 151

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM30)

### XMM30 = 152

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.XMM31)

### XMM31 = 153

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM0)

### YMM0 = 154

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM1)

### YMM1 = 155

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM2)

### YMM2 = 156

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM3)

### YMM3 = 157

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM4)

### YMM4 = 158

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM5)

### YMM5 = 159

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM6)

### YMM6 = 160

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM7)

### YMM7 = 161

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM8)

### YMM8 = 162

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM9)

### YMM9 = 163

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM10)

### YMM10 = 164

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM11)

### YMM11 = 165

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM12)

### YMM12 = 166

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM13)

### YMM13 = 167

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM14)

### YMM14 = 168

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM15)

### YMM15 = 169

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM16)

### YMM16 = 170

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM17)

### YMM17 = 171

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM18)

### YMM18 = 172

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM19)

### YMM19 = 173

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM20)

### YMM20 = 174

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM21)

### YMM21 = 175

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM22)

### YMM22 = 176

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM23)

### YMM23 = 177

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM24)

### YMM24 = 178

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM25)

### YMM25 = 179

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM26)

### YMM26 = 180

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM27)

### YMM27 = 181

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM28)

### YMM28 = 182

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM29)

### YMM29 = 183

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM30)

### YMM30 = 184

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.YMM31)

### YMM31 = 185

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM0)

### ZMM0 = 186

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM1)

### ZMM1 = 187

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM2)

### ZMM2 = 188

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM3)

### ZMM3 = 189

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM4)

### ZMM4 = 190

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM5)

### ZMM5 = 191

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM6)

### ZMM6 = 192

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM7)

### ZMM7 = 193

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM8)

### ZMM8 = 194

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM9)

### ZMM9 = 195

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM10)

### ZMM10 = 196

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM11)

### ZMM11 = 197

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM12)

### ZMM12 = 198

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM13)

### ZMM13 = 199

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM14)

### ZMM14 = 200

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM15)

### ZMM15 = 201

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM16)

### ZMM16 = 202

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM17)

### ZMM17 = 203

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM18)

### ZMM18 = 204

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM19)

### ZMM19 = 205

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM20)

### ZMM20 = 206

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM21)

### ZMM21 = 207

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM22)

### ZMM22 = 208

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM23)

### ZMM23 = 209

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM24)

### ZMM24 = 210

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM25)

### ZMM25 = 211

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM26)

### ZMM26 = 212

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM27)

### ZMM27 = 213

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM28)

### ZMM28 = 214

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM29)

### ZMM29 = 215

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM30)

### ZMM30 = 216

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ZMM31)

### ZMM31 = 217

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R8B)

### R8B = 218

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R9B)

### R9B = 219

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R10B)

### R10B = 220

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R11B)

### R11B = 221

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R12B)

### R12B = 222

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R13B)

### R13B = 223

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R14B)

### R14B = 224

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R15B)

### R15B = 225

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R8D)

### R8D = 226

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R9D)

### R9D = 227

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R10D)

### R10D = 228

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R11D)

### R11D = 229

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R12D)

### R12D = 230

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R13D)

### R13D = 231

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R14D)

### R14D = 232

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R15D)

### R15D = 233

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R8W)

### R8W = 234

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R9W)

### R9W = 235

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R10W)

### R10W = 236

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R11W)

### R11W = 237

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R12W)

### R12W = 238

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R13W)

### R13W = 239

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R14W)

### R14W = 240

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.R15W)

### R15W = 241

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.IDTR)

### IDTR = 242

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.GDTR)

### GDTR = 243

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.LDTR)

### LDTR = 244

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.TR)

### TR = 245

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FPCW)

### FPCW = 246

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FPTAG)

### FPTAG = 247

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MSR)

### MSR = 248

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.MXCSR)

### MXCSR = 249

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FS_BASE)

### FS\_BASE = 250

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.GS_BASE)

### GS\_BASE = 251

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FLAGS)

### FLAGS = 252

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.RFLAGS)

### RFLAGS = 253

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FIP)

### FIP = 254

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FCS)

### FCS = 255

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FDP)

### FDP = 256

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FDS)

### FDS = 257

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.FOP)

### FOP = 258

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#variant.ENDING)

### ENDING = 259

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Clone-for-RegisterX86)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Debug-for-RegisterX86)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#250-254) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-From%3CRegisterX86%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#251-253) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(r: [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-PartialEq-for-RegisterX86)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Copy-for-RegisterX86)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-StructuralPartialEq-for-RegisterX86)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Freeze-for-RegisterX86)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-RefUnwindSafe-for-RegisterX86)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Send-for-RegisterX86)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Sync-for-RegisterX86)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Unpin-for-RegisterX86)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-UnwindSafe-for-RegisterX86)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
