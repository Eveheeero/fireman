---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html"
title: "RegisterMIPS in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.RegisterMIPS.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.RegisterMIPS.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum RegisterMIPSCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#7-161)

```

#[repr(C)]pub enum RegisterMIPS {
Show 141 variants    INVALID = 0,
    PC = 1,
    R0 = 2,
    R1 = 3,
    R2 = 4,
    R3 = 5,
    R4 = 6,
    R5 = 7,
    R6 = 8,
    R7 = 9,
    R8 = 10,
    R9 = 11,
    R10 = 12,
    R11 = 13,
    R12 = 14,
    R13 = 15,
    R14 = 16,
    R15 = 17,
    R16 = 18,
    R17 = 19,
    R18 = 20,
    R19 = 21,
    R20 = 22,
    R21 = 23,
    R22 = 24,
    R23 = 25,
    R24 = 26,
    R25 = 27,
    R26 = 28,
    R27 = 29,
    R28 = 30,
    R29 = 31,
    R30 = 32,
    R31 = 33,
    DSPCCOND = 34,
    DSPCARRY = 35,
    DSPEFI = 36,
    DSPOUTFLAG = 37,
    DSPOUTFLAG16_19 = 38,
    DSPOUTFLAG20 = 39,
    DSPOUTFLAG21 = 40,
    DSPOUTFLAG22 = 41,
    DSPOUTFLAG23 = 42,
    DSPPOS = 43,
    DSPSCOUNT = 44,
    AC0 = 45,
    AC1 = 46,
    AC2 = 47,
    AC3 = 48,
    CC0 = 49,
    CC1 = 50,
    CC2 = 51,
    CC3 = 52,
    CC4 = 53,
    CC5 = 54,
    CC6 = 55,
    CC7 = 56,
    F0 = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    F13 = 70,
    F14 = 71,
    F15 = 72,
    F16 = 73,
    F17 = 74,
    F18 = 75,
    F19 = 76,
    F20 = 77,
    F21 = 78,
    F22 = 79,
    F23 = 80,
    F24 = 81,
    F25 = 82,
    F26 = 83,
    F27 = 84,
    F28 = 85,
    F29 = 86,
    F30 = 87,
    F31 = 88,
    FCC0 = 89,
    FCC1 = 90,
    FCC2 = 91,
    FCC3 = 92,
    FCC4 = 93,
    FCC5 = 94,
    FCC6 = 95,
    FCC7 = 96,
    W0 = 97,
    W1 = 98,
    W2 = 99,
    W3 = 100,
    W4 = 101,
    W5 = 102,
    W6 = 103,
    W7 = 104,
    W8 = 105,
    W9 = 106,
    W10 = 107,
    W11 = 108,
    W12 = 109,
    W13 = 110,
    W14 = 111,
    W15 = 112,
    W16 = 113,
    W17 = 114,
    W18 = 115,
    W19 = 116,
    W20 = 117,
    W21 = 118,
    W22 = 119,
    W23 = 120,
    W24 = 121,
    W25 = 122,
    W26 = 123,
    W27 = 124,
    W28 = 125,
    W29 = 126,
    W30 = 127,
    W31 = 128,
    HI = 129,
    LO = 130,
    P0 = 131,
    P1 = 132,
    P2 = 133,
    MPL0 = 134,
    MPL1 = 135,
    MPL2 = 136,
    CP0_CONFIG3 = 137,
    CP0_USERLOCAL = 138,
    CP0_STATUS = 139,
    ENDING = 140,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.INVALID)

### INVALID = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.PC)

### PC = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R0)

### R0 = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R1)

### R1 = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R2)

### R2 = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R3)

### R3 = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R4)

### R4 = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R5)

### R5 = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R6)

### R6 = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R7)

### R7 = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R8)

### R8 = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R9)

### R9 = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R10)

### R10 = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R11)

### R11 = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R12)

### R12 = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R13)

### R13 = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R14)

### R14 = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R15)

### R15 = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R16)

### R16 = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R17)

### R17 = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R18)

### R18 = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R19)

### R19 = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R20)

### R20 = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R21)

### R21 = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R22)

### R22 = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R23)

### R23 = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R24)

### R24 = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R25)

### R25 = 27

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R26)

### R26 = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R27)

### R27 = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R28)

### R28 = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R29)

### R29 = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R30)

### R30 = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.R31)

### R31 = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPCCOND)

### DSPCCOND = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPCARRY)

### DSPCARRY = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPEFI)

### DSPEFI = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPOUTFLAG)

### DSPOUTFLAG = 37

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPOUTFLAG16_19)

### DSPOUTFLAG16\_19 = 38

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPOUTFLAG20)

### DSPOUTFLAG20 = 39

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPOUTFLAG21)

### DSPOUTFLAG21 = 40

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPOUTFLAG22)

### DSPOUTFLAG22 = 41

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPOUTFLAG23)

### DSPOUTFLAG23 = 42

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPPOS)

### DSPPOS = 43

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.DSPSCOUNT)

### DSPSCOUNT = 44

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.AC0)

### AC0 = 45

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.AC1)

### AC1 = 46

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.AC2)

### AC2 = 47

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.AC3)

### AC3 = 48

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CC0)

### CC0 = 49

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CC1)

### CC1 = 50

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CC2)

### CC2 = 51

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CC3)

### CC3 = 52

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CC4)

### CC4 = 53

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CC5)

### CC5 = 54

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CC6)

### CC6 = 55

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CC7)

### CC7 = 56

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F0)

### F0 = 57

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F1)

### F1 = 58

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F2)

### F2 = 59

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F3)

### F3 = 60

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F4)

### F4 = 61

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F5)

### F5 = 62

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F6)

### F6 = 63

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F7)

### F7 = 64

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F8)

### F8 = 65

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F9)

### F9 = 66

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F10)

### F10 = 67

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F11)

### F11 = 68

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F12)

### F12 = 69

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F13)

### F13 = 70

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F14)

### F14 = 71

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F15)

### F15 = 72

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F16)

### F16 = 73

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F17)

### F17 = 74

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F18)

### F18 = 75

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F19)

### F19 = 76

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F20)

### F20 = 77

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F21)

### F21 = 78

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F22)

### F22 = 79

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F23)

### F23 = 80

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F24)

### F24 = 81

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F25)

### F25 = 82

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F26)

### F26 = 83

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F27)

### F27 = 84

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F28)

### F28 = 85

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F29)

### F29 = 86

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F30)

### F30 = 87

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.F31)

### F31 = 88

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.FCC0)

### FCC0 = 89

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.FCC1)

### FCC1 = 90

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.FCC2)

### FCC2 = 91

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.FCC3)

### FCC3 = 92

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.FCC4)

### FCC4 = 93

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.FCC5)

### FCC5 = 94

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.FCC6)

### FCC6 = 95

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.FCC7)

### FCC7 = 96

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W0)

### W0 = 97

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W1)

### W1 = 98

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W2)

### W2 = 99

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W3)

### W3 = 100

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W4)

### W4 = 101

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W5)

### W5 = 102

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W6)

### W6 = 103

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W7)

### W7 = 104

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W8)

### W8 = 105

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W9)

### W9 = 106

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W10)

### W10 = 107

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W11)

### W11 = 108

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W12)

### W12 = 109

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W13)

### W13 = 110

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W14)

### W14 = 111

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W15)

### W15 = 112

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W16)

### W16 = 113

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W17)

### W17 = 114

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W18)

### W18 = 115

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W19)

### W19 = 116

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W20)

### W20 = 117

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W21)

### W21 = 118

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W22)

### W22 = 119

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W23)

### W23 = 120

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W24)

### W24 = 121

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W25)

### W25 = 122

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W26)

### W26 = 123

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W27)

### W27 = 124

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W28)

### W28 = 125

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W29)

### W29 = 126

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W30)

### W30 = 127

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.W31)

### W31 = 128

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.HI)

### HI = 129

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.LO)

### LO = 130

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.P0)

### P0 = 131

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.P1)

### P1 = 132

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.P2)

### P2 = 133

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.MPL0)

### MPL0 = 134

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.MPL1)

### MPL1 = 135

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.MPL2)

### MPL2 = 136

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CP0_CONFIG3)

### CP0\_CONFIG3 = 137

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CP0_USERLOCAL)

### CP0\_USERLOCAL = 138

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.CP0_STATUS)

### CP0\_STATUS = 139

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#variant.ENDING)

### ENDING = 140

## Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#163-247) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-RegisterMIPS)

### impl [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#206)

#### pub const [ZERO](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.ZERO): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R0

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#207)

#### pub const [AT](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.AT): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R1

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#208)

#### pub const [V0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.V0): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R2

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#209)

#### pub const [V1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.V1): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R3

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#210)

#### pub const [A0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.A0): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R4

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#211)

#### pub const [A1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.A1): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R5

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#212)

#### pub const [A2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.A2): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R6

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#213)

#### pub const [A3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.A3): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R7

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#214)

#### pub const [T0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T0): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R8

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#215)

#### pub const [T1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T1): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R9

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#216)

#### pub const [T2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T2): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R10

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#217)

#### pub const [T3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T3): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R11

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#218)

#### pub const [T4](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T4): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R12

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#219)

#### pub const [T5](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T5): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R13

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#220)

#### pub const [T6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T6): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R14

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#221)

#### pub const [T7](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T7): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R15

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#222)

#### pub const [S0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.S0): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R16

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#223)

#### pub const [S1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.S1): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R17

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#224)

#### pub const [S2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.S2): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R18

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#225)

#### pub const [S3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.S3): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R19

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#226)

#### pub const [S4](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.S4): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R20

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#227)

#### pub const [S5](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.S5): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R21

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#228)

#### pub const [S6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.S6): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R22

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#229)

#### pub const [S7](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.S7): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R23

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#230)

#### pub const [T8](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T8): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R24

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#231)

#### pub const [T9](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.T9): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R25

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#232)

#### pub const [K0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.K0): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R26

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#233)

#### pub const [K1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.K1): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R27

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#234)

#### pub const [GP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.GP): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R28

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#235)

#### pub const [SP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.SP): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R29

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#236)

#### pub const [FP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.FP): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R30

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#237)

#### pub const [S8](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.S8): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R30

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#238)

#### pub const [RA](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.RA): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::R31

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#239)

#### pub const [HI0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.HI0): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::AC0

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#240)

#### pub const [HI1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.HI1): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::AC1

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#241)

#### pub const [HI2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.HI2): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::AC2

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#242)

#### pub const [HI3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.HI3): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::AC3

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#243)

#### pub const [LO0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.LO0): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::AC0

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#244)

#### pub const [LO1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.LO1): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::AC1

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#245)

#### pub const [LO2](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.LO2): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::AC2

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#246)

#### pub const [LO3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#associatedconstant.LO3): [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") = RegisterMIPS::AC3

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Clone-for-RegisterMIPS)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Debug-for-RegisterMIPS)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#249-253) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-From%3CRegisterMIPS%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#250-252) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(r: [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-PartialEq-for-RegisterMIPS)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Copy-for-RegisterMIPS)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/mips.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-StructuralPartialEq-for-RegisterMIPS)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Freeze-for-RegisterMIPS)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-RefUnwindSafe-for-RegisterMIPS)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Send-for-RegisterMIPS)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Sync-for-RegisterMIPS)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Unpin-for-RegisterMIPS)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-UnwindSafe-for-RegisterMIPS)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
