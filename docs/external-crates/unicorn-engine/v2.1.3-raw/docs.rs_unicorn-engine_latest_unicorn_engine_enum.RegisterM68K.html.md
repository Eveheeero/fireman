---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html"
title: "RegisterM68K in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.RegisterM68K.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.RegisterM68K.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum RegisterM68KCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#6-27)

```

#[repr(C)]pub enum RegisterM68K {
Show 20 variants    INVALID = 0,
    A0 = 1,
    A1 = 2,
    A2 = 3,
    A3 = 4,
    A4 = 5,
    A5 = 6,
    A6 = 7,
    A7 = 8,
    D0 = 9,
    D1 = 10,
    D2 = 11,
    D3 = 12,
    D4 = 13,
    D5 = 14,
    D6 = 15,
    D7 = 16,
    SR = 17,
    PC = 18,
    ENDING = 19,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.INVALID)

### INVALID = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.A0)

### A0 = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.A1)

### A1 = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.A2)

### A2 = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.A3)

### A3 = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.A4)

### A4 = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.A5)

### A5 = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.A6)

### A6 = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.A7)

### A7 = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.D0)

### D0 = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.D1)

### D1 = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.D2)

### D2 = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.D3)

### D3 = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.D4)

### D4 = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.D5)

### D5 = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.D6)

### D6 = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.D7)

### D7 = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.SR)

### SR = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.PC)

### PC = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#variant.ENDING)

### ENDING = 19

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Clone-for-RegisterM68K)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Debug-for-RegisterM68K)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#29-33) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-From%3CRegisterM68K%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#30-32) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(r: [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-PartialEq-for-RegisterM68K)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Copy-for-RegisterM68K)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/m68k.rs.html#5) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-StructuralPartialEq-for-RegisterM68K)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Freeze-for-RegisterM68K)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-RefUnwindSafe-for-RegisterM68K)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Send-for-RegisterM68K)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Sync-for-RegisterM68K)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Unpin-for-RegisterM68K)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-UnwindSafe-for-RegisterM68K)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
