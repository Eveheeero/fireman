# ROUNDSS

Round Scalar Single Precision Floating-Point Values

Round the single precision floating-point value in the lowest dword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand).
The rounding process rounds a single precision floating-point input to an integer value and returns the result as a single precision floating-point value in the lowest position.
The upper three single precision floating-point values in the destination are retained.
The immediate operand specifies control fields for the rounding operation, three bit fields are defined and shown in Figure4-24.
Bit 3 of the immediate byte controls processor behavior for a precision exception, bit 2 selects the source of rounding mode control.
Bits 1:0 specify a non-sticky rounding-mode value (Table 4-18 lists the encoded values for rounding-mode field).
The Precision Floating-Point Exception is signaled according to the immediate operand.
If any source operand is an SNaN then it will be converted to a QNaN.
If DAZ is set to 1 then denormals will be converted to zero before rounding.128-bit Legacy SSE version: The first source operand and the destination operand are the same.
Bits (MAXVL-1:32) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: Bits (MAXVL-1:128) of the destination YMM register are zeroed.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid (signaled only if SRC = SNaN).
  > Precision (signaled only if imm[3] = 0
  > ; if imm[3] = 1, then the Precision Mask in the MXSCSR is ignored and preci-
  > sion exception is not signaled.)
  > Note that Denormal is not signaled by ROUNDSS.

## Operation

```C
IF (imm[2] = 1) THEN // rounding mode is determined by MXCSR.RC DEST[31:0] := ConvertSPFPToInteger_M(SRC[31:0]);ELSE// rounding mode is determined by IMM8.RCDEST[31:0] := ConvertSPFPToInteger_Imm(SRC[31:0]);FI;DEST[127:32] remains unchanged ;ROUNDSS (128-bit Legacy SSE Version)VROUNDSS (VEX.128 Encoded Version)DEST[31:0] := RoundToInteger(SRC2[31:0], ROUND_CONTROL)DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentROUNDSS __m128 mm_round_ss(__m128 dst, __m128 s1, int iRoundMode);ROUNDSS __m128 mm_floor_ss(__m128 dst, __m128 s1);ROUNDSS __m128 mm_ceil_ss(__m128 dst, __m128 s1);
```
