# VZEROALL

Zero XMM, YMM, and ZMM Registers

In 64-bit mode, the instruction zeroes XMM0-XMM15, YMM0-YMM15, and ZMM0-ZMM15.
Outside 64-bit mode, it zeroes only XMM0-XMM7, YMM0-YMM7, and ZMM0-ZMM7.
VZEROALL does not modify ZMM16-ZMM31.Note: VEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.
In Compatibility and legacy 32-bit mode only the lower 8 registers are modified.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
simd_reg_file[][] is a two dimensional array representing the SIMD register file containing all the overlapping xmm, ymm, and zmm registers present in that implementation. The major dimension is the register number: 0 for xmm0, ymm0, and zmm0; 1 for xmm1, ymm1, and zmm1; etc. The minor dimension size is the width of the implemented SIMD state measured in bits. On a machine supporting Intel AVX-512, the width is 512.VZEROALL (VEX.256 encoded version)IF (64-bit mode)limit :=15ELSElimit := 7FOR i in 0 .. limit:simd_reg_file[i][MAXVL-1:0] := 0Intel C/C++ Compiler Intrinsic EquivalentVZEROALL: _mm256_zeroall()
```
