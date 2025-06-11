---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html"
title: "UcHook in unicorn_engine::ffi - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/ffi/struct.UcHook.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/ffi/struct.UcHook.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html):: [ffi](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/index.html)

# Struct UcHookCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ffi.rs.html#106-109)

```
pub struct UcHook<'a, D: 'a, F: 'a> {
    pub callback: F,
    pub uc: Weak<UnsafeCell<UnicornInner<'a, D>>>,
}
```

## Fields [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html\#fields)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#structfield.callback) `callback: F`[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#structfield.uc) `uc: Weak<UnsafeCell<UnicornInner<'a, D>>>`

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ffi.rs.html#113) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-IsUcHook%3C'a%3E-for-UcHook%3C'a,+D,+F%3E)

### impl<'a, D, F> [IsUcHook](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/trait.IsUcHook.html "trait unicorn_engine::ffi::IsUcHook") <'a> for [UcHook](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html "struct unicorn_engine::ffi::UcHook") <'a, D, F>

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-Freeze-for-UcHook%3C'a,+D,+F%3E)

### impl<'a, D, F> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [UcHook](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html "struct unicorn_engine::ffi::UcHook") <'a, D, F>  where F: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"),

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-RefUnwindSafe-for-UcHook%3C'a,+D,+F%3E)

### impl<'a, D, F> ! [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [UcHook](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html "struct unicorn_engine::ffi::UcHook") <'a, D, F>

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-Send-for-UcHook%3C'a,+D,+F%3E)

### impl<'a, D, F> ! [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [UcHook](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html "struct unicorn_engine::ffi::UcHook") <'a, D, F>

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-Sync-for-UcHook%3C'a,+D,+F%3E)

### impl<'a, D, F> ! [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [UcHook](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html "struct unicorn_engine::ffi::UcHook") <'a, D, F>

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-Unpin-for-UcHook%3C'a,+D,+F%3E)

### impl<'a, D, F> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [UcHook](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html "struct unicorn_engine::ffi::UcHook") <'a, D, F>  where F: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-UnwindSafe-for-UcHook%3C'a,+D,+F%3E)

### impl<'a, D, F> ! [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [UcHook](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html "struct unicorn_engine::ffi::UcHook") <'a, D, F>

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/struct.UcHook.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
