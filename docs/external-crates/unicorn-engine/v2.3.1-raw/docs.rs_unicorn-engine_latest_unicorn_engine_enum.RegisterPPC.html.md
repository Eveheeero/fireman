---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html"
title: "RegisterPPC in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.RegisterPPC.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.RegisterPPC.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum RegisterPPCCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#7-89)

```

#[repr(C)]pub enum RegisterPPC {
Show 81 variants    INVALID = 0,
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
    CR0 = 34,
    CR1 = 35,
    CR2 = 36,
    CR3 = 37,
    CR4 = 38,
    CR5 = 39,
    CR6 = 40,
    CR7 = 41,
    FPR0 = 42,
    FPR1 = 43,
    FPR2 = 44,
    FPR3 = 45,
    FPR4 = 46,
    FPR5 = 47,
    FPR6 = 48,
    FPR7 = 49,
    FPR8 = 50,
    FPR9 = 51,
    FPR10 = 52,
    FPR11 = 53,
    FPR12 = 54,
    FPR13 = 55,
    FPR14 = 56,
    FPR15 = 57,
    FPR16 = 58,
    FPR17 = 59,
    FPR18 = 60,
    FPR19 = 61,
    FPR20 = 62,
    FPR21 = 63,
    FPR22 = 64,
    FPR23 = 65,
    FPR24 = 66,
    FPR25 = 67,
    FPR26 = 68,
    FPR27 = 69,
    FPR28 = 70,
    FPR29 = 71,
    FPR30 = 72,
    FPR31 = 73,
    LR = 74,
    XER = 75,
    CTR = 76,
    MSR = 77,
    FPSCR = 78,
    CR = 79,
    ENDING = 80,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.INVALID)

### INVALID = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.PC)

### PC = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R0)

### R0 = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R1)

### R1 = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R2)

### R2 = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R3)

### R3 = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R4)

### R4 = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R5)

### R5 = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R6)

### R6 = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R7)

### R7 = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R8)

### R8 = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R9)

### R9 = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R10)

### R10 = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R11)

### R11 = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R12)

### R12 = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R13)

### R13 = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R14)

### R14 = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R15)

### R15 = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R16)

### R16 = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R17)

### R17 = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R18)

### R18 = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R19)

### R19 = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R20)

### R20 = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R21)

### R21 = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R22)

### R22 = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R23)

### R23 = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R24)

### R24 = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R25)

### R25 = 27

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R26)

### R26 = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R27)

### R27 = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R28)

### R28 = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R29)

### R29 = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R30)

### R30 = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.R31)

### R31 = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CR0)

### CR0 = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CR1)

### CR1 = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CR2)

### CR2 = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CR3)

### CR3 = 37

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CR4)

### CR4 = 38

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CR5)

### CR5 = 39

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CR6)

### CR6 = 40

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CR7)

### CR7 = 41

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR0)

### FPR0 = 42

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR1)

### FPR1 = 43

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR2)

### FPR2 = 44

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR3)

### FPR3 = 45

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR4)

### FPR4 = 46

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR5)

### FPR5 = 47

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR6)

### FPR6 = 48

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR7)

### FPR7 = 49

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR8)

### FPR8 = 50

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR9)

### FPR9 = 51

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR10)

### FPR10 = 52

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR11)

### FPR11 = 53

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR12)

### FPR12 = 54

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR13)

### FPR13 = 55

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR14)

### FPR14 = 56

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR15)

### FPR15 = 57

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR16)

### FPR16 = 58

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR17)

### FPR17 = 59

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR18)

### FPR18 = 60

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR19)

### FPR19 = 61

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR20)

### FPR20 = 62

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR21)

### FPR21 = 63

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR22)

### FPR22 = 64

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR23)

### FPR23 = 65

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR24)

### FPR24 = 66

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR25)

### FPR25 = 67

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR26)

### FPR26 = 68

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR27)

### FPR27 = 69

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR28)

### FPR28 = 70

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR29)

### FPR29 = 71

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR30)

### FPR30 = 72

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPR31)

### FPR31 = 73

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.LR)

### LR = 74

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.XER)

### XER = 75

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CTR)

### CTR = 76

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.MSR)

### MSR = 77

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.FPSCR)

### FPSCR = 78

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.CR)

### CR = 79

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#variant.ENDING)

### ENDING = 80

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Clone-for-RegisterPPC)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Debug-for-RegisterPPC)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#91-95) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-From%3CRegisterPPC%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#92-94) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(r: [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-PartialEq-for-RegisterPPC)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Copy-for-RegisterPPC)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#6) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-StructuralPartialEq-for-RegisterPPC)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Freeze-for-RegisterPPC)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-RefUnwindSafe-for-RegisterPPC)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Send-for-RegisterPPC)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Sync-for-RegisterPPC)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Unpin-for-RegisterPPC)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-UnwindSafe-for-RegisterPPC)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
