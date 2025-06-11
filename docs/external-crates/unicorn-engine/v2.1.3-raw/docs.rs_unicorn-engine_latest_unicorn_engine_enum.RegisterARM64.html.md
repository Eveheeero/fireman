---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html"
title: "RegisterARM64 in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.RegisterARM64.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.RegisterARM64.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum RegisterARM64Copy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#6-309)

```

#[repr(C)]pub enum RegisterARM64 {
Show 292 variants    INVALID = 0,
    X29 = 1,
    X30 = 2,
    NZCV = 3,
    SP = 4,
    WSP = 5,
    WZR = 6,
    XZR = 7,
    B0 = 8,
    B1 = 9,
    B2 = 10,
    B3 = 11,
    B4 = 12,
    B5 = 13,
    B6 = 14,
    B7 = 15,
    B8 = 16,
    B9 = 17,
    B10 = 18,
    B11 = 19,
    B12 = 20,
    B13 = 21,
    B14 = 22,
    B15 = 23,
    B16 = 24,
    B17 = 25,
    B18 = 26,
    B19 = 27,
    B20 = 28,
    B21 = 29,
    B22 = 30,
    B23 = 31,
    B24 = 32,
    B25 = 33,
    B26 = 34,
    B27 = 35,
    B28 = 36,
    B29 = 37,
    B30 = 38,
    B31 = 39,
    D0 = 40,
    D1 = 41,
    D2 = 42,
    D3 = 43,
    D4 = 44,
    D5 = 45,
    D6 = 46,
    D7 = 47,
    D8 = 48,
    D9 = 49,
    D10 = 50,
    D11 = 51,
    D12 = 52,
    D13 = 53,
    D14 = 54,
    D15 = 55,
    D16 = 56,
    D17 = 57,
    D18 = 58,
    D19 = 59,
    D20 = 60,
    D21 = 61,
    D22 = 62,
    D23 = 63,
    D24 = 64,
    D25 = 65,
    D26 = 66,
    D27 = 67,
    D28 = 68,
    D29 = 69,
    D30 = 70,
    D31 = 71,
    H0 = 72,
    H1 = 73,
    H2 = 74,
    H3 = 75,
    H4 = 76,
    H5 = 77,
    H6 = 78,
    H7 = 79,
    H8 = 80,
    H9 = 81,
    H10 = 82,
    H11 = 83,
    H12 = 84,
    H13 = 85,
    H14 = 86,
    H15 = 87,
    H16 = 88,
    H17 = 89,
    H18 = 90,
    H19 = 91,
    H20 = 92,
    H21 = 93,
    H22 = 94,
    H23 = 95,
    H24 = 96,
    H25 = 97,
    H26 = 98,
    H27 = 99,
    H28 = 100,
    H29 = 101,
    H30 = 102,
    H31 = 103,
    Q0 = 104,
    Q1 = 105,
    Q2 = 106,
    Q3 = 107,
    Q4 = 108,
    Q5 = 109,
    Q6 = 110,
    Q7 = 111,
    Q8 = 112,
    Q9 = 113,
    Q10 = 114,
    Q11 = 115,
    Q12 = 116,
    Q13 = 117,
    Q14 = 118,
    Q15 = 119,
    Q16 = 120,
    Q17 = 121,
    Q18 = 122,
    Q19 = 123,
    Q20 = 124,
    Q21 = 125,
    Q22 = 126,
    Q23 = 127,
    Q24 = 128,
    Q25 = 129,
    Q26 = 130,
    Q27 = 131,
    Q28 = 132,
    Q29 = 133,
    Q30 = 134,
    Q31 = 135,
    S0 = 136,
    S1 = 137,
    S2 = 138,
    S3 = 139,
    S4 = 140,
    S5 = 141,
    S6 = 142,
    S7 = 143,
    S8 = 144,
    S9 = 145,
    S10 = 146,
    S11 = 147,
    S12 = 148,
    S13 = 149,
    S14 = 150,
    S15 = 151,
    S16 = 152,
    S17 = 153,
    S18 = 154,
    S19 = 155,
    S20 = 156,
    S21 = 157,
    S22 = 158,
    S23 = 159,
    S24 = 160,
    S25 = 161,
    S26 = 162,
    S27 = 163,
    S28 = 164,
    S29 = 165,
    S30 = 166,
    S31 = 167,
    W0 = 168,
    W1 = 169,
    W2 = 170,
    W3 = 171,
    W4 = 172,
    W5 = 173,
    W6 = 174,
    W7 = 175,
    W8 = 176,
    W9 = 177,
    W10 = 178,
    W11 = 179,
    W12 = 180,
    W13 = 181,
    W14 = 182,
    W15 = 183,
    W16 = 184,
    W17 = 185,
    W18 = 186,
    W19 = 187,
    W20 = 188,
    W21 = 189,
    W22 = 190,
    W23 = 191,
    W24 = 192,
    W25 = 193,
    W26 = 194,
    W27 = 195,
    W28 = 196,
    W29 = 197,
    W30 = 198,
    X0 = 199,
    X1 = 200,
    X2 = 201,
    X3 = 202,
    X4 = 203,
    X5 = 204,
    X6 = 205,
    X7 = 206,
    X8 = 207,
    X9 = 208,
    X10 = 209,
    X11 = 210,
    X12 = 211,
    X13 = 212,
    X14 = 213,
    X15 = 214,
    X16 = 215,
    X17 = 216,
    X18 = 217,
    X19 = 218,
    X20 = 219,
    X21 = 220,
    X22 = 221,
    X23 = 222,
    X24 = 223,
    X25 = 224,
    X26 = 225,
    X27 = 226,
    X28 = 227,
    V0 = 228,
    V1 = 229,
    V2 = 230,
    V3 = 231,
    V4 = 232,
    V5 = 233,
    V6 = 234,
    V7 = 235,
    V8 = 236,
    V9 = 237,
    V10 = 238,
    V11 = 239,
    V12 = 240,
    V13 = 241,
    V14 = 242,
    V15 = 243,
    V16 = 244,
    V17 = 245,
    V18 = 246,
    V19 = 247,
    V20 = 248,
    V21 = 249,
    V22 = 250,
    V23 = 251,
    V24 = 252,
    V25 = 253,
    V26 = 254,
    V27 = 255,
    V28 = 256,
    V29 = 257,
    V30 = 258,
    V31 = 259,
    PC = 260,
    CPACR_EL1 = 261,
    TPIDR_EL0 = 262,
    TPIDRRO_EL0 = 263,
    TPIDR_EL1 = 264,
    PSTATE = 265,
    ELR_EL0 = 266,
    ELR_EL1 = 267,
    ELR_EL2 = 268,
    ELR_EL3 = 269,
    SP_EL0 = 270,
    SP_EL1 = 271,
    SP_EL2 = 272,
    SP_EL3 = 273,
    TTBR0_EL1 = 274,
    TTBR1_EL1 = 275,
    ESR_EL0 = 276,
    ESR_EL1 = 277,
    ESR_EL2 = 278,
    ESR_EL3 = 279,
    FAR_EL0 = 280,
    FAR_EL1 = 281,
    FAR_EL2 = 282,
    FAR_EL3 = 283,
    PAR_EL1 = 284,
    MAIR_EL1 = 285,
    VBAR_EL0 = 286,
    VBAR_EL1 = 287,
    VBAR_EL2 = 288,
    VBAR_EL3 = 289,
    CP_REG = 290,
    ENDING = 291,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.INVALID)

### INVALID = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X29)

### X29 = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X30)

### X30 = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.NZCV)

### NZCV = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.SP)

### SP = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.WSP)

### WSP = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.WZR)

### WZR = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.XZR)

### XZR = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B0)

### B0 = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B1)

### B1 = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B2)

### B2 = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B3)

### B3 = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B4)

### B4 = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B5)

### B5 = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B6)

### B6 = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B7)

### B7 = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B8)

### B8 = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B9)

### B9 = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B10)

### B10 = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B11)

### B11 = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B12)

### B12 = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B13)

### B13 = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B14)

### B14 = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B15)

### B15 = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B16)

### B16 = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B17)

### B17 = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B18)

### B18 = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B19)

### B19 = 27

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B20)

### B20 = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B21)

### B21 = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B22)

### B22 = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B23)

### B23 = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B24)

### B24 = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B25)

### B25 = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B26)

### B26 = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B27)

### B27 = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B28)

### B28 = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B29)

### B29 = 37

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B30)

### B30 = 38

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.B31)

### B31 = 39

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D0)

### D0 = 40

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D1)

### D1 = 41

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D2)

### D2 = 42

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D3)

### D3 = 43

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D4)

### D4 = 44

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D5)

### D5 = 45

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D6)

### D6 = 46

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D7)

### D7 = 47

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D8)

### D8 = 48

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D9)

### D9 = 49

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D10)

### D10 = 50

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D11)

### D11 = 51

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D12)

### D12 = 52

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D13)

### D13 = 53

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D14)

### D14 = 54

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D15)

### D15 = 55

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D16)

### D16 = 56

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D17)

### D17 = 57

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D18)

### D18 = 58

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D19)

### D19 = 59

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D20)

### D20 = 60

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D21)

### D21 = 61

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D22)

### D22 = 62

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D23)

### D23 = 63

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D24)

### D24 = 64

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D25)

### D25 = 65

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D26)

### D26 = 66

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D27)

### D27 = 67

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D28)

### D28 = 68

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D29)

### D29 = 69

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D30)

### D30 = 70

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.D31)

### D31 = 71

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H0)

### H0 = 72

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H1)

### H1 = 73

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H2)

### H2 = 74

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H3)

### H3 = 75

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H4)

### H4 = 76

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H5)

### H5 = 77

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H6)

### H6 = 78

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H7)

### H7 = 79

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H8)

### H8 = 80

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H9)

### H9 = 81

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H10)

### H10 = 82

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H11)

### H11 = 83

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H12)

### H12 = 84

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H13)

### H13 = 85

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H14)

### H14 = 86

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H15)

### H15 = 87

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H16)

### H16 = 88

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H17)

### H17 = 89

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H18)

### H18 = 90

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H19)

### H19 = 91

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H20)

### H20 = 92

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H21)

### H21 = 93

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H22)

### H22 = 94

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H23)

### H23 = 95

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H24)

### H24 = 96

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H25)

### H25 = 97

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H26)

### H26 = 98

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H27)

### H27 = 99

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H28)

### H28 = 100

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H29)

### H29 = 101

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H30)

### H30 = 102

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.H31)

### H31 = 103

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q0)

### Q0 = 104

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q1)

### Q1 = 105

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q2)

### Q2 = 106

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q3)

### Q3 = 107

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q4)

### Q4 = 108

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q5)

### Q5 = 109

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q6)

### Q6 = 110

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q7)

### Q7 = 111

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q8)

### Q8 = 112

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q9)

### Q9 = 113

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q10)

### Q10 = 114

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q11)

### Q11 = 115

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q12)

### Q12 = 116

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q13)

### Q13 = 117

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q14)

### Q14 = 118

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q15)

### Q15 = 119

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q16)

### Q16 = 120

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q17)

### Q17 = 121

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q18)

### Q18 = 122

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q19)

### Q19 = 123

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q20)

### Q20 = 124

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q21)

### Q21 = 125

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q22)

### Q22 = 126

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q23)

### Q23 = 127

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q24)

### Q24 = 128

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q25)

### Q25 = 129

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q26)

### Q26 = 130

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q27)

### Q27 = 131

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q28)

### Q28 = 132

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q29)

### Q29 = 133

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q30)

### Q30 = 134

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.Q31)

### Q31 = 135

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S0)

### S0 = 136

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S1)

### S1 = 137

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S2)

### S2 = 138

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S3)

### S3 = 139

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S4)

### S4 = 140

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S5)

### S5 = 141

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S6)

### S6 = 142

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S7)

### S7 = 143

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S8)

### S8 = 144

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S9)

### S9 = 145

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S10)

### S10 = 146

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S11)

### S11 = 147

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S12)

### S12 = 148

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S13)

### S13 = 149

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S14)

### S14 = 150

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S15)

### S15 = 151

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S16)

### S16 = 152

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S17)

### S17 = 153

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S18)

### S18 = 154

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S19)

### S19 = 155

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S20)

### S20 = 156

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S21)

### S21 = 157

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S22)

### S22 = 158

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S23)

### S23 = 159

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S24)

### S24 = 160

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S25)

### S25 = 161

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S26)

### S26 = 162

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S27)

### S27 = 163

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S28)

### S28 = 164

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S29)

### S29 = 165

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S30)

### S30 = 166

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.S31)

### S31 = 167

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W0)

### W0 = 168

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W1)

### W1 = 169

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W2)

### W2 = 170

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W3)

### W3 = 171

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W4)

### W4 = 172

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W5)

### W5 = 173

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W6)

### W6 = 174

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W7)

### W7 = 175

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W8)

### W8 = 176

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W9)

### W9 = 177

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W10)

### W10 = 178

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W11)

### W11 = 179

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W12)

### W12 = 180

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W13)

### W13 = 181

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W14)

### W14 = 182

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W15)

### W15 = 183

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W16)

### W16 = 184

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W17)

### W17 = 185

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W18)

### W18 = 186

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W19)

### W19 = 187

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W20)

### W20 = 188

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W21)

### W21 = 189

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W22)

### W22 = 190

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W23)

### W23 = 191

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W24)

### W24 = 192

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W25)

### W25 = 193

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W26)

### W26 = 194

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W27)

### W27 = 195

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W28)

### W28 = 196

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W29)

### W29 = 197

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.W30)

### W30 = 198

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X0)

### X0 = 199

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X1)

### X1 = 200

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X2)

### X2 = 201

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X3)

### X3 = 202

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X4)

### X4 = 203

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X5)

### X5 = 204

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X6)

### X6 = 205

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X7)

### X7 = 206

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X8)

### X8 = 207

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X9)

### X9 = 208

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X10)

### X10 = 209

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X11)

### X11 = 210

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X12)

### X12 = 211

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X13)

### X13 = 212

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X14)

### X14 = 213

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X15)

### X15 = 214

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X16)

### X16 = 215

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X17)

### X17 = 216

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X18)

### X18 = 217

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X19)

### X19 = 218

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X20)

### X20 = 219

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X21)

### X21 = 220

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X22)

### X22 = 221

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X23)

### X23 = 222

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X24)

### X24 = 223

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X25)

### X25 = 224

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X26)

### X26 = 225

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X27)

### X27 = 226

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.X28)

### X28 = 227

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V0)

### V0 = 228

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V1)

### V1 = 229

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V2)

### V2 = 230

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V3)

### V3 = 231

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V4)

### V4 = 232

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V5)

### V5 = 233

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V6)

### V6 = 234

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V7)

### V7 = 235

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V8)

### V8 = 236

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V9)

### V9 = 237

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V10)

### V10 = 238

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V11)

### V11 = 239

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V12)

### V12 = 240

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V13)

### V13 = 241

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V14)

### V14 = 242

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V15)

### V15 = 243

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V16)

### V16 = 244

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V17)

### V17 = 245

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V18)

### V18 = 246

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V19)

### V19 = 247

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V20)

### V20 = 248

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V21)

### V21 = 249

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V22)

### V22 = 250

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V23)

### V23 = 251

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V24)

### V24 = 252

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V25)

### V25 = 253

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V26)

### V26 = 254

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V27)

### V27 = 255

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V28)

### V28 = 256

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V29)

### V29 = 257

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V30)

### V30 = 258

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.V31)

### V31 = 259

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.PC)

### PC = 260

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.CPACR_EL1)

### CPACR\_EL1 = 261

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.TPIDR_EL0)

### TPIDR\_EL0 = 262

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.TPIDRRO_EL0)

### TPIDRRO\_EL0 = 263

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.TPIDR_EL1)

### TPIDR\_EL1 = 264

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.PSTATE)

### PSTATE = 265

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.ELR_EL0)

### ELR\_EL0 = 266

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.ELR_EL1)

### ELR\_EL1 = 267

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.ELR_EL2)

### ELR\_EL2 = 268

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.ELR_EL3)

### ELR\_EL3 = 269

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.SP_EL0)

### SP\_EL0 = 270

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.SP_EL1)

### SP\_EL1 = 271

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.SP_EL2)

### SP\_EL2 = 272

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.SP_EL3)

### SP\_EL3 = 273

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.TTBR0_EL1)

### TTBR0\_EL1 = 274

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.TTBR1_EL1)

### TTBR1\_EL1 = 275

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.ESR_EL0)

### ESR\_EL0 = 276

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.ESR_EL1)

### ESR\_EL1 = 277

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.ESR_EL2)

### ESR\_EL2 = 278

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.ESR_EL3)

### ESR\_EL3 = 279

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.FAR_EL0)

### FAR\_EL0 = 280

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.FAR_EL1)

### FAR\_EL1 = 281

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.FAR_EL2)

### FAR\_EL2 = 282

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.FAR_EL3)

### FAR\_EL3 = 283

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.PAR_EL1)

### PAR\_EL1 = 284

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.MAIR_EL1)

### MAIR\_EL1 = 285

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.VBAR_EL0)

### VBAR\_EL0 = 286

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.VBAR_EL1)

### VBAR\_EL1 = 287

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.VBAR_EL2)

### VBAR\_EL2 = 288

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.VBAR_EL3)

### VBAR\_EL3 = 289

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.CP_REG)

### CP\_REG = 290

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#variant.ENDING)

### ENDING = 291

## Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html\#implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#311-321) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-RegisterARM64)

### impl [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#317)

#### pub const [IP0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html\#associatedconstant.IP0): [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64") = RegisterARM64::X16

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#318)

#### pub const [IP1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html\#associatedconstant.IP1): [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64") = RegisterARM64::X17

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#319)

#### pub const [FP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html\#associatedconstant.FP): [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64") = RegisterARM64::X29

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#320)

#### pub const [LR](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html\#associatedconstant.LR): [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64") = RegisterARM64::X30

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Clone-for-RegisterARM64)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Debug-for-RegisterARM64)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#323-327) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-From%3CRegisterARM64%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#324-326) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(r: [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-PartialEq-for-RegisterARM64)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Copy-for-RegisterARM64)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm64.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-StructuralPartialEq-for-RegisterARM64)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Freeze-for-RegisterARM64)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-RefUnwindSafe-for-RegisterARM64)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Send-for-RegisterARM64)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Sync-for-RegisterARM64)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Unpin-for-RegisterARM64)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-UnwindSafe-for-RegisterARM64)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
