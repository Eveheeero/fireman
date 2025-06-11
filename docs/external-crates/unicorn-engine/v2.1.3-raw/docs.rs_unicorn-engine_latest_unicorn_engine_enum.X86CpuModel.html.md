---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html"
title: "X86CpuModel in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.X86CpuModel.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.X86CpuModel.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum X86CpuModelCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#286-325)

```

#[repr(i32)]pub enum X86CpuModel {
Show 38 variants    UC_CPU_X86_QEMU64 = 0,
    UC_CPU_X86_PHENOM = 1,
    UC_CPU_X86_CORE2DUO = 2,
    UC_CPU_X86_KVM64 = 3,
    UC_CPU_X86_QEMU32 = 4,
    UC_CPU_X86_KVM32 = 5,
    UC_CPU_X86_COREDUO = 6,
    UC_CPU_X86_486 = 7,
    UC_CPU_X86_PENTIUM = 8,
    UC_CPU_X86_PENTIUM2 = 9,
    UC_CPU_X86_PENTIUM3 = 10,
    UC_CPU_X86_ATHLON = 11,
    UC_CPU_X86_N270 = 12,
    UC_CPU_X86_CONROE = 13,
    UC_CPU_X86_PENRYN = 14,
    UC_CPU_X86_NEHALEM = 15,
    UC_CPU_X86_WESTMERE = 16,
    UC_CPU_X86_SANDYBRIDGE = 17,
    UC_CPU_X86_IVYBRIDGE = 18,
    UC_CPU_X86_HASWELL = 19,
    UC_CPU_X86_BROADWELL = 20,
    UC_CPU_X86_SKYLAKE_CLIENT = 21,
    UC_CPU_X86_SKYLAKE_SERVER = 22,
    UC_CPU_X86_CASCADELAKE_SERVER = 23,
    UC_CPU_X86_COOPERLAKE = 24,
    UC_CPU_X86_ICELAKE_CLIENT = 25,
    UC_CPU_X86_ICELAKE_SERVER = 26,
    UC_CPU_X86_DENVERTON = 27,
    UC_CPU_X86_SNOWRIDGE = 28,
    UC_CPU_X86_KNIGHTSMILL = 29,
    UC_CPU_X86_OPTERON_G1 = 30,
    UC_CPU_X86_OPTERON_G2 = 31,
    UC_CPU_X86_OPTERON_G3 = 32,
    UC_CPU_X86_OPTERON_G4 = 33,
    UC_CPU_X86_OPTERON_G5 = 34,
    UC_CPU_X86_EPYC = 35,
    UC_CPU_X86_DHYANA = 36,
    UC_CPU_X86_EPYC_ROME = 37,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_QEMU64)

### UC\_CPU\_X86\_QEMU64 = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_PHENOM)

### UC\_CPU\_X86\_PHENOM = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_CORE2DUO)

### UC\_CPU\_X86\_CORE2DUO = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_KVM64)

### UC\_CPU\_X86\_KVM64 = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_QEMU32)

### UC\_CPU\_X86\_QEMU32 = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_KVM32)

### UC\_CPU\_X86\_KVM32 = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_COREDUO)

### UC\_CPU\_X86\_COREDUO = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_486)

### UC\_CPU\_X86\_486 = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_PENTIUM)

### UC\_CPU\_X86\_PENTIUM = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_PENTIUM2)

### UC\_CPU\_X86\_PENTIUM2 = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_PENTIUM3)

### UC\_CPU\_X86\_PENTIUM3 = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_ATHLON)

### UC\_CPU\_X86\_ATHLON = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_N270)

### UC\_CPU\_X86\_N270 = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_CONROE)

### UC\_CPU\_X86\_CONROE = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_PENRYN)

### UC\_CPU\_X86\_PENRYN = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_NEHALEM)

### UC\_CPU\_X86\_NEHALEM = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_WESTMERE)

### UC\_CPU\_X86\_WESTMERE = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_SANDYBRIDGE)

### UC\_CPU\_X86\_SANDYBRIDGE = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_IVYBRIDGE)

### UC\_CPU\_X86\_IVYBRIDGE = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_HASWELL)

### UC\_CPU\_X86\_HASWELL = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_BROADWELL)

### UC\_CPU\_X86\_BROADWELL = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_SKYLAKE_CLIENT)

### UC\_CPU\_X86\_SKYLAKE\_CLIENT = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_SKYLAKE_SERVER)

### UC\_CPU\_X86\_SKYLAKE\_SERVER = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_CASCADELAKE_SERVER)

### UC\_CPU\_X86\_CASCADELAKE\_SERVER = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_COOPERLAKE)

### UC\_CPU\_X86\_COOPERLAKE = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_ICELAKE_CLIENT)

### UC\_CPU\_X86\_ICELAKE\_CLIENT = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_ICELAKE_SERVER)

### UC\_CPU\_X86\_ICELAKE\_SERVER = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_DENVERTON)

### UC\_CPU\_X86\_DENVERTON = 27

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_SNOWRIDGE)

### UC\_CPU\_X86\_SNOWRIDGE = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_KNIGHTSMILL)

### UC\_CPU\_X86\_KNIGHTSMILL = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_OPTERON_G1)

### UC\_CPU\_X86\_OPTERON\_G1 = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_OPTERON_G2)

### UC\_CPU\_X86\_OPTERON\_G2 = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_OPTERON_G3)

### UC\_CPU\_X86\_OPTERON\_G3 = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_OPTERON_G4)

### UC\_CPU\_X86\_OPTERON\_G4 = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_OPTERON_G5)

### UC\_CPU\_X86\_OPTERON\_G5 = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_EPYC)

### UC\_CPU\_X86\_EPYC = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_DHYANA)

### UC\_CPU\_X86\_DHYANA = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#variant.UC_CPU_X86_EPYC_ROME)

### UC\_CPU\_X86\_EPYC\_ROME = 37

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#285) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Clone-for-X86CpuModel)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#285) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#285) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Debug-for-X86CpuModel)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#285) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#333-337) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-From%3C%26X86CpuModel%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <& [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#334-336) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(value: & [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#327-331) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-From%3CX86CpuModel%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#328-330) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(value: [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#285) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-PartialEq-for-X86CpuModel)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#285) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#285) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Copy-for-X86CpuModel)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#285) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Eq-for-X86CpuModel)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/x86.rs.html#285) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-StructuralPartialEq-for-X86CpuModel)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Freeze-for-X86CpuModel)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-RefUnwindSafe-for-X86CpuModel)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Send-for-X86CpuModel)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Sync-for-X86CpuModel)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Unpin-for-X86CpuModel)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-UnwindSafe-for-X86CpuModel)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.from-2)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
