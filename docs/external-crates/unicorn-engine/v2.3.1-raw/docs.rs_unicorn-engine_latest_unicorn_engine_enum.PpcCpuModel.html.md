---
url: "https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html"
title: "PpcCpuModel in unicorn_engine - Rust"
---

[Docs.rs](https://docs.rs/)

- [unicorn-engine-2.1.3](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html# "Rust bindings for the Unicorn emulator with utility functions")


- unicorn-engine 2.1.3

- [Permalink](https://docs.rs/unicorn-engine/2.1.3/unicorn_engine/enum.PpcCpuModel.html "Get a link to this specific version")
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

- [Platform](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#)  - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/unicorn-engine/latest/target-redirect/x86_64-unknown-linux-gnu/unicorn_engine/enum.PpcCpuModel.html)
- [Feature flags](https://docs.rs/crate/unicorn-engine/latest/features "Browse available feature flags of unicorn-engine-2.1.3")

- [docs.rs](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#)  - [About docs.rs](https://docs.rs/about)
  - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#)  - [Rust website](https://www.rust-lang.org/)
  - [The Book](https://doc.rust-lang.org/book/)
  - [Standard Library API Reference](https://doc.rust-lang.org/std/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

[unicorn\_engine](https://docs.rs/unicorn-engine/latest/unicorn_engine/index.html)

# Enum PpcCpuModelCopy item path

[Settings](https://docs.rs/unicorn-engine/latest/settings.html)

[Help](https://docs.rs/unicorn-engine/latest/help.html)

Summary[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#98-389)

```
pub enum PpcCpuModel {
Show 290 variants    UC_CPU_PPC32_401 = 0,
    UC_CPU_PPC32_401A1 = 1,
    UC_CPU_PPC32_401B2 = 2,
    UC_CPU_PPC32_401C2 = 3,
    UC_CPU_PPC32_401D2 = 4,
    UC_CPU_PPC32_401E2 = 5,
    UC_CPU_PPC32_401F2 = 6,
    UC_CPU_PPC32_401G2 = 7,
    UC_CPU_PPC32_IOP480 = 8,
    UC_CPU_PPC32_COBRA = 9,
    UC_CPU_PPC32_403GA = 10,
    UC_CPU_PPC32_403GB = 11,
    UC_CPU_PPC32_403GC = 12,
    UC_CPU_PPC32_403GCX = 13,
    UC_CPU_PPC32_405D2 = 14,
    UC_CPU_PPC32_405D4 = 15,
    UC_CPU_PPC32_405CRA = 16,
    UC_CPU_PPC32_405CRB = 17,
    UC_CPU_PPC32_405CRC = 18,
    UC_CPU_PPC32_405EP = 19,
    UC_CPU_PPC32_405EZ = 20,
    UC_CPU_PPC32_405GPA = 21,
    UC_CPU_PPC32_405GPB = 22,
    UC_CPU_PPC32_405GPC = 23,
    UC_CPU_PPC32_405GPD = 24,
    UC_CPU_PPC32_405GPR = 25,
    UC_CPU_PPC32_405LP = 26,
    UC_CPU_PPC32_NPE405H = 27,
    UC_CPU_PPC32_NPE405H2 = 28,
    UC_CPU_PPC32_NPE405L = 29,
    UC_CPU_PPC32_NPE4GS3 = 30,
    UC_CPU_PPC32_STB03 = 31,
    UC_CPU_PPC32_STB04 = 32,
    UC_CPU_PPC32_STB25 = 33,
    UC_CPU_PPC32_X2VP4 = 34,
    UC_CPU_PPC32_X2VP20 = 35,
    UC_CPU_PPC32_440_XILINX = 36,
    UC_CPU_PPC32_440_XILINX_W_DFPU = 37,
    UC_CPU_PPC32_440EPA = 38,
    UC_CPU_PPC32_440EPB = 39,
    UC_CPU_PPC32_440EPX = 40,
    UC_CPU_PPC32_460EXB = 41,
    UC_CPU_PPC32_G2 = 42,
    UC_CPU_PPC32_G2H4 = 43,
    UC_CPU_PPC32_G2GP = 44,
    UC_CPU_PPC32_G2LS = 45,
    UC_CPU_PPC32_G2HIP3 = 46,
    UC_CPU_PPC32_G2HIP4 = 47,
    UC_CPU_PPC32_MPC603 = 48,
    UC_CPU_PPC32_G2LE = 49,
    UC_CPU_PPC32_G2LEGP = 50,
    UC_CPU_PPC32_G2LELS = 51,
    UC_CPU_PPC32_G2LEGP1 = 52,
    UC_CPU_PPC32_G2LEGP3 = 53,
    UC_CPU_PPC32_MPC5200_V10 = 54,
    UC_CPU_PPC32_MPC5200_V11 = 55,
    UC_CPU_PPC32_MPC5200_V12 = 56,
    UC_CPU_PPC32_MPC5200B_V20 = 57,
    UC_CPU_PPC32_MPC5200B_V21 = 58,
    UC_CPU_PPC32_E200Z5 = 59,
    UC_CPU_PPC32_E200Z6 = 60,
    UC_CPU_PPC32_E300C1 = 61,
    UC_CPU_PPC32_E300C2 = 62,
    UC_CPU_PPC32_E300C3 = 63,
    UC_CPU_PPC32_E300C4 = 64,
    UC_CPU_PPC32_MPC8343 = 65,
    UC_CPU_PPC32_MPC8343A = 66,
    UC_CPU_PPC32_MPC8343E = 67,
    UC_CPU_PPC32_MPC8343EA = 68,
    UC_CPU_PPC32_MPC8347T = 69,
    UC_CPU_PPC32_MPC8347P = 70,
    UC_CPU_PPC32_MPC8347AT = 71,
    UC_CPU_PPC32_MPC8347AP = 72,
    UC_CPU_PPC32_MPC8347ET = 73,
    UC_CPU_PPC32_MPC8347EP = 74,
    UC_CPU_PPC32_MPC8347EAT = 75,
    UC_CPU_PPC32_MPC8347EAP = 76,
    UC_CPU_PPC32_MPC8349 = 77,
    UC_CPU_PPC32_MPC8349A = 78,
    UC_CPU_PPC32_MPC8349E = 79,
    UC_CPU_PPC32_MPC8349EA = 80,
    UC_CPU_PPC32_MPC8377 = 81,
    UC_CPU_PPC32_MPC8377E = 82,
    UC_CPU_PPC32_MPC8378 = 83,
    UC_CPU_PPC32_MPC8378E = 84,
    UC_CPU_PPC32_MPC8379 = 85,
    UC_CPU_PPC32_MPC8379E = 86,
    UC_CPU_PPC32_E500_V10 = 87,
    UC_CPU_PPC32_E500_V20 = 88,
    UC_CPU_PPC32_E500V2_V10 = 89,
    UC_CPU_PPC32_E500V2_V20 = 90,
    UC_CPU_PPC32_E500V2_V21 = 91,
    UC_CPU_PPC32_E500V2_V22 = 92,
    UC_CPU_PPC32_E500V2_V30 = 93,
    UC_CPU_PPC32_E500MC = 94,
    UC_CPU_PPC32_MPC8533_V10 = 95,
    UC_CPU_PPC32_MPC8533_V11 = 96,
    UC_CPU_PPC32_MPC8533E_V10 = 97,
    UC_CPU_PPC32_MPC8533E_V11 = 98,
    UC_CPU_PPC32_MPC8540_V10 = 99,
    UC_CPU_PPC32_MPC8540_V20 = 100,
    UC_CPU_PPC32_MPC8540_V21 = 101,
    UC_CPU_PPC32_MPC8541_V10 = 102,
    UC_CPU_PPC32_MPC8541_V11 = 103,
    UC_CPU_PPC32_MPC8541E_V10 = 104,
    UC_CPU_PPC32_MPC8541E_V11 = 105,
    UC_CPU_PPC32_MPC8543_V10 = 106,
    UC_CPU_PPC32_MPC8543_V11 = 107,
    UC_CPU_PPC32_MPC8543_V20 = 108,
    UC_CPU_PPC32_MPC8543_V21 = 109,
    UC_CPU_PPC32_MPC8543E_V10 = 110,
    UC_CPU_PPC32_MPC8543E_V11 = 111,
    UC_CPU_PPC32_MPC8543E_V20 = 112,
    UC_CPU_PPC32_MPC8543E_V21 = 113,
    UC_CPU_PPC32_MPC8544_V10 = 114,
    UC_CPU_PPC32_MPC8544_V11 = 115,
    UC_CPU_PPC32_MPC8544E_V10 = 116,
    UC_CPU_PPC32_MPC8544E_V11 = 117,
    UC_CPU_PPC32_MPC8545_V20 = 118,
    UC_CPU_PPC32_MPC8545_V21 = 119,
    UC_CPU_PPC32_MPC8545E_V20 = 120,
    UC_CPU_PPC32_MPC8545E_V21 = 121,
    UC_CPU_PPC32_MPC8547E_V20 = 122,
    UC_CPU_PPC32_MPC8547E_V21 = 123,
    UC_CPU_PPC32_MPC8548_V10 = 124,
    UC_CPU_PPC32_MPC8548_V11 = 125,
    UC_CPU_PPC32_MPC8548_V20 = 126,
    UC_CPU_PPC32_MPC8548_V21 = 127,
    UC_CPU_PPC32_MPC8548E_V10 = 128,
    UC_CPU_PPC32_MPC8548E_V11 = 129,
    UC_CPU_PPC32_MPC8548E_V20 = 130,
    UC_CPU_PPC32_MPC8548E_V21 = 131,
    UC_CPU_PPC32_MPC8555_V10 = 132,
    UC_CPU_PPC32_MPC8555_V11 = 133,
    UC_CPU_PPC32_MPC8555E_V10 = 134,
    UC_CPU_PPC32_MPC8555E_V11 = 135,
    UC_CPU_PPC32_MPC8560_V10 = 136,
    UC_CPU_PPC32_MPC8560_V20 = 137,
    UC_CPU_PPC32_MPC8560_V21 = 138,
    UC_CPU_PPC32_MPC8567 = 139,
    UC_CPU_PPC32_MPC8567E = 140,
    UC_CPU_PPC32_MPC8568 = 141,
    UC_CPU_PPC32_MPC8568E = 142,
    UC_CPU_PPC32_MPC8572 = 143,
    UC_CPU_PPC32_MPC8572E = 144,
    UC_CPU_PPC32_E600 = 145,
    UC_CPU_PPC32_MPC8610 = 146,
    UC_CPU_PPC32_MPC8641 = 147,
    UC_CPU_PPC32_MPC8641D = 148,
    UC_CPU_PPC32_601_V0 = 149,
    UC_CPU_PPC32_601_V1 = 150,
    UC_CPU_PPC32_601_V2 = 151,
    UC_CPU_PPC32_602 = 152,
    UC_CPU_PPC32_603 = 153,
    UC_CPU_PPC32_603E_V1_1 = 154,
    UC_CPU_PPC32_603E_V1_2 = 155,
    UC_CPU_PPC32_603E_V1_3 = 156,
    UC_CPU_PPC32_603E_V1_4 = 157,
    UC_CPU_PPC32_603E_V2_2 = 158,
    UC_CPU_PPC32_603E_V3 = 159,
    UC_CPU_PPC32_603E_V4 = 160,
    UC_CPU_PPC32_603E_V4_1 = 161,
    UC_CPU_PPC32_603E7 = 162,
    UC_CPU_PPC32_603E7T = 163,
    UC_CPU_PPC32_603E7V = 164,
    UC_CPU_PPC32_603E7V1 = 165,
    UC_CPU_PPC32_603E7V2 = 166,
    UC_CPU_PPC32_603P = 167,
    UC_CPU_PPC32_604 = 168,
    UC_CPU_PPC32_604E_V1_0 = 169,
    UC_CPU_PPC32_604E_V2_2 = 170,
    UC_CPU_PPC32_604E_V2_4 = 171,
    UC_CPU_PPC32_604R = 172,
    UC_CPU_PPC32_740_V1_0 = 173,
    UC_CPU_PPC32_750_V1_0 = 174,
    UC_CPU_PPC32_740_V2_0 = 175,
    UC_CPU_PPC32_750_V2_0 = 176,
    UC_CPU_PPC32_740_V2_1 = 177,
    UC_CPU_PPC32_750_V2_1 = 178,
    UC_CPU_PPC32_740_V2_2 = 179,
    UC_CPU_PPC32_750_V2_2 = 180,
    UC_CPU_PPC32_740_V3_0 = 181,
    UC_CPU_PPC32_750_V3_0 = 182,
    UC_CPU_PPC32_740_V3_1 = 183,
    UC_CPU_PPC32_750_V3_1 = 184,
    UC_CPU_PPC32_740E = 185,
    UC_CPU_PPC32_750E = 186,
    UC_CPU_PPC32_740P = 187,
    UC_CPU_PPC32_750P = 188,
    UC_CPU_PPC32_750CL_V1_0 = 189,
    UC_CPU_PPC32_750CL_V2_0 = 190,
    UC_CPU_PPC32_750CX_V1_0 = 191,
    UC_CPU_PPC32_750CX_V2_0 = 192,
    UC_CPU_PPC32_750CX_V2_1 = 193,
    UC_CPU_PPC32_750CX_V2_2 = 194,
    UC_CPU_PPC32_750CXE_V2_1 = 195,
    UC_CPU_PPC32_750CXE_V2_2 = 196,
    UC_CPU_PPC32_750CXE_V2_3 = 197,
    UC_CPU_PPC32_750CXE_V2_4 = 198,
    UC_CPU_PPC32_750CXE_V2_4B = 199,
    UC_CPU_PPC32_750CXE_V3_0 = 200,
    UC_CPU_PPC32_750CXE_V3_1 = 201,
    UC_CPU_PPC32_750CXE_V3_1B = 202,
    UC_CPU_PPC32_750CXR = 203,
    UC_CPU_PPC32_750FL = 204,
    UC_CPU_PPC32_750FX_V1_0 = 205,
    UC_CPU_PPC32_750FX_V2_0 = 206,
    UC_CPU_PPC32_750FX_V2_1 = 207,
    UC_CPU_PPC32_750FX_V2_2 = 208,
    UC_CPU_PPC32_750FX_V2_3 = 209,
    UC_CPU_PPC32_750GL = 210,
    UC_CPU_PPC32_750GX_V1_0 = 211,
    UC_CPU_PPC32_750GX_V1_1 = 212,
    UC_CPU_PPC32_750GX_V1_2 = 213,
    UC_CPU_PPC32_750L_V2_0 = 214,
    UC_CPU_PPC32_750L_V2_1 = 215,
    UC_CPU_PPC32_750L_V2_2 = 216,
    UC_CPU_PPC32_750L_V3_0 = 217,
    UC_CPU_PPC32_750L_V3_2 = 218,
    UC_CPU_PPC32_745_V1_0 = 219,
    UC_CPU_PPC32_755_V1_0 = 220,
    UC_CPU_PPC32_745_V1_1 = 221,
    UC_CPU_PPC32_755_V1_1 = 222,
    UC_CPU_PPC32_745_V2_0 = 223,
    UC_CPU_PPC32_755_V2_0 = 224,
    UC_CPU_PPC32_745_V2_1 = 225,
    UC_CPU_PPC32_755_V2_1 = 226,
    UC_CPU_PPC32_745_V2_2 = 227,
    UC_CPU_PPC32_755_V2_2 = 228,
    UC_CPU_PPC32_745_V2_3 = 229,
    UC_CPU_PPC32_755_V2_3 = 230,
    UC_CPU_PPC32_745_V2_4 = 231,
    UC_CPU_PPC32_755_V2_4 = 232,
    UC_CPU_PPC32_745_V2_5 = 233,
    UC_CPU_PPC32_755_V2_5 = 234,
    UC_CPU_PPC32_745_V2_6 = 235,
    UC_CPU_PPC32_755_V2_6 = 236,
    UC_CPU_PPC32_745_V2_7 = 237,
    UC_CPU_PPC32_755_V2_7 = 238,
    UC_CPU_PPC32_745_V2_8 = 239,
    UC_CPU_PPC32_755_V2_8 = 240,
    UC_CPU_PPC32_7400_V1_0 = 241,
    UC_CPU_PPC32_7400_V1_1 = 242,
    UC_CPU_PPC32_7400_V2_0 = 243,
    UC_CPU_PPC32_7400_V2_1 = 244,
    UC_CPU_PPC32_7400_V2_2 = 245,
    UC_CPU_PPC32_7400_V2_6 = 246,
    UC_CPU_PPC32_7400_V2_7 = 247,
    UC_CPU_PPC32_7400_V2_8 = 248,
    UC_CPU_PPC32_7400_V2_9 = 249,
    UC_CPU_PPC32_7410_V1_0 = 250,
    UC_CPU_PPC32_7410_V1_1 = 251,
    UC_CPU_PPC32_7410_V1_2 = 252,
    UC_CPU_PPC32_7410_V1_3 = 253,
    UC_CPU_PPC32_7410_V1_4 = 254,
    UC_CPU_PPC32_7448_V1_0 = 255,
    UC_CPU_PPC32_7448_V1_1 = 256,
    UC_CPU_PPC32_7448_V2_0 = 257,
    UC_CPU_PPC32_7448_V2_1 = 258,
    UC_CPU_PPC32_7450_V1_0 = 259,
    UC_CPU_PPC32_7450_V1_1 = 260,
    UC_CPU_PPC32_7450_V1_2 = 261,
    UC_CPU_PPC32_7450_V2_0 = 262,
    UC_CPU_PPC32_7450_V2_1 = 263,
    UC_CPU_PPC32_7441_V2_1 = 264,
    UC_CPU_PPC32_7441_V2_3 = 265,
    UC_CPU_PPC32_7451_V2_3 = 266,
    UC_CPU_PPC32_7441_V2_10 = 267,
    UC_CPU_PPC32_7451_V2_10 = 268,
    UC_CPU_PPC32_7445_V1_0 = 269,
    UC_CPU_PPC32_7455_V1_0 = 270,
    UC_CPU_PPC32_7445_V2_1 = 271,
    UC_CPU_PPC32_7455_V2_1 = 272,
    UC_CPU_PPC32_7445_V3_2 = 273,
    UC_CPU_PPC32_7455_V3_2 = 274,
    UC_CPU_PPC32_7445_V3_3 = 275,
    UC_CPU_PPC32_7455_V3_3 = 276,
    UC_CPU_PPC32_7445_V3_4 = 277,
    UC_CPU_PPC32_7455_V3_4 = 278,
    UC_CPU_PPC32_7447_V1_0 = 279,
    UC_CPU_PPC32_7457_V1_0 = 280,
    UC_CPU_PPC32_7447_V1_1 = 281,
    UC_CPU_PPC32_7457_V1_1 = 282,
    UC_CPU_PPC32_7457_V1_2 = 283,
    UC_CPU_PPC32_7447A_V1_0 = 284,
    UC_CPU_PPC32_7457A_V1_0 = 285,
    UC_CPU_PPC32_7447A_V1_1 = 286,
    UC_CPU_PPC32_7457A_V1_1 = 287,
    UC_CPU_PPC32_7447A_V1_2 = 288,
    UC_CPU_PPC32_7457A_V1_2 = 289,
}
```

## Variants [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html\#variants)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_401)

### UC\_CPU\_PPC32\_401 = 0

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_401A1)

### UC\_CPU\_PPC32\_401A1 = 1

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_401B2)

### UC\_CPU\_PPC32\_401B2 = 2

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_401C2)

### UC\_CPU\_PPC32\_401C2 = 3

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_401D2)

### UC\_CPU\_PPC32\_401D2 = 4

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_401E2)

### UC\_CPU\_PPC32\_401E2 = 5

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_401F2)

### UC\_CPU\_PPC32\_401F2 = 6

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_401G2)

### UC\_CPU\_PPC32\_401G2 = 7

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_IOP480)

### UC\_CPU\_PPC32\_IOP480 = 8

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_COBRA)

### UC\_CPU\_PPC32\_COBRA = 9

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_403GA)

### UC\_CPU\_PPC32\_403GA = 10

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_403GB)

### UC\_CPU\_PPC32\_403GB = 11

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_403GC)

### UC\_CPU\_PPC32\_403GC = 12

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_403GCX)

### UC\_CPU\_PPC32\_403GCX = 13

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405D2)

### UC\_CPU\_PPC32\_405D2 = 14

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405D4)

### UC\_CPU\_PPC32\_405D4 = 15

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405CRA)

### UC\_CPU\_PPC32\_405CRA = 16

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405CRB)

### UC\_CPU\_PPC32\_405CRB = 17

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405CRC)

### UC\_CPU\_PPC32\_405CRC = 18

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405EP)

### UC\_CPU\_PPC32\_405EP = 19

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405EZ)

### UC\_CPU\_PPC32\_405EZ = 20

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405GPA)

### UC\_CPU\_PPC32\_405GPA = 21

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405GPB)

### UC\_CPU\_PPC32\_405GPB = 22

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405GPC)

### UC\_CPU\_PPC32\_405GPC = 23

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405GPD)

### UC\_CPU\_PPC32\_405GPD = 24

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405GPR)

### UC\_CPU\_PPC32\_405GPR = 25

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_405LP)

### UC\_CPU\_PPC32\_405LP = 26

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_NPE405H)

### UC\_CPU\_PPC32\_NPE405H = 27

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_NPE405H2)

### UC\_CPU\_PPC32\_NPE405H2 = 28

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_NPE405L)

### UC\_CPU\_PPC32\_NPE405L = 29

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_NPE4GS3)

### UC\_CPU\_PPC32\_NPE4GS3 = 30

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_STB03)

### UC\_CPU\_PPC32\_STB03 = 31

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_STB04)

### UC\_CPU\_PPC32\_STB04 = 32

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_STB25)

### UC\_CPU\_PPC32\_STB25 = 33

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_X2VP4)

### UC\_CPU\_PPC32\_X2VP4 = 34

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_X2VP20)

### UC\_CPU\_PPC32\_X2VP20 = 35

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_440_XILINX)

### UC\_CPU\_PPC32\_440\_XILINX = 36

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_440_XILINX_W_DFPU)

### UC\_CPU\_PPC32\_440\_XILINX\_W\_DFPU = 37

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_440EPA)

### UC\_CPU\_PPC32\_440EPA = 38

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_440EPB)

### UC\_CPU\_PPC32\_440EPB = 39

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_440EPX)

### UC\_CPU\_PPC32\_440EPX = 40

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_460EXB)

### UC\_CPU\_PPC32\_460EXB = 41

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2)

### UC\_CPU\_PPC32\_G2 = 42

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2H4)

### UC\_CPU\_PPC32\_G2H4 = 43

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2GP)

### UC\_CPU\_PPC32\_G2GP = 44

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2LS)

### UC\_CPU\_PPC32\_G2LS = 45

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2HIP3)

### UC\_CPU\_PPC32\_G2HIP3 = 46

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2HIP4)

### UC\_CPU\_PPC32\_G2HIP4 = 47

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC603)

### UC\_CPU\_PPC32\_MPC603 = 48

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2LE)

### UC\_CPU\_PPC32\_G2LE = 49

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2LEGP)

### UC\_CPU\_PPC32\_G2LEGP = 50

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2LELS)

### UC\_CPU\_PPC32\_G2LELS = 51

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2LEGP1)

### UC\_CPU\_PPC32\_G2LEGP1 = 52

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_G2LEGP3)

### UC\_CPU\_PPC32\_G2LEGP3 = 53

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC5200_V10)

### UC\_CPU\_PPC32\_MPC5200\_V10 = 54

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC5200_V11)

### UC\_CPU\_PPC32\_MPC5200\_V11 = 55

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC5200_V12)

### UC\_CPU\_PPC32\_MPC5200\_V12 = 56

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC5200B_V20)

### UC\_CPU\_PPC32\_MPC5200B\_V20 = 57

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC5200B_V21)

### UC\_CPU\_PPC32\_MPC5200B\_V21 = 58

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E200Z5)

### UC\_CPU\_PPC32\_E200Z5 = 59

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E200Z6)

### UC\_CPU\_PPC32\_E200Z6 = 60

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E300C1)

### UC\_CPU\_PPC32\_E300C1 = 61

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E300C2)

### UC\_CPU\_PPC32\_E300C2 = 62

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E300C3)

### UC\_CPU\_PPC32\_E300C3 = 63

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E300C4)

### UC\_CPU\_PPC32\_E300C4 = 64

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8343)

### UC\_CPU\_PPC32\_MPC8343 = 65

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8343A)

### UC\_CPU\_PPC32\_MPC8343A = 66

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8343E)

### UC\_CPU\_PPC32\_MPC8343E = 67

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8343EA)

### UC\_CPU\_PPC32\_MPC8343EA = 68

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8347T)

### UC\_CPU\_PPC32\_MPC8347T = 69

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8347P)

### UC\_CPU\_PPC32\_MPC8347P = 70

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8347AT)

### UC\_CPU\_PPC32\_MPC8347AT = 71

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8347AP)

### UC\_CPU\_PPC32\_MPC8347AP = 72

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8347ET)

### UC\_CPU\_PPC32\_MPC8347ET = 73

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8347EP)

### UC\_CPU\_PPC32\_MPC8347EP = 74

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8347EAT)

### UC\_CPU\_PPC32\_MPC8347EAT = 75

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8347EAP)

### UC\_CPU\_PPC32\_MPC8347EAP = 76

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8349)

### UC\_CPU\_PPC32\_MPC8349 = 77

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8349A)

### UC\_CPU\_PPC32\_MPC8349A = 78

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8349E)

### UC\_CPU\_PPC32\_MPC8349E = 79

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8349EA)

### UC\_CPU\_PPC32\_MPC8349EA = 80

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8377)

### UC\_CPU\_PPC32\_MPC8377 = 81

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8377E)

### UC\_CPU\_PPC32\_MPC8377E = 82

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8378)

### UC\_CPU\_PPC32\_MPC8378 = 83

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8378E)

### UC\_CPU\_PPC32\_MPC8378E = 84

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8379)

### UC\_CPU\_PPC32\_MPC8379 = 85

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8379E)

### UC\_CPU\_PPC32\_MPC8379E = 86

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E500_V10)

### UC\_CPU\_PPC32\_E500\_V10 = 87

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E500_V20)

### UC\_CPU\_PPC32\_E500\_V20 = 88

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E500V2_V10)

### UC\_CPU\_PPC32\_E500V2\_V10 = 89

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E500V2_V20)

### UC\_CPU\_PPC32\_E500V2\_V20 = 90

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E500V2_V21)

### UC\_CPU\_PPC32\_E500V2\_V21 = 91

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E500V2_V22)

### UC\_CPU\_PPC32\_E500V2\_V22 = 92

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E500V2_V30)

### UC\_CPU\_PPC32\_E500V2\_V30 = 93

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E500MC)

### UC\_CPU\_PPC32\_E500MC = 94

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8533_V10)

### UC\_CPU\_PPC32\_MPC8533\_V10 = 95

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8533_V11)

### UC\_CPU\_PPC32\_MPC8533\_V11 = 96

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8533E_V10)

### UC\_CPU\_PPC32\_MPC8533E\_V10 = 97

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8533E_V11)

### UC\_CPU\_PPC32\_MPC8533E\_V11 = 98

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8540_V10)

### UC\_CPU\_PPC32\_MPC8540\_V10 = 99

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8540_V20)

### UC\_CPU\_PPC32\_MPC8540\_V20 = 100

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8540_V21)

### UC\_CPU\_PPC32\_MPC8540\_V21 = 101

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8541_V10)

### UC\_CPU\_PPC32\_MPC8541\_V10 = 102

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8541_V11)

### UC\_CPU\_PPC32\_MPC8541\_V11 = 103

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8541E_V10)

### UC\_CPU\_PPC32\_MPC8541E\_V10 = 104

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8541E_V11)

### UC\_CPU\_PPC32\_MPC8541E\_V11 = 105

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8543_V10)

### UC\_CPU\_PPC32\_MPC8543\_V10 = 106

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8543_V11)

### UC\_CPU\_PPC32\_MPC8543\_V11 = 107

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8543_V20)

### UC\_CPU\_PPC32\_MPC8543\_V20 = 108

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8543_V21)

### UC\_CPU\_PPC32\_MPC8543\_V21 = 109

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8543E_V10)

### UC\_CPU\_PPC32\_MPC8543E\_V10 = 110

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8543E_V11)

### UC\_CPU\_PPC32\_MPC8543E\_V11 = 111

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8543E_V20)

### UC\_CPU\_PPC32\_MPC8543E\_V20 = 112

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8543E_V21)

### UC\_CPU\_PPC32\_MPC8543E\_V21 = 113

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8544_V10)

### UC\_CPU\_PPC32\_MPC8544\_V10 = 114

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8544_V11)

### UC\_CPU\_PPC32\_MPC8544\_V11 = 115

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8544E_V10)

### UC\_CPU\_PPC32\_MPC8544E\_V10 = 116

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8544E_V11)

### UC\_CPU\_PPC32\_MPC8544E\_V11 = 117

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8545_V20)

### UC\_CPU\_PPC32\_MPC8545\_V20 = 118

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8545_V21)

### UC\_CPU\_PPC32\_MPC8545\_V21 = 119

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8545E_V20)

### UC\_CPU\_PPC32\_MPC8545E\_V20 = 120

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8545E_V21)

### UC\_CPU\_PPC32\_MPC8545E\_V21 = 121

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8547E_V20)

### UC\_CPU\_PPC32\_MPC8547E\_V20 = 122

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8547E_V21)

### UC\_CPU\_PPC32\_MPC8547E\_V21 = 123

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8548_V10)

### UC\_CPU\_PPC32\_MPC8548\_V10 = 124

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8548_V11)

### UC\_CPU\_PPC32\_MPC8548\_V11 = 125

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8548_V20)

### UC\_CPU\_PPC32\_MPC8548\_V20 = 126

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8548_V21)

### UC\_CPU\_PPC32\_MPC8548\_V21 = 127

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8548E_V10)

### UC\_CPU\_PPC32\_MPC8548E\_V10 = 128

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8548E_V11)

### UC\_CPU\_PPC32\_MPC8548E\_V11 = 129

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8548E_V20)

### UC\_CPU\_PPC32\_MPC8548E\_V20 = 130

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8548E_V21)

### UC\_CPU\_PPC32\_MPC8548E\_V21 = 131

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8555_V10)

### UC\_CPU\_PPC32\_MPC8555\_V10 = 132

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8555_V11)

### UC\_CPU\_PPC32\_MPC8555\_V11 = 133

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8555E_V10)

### UC\_CPU\_PPC32\_MPC8555E\_V10 = 134

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8555E_V11)

### UC\_CPU\_PPC32\_MPC8555E\_V11 = 135

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8560_V10)

### UC\_CPU\_PPC32\_MPC8560\_V10 = 136

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8560_V20)

### UC\_CPU\_PPC32\_MPC8560\_V20 = 137

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8560_V21)

### UC\_CPU\_PPC32\_MPC8560\_V21 = 138

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8567)

### UC\_CPU\_PPC32\_MPC8567 = 139

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8567E)

### UC\_CPU\_PPC32\_MPC8567E = 140

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8568)

### UC\_CPU\_PPC32\_MPC8568 = 141

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8568E)

### UC\_CPU\_PPC32\_MPC8568E = 142

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8572)

### UC\_CPU\_PPC32\_MPC8572 = 143

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8572E)

### UC\_CPU\_PPC32\_MPC8572E = 144

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_E600)

### UC\_CPU\_PPC32\_E600 = 145

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8610)

### UC\_CPU\_PPC32\_MPC8610 = 146

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8641)

### UC\_CPU\_PPC32\_MPC8641 = 147

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_MPC8641D)

### UC\_CPU\_PPC32\_MPC8641D = 148

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_601_V0)

### UC\_CPU\_PPC32\_601\_V0 = 149

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_601_V1)

### UC\_CPU\_PPC32\_601\_V1 = 150

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_601_V2)

### UC\_CPU\_PPC32\_601\_V2 = 151

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_602)

### UC\_CPU\_PPC32\_602 = 152

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603)

### UC\_CPU\_PPC32\_603 = 153

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E_V1_1)

### UC\_CPU\_PPC32\_603E\_V1\_1 = 154

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E_V1_2)

### UC\_CPU\_PPC32\_603E\_V1\_2 = 155

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E_V1_3)

### UC\_CPU\_PPC32\_603E\_V1\_3 = 156

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E_V1_4)

### UC\_CPU\_PPC32\_603E\_V1\_4 = 157

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E_V2_2)

### UC\_CPU\_PPC32\_603E\_V2\_2 = 158

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E_V3)

### UC\_CPU\_PPC32\_603E\_V3 = 159

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E_V4)

### UC\_CPU\_PPC32\_603E\_V4 = 160

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E_V4_1)

### UC\_CPU\_PPC32\_603E\_V4\_1 = 161

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E7)

### UC\_CPU\_PPC32\_603E7 = 162

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E7T)

### UC\_CPU\_PPC32\_603E7T = 163

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E7V)

### UC\_CPU\_PPC32\_603E7V = 164

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E7V1)

### UC\_CPU\_PPC32\_603E7V1 = 165

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603E7V2)

### UC\_CPU\_PPC32\_603E7V2 = 166

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_603P)

### UC\_CPU\_PPC32\_603P = 167

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_604)

### UC\_CPU\_PPC32\_604 = 168

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_604E_V1_0)

### UC\_CPU\_PPC32\_604E\_V1\_0 = 169

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_604E_V2_2)

### UC\_CPU\_PPC32\_604E\_V2\_2 = 170

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_604E_V2_4)

### UC\_CPU\_PPC32\_604E\_V2\_4 = 171

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_604R)

### UC\_CPU\_PPC32\_604R = 172

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_740_V1_0)

### UC\_CPU\_PPC32\_740\_V1\_0 = 173

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750_V1_0)

### UC\_CPU\_PPC32\_750\_V1\_0 = 174

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_740_V2_0)

### UC\_CPU\_PPC32\_740\_V2\_0 = 175

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750_V2_0)

### UC\_CPU\_PPC32\_750\_V2\_0 = 176

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_740_V2_1)

### UC\_CPU\_PPC32\_740\_V2\_1 = 177

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750_V2_1)

### UC\_CPU\_PPC32\_750\_V2\_1 = 178

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_740_V2_2)

### UC\_CPU\_PPC32\_740\_V2\_2 = 179

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750_V2_2)

### UC\_CPU\_PPC32\_750\_V2\_2 = 180

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_740_V3_0)

### UC\_CPU\_PPC32\_740\_V3\_0 = 181

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750_V3_0)

### UC\_CPU\_PPC32\_750\_V3\_0 = 182

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_740_V3_1)

### UC\_CPU\_PPC32\_740\_V3\_1 = 183

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750_V3_1)

### UC\_CPU\_PPC32\_750\_V3\_1 = 184

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_740E)

### UC\_CPU\_PPC32\_740E = 185

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750E)

### UC\_CPU\_PPC32\_750E = 186

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_740P)

### UC\_CPU\_PPC32\_740P = 187

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750P)

### UC\_CPU\_PPC32\_750P = 188

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CL_V1_0)

### UC\_CPU\_PPC32\_750CL\_V1\_0 = 189

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CL_V2_0)

### UC\_CPU\_PPC32\_750CL\_V2\_0 = 190

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CX_V1_0)

### UC\_CPU\_PPC32\_750CX\_V1\_0 = 191

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CX_V2_0)

### UC\_CPU\_PPC32\_750CX\_V2\_0 = 192

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CX_V2_1)

### UC\_CPU\_PPC32\_750CX\_V2\_1 = 193

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CX_V2_2)

### UC\_CPU\_PPC32\_750CX\_V2\_2 = 194

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CXE_V2_1)

### UC\_CPU\_PPC32\_750CXE\_V2\_1 = 195

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CXE_V2_2)

### UC\_CPU\_PPC32\_750CXE\_V2\_2 = 196

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CXE_V2_3)

### UC\_CPU\_PPC32\_750CXE\_V2\_3 = 197

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CXE_V2_4)

### UC\_CPU\_PPC32\_750CXE\_V2\_4 = 198

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CXE_V2_4B)

### UC\_CPU\_PPC32\_750CXE\_V2\_4B = 199

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CXE_V3_0)

### UC\_CPU\_PPC32\_750CXE\_V3\_0 = 200

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CXE_V3_1)

### UC\_CPU\_PPC32\_750CXE\_V3\_1 = 201

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CXE_V3_1B)

### UC\_CPU\_PPC32\_750CXE\_V3\_1B = 202

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750CXR)

### UC\_CPU\_PPC32\_750CXR = 203

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750FL)

### UC\_CPU\_PPC32\_750FL = 204

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750FX_V1_0)

### UC\_CPU\_PPC32\_750FX\_V1\_0 = 205

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750FX_V2_0)

### UC\_CPU\_PPC32\_750FX\_V2\_0 = 206

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750FX_V2_1)

### UC\_CPU\_PPC32\_750FX\_V2\_1 = 207

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750FX_V2_2)

### UC\_CPU\_PPC32\_750FX\_V2\_2 = 208

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750FX_V2_3)

### UC\_CPU\_PPC32\_750FX\_V2\_3 = 209

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750GL)

### UC\_CPU\_PPC32\_750GL = 210

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750GX_V1_0)

### UC\_CPU\_PPC32\_750GX\_V1\_0 = 211

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750GX_V1_1)

### UC\_CPU\_PPC32\_750GX\_V1\_1 = 212

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750GX_V1_2)

### UC\_CPU\_PPC32\_750GX\_V1\_2 = 213

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750L_V2_0)

### UC\_CPU\_PPC32\_750L\_V2\_0 = 214

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750L_V2_1)

### UC\_CPU\_PPC32\_750L\_V2\_1 = 215

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750L_V2_2)

### UC\_CPU\_PPC32\_750L\_V2\_2 = 216

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750L_V3_0)

### UC\_CPU\_PPC32\_750L\_V3\_0 = 217

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_750L_V3_2)

### UC\_CPU\_PPC32\_750L\_V3\_2 = 218

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V1_0)

### UC\_CPU\_PPC32\_745\_V1\_0 = 219

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V1_0)

### UC\_CPU\_PPC32\_755\_V1\_0 = 220

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V1_1)

### UC\_CPU\_PPC32\_745\_V1\_1 = 221

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V1_1)

### UC\_CPU\_PPC32\_755\_V1\_1 = 222

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V2_0)

### UC\_CPU\_PPC32\_745\_V2\_0 = 223

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V2_0)

### UC\_CPU\_PPC32\_755\_V2\_0 = 224

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V2_1)

### UC\_CPU\_PPC32\_745\_V2\_1 = 225

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V2_1)

### UC\_CPU\_PPC32\_755\_V2\_1 = 226

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V2_2)

### UC\_CPU\_PPC32\_745\_V2\_2 = 227

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V2_2)

### UC\_CPU\_PPC32\_755\_V2\_2 = 228

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V2_3)

### UC\_CPU\_PPC32\_745\_V2\_3 = 229

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V2_3)

### UC\_CPU\_PPC32\_755\_V2\_3 = 230

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V2_4)

### UC\_CPU\_PPC32\_745\_V2\_4 = 231

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V2_4)

### UC\_CPU\_PPC32\_755\_V2\_4 = 232

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V2_5)

### UC\_CPU\_PPC32\_745\_V2\_5 = 233

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V2_5)

### UC\_CPU\_PPC32\_755\_V2\_5 = 234

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V2_6)

### UC\_CPU\_PPC32\_745\_V2\_6 = 235

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V2_6)

### UC\_CPU\_PPC32\_755\_V2\_6 = 236

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V2_7)

### UC\_CPU\_PPC32\_745\_V2\_7 = 237

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V2_7)

### UC\_CPU\_PPC32\_755\_V2\_7 = 238

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_745_V2_8)

### UC\_CPU\_PPC32\_745\_V2\_8 = 239

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_755_V2_8)

### UC\_CPU\_PPC32\_755\_V2\_8 = 240

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7400_V1_0)

### UC\_CPU\_PPC32\_7400\_V1\_0 = 241

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7400_V1_1)

### UC\_CPU\_PPC32\_7400\_V1\_1 = 242

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7400_V2_0)

### UC\_CPU\_PPC32\_7400\_V2\_0 = 243

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7400_V2_1)

### UC\_CPU\_PPC32\_7400\_V2\_1 = 244

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7400_V2_2)

### UC\_CPU\_PPC32\_7400\_V2\_2 = 245

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7400_V2_6)

### UC\_CPU\_PPC32\_7400\_V2\_6 = 246

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7400_V2_7)

### UC\_CPU\_PPC32\_7400\_V2\_7 = 247

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7400_V2_8)

### UC\_CPU\_PPC32\_7400\_V2\_8 = 248

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7400_V2_9)

### UC\_CPU\_PPC32\_7400\_V2\_9 = 249

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7410_V1_0)

### UC\_CPU\_PPC32\_7410\_V1\_0 = 250

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7410_V1_1)

### UC\_CPU\_PPC32\_7410\_V1\_1 = 251

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7410_V1_2)

### UC\_CPU\_PPC32\_7410\_V1\_2 = 252

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7410_V1_3)

### UC\_CPU\_PPC32\_7410\_V1\_3 = 253

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7410_V1_4)

### UC\_CPU\_PPC32\_7410\_V1\_4 = 254

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7448_V1_0)

### UC\_CPU\_PPC32\_7448\_V1\_0 = 255

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7448_V1_1)

### UC\_CPU\_PPC32\_7448\_V1\_1 = 256

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7448_V2_0)

### UC\_CPU\_PPC32\_7448\_V2\_0 = 257

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7448_V2_1)

### UC\_CPU\_PPC32\_7448\_V2\_1 = 258

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7450_V1_0)

### UC\_CPU\_PPC32\_7450\_V1\_0 = 259

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7450_V1_1)

### UC\_CPU\_PPC32\_7450\_V1\_1 = 260

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7450_V1_2)

### UC\_CPU\_PPC32\_7450\_V1\_2 = 261

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7450_V2_0)

### UC\_CPU\_PPC32\_7450\_V2\_0 = 262

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7450_V2_1)

### UC\_CPU\_PPC32\_7450\_V2\_1 = 263

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7441_V2_1)

### UC\_CPU\_PPC32\_7441\_V2\_1 = 264

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7441_V2_3)

### UC\_CPU\_PPC32\_7441\_V2\_3 = 265

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7451_V2_3)

### UC\_CPU\_PPC32\_7451\_V2\_3 = 266

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7441_V2_10)

### UC\_CPU\_PPC32\_7441\_V2\_10 = 267

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7451_V2_10)

### UC\_CPU\_PPC32\_7451\_V2\_10 = 268

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7445_V1_0)

### UC\_CPU\_PPC32\_7445\_V1\_0 = 269

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7455_V1_0)

### UC\_CPU\_PPC32\_7455\_V1\_0 = 270

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7445_V2_1)

### UC\_CPU\_PPC32\_7445\_V2\_1 = 271

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7455_V2_1)

### UC\_CPU\_PPC32\_7455\_V2\_1 = 272

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7445_V3_2)

### UC\_CPU\_PPC32\_7445\_V3\_2 = 273

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7455_V3_2)

### UC\_CPU\_PPC32\_7455\_V3\_2 = 274

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7445_V3_3)

### UC\_CPU\_PPC32\_7445\_V3\_3 = 275

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7455_V3_3)

### UC\_CPU\_PPC32\_7455\_V3\_3 = 276

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7445_V3_4)

### UC\_CPU\_PPC32\_7445\_V3\_4 = 277

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7455_V3_4)

### UC\_CPU\_PPC32\_7455\_V3\_4 = 278

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7447_V1_0)

### UC\_CPU\_PPC32\_7447\_V1\_0 = 279

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7457_V1_0)

### UC\_CPU\_PPC32\_7457\_V1\_0 = 280

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7447_V1_1)

### UC\_CPU\_PPC32\_7447\_V1\_1 = 281

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7457_V1_1)

### UC\_CPU\_PPC32\_7457\_V1\_1 = 282

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7457_V1_2)

### UC\_CPU\_PPC32\_7457\_V1\_2 = 283

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7447A_V1_0)

### UC\_CPU\_PPC32\_7447A\_V1\_0 = 284

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7457A_V1_0)

### UC\_CPU\_PPC32\_7457A\_V1\_0 = 285

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7447A_V1_1)

### UC\_CPU\_PPC32\_7447A\_V1\_1 = 286

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7457A_V1_1)

### UC\_CPU\_PPC32\_7457A\_V1\_1 = 287

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7447A_V1_2)

### UC\_CPU\_PPC32\_7447A\_V1\_2 = 288

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#variant.UC_CPU_PPC32_7457A_V1_2)

### UC\_CPU\_PPC32\_7457A\_V1\_2 = 289

## Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html\#trait-implementations)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#97) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Clone-for-PpcCpuModel)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#97) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#tymethod.clone)(&self) -> [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#97) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Debug-for-PpcCpuModel)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#97) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter") <'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#397-401) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-From%3C%26PpcCpuModel%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <& [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#398-400) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(value: & [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#391-395) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-From%3CPpcCpuModel%3E-for-i32)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") < [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel") \> for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#392-394) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(value: [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")) -\> Self

Converts to this type from the input type.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#97) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-PartialEq-for-PpcCpuModel)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#97) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#tymethod.eq)(&self, other: & [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#97) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Copy-for-PpcCpuModel)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#97) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Eq-for-PpcCpuModel)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[Source](https://docs.rs/unicorn-engine/latest/src/unicorn_engine/ppc.rs.html#97) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-StructuralPartialEq-for-PpcCpuModel)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

## Auto Trait Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html\#synthetic-implementations)

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Freeze-for-PpcCpuModel)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-RefUnwindSafe-for-PpcCpuModel)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Send-for-PpcCpuModel)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Sync-for-PpcCpuModel)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Unpin-for-PpcCpuModel)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

[§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-UnwindSafe-for-PpcCpuModel)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [PpcCpuModel](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html "enum unicorn_engine::PpcCpuModel")

## Blanket Implementations [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html\#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T  where T: 'static + ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html\#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") <T> for T  where T: ? [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html\#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html\#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. ( `clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#770) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.from-2)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html\#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#750-752) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <U> for T  where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#760) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T  where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.to_owned)

#### fn [to\_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.clone_into)

#### fn [clone\_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html\#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#806-808) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U> for T  where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#810) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <U>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#791-793) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto") <U> for T  where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#795) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798) [§](https://docs.rs/unicorn-engine/latest/unicorn_engine/enum.PpcCpuModel.html#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html\#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") <U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") <T>>:: [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\#associatedtype.Error "type core::convert::TryFrom::Error") >

Performs the conversion.
