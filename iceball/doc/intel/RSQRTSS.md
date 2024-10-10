# RSQRTSS

Compute Reciprocal of Square Root of Scalar Single Precision Floating-Point Value

Computes an approximate reciprocal of the square root of the low single precision floating-point value in the source operand (second operand) stores the single precision floating-point result in the destination operand.
The source operand can be an XMM register or a 32-bit memory location.
The destination operand is an XMM register.
The ® 64 and three high-order doublewords of the destination operand remain unchanged.
See Figure 10-6 in the IntelIA-32 Architectures Software Developer's Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.The relative error for this approximation is:-12|Relative Error|  1.5  2 The RSQRTSS instruction is not affected by the rounding control bits in the MXCSR register.
When a source value is a 0.0, an  of the sign of the source value is returned.
A denormal source value is treated as a 0.0 (of the same sign).
When a source value is a negative value (other than -0.0), a floating-point indefinite is returned.
When a source value is an SNaN or QNaN, the SNaN is converted to a QNaN or the source QNaN is returned.
In 64-bit mode, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).128-bit Legacy SSE version: The first source operand and the destination operand are the same.
Bits (MAXVL-1:32) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: Bits (MAXVL-1:128) of the destination YMM register are zeroed.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
RSQRTSS (128-bit Legacy SSE Version)DEST[31:0] := APPROXIMATE(1/SQRT(SRC2[31:0]))DEST[MAXVL-1:32] (Unmodified)VRSQRTSS (VEX.128 Encoded Version)DEST[31:0] := APPROXIMATE(1/SQRT(SRC2[31:0]))Intel C/C++ Compiler Intrinsic EquivalentRSQRTSS __m128 _mm_rsqrt_ss(__m128 a)
```
