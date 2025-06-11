---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html"
title: "Unicorn in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/struct.Unicorn.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/struct.Unicorn.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Struct UnicornCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#208-210)

```
pub struct Unicorn<'a, D: 'a> { /* private fields */ }
```

Expand description

A Unicorn emulator instance.

## Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#212-245) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Unicorn%3C'a,+()%3E)

### impl<'a> [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#215-217)

#### pub fn [new](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.new)(arch: [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch"), mode: [Mode](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.Mode.html "struct unicorn_engine::unicorn_const::Mode")) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) >, [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Create a new instance of the unicorn engine for the specified architecture
and hardware mode.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#225-244)

#### pub unsafe fn [from\_handle](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.from_handle)(  handle: [uc\_handle](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/type.uc_handle.html "type unicorn_engine::ffi::uc_handle"), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) >, [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

##### [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#safety) Safety

The function has to be called with a valid uc\_handle pointer
that was previously allocated by a call to uc\_open.
Calling the function with a non null pointer value that
does not point to a unicorn instance will cause undefined
behavior.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#247-268) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Unicorn%3C'a,+D%3E)

### impl<'a, D> [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>  where D: 'a,

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#253-267)

#### pub fn [new\_with\_data](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.new_with_data)(  arch: [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch"),  mode: [Mode](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.Mode.html "struct unicorn_engine::unicorn_const::Mode"),  data: D, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>, [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Create a new instance of the unicorn engine for the specified architecture
and hardware mode.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#276-1291) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Unicorn%3C'a,+D%3E-1)

### impl<'a, D> [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#290-292)

#### pub fn [get\_data](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.get_data)(&self) -> [&D](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Return whatever data was passed during initialization.

For an example, have a look at `utils::init_emu_with_heap` where
a struct is passed which is used for a custom allocator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#296-298)

#### pub fn [get\_data\_mut](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.get_data_mut)(&mut self) -> [&mut D](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Return a mutable reference to whatever data was passed during initialization.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#302-304)

#### pub fn [get\_arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.get_arch)(&self) -> [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch")

Return the architecture of the current emulator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#308-310)

#### pub fn [get\_handle](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.get_handle)(&self) -> [uc\_handle](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/type.uc_handle.html "type unicorn_engine::ffi::uc_handle")

Return the handle of the current emulator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#313-326)

#### pub fn [mem\_regions](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mem_regions)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") < [MemRegion](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.MemRegion.html "struct unicorn_engine::unicorn_const::MemRegion") >, [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Returns a vector with the memory regions that are mapped in the emulator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#329-331)

#### pub fn [mem\_read](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mem_read)(&self, address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), buf: &mut \[ [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Read a range of bytes from memory at the specified emulated physical address.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#334-337)

#### pub fn [mem\_read\_as\_vec](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mem_read_as_vec)(  &self,  address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") < [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html) >, [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Return a range of bytes from memory at the specified emulated physical address as vector.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#340-342)

#### pub fn [mem\_write](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mem_write)(&mut self, address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), bytes: &\[ [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Write the data in `bytes` to the emulated physical address `address`

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#357-365)

#### pub unsafe fn [mem\_map\_ptr](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mem_map_ptr)(  &mut self,  address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html),  perms: [Permission](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.Permission.html "struct unicorn_engine::unicorn_const::Permission"),  ptr: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [c\_void](https://doc.rust-lang.org/nightly/core/ffi/enum.c_void.html "enum core::ffi::c_void"), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Map an existing memory region in the emulator at the specified address.

##### [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#safety-1) Safety

This function is marked unsafe because it is the responsibility of the caller to
ensure that `size` matches the size of the passed buffer, an invalid `size` value will
likely cause a crash in unicorn.

`address` must be aligned to 4kb or this will return `Error::ARG`.

`size` must be a multiple of 4kb or this will return `Error::ARG`.

`ptr` is a pointer to the provided memory region that will be used by the emulator.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#371-378)

#### pub fn [mem\_map](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mem_map)(  &mut self,  address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  size: [size\_t](https://docs.rs/libc/0.2.170/x86_64-unknown-linux-gnu/libc/unix/type.size_t.html "type libc::unix::size_t"),  perms: [Permission](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.Permission.html "struct unicorn_engine::unicorn_const::Permission"), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Map a memory region in the emulator at the specified address.

`address` must be aligned to 4kb or this will return `Error::ARG`.
`size` must be a multiple of 4kb or this will return `Error::ARG`.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#384-442)

#### pub fn [mmio\_map](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mmio_map) <R, W>(  &mut self,  address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  size: [size\_t](https://docs.rs/libc/0.2.170/x86_64-unknown-linux-gnu/libc/unix/type.size_t.html "type libc::unix::size_t"),  read\_callback: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") <R>,  write\_callback: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") <W>, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where R: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -\> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html) \+ 'a, W: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) \+ 'a,

Map in am MMIO region backed by callbacks.

`address` must be aligned to 4kb or this will return `Error::ARG`.
`size` must be a multiple of 4kb or this will return `Error::ARG`.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#448-463)

#### pub fn [mmio\_map\_ro](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mmio_map_ro) <F>(  &mut self,  address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  size: [size\_t](https://docs.rs/libc/0.2.170/x86_64-unknown-linux-gnu/libc/unix/type.size_t.html "type libc::unix::size_t"),  callback: F, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -\> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html) \+ 'a,

Map in a read-only MMIO region backed by a callback.

`address` must be aligned to 4kb or this will return `Error::ARG`.
`size` must be a multiple of 4kb or this will return `Error::ARG`.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#469-484)

#### pub fn [mmio\_map\_wo](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mmio_map_wo) <F>(  &mut self,  address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  size: [size\_t](https://docs.rs/libc/0.2.170/x86_64-unknown-linux-gnu/libc/unix/type.size_t.html "type libc::unix::size_t"),  callback: F, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) \+ 'a,

Map in a write-only MMIO region backed by a callback.

`address` must be aligned to 4kb or this will return `Error::ARG`.
`size` must be a multiple of 4kb or this will return `Error::ARG`.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#490-494)

#### pub fn [mem\_unmap](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mem_unmap)(&mut self, address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), size: [size\_t](https://docs.rs/libc/0.2.170/x86_64-unknown-linux-gnu/libc/unix/type.size_t.html "type libc::unix::size_t")) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Unmap a memory region.

`address` must be aligned to 4kb or this will return `Error::ARG`.
`size` must be a multiple of 4kb or this will return `Error::ARG`.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#509-516)

#### pub fn [mem\_protect](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.mem_protect)(  &mut self,  address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  size: [size\_t](https://docs.rs/libc/0.2.170/x86_64-unknown-linux-gnu/libc/unix/type.size_t.html "type libc::unix::size_t"),  perms: [Permission](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.Permission.html "struct unicorn_engine::unicorn_const::Permission"), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Set the memory permissions for an existing memory region.

`address` must be aligned to 4kb or this will return `Error::ARG`.
`size` must be a multiple of 4kb or this will return `Error::ARG`.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#519-522)

#### pub fn [reg\_write](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.reg_write) <T: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") < [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html) >>(  &mut self,  regid: T,  value: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Write an unsigned value from a register.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#525-544)

#### pub fn [reg\_write\_batch](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.reg_write_batch) <T: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") < [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html) >>(  &self,  regids: & [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html),  values: &\[ [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\],  count: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Write values into batch of registers

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#550-552)

#### pub fn [reg\_write\_long](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.reg_write_long) <T: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") < [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html) >>(  &self,  regid: T,  value: &\[ [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Write variable sized values into registers.

The user has to make sure that the buffer length matches the register size.
This adds support for registers >64 bit (GDTR/IDTR, XMM, YMM, ZMM (x86); Q, V (arm64)).

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#557-561)

#### pub fn [reg\_read](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.reg_read) <T: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") < [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html) >>(&self, regid: T) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Read an unsigned value from a register.

Not to be used with registers larger than 64 bit.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#566-588)

#### pub fn [reg\_read\_batch](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.reg_read_batch) <T: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") < [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html) >>(  &self,  regids: & [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html),  count: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") < [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html) >, [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Read batch of registers

Not to be used with registers larger than 64 bit

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#593-607)

#### pub fn [reg\_read\_long](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.reg_read_long) <T: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") < [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html) >>(  &self,  regid: T, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Box](https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html "struct alloc::boxed::Box") <\[ [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>, [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Read 128, 256 or 512 bit register value into heap allocated byte array.

This adds safe support for registers >64 bit (GDTR/IDTR, XMM, YMM, ZMM, ST (x86); Q, V (arm64)).

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#638-642)

#### pub fn [reg\_read\_i32](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.reg_read_i32) <T: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") < [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html) >>(&self, regid: T) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Read a signed 32-bit value from a register.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#645-676)

#### pub fn [add\_code\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.add_code_hook) <F>(  &mut self,  begin: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  end: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  callback: F, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) \+ 'a,

Add a code hook.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#679-710)

#### pub fn [add\_block\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.add_block_hook) <F>(  &mut self,  begin: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  end: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  callback: F, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) \+ 'a,

Add a block hook.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#713-749)

#### pub fn [add\_mem\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.add_mem_hook) <F>(  &mut self,  hook\_type: [HookType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.HookType.html "struct unicorn_engine::unicorn_const::HookType"),  begin: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  end: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  callback: F, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [MemType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.MemType.html "enum unicorn_engine::unicorn_const::MemType"), [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) \+ 'a,

Add a memory hook.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#752-778)

#### pub fn [add\_intr\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.add_intr_hook) <F>(&mut self, callback: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) \+ 'a,

Add an interrupt hook.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#781-807)

#### pub fn [add\_insn\_invalid\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.add_insn_invalid_hook) <F>(  &mut self,  callback: F, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) \+ 'a,

Add hook for invalid instructions

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#811-838)

#### pub fn [add\_insn\_in\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.add_insn_in_hook) <F>(&mut self, callback: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -\> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html) \+ 'a,

Add hook for x86 IN instruction.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#842-869)

#### pub fn [add\_insn\_out\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.add_insn_out_hook) <F>(  &mut self,  callback: F, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) \+ 'a,

Add hook for x86 OUT instruction.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#873-906)

#### pub fn [add\_insn\_sys\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.add_insn_sys_hook) <F>(  &mut self,  insn\_type: [InsnSysX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.InsnSysX86.html "enum unicorn_engine::InsnSysX86"),  begin: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  end: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  callback: F, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>) + 'a,

Add hook for x86 SYSCALL or SYSENTER.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#908-939)

#### pub fn [add\_tlb\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.add_tlb_hook) <F>(  &mut self,  begin: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  end: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  callback: F, ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >  where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'\_, D>, [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [MemType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.MemType.html "enum unicorn_engine::unicorn_const::MemType")) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") < [TlbEntry](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.TlbEntry.html "struct unicorn_engine::unicorn_const::TlbEntry") \> \+ 'a,

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#944-950)

#### pub fn [remove\_hook](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.remove_hook)(&mut self, hook\_id: [UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId")) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Remove a hook.

`hook_id` is the value returned by `add_*_hook` functions.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#955-960)

#### pub fn [context\_alloc](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.context_alloc)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Context](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Context.html "struct unicorn_engine::Context"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Allocate and return an empty Unicorn context.

To be populated via `context_save`.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#963-965)

#### pub fn [context\_save](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.context_save)(&self, context: &mut [Context](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Context.html "struct unicorn_engine::Context")) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Save current Unicorn context to previously allocated Context struct.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#972-986)

#### pub fn [context\_init](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.context_init)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Context](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Context.html "struct unicorn_engine::Context"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Allocate and return a Context struct initialized with the current CPU context.

This can be used for fast rollbacks with `context_restore`.
In case of many non-concurrent context saves, use `context_alloc` and \*\_save
individually to avoid unnecessary allocations.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#993-995)

#### pub fn [context\_restore](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.context_restore)(&self, context: & [Context](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Context.html "struct unicorn_engine::Context")) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Restore a previously saved Unicorn context.

Perform a quick rollback of the CPU context, including registers and some
internal metadata. Contexts may not be shared across engine instances with
differing arches or modes. Memory has to be restored manually, if needed.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1003-1011)

#### pub fn [emu\_start](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.emu_start)(  &mut self,  begin: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  until: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  timeout: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  count: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Emulate machine code for a specified duration.

`begin` is the address where to start the emulation. The emulation stops if `until`
is hit. `timeout` specifies a duration in microseconds after which the emulation is
stopped (infinite execution if set to 0). `count` is the maximum number of instructions
to emulate (emulate all the available instructions if set to 0).

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1017-1019)

#### pub fn [emu\_stop](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.emu_stop)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Stop the emulation.

This is usually called from callback function in hooks.
NOTE: For now, this will stop the execution only after the current block.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1024-1027)

#### pub fn [query](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.query)(&self, query: [Query](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Query.html "enum unicorn_engine::unicorn_const::Query")) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Query the internal status of the engine.

supported: `MODE`, `PAGE_SIZE`, `ARCH`

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1063-1067)

#### pub fn [pc\_read](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.pc_read)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Gets the current program counter for this `unicorn` instance.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1071-1075)

#### pub fn [set\_pc](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.set_pc)(&mut self, value: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

Sets the program counter for this `unicorn` instance.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1077-1087)

#### pub fn [ctl\_get\_mode](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_get_mode)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Mode](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.Mode.html "struct unicorn_engine::unicorn_const::Mode"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1089-1099)

#### pub fn [ctl\_get\_page\_size](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_get_page_size)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1101-1110)

#### pub fn [ctl\_set\_page\_size](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_set_page_size)(&self, page\_size: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1112-1122)

#### pub fn [ctl\_get\_arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_get_arch)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Arch](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.Arch.html "enum unicorn_engine::unicorn_const::Arch"), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1124-1134)

#### pub fn [ctl\_get\_timeout](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_get_timeout)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1136-1145)

#### pub fn [ctl\_exits\_enable](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_exits_enable)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1147-1156)

#### pub fn [ctl\_exits\_disable](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_exits_disable)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1158-1168)

#### pub fn [ctl\_get\_exits\_count](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_get_exits_count)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1170-1185)

#### pub fn [ctl\_get\_exits](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_get_exits)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") < [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html) >, [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1187-1197)

#### pub fn [ctl\_set\_exits](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_set_exits)(&self, exits: &\[ [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\]) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1199-1209)

#### pub fn [ctl\_get\_cpu\_model](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_get_cpu_model)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1211-1220)

#### pub fn [ctl\_set\_cpu\_model](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_set_cpu_model)(&self, cpu\_model: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1222-1232)

#### pub fn [ctl\_remove\_cache](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_remove_cache)(&self, address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), end: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1234-1248)

#### pub fn [ctl\_request\_cache](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_request_cache)(  &self,  address: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html),  tb: &mut [TranslationBlock](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.TranslationBlock.html "struct unicorn_engine::unicorn_const::TranslationBlock"), ) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1250-1258)

#### pub fn [ctl\_flush\_tb](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_flush_tb)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1260-1268)

#### pub fn [ctl\_flush\_tlb](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_flush_tlb)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1270-1279)

#### pub fn [ctl\_context\_mode](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_context_mode)(&self, mode: [ContextMode](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/struct.ContextMode.html "struct unicorn_engine::unicorn_const::ContextMode")) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1281-1290)

#### pub fn [ctl\_tlb\_type](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#method.ctl_tlb_type)(&self, t: [TlbType](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.TlbType.html "enum unicorn_engine::unicorn_const::TlbType")) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") < [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [uc\_error](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/enum.uc_error.html "enum unicorn_engine::unicorn_const::uc_error") >

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#270-274) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Debug-for-Unicorn%3C'a,+D%3E)

### impl<'a, D> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#271-273) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, formatter: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Freeze-for-Unicorn%3C'a,+D%3E)

### impl<'a, D> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-RefUnwindSafe-for-Unicorn%3C'a,+D%3E)

### impl<'a, D> ! [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Send-for-Unicorn%3C'a,+D%3E)

### impl<'a, D> ! [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Sync-for-Unicorn%3C'a,+D%3E)

### impl<'a, D> ! [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Unpin-for-Unicorn%3C'a,+D%3E)

### impl<'a, D> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-UnwindSafe-for-Unicorn%3C'a,+D%3E)

### impl<'a, D> ! [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn") <'a, D>

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
