---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/"
title: "unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/index.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/index.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

# Crate unicorn\_engineCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/lib.rs.html#1-1291)

Expand description

Bindings for the Unicorn emulator.

## [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/\#example-use) Example use

```

use unicorn_engine::RegisterARM;
use unicorn_engine::unicorn_const::{Arch, Mode, Permission, SECOND_SCALE};

fn emulate() {
    let arm_code32 = [0x17, 0x00, 0x40, 0xe2]; // sub r0, #23

    let mut emu = unicorn_engine::Unicorn::new(Arch::ARM, Mode::LITTLE_ENDIAN).expect("failed to initialize Unicorn instance");
    emu.mem_map(0x1000, 0x4000, Permission::ALL).expect("failed to map code page");
    emu.mem_write(0x1000, &arm_code32).expect("failed to write instructions");

    emu.reg_write(RegisterARM::R0, 123).expect("failed write R0");
    emu.reg_write(RegisterARM::R5, 1337).expect("failed write R5");

    emu.emu_start(0x1000, (0x1000 + arm_code32.len()) as u64, 10 * SECOND_SCALE, 1000).unwrap();
    assert_eq!(emu.reg_read(RegisterARM::R0), Ok(100));
    assert_eq!(emu.reg_read(RegisterARM::R5), Ok(1337));
}
```

## Re-exports [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/\#reexports)

`pub use unicorn_const::*;`

## Modules [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/\#modules)

[ffi](https://docs.rs/unicorn-engine/latest/unicorn_engine/ffi/index.html "mod unicorn_engine::ffi")[unicorn\_const](https://docs.rs/unicorn-engine/latest/unicorn_engine/unicorn_const/index.html "mod unicorn_engine::unicorn_const")

## Structs [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/\#structs)

[Context](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Context.html "struct unicorn_engine::Context")[MmioCallbackScope](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.MmioCallbackScope.html "struct unicorn_engine::MmioCallbackScope")[UcHookId](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UcHookId.html "struct unicorn_engine::UcHookId")[Unicorn](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.Unicorn.html "struct unicorn_engine::Unicorn")A Unicorn emulator instance.[UnicornInner](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.UnicornInner.html "struct unicorn_engine::UnicornInner")[X86Mmr](https://docs.rs/unicorn-engine/latest/unicorn_engine/struct.X86Mmr.html "struct unicorn_engine::X86Mmr")

## Enums [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/\#enums)

[Arm64CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.Arm64CpuModel.html "enum unicorn_engine::Arm64CpuModel")[ArmCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.ArmCpuModel.html "enum unicorn_engine::ArmCpuModel")[InsnSysX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.InsnSysX86.html "enum unicorn_engine::InsnSysX86")[InsnX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.InsnX86.html "enum unicorn_engine::InsnX86")[M68kCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.M68kCpuModel.html "enum unicorn_engine::M68kCpuModel")[Mips32CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.Mips32CpuModel.html "enum unicorn_engine::Mips32CpuModel")[Ppc64CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.Ppc64CpuModel.html "enum unicorn_engine::Ppc64CpuModel")[PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")[RegisterARM](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM.html "enum unicorn_engine::RegisterARM")[RegisterARM64](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterARM64.html "enum unicorn_engine::RegisterARM64")[RegisterM68K](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterM68K.html "enum unicorn_engine::RegisterM68K")[RegisterMIPS](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterMIPS.html "enum unicorn_engine::RegisterMIPS")[RegisterPPC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterPPC.html "enum unicorn_engine::RegisterPPC")[RegisterRISCV](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterRISCV.html "enum unicorn_engine::RegisterRISCV")[RegisterS390X](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterS390X.html "enum unicorn_engine::RegisterS390X")[RegisterSPARC](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterSPARC.html "enum unicorn_engine::RegisterSPARC")[RegisterTRICORE](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterTRICORE.html "enum unicorn_engine::RegisterTRICORE")[RegisterX86](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.RegisterX86.html "enum unicorn_engine::RegisterX86")[Riscv32CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.Riscv32CpuModel.html "enum unicorn_engine::Riscv32CpuModel")[Riscv64CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.Riscv64CpuModel.html "enum unicorn_engine::Riscv64CpuModel")[S390xCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.S390xCpuModel.html "enum unicorn_engine::S390xCpuModel")[Sparc32CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.Sparc32CpuModel.html "enum unicorn_engine::Sparc32CpuModel")[Sparc64CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.Sparc64CpuModel.html "enum unicorn_engine::Sparc64CpuModel")[TricoreCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.TricoreCpuModel.html "enum unicorn_engine::TricoreCpuModel")[X86CpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.X86CpuModel.html "enum unicorn_engine::X86CpuModel")
