---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html"
title: "RegisterTRICORE in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.RegisterTRICORE.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.RegisterTRICORE.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum RegisterTRICORECopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#6-112)

```

#[repr(C)]pub enum RegisterTRICORE {
Show 105 variants    INVALID = 0,
    A0 = 1,
    A1 = 2,
    A2 = 3,
    A3 = 4,
    A4 = 5,
    A5 = 6,
    A6 = 7,
    A7 = 8,
    A8 = 9,
    A9 = 10,
    A10 = 11,
    A11 = 12,
    A12 = 13,
    A13 = 14,
    A14 = 15,
    A15 = 16,
    D0 = 17,
    D1 = 18,
    D2 = 19,
    D3 = 20,
    D4 = 21,
    D5 = 22,
    D6 = 23,
    D7 = 24,
    D8 = 25,
    D9 = 26,
    D10 = 27,
    D11 = 28,
    D12 = 29,
    D13 = 30,
    D14 = 31,
    D15 = 32,
    PCXI = 33,
    PSW = 34,
    PSW_USB_C = 35,
    PSW_USB_V = 36,
    PSW_USB_SV = 37,
    PSW_USB_AV = 38,
    PSW_USB_SAV = 39,
    PC = 40,
    SYSCON = 41,
    CPU_ID = 42,
    BIV = 43,
    BTV = 44,
    ISP = 45,
    ICR = 46,
    FCX = 47,
    LCX = 48,
    COMPAT = 49,
    DPR0_U = 50,
    DPR1_U = 51,
    DPR2_U = 52,
    DPR3_U = 53,
    DPR0_L = 54,
    DPR1_L = 55,
    DPR2_L = 56,
    DPR3_L = 57,
    CPR0_U = 58,
    CPR1_U = 59,
    CPR2_U = 60,
    CPR3_U = 61,
    CPR0_L = 62,
    CPR1_L = 63,
    CPR2_L = 64,
    CPR3_L = 65,
    DPM0 = 66,
    DPM1 = 67,
    DPM2 = 68,
    DPM3 = 69,
    CPM0 = 70,
    CPM1 = 71,
    CPM2 = 72,
    CPM3 = 73,
    MMU_CON = 74,
    MMU_ASI = 75,
    MMU_TVA = 76,
    MMU_TPA = 77,
    MMU_TPX = 78,
    MMU_TFA = 79,
    BMACON = 80,
    SMACON = 81,
    DIEAR = 82,
    DIETR = 83,
    CCDIER = 84,
    MIECON = 85,
    PIEAR = 86,
    PIETR = 87,
    CCPIER = 88,
    DBGSR = 89,
    EXEVT = 90,
    CREVT = 91,
    SWEVT = 92,
    TR0EVT = 93,
    TR1EVT = 94,
    DMS = 95,
    DCX = 96,
    DBGTCR = 97,
    CCTRL = 98,
    CCNT = 99,
    ICNT = 100,
    M1CNT = 101,
    M2CNT = 102,
    M3CNT = 103,
    ENDING = 104,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.INVALID)

### INVALID = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A0)

### A0 = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A1)

### A1 = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A2)

### A2 = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A3)

### A3 = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A4)

### A4 = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A5)

### A5 = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A6)

### A6 = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A7)

### A7 = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A8)

### A8 = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A9)

### A9 = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A10)

### A10 = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A11)

### A11 = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A12)

### A12 = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A13)

### A13 = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A14)

### A14 = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.A15)

### A15 = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D0)

### D0 = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D1)

### D1 = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D2)

### D2 = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D3)

### D3 = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D4)

### D4 = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D5)

### D5 = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D6)

### D6 = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D7)

### D7 = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D8)

### D8 = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D9)

### D9 = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D10)

### D10 = 27

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D11)

### D11 = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D12)

### D12 = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D13)

### D13 = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D14)

### D14 = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.D15)

### D15 = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PCXI)

### PCXI = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PSW)

### PSW = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PSW_USB_C)

### PSW\_USB\_C = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PSW_USB_V)

### PSW\_USB\_V = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PSW_USB_SV)

### PSW\_USB\_SV = 37

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PSW_USB_AV)

### PSW\_USB\_AV = 38

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PSW_USB_SAV)

### PSW\_USB\_SAV = 39

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PC)

### PC = 40

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.SYSCON)

### SYSCON = 41

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPU_ID)

### CPU\_ID = 42

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.BIV)

### BIV = 43

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.BTV)

### BTV = 44

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.ISP)

### ISP = 45

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.ICR)

### ICR = 46

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.FCX)

### FCX = 47

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.LCX)

### LCX = 48

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.COMPAT)

### COMPAT = 49

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPR0_U)

### DPR0\_U = 50

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPR1_U)

### DPR1\_U = 51

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPR2_U)

### DPR2\_U = 52

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPR3_U)

### DPR3\_U = 53

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPR0_L)

### DPR0\_L = 54

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPR1_L)

### DPR1\_L = 55

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPR2_L)

### DPR2\_L = 56

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPR3_L)

### DPR3\_L = 57

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPR0_U)

### CPR0\_U = 58

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPR1_U)

### CPR1\_U = 59

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPR2_U)

### CPR2\_U = 60

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPR3_U)

### CPR3\_U = 61

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPR0_L)

### CPR0\_L = 62

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPR1_L)

### CPR1\_L = 63

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPR2_L)

### CPR2\_L = 64

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPR3_L)

### CPR3\_L = 65

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPM0)

### DPM0 = 66

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPM1)

### DPM1 = 67

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPM2)

### DPM2 = 68

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DPM3)

### DPM3 = 69

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPM0)

### CPM0 = 70

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPM1)

### CPM1 = 71

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPM2)

### CPM2 = 72

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CPM3)

### CPM3 = 73

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.MMU_CON)

### MMU\_CON = 74

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.MMU_ASI)

### MMU\_ASI = 75

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.MMU_TVA)

### MMU\_TVA = 76

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.MMU_TPA)

### MMU\_TPA = 77

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.MMU_TPX)

### MMU\_TPX = 78

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.MMU_TFA)

### MMU\_TFA = 79

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.BMACON)

### BMACON = 80

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.SMACON)

### SMACON = 81

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DIEAR)

### DIEAR = 82

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DIETR)

### DIETR = 83

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CCDIER)

### CCDIER = 84

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.MIECON)

### MIECON = 85

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PIEAR)

### PIEAR = 86

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.PIETR)

### PIETR = 87

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CCPIER)

### CCPIER = 88

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DBGSR)

### DBGSR = 89

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.EXEVT)

### EXEVT = 90

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CREVT)

### CREVT = 91

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.SWEVT)

### SWEVT = 92

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.TR0EVT)

### TR0EVT = 93

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.TR1EVT)

### TR1EVT = 94

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DMS)

### DMS = 95

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DCX)

### DCX = 96

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.DBGTCR)

### DBGTCR = 97

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CCTRL)

### CCTRL = 98

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.CCNT)

### CCNT = 99

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.ICNT)

### ICNT = 100

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.M1CNT)

### M1CNT = 101

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.M2CNT)

### M2CNT = 102

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.M3CNT)

### M3CNT = 103

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#variant.ENDING)

### ENDING = 104

## Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#114-132) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-RegisterTRICORE)

### impl [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#124)

#### pub const [GA0](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#associatedconstant.GA0): [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE") = RegisterTRICORE::A0

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#125)

#### pub const [GA1](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#associatedconstant.GA1): [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE") = RegisterTRICORE::A1

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#126)

#### pub const [GA8](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#associatedconstant.GA8): [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE") = RegisterTRICORE::A8

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#127)

#### pub const [GA9](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#associatedconstant.GA9): [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE") = RegisterTRICORE::A9

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#128)

#### pub const [SP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#associatedconstant.SP): [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE") = RegisterTRICORE::A10

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#129)

#### pub const [LR](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#associatedconstant.LR): [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE") = RegisterTRICORE::A11

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#130)

#### pub const [IA](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#associatedconstant.IA): [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE") = RegisterTRICORE::A15

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#131)

#### pub const [ID](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#associatedconstant.ID): [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE") = RegisterTRICORE::D15

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Clone-for-RegisterTRICORE)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Debug-for-RegisterTRICORE)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#134-138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-From%3CRegisterTRICORE%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#135-137) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(r: [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-PartialEq-for-RegisterTRICORE)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Copy-for-RegisterTRICORE)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/tricore.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-StructuralPartialEq-for-RegisterTRICORE)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Freeze-for-RegisterTRICORE)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-RefUnwindSafe-for-RegisterTRICORE)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Send-for-RegisterTRICORE)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Sync-for-RegisterTRICORE)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Unpin-for-RegisterTRICORE)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-UnwindSafe-for-RegisterTRICORE)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
