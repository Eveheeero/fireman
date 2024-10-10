# VPERM2F128

Permute Floating-Point Values

Permute 128 bit floating-point-containing fields from the first source operand (second operand) and second source operand (third operand) using bits in the 8-bit immediate and store results in the destination operand (first operand).
The first source operand is a YMM register, the second source operand is a YMM register or a 256-bit memory location, and the destination operand is a YMM register.Y1Y0SRC2X1X0SRC1X0, X1, Y0, or Y1DESTX0, X1, Y0, or Y1Figure 5-21.
 VPERM2F128 OperationImm8[1:0] select the source for the first destination 128-bit field, imm8[5:4] select the source for the second destination field.
If imm8[3] is set, the low 128-bit field is 

## Exceptions

- Other Exceptions
  > See Table2-23, "Type 6 Class Exception Conditions."
  > Additionally:
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPERM2F128CASE IMM8[1:0] of 0: DEST[127:0] := SRC1[127:0]1: DEST[127:0] := SRC1[255:128]2: DEST[127:0] := SRC2[127:0]3: DEST[127:0] := SRC2[255:128]ESACCASE IMM8[5:4] of 0: DEST[255:128] := SRC1[127:0]1: DEST[255:128] := SRC1[255:128]2: DEST[255:128] := SRC2[127:0]3: DEST[255:128] := SRC2[255:128]ESACIF (imm8[3])DEST[127:0] := 0FIIF (imm8[7])DEST[MAXVL-1:128] := 0FIIntel C/C++ Compiler Intrinsic EquivalentVPERM2F128: __m256 _mm256_permute2f128_ps (__m256 a, __m256 b, int control)VPERM2F128: __m256d _mm256_permute2f128_pd (__m256d a, __m256d b, int control)VPERM2F128: __m256i _mm256_permute2f128_si256 (__m256i a, __m256i b, int control)
```
