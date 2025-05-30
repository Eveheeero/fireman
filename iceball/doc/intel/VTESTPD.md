# VTESTPD/VTESTPS

Packed Bit Test

VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand.
If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear.
If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear.
An attempt to execute VTESTPS with VEX.W=1 will cause #UD.VTESTPD performs a bitwise comparison of all the sign bits of the double precision elements in the first source operation and corresponding sign bits in the second source operand.
If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear.
If the AND the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear.
An attempt to execute VTESTPS with VEX.W=1 will cause #UD.The first source register is specified by the ModR/M reg field.128-bit version: The first source register is an XMM register.
The second source register can be an XMM register or a 128-bit memory location.
The destination register is not modified.VEX.256 encoded version: The first source register is a YMM register.
The second source register can be a YMM register or a 256-bit memory location.
The destination register is not modified.

## Flags affected

- The OF, AF, PF, SF flags are cleared and the ZF, CF flags are set according to the operation.

## Exceptions

- Other Exceptions
  > See Table2-21, "Type 4 Class Exception Conditions."
  > Additionally:
  - #UD - If VEX.vvvv
  >  
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VTESTPS (128-bit version)TEMP[127:0] := SRC[127:0] AND DEST[127:0]IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127] = 0)THEN ZF := 1;ELSE ZF := 0;TEMP[127:0] := SRC[127:0] AND NOT DEST[127:0]IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127] = 0)THEN CF := 1;ELSE CF := 0;DEST (unmodified)AF := OF := PF := SF := 0;VTESTPS (VEX.256 encoded version)TEMP[255:0] := SRC[255:0] AND DEST[255:0]IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127]= TEMP[160] =TEMP[191] = TEMP[224] = TEMP[255] = 0)THEN ZF := 1;ELSE ZF := 0;TEMP[255:0] := SRC[255:0] AND NOT DEST[255:0]IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127]= TEMP[160] =TEMP[191] = TEMP[224] = TEMP[255] = 0)THEN CF := 1;ELSE CF := 0;DEST (unmodified)AF := OF := PF := SF := 0;VTESTPD (128-bit version)TEMP[127:0] := SRC[127:0] AND DEST[127:0]IF ( TEMP[63] = TEMP[127] = 0)THEN ZF := 1;ELSE ZF := 0;TEMP[127:0] := SRC[127:0] AND NOT DEST[127:0]IF ( TEMP[63] = TEMP[127] = 0)THEN CF := 1;ELSE CF := 0;DEST (unmodified)AF := OF := PF := SF := 0;VTESTPD (VEX.256 encoded version)TEMP[255:0] := SRC[255:0] AND DEST[255:0]IF (TEMP[63] = TEMP[127] = TEMP[191] = TEMP[255] = 0)THEN ZF := 1;ELSE ZF := 0;TEMP[255:0] := SRC[255:0] AND NOT DEST[255:0]IF (TEMP[63] = TEMP[127] = TEMP[191] = TEMP[255] = 0)THEN CF := 1;ELSE CF := 0;Intel C/C++ Compiler Intrinsic EquivalentVTESTPSint _mm256_testz_ps (__m256 s1, __m256 s2);int _mm256_testc_ps (__m256 s1, __m256 s2);int _mm256_testnzc_ps (__m256 s1, __m128 s2);int _mm_testz_ps (__m128 s1, __m128 s2);int _mm_testc_ps (__m128 s1, __m128 s2);int _mm_testnzc_ps (__m128 s1, __m128 s2);VTESTPDint _mm256_testz_pd (__m256d s1, __m256d s2);int _mm256_testc_pd (__m256d s1, __m256d s2);int _mm256_testnzc_pd (__m256d s1, __m256d s2);int _mm_testz_pd (__m128d s1, __m128d s2);int _mm_testc_pd (__m128d s1, __m128d s2);int _mm_testnzc_pd (__m128d s1, __m128d s2);
```
