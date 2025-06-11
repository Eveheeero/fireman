---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html"
title: "Arch in unicorn_engine::unicorn_const - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/unicorn_const/enum.Arch.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/unicorn_const/enum.Arch.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html):: [unicorn\_const](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/index.html)

# Enum ArchCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#167-179)

```

#[repr(C)]pub enum Arch {
    ARM = 1,
    ARM64 = 2,
    MIPS = 3,
    X86 = 4,
    PPC = 5,
    SPARC = 6,
    M68K = 7,
    RISCV = 8,
    S390X = 9,
    TRICORE = 10,
    MAX = 11,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.ARM)

### ARM = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.ARM64)

### ARM64 = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.MIPS)

### MIPS = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.X86)

### X86 = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.PPC)

### PPC = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.SPARC)

### SPARC = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.M68K)

### M68K = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.RISCV)

### RISCV = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.S390X)

### S390X = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.TRICORE)

### TRICORE = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#variant.MAX)

### MAX = 11

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#166) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Clone-for-Arch)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#166) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#166) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Debug-for-Arch)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#166) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#166) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-PartialEq-for-Arch)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#166) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#181-200) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-TryFrom%3Cusize%3E-for-Arch)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") < [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html) \> for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#182) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#184-199) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(v: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <Self, Self:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#166) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Copy-for-Arch)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#166) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-StructuralPartialEq-for-Arch)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Freeze-for-Arch)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-RefUnwindSafe-for-Arch)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Send-for-Arch)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Sync-for-Arch)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Unpin-for-Arch)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-UnwindSafe-for-Arch)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#associatedtype.Error-2)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.try_from-1)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
