# VRNDSCALESS

Round Scalar Float32 Value to Include a Given Number of Fraction Bits

Rounds the single-precision floating-point value in the low doubleword element of the second source operand (the third operand) by the rounding mode specified in the immediate operand (see Figure5-29) and places the result in the corresponding element of the destination operand (the first operand) according to the writemask.
The double-word elements at bits 127:32 of the destination are copied from the first source operand (the second operand).The destination and first source operands are XMM registers, the 2nd source operand can be an XMM register or memory location.
Bits MAXVL-1:128 of the destination register are cleared.The rounding process rounds the input to an integral value, plus number bits of fraction that are specified by imm8[7:4] (to be included in the result) and returns the result as a single-precision floating-point value.It should be noticed that no overflow is induced while executing this instruction (although the source is scaled by the imm8[7:4] value).The immediate operand also specifies control fields for the rounding operation, three bit fields are defined and shown in the "Immediate Control Description" figure below.
Bit 3 of the immediate byte controls the processor behavior for a precision exception, bit 2 selects the source of rounding mode control.
Bits 1:0 specify a non-sticky rounding-mode value (immediate control tables below lists the encoded values for rounding-mode field).The Precision Floating-Point Exception is signaled according to the immediate operand.
If any source operand is an SNaN then it will be converted to a QNaN.
If DAZ is set to 1 then denormals will be converted to zero before rounding.The sign of the result of this instruction is preserved, including the sign of zero.The formula of the operation for VRNDSCALESS is-MM*Round_to_INT(x*2, round_ctrl), ROUND(x) = 2round_ctrl = imm[3:0];M=imm[7:4];The operation of x*2M is computed as if the exponent range is unlimited (i.e., no overflow ever occurs).VRNDSCALESS is a more general form of the VEX-encoded VROUNDSS instruction.
In VROUNDSS, the formula of the operation on each element isROUND(x) = Round_to_INT(x, round_ctrl), round_ctrl = imm[3:0];EVEX encoded version: The source operand is a XMM register or a 32-bit memory location.
The destination operand 

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
  > If SPE is enabled, precision ex
  > ception is not reported (regardless of MXCSR exception mask).

## Operation

```C
RoundToIntegerSP(SRC[31:0], imm8[7:0]) {if (imm8[2] = 1)rounding_direction := MXCSR:RC; get round control from MXCSRelserounding_direction := imm8[1:0]; get round control from imm8[1:0]FIM := imm8[7:4]; get the scaling factorcase (rounding_direction)M*SRC[31:0])00: TMP[31:0] := round_to_nearest_even_integer(2M01: TMP[31:0] := round_to_equal_or_smaller_integer(2*SRC[31:0])M10: TMP[31:0] := round_to_equal_or_larger_integer(2*SRC[31:0])M11: TMP[31:0] := round_to_nearest_smallest_magnitude_integer(2*SRC[31:0])ESAC;-M-MDest[31:0] := 2* TMP[31:0] ; scale down back to 2if (imm8[3] = 0) Then; check SPEif (SRC[31:0] != Dest[31:0]) Then; check precision lostset_precision(); set #PEFI;FI;return(Dest[31:0])}VRNDSCALESS (EVEX encoded version)IF k1[0] or *no writemask*THENDEST[31:0] := RoundToIntegerSP(SRC2[31:0], Zero_upper_imm[7:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0FI;FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVRNDSCALESS __m128 _mm_roundscale_ss ( __m128 a, __m128 b, int imm);VRNDSCALESS __m128 _mm_roundscale_round_ss ( __m128 a, __m128 b, int imm, int sae);VRNDSCALESS __m128 _mm_mask_roundscale_ss (__m128 s, __mmask8 k, __m128 a, __m128 b, int imm);VRNDSCALESS __m128 _mm_mask_roundscale_round_ss (__m128 s, __mmask8 k, __m128 a, __m128 b, int imm, int sae);VRNDSCALESS __m128 _mm_maskz_roundscale_ss ( __mmask8 k, __m128 a, __m128 b, int imm);VRNDSCALESS __m128 _mm_maskz_roundscale_round_ss ( __mmask8 k, __m128 a, __m128 b, int imm, int sae);
```
