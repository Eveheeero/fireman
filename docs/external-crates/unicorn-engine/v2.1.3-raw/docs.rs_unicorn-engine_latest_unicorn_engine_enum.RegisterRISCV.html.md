---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html"
title: "RegisterRISCV in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.RegisterRISCV.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.RegisterRISCV.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum RegisterRISCVCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#6-206)

```

#[repr(C)]pub enum RegisterRISCV {
Show 193 variants    INVALID = 0,
    X0 = 1,
    X1 = 2,
    X2 = 3,
    X3 = 4,
    X4 = 5,
    X5 = 6,
    X6 = 7,
    X7 = 8,
    X8 = 9,
    X9 = 10,
    X10 = 11,
    X11 = 12,
    X12 = 13,
    X13 = 14,
    X14 = 15,
    X15 = 16,
    X16 = 17,
    X17 = 18,
    X18 = 19,
    X19 = 20,
    X20 = 21,
    X21 = 22,
    X22 = 23,
    X23 = 24,
    X24 = 25,
    X25 = 26,
    X26 = 27,
    X27 = 28,
    X28 = 29,
    X29 = 30,
    X30 = 31,
    X31 = 32,
    USTATUS = 33,
    UIE = 34,
    UTVEC = 35,
    USCRATCH = 36,
    UEPC = 37,
    UCAUSE = 38,
    UTVAL = 39,
    UIP = 40,
    FFLAGS = 41,
    FRM = 42,
    FCSR = 43,
    CYCLE = 44,
    TIME = 45,
    INSTRET = 46,
    HPMCOUNTER3 = 47,
    HPMCOUNTER4 = 48,
    HPMCOUNTER5 = 49,
    HPMCOUNTER6 = 50,
    HPMCOUNTER7 = 51,
    HPMCOUNTER8 = 52,
    HPMCOUNTER9 = 53,
    HPMCOUNTER10 = 54,
    HPMCOUNTER11 = 55,
    HPMCOUNTER12 = 56,
    HPMCOUNTER13 = 57,
    HPMCOUNTER14 = 58,
    HPMCOUNTER15 = 59,
    HPMCOUNTER16 = 60,
    HPMCOUNTER17 = 61,
    HPMCOUNTER18 = 62,
    HPMCOUNTER19 = 63,
    HPMCOUNTER20 = 64,
    HPMCOUNTER21 = 65,
    HPMCOUNTER22 = 66,
    HPMCOUNTER23 = 67,
    HPMCOUNTER24 = 68,
    HPMCOUNTER25 = 69,
    HPMCOUNTER26 = 70,
    HPMCOUNTER27 = 71,
    HPMCOUNTER28 = 72,
    HPMCOUNTER29 = 73,
    HPMCOUNTER30 = 74,
    HPMCOUNTER31 = 75,
    CYCLEH = 76,
    TIMEH = 77,
    INSTRETH = 78,
    HPMCOUNTER3H = 79,
    HPMCOUNTER4H = 80,
    HPMCOUNTER5H = 81,
    HPMCOUNTER6H = 82,
    HPMCOUNTER7H = 83,
    HPMCOUNTER8H = 84,
    HPMCOUNTER9H = 85,
    HPMCOUNTER10H = 86,
    HPMCOUNTER11H = 87,
    HPMCOUNTER12H = 88,
    HPMCOUNTER13H = 89,
    HPMCOUNTER14H = 90,
    HPMCOUNTER15H = 91,
    HPMCOUNTER16H = 92,
    HPMCOUNTER17H = 93,
    HPMCOUNTER18H = 94,
    HPMCOUNTER19H = 95,
    HPMCOUNTER20H = 96,
    HPMCOUNTER21H = 97,
    HPMCOUNTER22H = 98,
    HPMCOUNTER23H = 99,
    HPMCOUNTER24H = 100,
    HPMCOUNTER25H = 101,
    HPMCOUNTER26H = 102,
    HPMCOUNTER27H = 103,
    HPMCOUNTER28H = 104,
    HPMCOUNTER29H = 105,
    HPMCOUNTER30H = 106,
    HPMCOUNTER31H = 107,
    MCYCLE = 108,
    MINSTRET = 109,
    MCYCLEH = 110,
    MINSTRETH = 111,
    MVENDORID = 112,
    MARCHID = 113,
    MIMPID = 114,
    MHARTID = 115,
    MSTATUS = 116,
    MISA = 117,
    MEDELEG = 118,
    MIDELEG = 119,
    MIE = 120,
    MTVEC = 121,
    MCOUNTEREN = 122,
    MSTATUSH = 123,
    MUCOUNTEREN = 124,
    MSCOUNTEREN = 125,
    MHCOUNTEREN = 126,
    MSCRATCH = 127,
    MEPC = 128,
    MCAUSE = 129,
    MTVAL = 130,
    MIP = 131,
    MBADADDR = 132,
    SSTATUS = 133,
    SEDELEG = 134,
    SIDELEG = 135,
    SIE = 136,
    STVEC = 137,
    SCOUNTEREN = 138,
    SSCRATCH = 139,
    SEPC = 140,
    SCAUSE = 141,
    STVAL = 142,
    SIP = 143,
    SBADADDR = 144,
    SPTBR = 145,
    SATP = 146,
    HSTATUS = 147,
    HEDELEG = 148,
    HIDELEG = 149,
    HIE = 150,
    HCOUNTEREN = 151,
    HTVAL = 152,
    HIP = 153,
    HTINST = 154,
    HGATP = 155,
    HTIMEDELTA = 156,
    HTIMEDELTAH = 157,
    F0 = 158,
    F1 = 159,
    F2 = 160,
    F3 = 161,
    F4 = 162,
    F5 = 163,
    F6 = 164,
    F7 = 165,
    F8 = 166,
    F9 = 167,
    F10 = 168,
    F11 = 169,
    F12 = 170,
    F13 = 171,
    F14 = 172,
    F15 = 173,
    F16 = 174,
    F17 = 175,
    F18 = 176,
    F19 = 177,
    F20 = 178,
    F21 = 179,
    F22 = 180,
    F23 = 181,
    F24 = 182,
    F25 = 183,
    F26 = 184,
    F27 = 185,
    F28 = 186,
    F29 = 187,
    F30 = 188,
    F31 = 189,
    PC = 190,
    PRIV = 191,
    ENDING = 192,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.INVALID)

### INVALID = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X0)

### X0 = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X1)

### X1 = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X2)

### X2 = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X3)

### X3 = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X4)

### X4 = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X5)

### X5 = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X6)

### X6 = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X7)

### X7 = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X8)

### X8 = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X9)

### X9 = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X10)

### X10 = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X11)

### X11 = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X12)

### X12 = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X13)

### X13 = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X14)

### X14 = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X15)

### X15 = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X16)

### X16 = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X17)

### X17 = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X18)

### X18 = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X19)

### X19 = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X20)

### X20 = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X21)

### X21 = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X22)

### X22 = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X23)

### X23 = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X24)

### X24 = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X25)

### X25 = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X26)

### X26 = 27

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X27)

### X27 = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X28)

### X28 = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X29)

### X29 = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X30)

### X30 = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.X31)

### X31 = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.USTATUS)

### USTATUS = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.UIE)

### UIE = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.UTVEC)

### UTVEC = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.USCRATCH)

### USCRATCH = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.UEPC)

### UEPC = 37

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.UCAUSE)

### UCAUSE = 38

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.UTVAL)

### UTVAL = 39

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.UIP)

### UIP = 40

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.FFLAGS)

### FFLAGS = 41

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.FRM)

### FRM = 42

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.FCSR)

### FCSR = 43

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.CYCLE)

### CYCLE = 44

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.TIME)

### TIME = 45

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.INSTRET)

### INSTRET = 46

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER3)

### HPMCOUNTER3 = 47

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER4)

### HPMCOUNTER4 = 48

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER5)

### HPMCOUNTER5 = 49

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER6)

### HPMCOUNTER6 = 50

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER7)

### HPMCOUNTER7 = 51

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER8)

### HPMCOUNTER8 = 52

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER9)

### HPMCOUNTER9 = 53

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER10)

### HPMCOUNTER10 = 54

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER11)

### HPMCOUNTER11 = 55

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER12)

### HPMCOUNTER12 = 56

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER13)

### HPMCOUNTER13 = 57

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER14)

### HPMCOUNTER14 = 58

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER15)

### HPMCOUNTER15 = 59

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER16)

### HPMCOUNTER16 = 60

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER17)

### HPMCOUNTER17 = 61

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER18)

### HPMCOUNTER18 = 62

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER19)

### HPMCOUNTER19 = 63

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER20)

### HPMCOUNTER20 = 64

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER21)

### HPMCOUNTER21 = 65

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER22)

### HPMCOUNTER22 = 66

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER23)

### HPMCOUNTER23 = 67

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER24)

### HPMCOUNTER24 = 68

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER25)

### HPMCOUNTER25 = 69

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER26)

### HPMCOUNTER26 = 70

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER27)

### HPMCOUNTER27 = 71

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER28)

### HPMCOUNTER28 = 72

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER29)

### HPMCOUNTER29 = 73

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER30)

### HPMCOUNTER30 = 74

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER31)

### HPMCOUNTER31 = 75

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.CYCLEH)

### CYCLEH = 76

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.TIMEH)

### TIMEH = 77

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.INSTRETH)

### INSTRETH = 78

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER3H)

### HPMCOUNTER3H = 79

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER4H)

### HPMCOUNTER4H = 80

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER5H)

### HPMCOUNTER5H = 81

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER6H)

### HPMCOUNTER6H = 82

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER7H)

### HPMCOUNTER7H = 83

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER8H)

### HPMCOUNTER8H = 84

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER9H)

### HPMCOUNTER9H = 85

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER10H)

### HPMCOUNTER10H = 86

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER11H)

### HPMCOUNTER11H = 87

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER12H)

### HPMCOUNTER12H = 88

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER13H)

### HPMCOUNTER13H = 89

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER14H)

### HPMCOUNTER14H = 90

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER15H)

### HPMCOUNTER15H = 91

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER16H)

### HPMCOUNTER16H = 92

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER17H)

### HPMCOUNTER17H = 93

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER18H)

### HPMCOUNTER18H = 94

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER19H)

### HPMCOUNTER19H = 95

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER20H)

### HPMCOUNTER20H = 96

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER21H)

### HPMCOUNTER21H = 97

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER22H)

### HPMCOUNTER22H = 98

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER23H)

### HPMCOUNTER23H = 99

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER24H)

### HPMCOUNTER24H = 100

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER25H)

### HPMCOUNTER25H = 101

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER26H)

### HPMCOUNTER26H = 102

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER27H)

### HPMCOUNTER27H = 103

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER28H)

### HPMCOUNTER28H = 104

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER29H)

### HPMCOUNTER29H = 105

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER30H)

### HPMCOUNTER30H = 106

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HPMCOUNTER31H)

### HPMCOUNTER31H = 107

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MCYCLE)

### MCYCLE = 108

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MINSTRET)

### MINSTRET = 109

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MCYCLEH)

### MCYCLEH = 110

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MINSTRETH)

### MINSTRETH = 111

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MVENDORID)

### MVENDORID = 112

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MARCHID)

### MARCHID = 113

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MIMPID)

### MIMPID = 114

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MHARTID)

### MHARTID = 115

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MSTATUS)

### MSTATUS = 116

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MISA)

### MISA = 117

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MEDELEG)

### MEDELEG = 118

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MIDELEG)

### MIDELEG = 119

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MIE)

### MIE = 120

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MTVEC)

### MTVEC = 121

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MCOUNTEREN)

### MCOUNTEREN = 122

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MSTATUSH)

### MSTATUSH = 123

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MUCOUNTEREN)

### MUCOUNTEREN = 124

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MSCOUNTEREN)

### MSCOUNTEREN = 125

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MHCOUNTEREN)

### MHCOUNTEREN = 126

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MSCRATCH)

### MSCRATCH = 127

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MEPC)

### MEPC = 128

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MCAUSE)

### MCAUSE = 129

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MTVAL)

### MTVAL = 130

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MIP)

### MIP = 131

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.MBADADDR)

### MBADADDR = 132

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SSTATUS)

### SSTATUS = 133

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SEDELEG)

### SEDELEG = 134

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SIDELEG)

### SIDELEG = 135

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SIE)

### SIE = 136

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.STVEC)

### STVEC = 137

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SCOUNTEREN)

### SCOUNTEREN = 138

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SSCRATCH)

### SSCRATCH = 139

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SEPC)

### SEPC = 140

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SCAUSE)

### SCAUSE = 141

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.STVAL)

### STVAL = 142

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SIP)

### SIP = 143

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SBADADDR)

### SBADADDR = 144

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SPTBR)

### SPTBR = 145

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.SATP)

### SATP = 146

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HSTATUS)

### HSTATUS = 147

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HEDELEG)

### HEDELEG = 148

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HIDELEG)

### HIDELEG = 149

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HIE)

### HIE = 150

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HCOUNTEREN)

### HCOUNTEREN = 151

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HTVAL)

### HTVAL = 152

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HIP)

### HIP = 153

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HTINST)

### HTINST = 154

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HGATP)

### HGATP = 155

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HTIMEDELTA)

### HTIMEDELTA = 156

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.HTIMEDELTAH)

### HTIMEDELTAH = 157

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F0)

### F0 = 158

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F1)

### F1 = 159

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F2)

### F2 = 160

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F3)

### F3 = 161

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F4)

### F4 = 162

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F5)

### F5 = 163

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F6)

### F6 = 164

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F7)

### F7 = 165

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F8)

### F8 = 166

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F9)

### F9 = 167

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F10)

### F10 = 168

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F11)

### F11 = 169

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F12)

### F12 = 170

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F13)

### F13 = 171

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F14)

### F14 = 172

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F15)

### F15 = 173

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F16)

### F16 = 174

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F17)

### F17 = 175

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F18)

### F18 = 176

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F19)

### F19 = 177

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F20)

### F20 = 178

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F21)

### F21 = 179

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F22)

### F22 = 180

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F23)

### F23 = 181

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F24)

### F24 = 182

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F25)

### F25 = 183

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F26)

### F26 = 184

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F27)

### F27 = 185

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F28)

### F28 = 186

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F29)

### F29 = 187

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F30)

### F30 = 188

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.F31)

### F31 = 189

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.PC)

### PC = 190

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.PRIV)

### PRIV = 191

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#variant.ENDING)

### ENDING = 192

## Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#208-340) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-RegisterRISCV)

### impl [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#275)

#### pub const [ZERO](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.ZERO): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X0

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#276)

#### pub const [RA](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.RA): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X1

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#277)

#### pub const [SP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.SP): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X2

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#278)

#### pub const [GP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.GP): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X3

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#279)

#### pub const [TP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.TP): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X4

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#280)

#### pub const [T0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.T0): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X5

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#281)

#### pub const [T1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.T1): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X6

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#282)

#### pub const [T2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.T2): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X7

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#283)

#### pub const [S0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S0): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X8

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#284)

#### pub const [FP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FP): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X8

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#285)

#### pub const [S1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S1): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X9

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#286)

#### pub const [A0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.A0): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X10

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#287)

#### pub const [A1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.A1): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X11

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#288)

#### pub const [A2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.A2): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X12

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#289)

#### pub const [A3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.A3): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X13

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#290)

#### pub const [A4](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.A4): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X14

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#291)

#### pub const [A5](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.A5): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X15

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#292)

#### pub const [A6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.A6): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X16

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#293)

#### pub const [A7](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.A7): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X17

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#294)

#### pub const [S2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S2): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X18

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#295)

#### pub const [S3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S3): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X19

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#296)

#### pub const [S4](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S4): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X20

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#297)

#### pub const [S5](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S5): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X21

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#298)

#### pub const [S6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S6): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X22

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#299)

#### pub const [S7](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S7): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X23

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#300)

#### pub const [S8](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S8): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X24

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#301)

#### pub const [S9](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S9): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X25

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#302)

#### pub const [S10](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S10): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X26

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#303)

#### pub const [S11](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.S11): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X27

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#304)

#### pub const [T3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.T3): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X28

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#305)

#### pub const [T4](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.T4): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X29

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#306)

#### pub const [T5](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.T5): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X30

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#307)

#### pub const [T6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.T6): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::X31

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#308)

#### pub const [FT0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT0): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F0

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#309)

#### pub const [FT1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT1): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F1

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#310)

#### pub const [FT2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT2): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F2

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#311)

#### pub const [FT3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT3): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F3

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#312)

#### pub const [FT4](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT4): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F4

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#313)

#### pub const [FT5](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT5): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F5

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#314)

#### pub const [FT6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT6): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F6

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#315)

#### pub const [FT7](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT7): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F7

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#316)

#### pub const [FS0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS0): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F8

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#317)

#### pub const [FS1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS1): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F9

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#318)

#### pub const [FA0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FA0): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F10

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#319)

#### pub const [FA1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FA1): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F11

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#320)

#### pub const [FA2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FA2): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F12

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#321)

#### pub const [FA3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FA3): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F13

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#322)

#### pub const [FA4](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FA4): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F14

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#323)

#### pub const [FA5](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FA5): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F15

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#324)

#### pub const [FA6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FA6): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F16

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#325)

#### pub const [FA7](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FA7): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F17

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#326)

#### pub const [FS2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS2): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F18

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#327)

#### pub const [FS3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS3): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F19

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#328)

#### pub const [FS4](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS4): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F20

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#329)

#### pub const [FS5](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS5): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F21

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#330)

#### pub const [FS6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS6): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F22

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#331)

#### pub const [FS7](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS7): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F23

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#332)

#### pub const [FS8](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS8): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F24

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#333)

#### pub const [FS9](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS9): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F25

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#334)

#### pub const [FS10](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS10): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F26

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#335)

#### pub const [FS11](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FS11): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F27

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#336)

#### pub const [FT8](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT8): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F28

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#337)

#### pub const [FT9](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT9): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F29

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#338)

#### pub const [FT10](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT10): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F30

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#339)

#### pub const [FT11](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#associatedconstant.FT11): [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") = RegisterRISCV::F31

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Clone-for-RegisterRISCV)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Debug-for-RegisterRISCV)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#342-346) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-From%3CRegisterRISCV%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#343-345) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(r: [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-PartialEq-for-RegisterRISCV)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Copy-for-RegisterRISCV)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/riscv.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-StructuralPartialEq-for-RegisterRISCV)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Freeze-for-RegisterRISCV)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-RefUnwindSafe-for-RegisterRISCV)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Send-for-RegisterRISCV)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Sync-for-RegisterRISCV)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Unpin-for-RegisterRISCV)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-UnwindSafe-for-RegisterRISCV)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
