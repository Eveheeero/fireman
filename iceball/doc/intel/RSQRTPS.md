# RSQRTPS

Compute Reciprocals of Square Roots of Packed Single Precision Floating-Point Values

Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand.
The source operand can be an XMM register or a 128-bit memory location.
The ® 64 and IA-32 Architectures Software Devel-destination operand is an XMM register.
See Figure 10-5 in the Inteloper's Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.The relative error for this approximation is:-12 |Relative Error| 1.5  2 The RSQRTPS instruction is not affected by the rounding control bits in the MXCSR register.
When a source value is a 0.0, an  of the sign of the source value is returned.
A denormal source value is treated as a 0.0 (of the same sign).
When a source value is a negative value (other than -0.0), a floating-point indefinite is returned.
When a source value is an SNaN or QNaN, the SNaN is converted to a QNaN or the source QNaN is returned.
In 64-bit mode, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding YMM register destination are unmodified.VEX.128 encoded version: the first source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding YMM register destination are zeroed.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.


## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > See Table2-21, "Type 4 Class Exception Conditions," additionally:
  - #UD - If VEX.vvvv

## Operation

```C
RSQRTPS (128-bit Legacy SSE Version)DEST[31:0] := APPROXIMATE(1/SQRT(SRC[31:0]))DEST[63:32] := APPROXIMATE(1/SQRT(SRC1[63:32]))DEST[95:64] := APPROXIMATE(1/SQRT(SRC1[95:64]))DEST[127:96] := APPROXIMATE(1/SQRT(SRC2[127:96]))DEST[MAXVL-1:128] (Unmodified)VRSQRTPS (VEX.128 Encoded Version)DEST[31:0] := APPROXIMATE(1/SQRT(SRC[31:0]))DEST[63:32] := APPROXIMATE(1/SQRT(SRC1[63:32]))DEST[95:64] := APPROXIMATE(1/SQRT(SRC1[95:64]))DEST[127:96] := APPROXIMATE(1/SQRT(SRC2[127:96]))DEST[MAXVL-1:128] := 0VRSQRTPS (VEX.256 Encoded Version)DEST[31:0] := APPROXIMATE(1/SQRT(SRC[31:0]))DEST[63:32] := APPROXIMATE(1/SQRT(SRC1[63:32]))DEST[95:64] := APPROXIMATE(1/SQRT(SRC1[95:64]))DEST[127:96] := APPROXIMATE(1/SQRT(SRC2[127:96]))DEST[159:128] := APPROXIMATE(1/SQRT(SRC2[159:128]))DEST[191:160] := APPROXIMATE(1/SQRT(SRC2[191:160]))DEST[223:192] := APPROXIMATE(1/SQRT(SRC2[223:192]))DEST[255:224] := APPROXIMATE(1/SQRT(SRC2[255:224]))Intel C/C++ Compiler Intrinsic EquivalentRSQRTPS __m128 _mm_rsqrt_ps(__m128 a)RSQRTPS __m256 _mm256_rsqrt_ps (__m256 a);
```
