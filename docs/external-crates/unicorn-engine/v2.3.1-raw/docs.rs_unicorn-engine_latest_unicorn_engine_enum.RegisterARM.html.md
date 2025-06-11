---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html"
title: "RegisterARM in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.RegisterARM.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.RegisterARM.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum RegisterARMCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#6-149)

```

#[repr(C)]pub enum RegisterARM {
Show 141 variants    INVALID = 0,
    APSR = 1,
    APSR_NZCV = 2,
    CPSR = 3,
    FPEXC = 4,
    FPINST = 5,
    FPSCR = 6,
    FPSCR_NZCV = 7,
    FPSID = 8,
    ITSTATE = 9,
    LR = 10,
    PC = 11,
    SP = 12,
    SPSR = 13,
    D0 = 14,
    D1 = 15,
    D2 = 16,
    D3 = 17,
    D4 = 18,
    D5 = 19,
    D6 = 20,
    D7 = 21,
    D8 = 22,
    D9 = 23,
    D10 = 24,
    D11 = 25,
    D12 = 26,
    D13 = 27,
    D14 = 28,
    D15 = 29,
    D16 = 30,
    D17 = 31,
    D18 = 32,
    D19 = 33,
    D20 = 34,
    D21 = 35,
    D22 = 36,
    D23 = 37,
    D24 = 38,
    D25 = 39,
    D26 = 40,
    D27 = 41,
    D28 = 42,
    D29 = 43,
    D30 = 44,
    D31 = 45,
    FPINST2 = 46,
    MVFR0 = 47,
    MVFR1 = 48,
    MVFR2 = 49,
    Q0 = 50,
    Q1 = 51,
    Q2 = 52,
    Q3 = 53,
    Q4 = 54,
    Q5 = 55,
    Q6 = 56,
    Q7 = 57,
    Q8 = 58,
    Q9 = 59,
    Q10 = 60,
    Q11 = 61,
    Q12 = 62,
    Q13 = 63,
    Q14 = 64,
    Q15 = 65,
    R0 = 66,
    R1 = 67,
    R2 = 68,
    R3 = 69,
    R4 = 70,
    R5 = 71,
    R6 = 72,
    R7 = 73,
    R8 = 74,
    R9 = 75,
    R10 = 76,
    R11 = 77,
    R12 = 78,
    S0 = 79,
    S1 = 80,
    S2 = 81,
    S3 = 82,
    S4 = 83,
    S5 = 84,
    S6 = 85,
    S7 = 86,
    S8 = 87,
    S9 = 88,
    S10 = 89,
    S11 = 90,
    S12 = 91,
    S13 = 92,
    S14 = 93,
    S15 = 94,
    S16 = 95,
    S17 = 96,
    S18 = 97,
    S19 = 98,
    S20 = 99,
    S21 = 100,
    S22 = 101,
    S23 = 102,
    S24 = 103,
    S25 = 104,
    S26 = 105,
    S27 = 106,
    S28 = 107,
    S29 = 108,
    S30 = 109,
    S31 = 110,
    C1_C0_2 = 111,
    C13_C0_2 = 112,
    C13_C0_3 = 113,
    IPSR = 114,
    MSP = 115,
    PSP = 116,
    CONTROL = 117,
    IAPSR = 118,
    EAPSR = 119,
    XPSR = 120,
    EPSR = 121,
    IEPSR = 122,
    PRIMASK = 123,
    BASEPRI = 124,
    BASEPRI_MAX = 125,
    FAULTMASK = 126,
    APSR_NZCVQ = 127,
    APSR_G = 128,
    APSR_NZCVQG = 129,
    IAPSR_NZCVQ = 130,
    IAPSR_G = 131,
    IAPSR_NZCVQG = 132,
    EAPSR_NZCVQ = 133,
    EAPSR_G = 134,
    EAPSR_NZCVQG = 135,
    XPSR_NZCVQ = 136,
    XPSR_G = 137,
    XPSR_NZCVQG = 138,
    CP_REG = 139,
    ENDING = 140,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.INVALID)

### INVALID = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.APSR)

### APSR = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.APSR_NZCV)

### APSR\_NZCV = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.CPSR)

### CPSR = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.FPEXC)

### FPEXC = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.FPINST)

### FPINST = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.FPSCR)

### FPSCR = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.FPSCR_NZCV)

### FPSCR\_NZCV = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.FPSID)

### FPSID = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.ITSTATE)

### ITSTATE = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.LR)

### LR = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.PC)

### PC = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.SP)

### SP = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.SPSR)

### SPSR = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D0)

### D0 = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D1)

### D1 = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D2)

### D2 = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D3)

### D3 = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D4)

### D4 = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D5)

### D5 = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D6)

### D6 = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D7)

### D7 = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D8)

### D8 = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D9)

### D9 = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D10)

### D10 = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D11)

### D11 = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D12)

### D12 = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D13)

### D13 = 27

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D14)

### D14 = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D15)

### D15 = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D16)

### D16 = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D17)

### D17 = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D18)

### D18 = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D19)

### D19 = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D20)

### D20 = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D21)

### D21 = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D22)

### D22 = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D23)

### D23 = 37

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D24)

### D24 = 38

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D25)

### D25 = 39

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D26)

### D26 = 40

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D27)

### D27 = 41

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D28)

### D28 = 42

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D29)

### D29 = 43

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D30)

### D30 = 44

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.D31)

### D31 = 45

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.FPINST2)

### FPINST2 = 46

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.MVFR0)

### MVFR0 = 47

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.MVFR1)

### MVFR1 = 48

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.MVFR2)

### MVFR2 = 49

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q0)

### Q0 = 50

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q1)

### Q1 = 51

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q2)

### Q2 = 52

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q3)

### Q3 = 53

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q4)

### Q4 = 54

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q5)

### Q5 = 55

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q6)

### Q6 = 56

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q7)

### Q7 = 57

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q8)

### Q8 = 58

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q9)

### Q9 = 59

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q10)

### Q10 = 60

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q11)

### Q11 = 61

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q12)

### Q12 = 62

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q13)

### Q13 = 63

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q14)

### Q14 = 64

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.Q15)

### Q15 = 65

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R0)

### R0 = 66

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R1)

### R1 = 67

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R2)

### R2 = 68

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R3)

### R3 = 69

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R4)

### R4 = 70

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R5)

### R5 = 71

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R6)

### R6 = 72

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R7)

### R7 = 73

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R8)

### R8 = 74

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R9)

### R9 = 75

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R10)

### R10 = 76

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R11)

### R11 = 77

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.R12)

### R12 = 78

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S0)

### S0 = 79

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S1)

### S1 = 80

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S2)

### S2 = 81

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S3)

### S3 = 82

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S4)

### S4 = 83

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S5)

### S5 = 84

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S6)

### S6 = 85

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S7)

### S7 = 86

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S8)

### S8 = 87

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S9)

### S9 = 88

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S10)

### S10 = 89

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S11)

### S11 = 90

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S12)

### S12 = 91

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S13)

### S13 = 92

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S14)

### S14 = 93

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S15)

### S15 = 94

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S16)

### S16 = 95

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S17)

### S17 = 96

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S18)

### S18 = 97

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S19)

### S19 = 98

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S20)

### S20 = 99

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S21)

### S21 = 100

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S22)

### S22 = 101

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S23)

### S23 = 102

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S24)

### S24 = 103

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S25)

### S25 = 104

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S26)

### S26 = 105

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S27)

### S27 = 106

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S28)

### S28 = 107

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S29)

### S29 = 108

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S30)

### S30 = 109

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.S31)

### S31 = 110

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.C1_C0_2)

### C1\_C0\_2 = 111

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.C13_C0_2)

### C13\_C0\_2 = 112

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.C13_C0_3)

### C13\_C0\_3 = 113

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.IPSR)

### IPSR = 114

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.MSP)

### MSP = 115

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.PSP)

### PSP = 116

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.CONTROL)

### CONTROL = 117

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.IAPSR)

### IAPSR = 118

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.EAPSR)

### EAPSR = 119

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.XPSR)

### XPSR = 120

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.EPSR)

### EPSR = 121

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.IEPSR)

### IEPSR = 122

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.PRIMASK)

### PRIMASK = 123

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.BASEPRI)

### BASEPRI = 124

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.BASEPRI_MAX)

### BASEPRI\_MAX = 125

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.FAULTMASK)

### FAULTMASK = 126

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.APSR_NZCVQ)

### APSR\_NZCVQ = 127

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.APSR_G)

### APSR\_G = 128

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.APSR_NZCVQG)

### APSR\_NZCVQG = 129

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.IAPSR_NZCVQ)

### IAPSR\_NZCVQ = 130

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.IAPSR_G)

### IAPSR\_G = 131

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.IAPSR_NZCVQG)

### IAPSR\_NZCVQG = 132

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.EAPSR_NZCVQ)

### EAPSR\_NZCVQ = 133

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.EAPSR_G)

### EAPSR\_G = 134

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.EAPSR_NZCVQG)

### EAPSR\_NZCVQG = 135

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.XPSR_NZCVQ)

### XPSR\_NZCVQ = 136

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.XPSR_G)

### XPSR\_G = 137

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.XPSR_NZCVQG)

### XPSR\_NZCVQG = 138

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.CP_REG)

### CP\_REG = 139

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#variant.ENDING)

### ENDING = 140

## Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#151-167) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-RegisterARM)

### impl [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#160)

#### pub const [R13](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#associatedconstant.R13): [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM") = RegisterARM::SP

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#161)

#### pub const [R14](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#associatedconstant.R14): [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM") = RegisterARM::LR

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#162)

#### pub const [R15](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#associatedconstant.R15): [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM") = RegisterARM::PC

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#163)

#### pub const [SB](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#associatedconstant.SB): [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM") = RegisterARM::R9

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#164)

#### pub const [SL](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#associatedconstant.SL): [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM") = RegisterARM::R10

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#165)

#### pub const [FP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#associatedconstant.FP): [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM") = RegisterARM::R11

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#166)

#### pub const [IP](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#associatedconstant.IP): [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM") = RegisterARM::R12

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Clone-for-RegisterARM)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Debug-for-RegisterARM)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#169-173) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-From%3CRegisterARM%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#170-172) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(r: [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-PartialEq-for-RegisterARM)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Copy-for-RegisterARM)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/arm.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-StructuralPartialEq-for-RegisterARM)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Freeze-for-RegisterARM)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-RefUnwindSafe-for-RegisterARM)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Send-for-RegisterARM)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Sync-for-RegisterARM)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Unpin-for-RegisterARM)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-UnwindSafe-for-RegisterARM)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
