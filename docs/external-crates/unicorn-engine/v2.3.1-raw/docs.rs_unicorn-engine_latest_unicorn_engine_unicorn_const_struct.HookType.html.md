---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html"
title: "HookType in unicorn_engine::unicorn_const - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/unicorn_const/struct.HookType.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/unicorn_const/struct.HookType.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html):: [unicorn\_const](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/index.html)

# Struct HookTypeCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

```

#[repr(C)]pub struct HookType(/* private fields */);
```

## Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-HookType)

### impl [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [INTR](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.INTR): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [INSN](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.INSN): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [CODE](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.CODE): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [BLOCK](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.BLOCK): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_READ\_UNMAPPED](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_READ_UNMAPPED): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_WRITE\_UNMAPPED](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_WRITE_UNMAPPED): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_FETCH\_UNMAPPED](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_FETCH_UNMAPPED): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_UNMAPPED](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_UNMAPPED): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_READ\_PROT](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_READ_PROT): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_WRITE\_PROT](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_WRITE_PROT): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_FETCH\_PROT](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_FETCH_PROT): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_PROT](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_PROT): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_READ](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_READ): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_WRITE](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_WRITE): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_FETCH](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_FETCH): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_VALID](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_VALID): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_READ\_AFTER](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_READ_AFTER): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [INSN\_INVALID](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.INSN_INVALID): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_READ\_INVALID](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_READ_INVALID): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_WRITE\_INVALID](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_WRITE_INVALID): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_FETCH\_INVALID](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_FETCH_INVALID): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_INVALID](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_INVALID): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [MEM\_ALL](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.MEM_ALL): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const [TLB](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedconstant.TLB): Self

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-HookType-1)

### impl [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [empty](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.empty)() -\> Self

Get a flags value with all bits unset.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [all](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.all)() -\> Self

Get a flags value with all known bits set.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [bits](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.bits)(&self) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

Get the underlying bits value.

The returned value is exactly the bits set in this flags value.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [from\_bits](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.from_bits)(bits: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") <Self>

Convert from a bits value.

This method will return `None` if any unknown bits are set.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [from\_bits\_truncate](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.from_bits_truncate)(bits: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -\> Self

Convert from a bits value, unsetting any unknown bits.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [from\_bits\_retain](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.from_bits_retain)(bits: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -\> Self

Convert from a bits value exactly.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub fn [from\_name](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.from_name)(name: & [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") <Self>

Get a flags value with the bits of a flag with the given name set.

This method will return `None` if `name` is empty or doesn’t
correspond to any named flag.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [is\_empty](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all bits in this flags value are unset.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [is\_all](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.is_all)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all known bits in this flags value are set.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [intersects](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.intersects)(&self, other: Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether any set bits in a source flags value are also set in a target flags value.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [contains](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.contains)(&self, other: Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all set bits in a source flags value are also set in a target flags value.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub fn [insert](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.insert)(&mut self, other: Self)

The bitwise or ( `|`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub fn [remove](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.remove)(&mut self, other: Self)

The intersection of a source flags value with the complement of a target flags value ( `&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set.
`remove` won’t truncate `other`, but the `!` operator will.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub fn [toggle](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.toggle)(&mut self, other: Self)

The bitwise exclusive-or ( `^`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub fn [set](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.set)(&mut self, other: Self, value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

Call `insert` when `value` is `true` or `remove` when `value` is `false`.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [intersection](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.intersection)(self, other: Self) -> Self

The bitwise and ( `&`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [union](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.union)(self, other: Self) -> Self

The bitwise or ( `|`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [difference](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.difference)(self, other: Self) -> Self

The intersection of a source flags value with the complement of a target flags value ( `&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set.
`difference` won’t truncate `other`, but the `!` operator will.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [symmetric\_difference](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.symmetric_difference)(self, other: Self) -> Self

The bitwise exclusive-or ( `^`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [complement](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.complement)(self) -> Self

The bitwise negation ( `!`) of the bits in a flags value, truncating the result.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-HookType-2)

### impl [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [iter](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.iter)(&self) -> [Iter](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html "struct bitflags::iter::Iter") < [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType") >

Yield a set of contained flags values.

Each yielded flags value will correspond to a defined named flag. Any unknown bits
will be yielded together as a final flags value.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133)

#### pub const fn [iter\_names](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#method.iter_names)(&self) -> [IterNames](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html "struct bitflags::iter::IterNames") < [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType") >

Yield a set of contained named flags values.

This method is like [`iter`](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.iter), except only yields bits in contained named flags.
Any unknown bits, or bits not corresponding to a contained flag will not be yielded.

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Binary-for-HookType)

### impl [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-BitAnd-for-HookType)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.bitand)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\#tymethod.bitand)(self, other: Self) -> Self

The bitwise and ( `&`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Output-2)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\#associatedtype.Output) = [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-BitAndAssign-for-HookType)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.bitand_assign)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html\#tymethod.bitand_assign)(&mut self, other: Self)

The bitwise and ( `&`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-BitOr-for-HookType)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.bitor)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html\#tymethod.bitor)(self, other: [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")) -\> Self

The bitwise or ( `|`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Output)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html\#associatedtype.Output) = [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-BitOrAssign-for-HookType)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.bitor_assign)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html\#tymethod.bitor_assign)(&mut self, other: Self)

The bitwise or ( `|`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-BitXor-for-HookType)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.bitxor)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\#tymethod.bitxor)(self, other: Self) -> Self

The bitwise exclusive-or ( `^`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Output-1)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html\#associatedtype.Output) = [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-BitXorAssign-for-HookType)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.bitxor_assign)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html\#tymethod.bitxor_assign)(&mut self, other: Self)

The bitwise exclusive-or ( `^`) of the bits in two flags values.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#98) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Clone-for-HookType)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#98) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Extend%3CHookType%3E-for-HookType)

### impl [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend") < [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType") \> for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.extend)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\#tymethod.extend) <T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") <Item = Self>>(&mut self, iterator: T)

The bitwise or ( `|`) of the bits in each flags value.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.extend_one)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\#method.extend_one)(&mut self, item: A)

🔬This is a nightly-only experimental API. ( `extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.extend_reserve)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. ( `extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Flags-for-HookType)

### impl [Flags](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html "trait bitflags::traits::Flags") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedconstant.FLAGS)

#### const [FLAGS](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#associatedconstant.FLAGS): &'static \[ [Flag](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/struct.Flag.html "struct bitflags::traits::Flag") < [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType") >\]

The set of defined flags.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Bits)

#### type [Bits](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#associatedtype.Bits) = [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

The underlying bits type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.bits-1)

#### fn [bits](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#tymethod.bits)(&self) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

Get the underlying bits value. [Read more](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.bits)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.from_bits_retain-1)

#### fn [from\_bits\_retain](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#tymethod.from_bits_retain)(bits: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -\> [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

Convert from a bits value exactly.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#140) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.empty-1)

#### fn [empty](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.empty)() -\> Self

Get a flags value with all bits unset.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#145) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.all-1)

#### fn [all](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.all)() -\> Self

Get a flags value with all known bits set.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#156) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.contains_unknown_bits)

#### fn [contains\_unknown\_bits](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.contains_unknown_bits)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method will return `true` if any unknown bits are set.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#168) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.from_bits-1)

#### fn [from\_bits](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.from_bits)(bits: Self:: [Bits](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#associatedtype.Bits "type bitflags::traits::Flags::Bits")) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") <Self>

Convert from a bits value. [Read more](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits)

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#179) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.from_bits_truncate-1)

#### fn [from\_bits\_truncate](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.from_bits_truncate)(bits: Self:: [Bits](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#associatedtype.Bits "type bitflags::traits::Flags::Bits")) -\> Self

Convert from a bits value, unsetting any unknown bits.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#190) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.from_name-1)

#### fn [from\_name](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.from_name)(name: & [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") <Self>

Get a flags value with the bits of a flag with the given name set. [Read more](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_name)

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.iter-1)

#### fn [iter](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.iter)(&self) -> [Iter](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html "struct bitflags::iter::Iter") <Self>

Yield a set of contained flags values. [Read more](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter)

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.iter_names-1)

#### fn [iter\_names](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.iter_names)(&self) -> [IterNames](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html "struct bitflags::iter::IterNames") <Self>

Yield a set of contained named flags values. [Read more](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_names)

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#222) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.is_empty-1)

#### fn [is\_empty](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all bits in this flags value are unset.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#227) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.is_all-1)

#### fn [is\_all](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.is_all)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all known bits in this flags value are set.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#234-236) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.intersects-1)

#### fn [intersects](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.intersects)(&self, other: Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)   where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Whether any set bits in a source flags value are also set in a target flags value.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#242-244) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.contains-1)

#### fn [contains](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.contains)(&self, other: Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)   where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Whether all set bits in a source flags value are also set in a target flags value.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#250-252) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.truncate)

#### fn [truncate](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.truncate)(&mut self)  where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove any unknown bits from the flags.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#258-260) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.insert-1)

#### fn [insert](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.insert)(&mut self, other: Self)  where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The bitwise or ( `|`) of the bits in two flags values.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#269-271) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.remove-1)

#### fn [remove](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.remove)(&mut self, other: Self)  where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The intersection of a source flags value with the complement of a target flags value ( `&!`). [Read more](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove)

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#277-279) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.toggle-1)

#### fn [toggle](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.toggle)(&mut self, other: Self)  where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The bitwise exclusive-or ( `^`) of the bits in two flags values.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#285-287) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.set-1)

#### fn [set](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.set)(&mut self, other: Self, value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))  where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Call [`Flags::insert`](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.insert "method bitflags::traits::Flags::insert") when `value` is `true` or [`Flags::remove`](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove "method bitflags::traits::Flags::remove") when `value` is `false`.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#297-299) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.clear)

#### fn [clear](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.clear)(&mut self)  where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Unsets all bits in the flags.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#306) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.intersection-1)

#### fn [intersection](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.intersection)(self, other: Self) -> Self

The bitwise and ( `&`) of the bits in two flags values.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#312) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.union-1)

#### fn [union](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.union)(self, other: Self) -> Self

The bitwise or ( `|`) of the bits in two flags values.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#321) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.difference-1)

#### fn [difference](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.difference)(self, other: Self) -> Self

The intersection of a source flags value with the complement of a target flags value ( `&!`). [Read more](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.difference)

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#327) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.symmetric_difference-1)

#### fn [symmetric\_difference](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.symmetric_difference)(self, other: Self) -> Self

The bitwise exclusive-or ( `^`) of the bits in two flags values.

[Source](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#333) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.complement-1)

#### fn [complement](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\#method.complement)(self) -> Self

The bitwise negation ( `!`) of the bits in a flags value, truncating the result.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-FromIterator%3CHookType%3E-for-HookType)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator") < [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType") \> for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.from_iter)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html\#tymethod.from_iter) <T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") <Item = Self>>(iterator: T) -> Self

The bitwise or ( `|`) of the bits in each flags value.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-IntoIterator-for-HookType)

### impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Item)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\#associatedtype.Item) = [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

The type of the elements being iterated over.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.IntoIter)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\#associatedtype.IntoIter) = [Iter](https://docs.rs/bitflags/2.9.0/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html "struct bitflags::iter::Iter") < [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType") >

Which kind of iterator are we turning this into?

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.into_iter)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\#tymethod.into_iter)(self) -> Self:: [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-LowerHex-for-HookType)

### impl [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.fmt-2)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Not-for-HookType)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.not)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html\#tymethod.not)(self) -> Self

The bitwise negation ( `!`) of the bits in a flags value, truncating the result.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Output-4)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html\#associatedtype.Output) = [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Octal-for-HookType)

### impl [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.fmt-1)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-PublicFlags-for-HookType)

### impl PublicFlags for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Primitive)

#### type [Primitive](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedtype.Primitive) = [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

The type of the underlying storage.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Internal)

#### type [Internal](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#associatedtype.Internal) = InternalBitFlags

The type of the internal field on the generated flags type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Sub-for-HookType)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.sub)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html\#tymethod.sub)(self, other: Self) -> Self

The intersection of a source flags value with the complement of a target flags value ( `&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set.
`difference` won’t truncate `other`, but the `!` operator will.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Output-3)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html\#associatedtype.Output) = [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-SubAssign-for-HookType)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.sub_assign)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html\#tymethod.sub_assign)(&mut self, other: Self)

The intersection of a source flags value with the complement of a target flags value ( `&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set.
`difference` won’t truncate `other`, but the `!` operator will.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-UpperHex-for-HookType)

### impl [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#96-133) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.fmt-3)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/unicorn_const.rs.html#98) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Copy-for-HookType)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Freeze-for-HookType)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-RefUnwindSafe-for-HookType)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Send-for-HookType)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Sync-for-HookType)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Unpin-for-HookType)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-UnwindSafe-for-HookType)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
