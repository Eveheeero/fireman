# ROUNDPD

Round Packed Double Precision Floating-Point Values

Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand).
The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
The immediate operand specifies control fields for the rounding operation, three bit fields are defined and shown in Figure4-24.
Bit 3 of the immediate byte controls processor behavior for a precision exception, bit 2 selects the source of rounding mode control.
Bits 1:0 specify a non-sticky rounding-mode value (Table 4-18 lists the encoded values for rounding-mode field).
The Precision Floating-Point Exception is signaled according to the immediate operand.
If any source operand is an SNaN then it will be converted to a QNaN.
If DAZ is set to 1 then denormals will be converted to zero before rounding.128-bit Legacy SSE version: The second source can be an XMM register or 128-bit memory location.
The destina-tion is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding YMM register destination are unmodified.VEX.128 encoded version: the source operand second source operand or a 128-bit memory location.
The destina-tion operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding YMM register destination are zeroed.VEX.256 encoded version: The source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
83102ReservedP-Precision Mask; 0: normal, 1: inexactRS-Rounding select; 1: MXCSR.RC, 0: Imm8.RCRC-Rounding modeFigure 4-24.
 Bit Control Fields of Immediate Byte for ROUNDxx InstructionTable 4-18.
 Rounding Modes and Encoding of Rounding Control (RC) FieldRounding RC Field ModeSettingRound to 00BRounded result is the closest to the infinitely precise result.
If two values are equally close, the result is nearest (even)the even value (i.e., the integer value with the least-significant bit of zero).
Round down 01BRounded result is closest to but no greater than the infinitely precise result.(toward -)Round up 10BRounded result is closest to but no less than the infinitely precise result.(toward +)Round toward 11BRounded result is closest to but no greater in absolute value than the infinitely precise result.zero (Truncate)

## Exceptions

- Other Exceptions
  > See Table2-19, "Type 2 Class Exception Conditions," additionally:
  - #UD - If VEX.vvvv
- SIMD Floating-Point Exceptions
  > Invalid (signaled only if SRC = SNaN).
  > Precision (signaled only if imm[3] = 0
  > ; if imm[3] = 1, then the Precision Mask in the MXSCSR is ignored and preci-
  > sion exception is not signaled.)
  > Note that Denormal is not signaled by ROUNDPD.

## Operation

```C
IF (imm[2] = 1) THEN // rounding mode is determined by MXCSR.RC DEST[63:0] := ConvertDPFPToInteger_M(SRC[63:0]);DEST[127:64] := ConvertDPFPToInteger_M(SRC[127:64]);ELSE// rounding mode is determined by IMM8.RCDEST[63:0] := ConvertDPFPToInteger_Imm(SRC[63:0]);DEST[127:64] := ConvertDPFPToInteger_Imm(SRC[127:64]);FIROUNDPD (128-bit Legacy SSE Version)DEST[63:0] := RoundToInteger(SRC[63:0]], ROUND_CONTROL)DEST[127:64] := RoundToInteger(SRC[127:64]], ROUND_CONTROL)DEST[MAXVL-1:128] (Unmodified)VROUNDPD (VEX.128 Encoded Version)DEST[63:0] := RoundToInteger(SRC[63:0]], ROUND_CONTROL)DEST[127:64] := RoundToInteger(SRC[127:64]], ROUND_CONTROL)DEST[MAXVL-1:128] := 0VROUNDPD (VEX.256 Encoded Version)DEST[63:0] := RoundToInteger(SRC[63:0], ROUND_CONTROL)DEST[127:64] := RoundToInteger(SRC[127:64]], ROUND_CONTROL)Intel C/C++ Compiler Intrinsic Equivalent__m128 _mm_round_pd(__m128d s1, int iRoundMode);__m128 _mm_floor_pd(__m128d s1);__m128 _mm_ceil_pd(__m128d s1)__m256 _mm256_round_pd(__m256d s1, int iRoundMode);__m256 _mm256_floor_pd(__m256d s1);__m256 _mm256_ceil_pd(__m256d s1)
```
