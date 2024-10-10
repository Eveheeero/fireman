# PTEST

Logical Compare

PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand).
VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.The first source register is specified by the ModR/M reg field.128-bit versions: The first source register is an XMM register.
The second source register can be an XMM register or a 128-bit memory location.
The destination register is not modified.VEX.256 encoded version: The first source register is a YMM register.
The second source register can be a YMM register or a 256-bit memory location.
The destination register is not modified.Note: In VEX-encoded versions, VEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Flags affected

- The OF, AF, PF, SF flags are cleared and the ZF, CF flags are set according to the operation.

## Exceptions

- Other Exceptions
  > See Table2-21, "Type 4 Class Exception Conditions," additionally:
  - #UD - If VEX.vvvv
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
(V)PTEST (128-bit Version)IF (SRC[127:0] BITWISE AND DEST[127:0] = 0) THEN ZF := 1;ELSE ZF := 0;IF (SRC[127:0] BITWISE AND NOT DEST[127:0] = 0) THEN CF := 1;ELSE CF := 0;DEST (unmodified)AF := OF := PF := SF := 0;VPTEST (VEX.256 Encoded Version)IF (SRC[255:0] BITWISE AND DEST[255:0] = 0) THEN ZF := 1;ELSE ZF := 0;IF (SRC[255:0] BITWISE AND NOT DEST[255:0] = 0) THEN CF := 1;ELSE CF := 0;Intel C/C++ Compiler Intrinsic EquivalentPTEST int _mm_testz_si128 (__m128i s1, __m128i s2);PTEST int _mm_testc_si128 (__m128i s1, __m128i s2);PTEST int _mm_testnzc_si128 (__m128i s1, __m128i s2);VPTEST int _mm256_testz_si256 (__m256i s1, __m256i s2);VPTEST int _mm256_testc_si256 (__m256i s1, __m256i s2);VPTEST int _mm256_testnzc_si256 (__m256i s1, __m256i s2);VPTEST int _mm_testz_si128 (__m128i s1, __m128i s2);VPTEST int _mm_testc_si128 (__m128i s1, __m128i s2);VPTEST int _mm_testnzc_si128 (__m128i s1, __m128i s2);
```
