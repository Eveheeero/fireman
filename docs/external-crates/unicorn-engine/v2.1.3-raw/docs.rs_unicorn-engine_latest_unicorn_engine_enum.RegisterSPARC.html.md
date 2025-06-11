---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html"
title: "RegisterSPARC in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.RegisterSPARC.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.RegisterSPARC.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum RegisterSPARCCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#7-98)

```

#[repr(C)]pub enum RegisterSPARC {
Show 90 variants    INVALID = 0,
    F0 = 1,
    F1 = 2,
    F2 = 3,
    F3 = 4,
    F4 = 5,
    F5 = 6,
    F6 = 7,
    F7 = 8,
    F8 = 9,
    F9 = 10,
    F10 = 11,
    F11 = 12,
    F12 = 13,
    F13 = 14,
    F14 = 15,
    F15 = 16,
    F16 = 17,
    F17 = 18,
    F18 = 19,
    F19 = 20,
    F20 = 21,
    F21 = 22,
    F22 = 23,
    F23 = 24,
    F24 = 25,
    F25 = 26,
    F26 = 27,
    F27 = 28,
    F28 = 29,
    F29 = 30,
    F30 = 31,
    F31 = 32,
    F32 = 33,
    F34 = 34,
    F36 = 35,
    F38 = 36,
    F40 = 37,
    F42 = 38,
    F44 = 39,
    F46 = 40,
    F48 = 41,
    F50 = 42,
    F52 = 43,
    F54 = 44,
    F56 = 45,
    F58 = 46,
    F60 = 47,
    F62 = 48,
    FCC0 = 49,
    FCC1 = 50,
    FCC2 = 51,
    FCC3 = 52,
    G0 = 53,
    G1 = 54,
    G2 = 55,
    G3 = 56,
    G4 = 57,
    G5 = 58,
    G6 = 59,
    G7 = 60,
    I0 = 61,
    I1 = 62,
    I2 = 63,
    I3 = 64,
    I4 = 65,
    I5 = 66,
    FP = 67,
    I7 = 68,
    ICC = 69,
    L0 = 70,
    L1 = 71,
    L2 = 72,
    L3 = 73,
    L4 = 74,
    L5 = 75,
    L6 = 76,
    L7 = 77,
    O0 = 78,
    O1 = 79,
    O2 = 80,
    O3 = 81,
    O4 = 82,
    O5 = 83,
    SP = 84,
    O7 = 85,
    Y = 86,
    XCC = 87,
    PC = 88,
    ENDING = 89,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.INVALID)

### INVALID = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F0)

### F0 = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F1)

### F1 = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F2)

### F2 = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F3)

### F3 = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F4)

### F4 = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F5)

### F5 = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F6)

### F6 = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F7)

### F7 = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F8)

### F8 = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F9)

### F9 = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F10)

### F10 = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F11)

### F11 = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F12)

### F12 = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F13)

### F13 = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F14)

### F14 = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F15)

### F15 = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F16)

### F16 = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F17)

### F17 = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F18)

### F18 = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F19)

### F19 = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F20)

### F20 = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F21)

### F21 = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F22)

### F22 = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F23)

### F23 = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F24)

### F24 = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F25)

### F25 = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F26)

### F26 = 27

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F27)

### F27 = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F28)

### F28 = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F29)

### F29 = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F30)

### F30 = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F31)

### F31 = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F32)

### F32 = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F34)

### F34 = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F36)

### F36 = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F38)

### F38 = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F40)

### F40 = 37

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F42)

### F42 = 38

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F44)

### F44 = 39

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F46)

### F46 = 40

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F48)

### F48 = 41

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F50)

### F50 = 42

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F52)

### F52 = 43

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F54)

### F54 = 44

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F56)

### F56 = 45

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F58)

### F58 = 46

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F60)

### F60 = 47

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.F62)

### F62 = 48

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.FCC0)

### FCC0 = 49

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.FCC1)

### FCC1 = 50

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.FCC2)

### FCC2 = 51

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.FCC3)

### FCC3 = 52

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.G0)

### G0 = 53

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.G1)

### G1 = 54

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.G2)

### G2 = 55

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.G3)

### G3 = 56

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.G4)

### G4 = 57

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.G5)

### G5 = 58

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.G6)

### G6 = 59

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.G7)

### G7 = 60

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.I0)

### I0 = 61

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.I1)

### I1 = 62

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.I2)

### I2 = 63

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.I3)

### I3 = 64

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.I4)

### I4 = 65

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.I5)

### I5 = 66

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.FP)

### FP = 67

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.I7)

### I7 = 68

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.ICC)

### ICC = 69

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.L0)

### L0 = 70

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.L1)

### L1 = 71

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.L2)

### L2 = 72

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.L3)

### L3 = 73

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.L4)

### L4 = 74

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.L5)

### L5 = 75

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.L6)

### L6 = 76

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.L7)

### L7 = 77

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.O0)

### O0 = 78

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.O1)

### O1 = 79

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.O2)

### O2 = 80

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.O3)

### O3 = 81

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.O4)

### O4 = 82

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.O5)

### O5 = 83

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.SP)

### SP = 84

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.O7)

### O7 = 85

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.Y)

### Y = 86

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.XCC)

### XCC = 87

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.PC)

### PC = 88

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#variant.ENDING)

### ENDING = 89

## Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html\#implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#100-106) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-RegisterSPARC)

### impl [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#104)

#### pub const [O6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html\#associatedconstant.O6): [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC") = RegisterSPARC::SP

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#105)

#### pub const [I6](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html\#associatedconstant.I6): [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC") = RegisterSPARC::FP

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Clone-for-RegisterSPARC)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Debug-for-RegisterSPARC)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#108-112) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-From%3CRegisterSPARC%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#109-111) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(r: [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-PartialEq-for-RegisterSPARC)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Copy-for-RegisterSPARC)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/sparc.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-StructuralPartialEq-for-RegisterSPARC)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Freeze-for-RegisterSPARC)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-RefUnwindSafe-for-RegisterSPARC)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Send-for-RegisterSPARC)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Sync-for-RegisterSPARC)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Unpin-for-RegisterSPARC)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-UnwindSafe-for-RegisterSPARC)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
