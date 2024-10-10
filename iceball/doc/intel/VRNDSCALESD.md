# VRNDSCALESD

Round Scalar Float64 Value to Include a Given Number of Fraction Bits

Rounds a double precision floating-point value in the low quadword (see Figure5-29) element of the second source operand (the third operand) by the rounding mode specified in the immediate operand and places the result in the corresponding element of the destination operand (the first operand) according to the writemask.
The quadword element at bits 127:64 of the destination is copied from the first source operand (the second operand).The destination and first source operands are XMM registers, the 2nd source operand can be an XMM register or memory location.
Bits MAXVL-1:128 of the destination register are cleared.The rounding process rounds the input to an integral value, plus number bits of fraction that are specified by imm8[7:4] (to be included in the result) and returns the result as a double precision floating-point value.It should be noticed that no overflow is induced while executing this instruction (although the source is scaled by the imm8[7:4] value).The immediate operand also specifies control fields for the rounding operation, three bit fields are defined and shown in the "Immediate Control Description" figure below.
Bit 3 of the immediate byte controls the processor behavior for a precision exception, bit 2 selects the source of rounding mode control.
Bits 1:0 specify a non-sticky rounding-mode value (immediate control table below lists the encoded values for rounding-mode field).The Precision Floating-Point Exception is signaled according to the immediate operand.
If any source operand is an SNaN then it will be converted to a QNaN.
If DAZ is set to 1 then denormals will be converted to zero before rounding.The sign of the result of this instruction is preserved, including the sign of zero.The formula of the operation for VRNDSCALESD is-MM*Round_to_INT(x*2, round_ctrl), ROUND(x) = 2round_ctrl = imm[3:0];M=imm[7:4];MThe operation of x*2 is computed as if the exponent range is unlimited (i.e., no overflow ever occurs).VRNDSCALESD is a more general form of the VEX-encoded VROUNDSD instruction.
In VROUNDSD, the formula of the operation isROUND(x) = Round_to_INT(x, round_ctrl), round_ctrl = imm[3:0];EVEX encoded version: The source operand is a XMM register or a 64-bit memory location.
The destination operand 

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
  > If SPE is enabled, precision ex
  > ception is not reported (regardless of MXCSR exception mask).

## Operation

```C
RoundToIntegerDP(SRC[63:0], imm8[7:0]) {if (imm8[2] = 1)rounding_direction := MXCSR:RC; get round control from MXCSRelserounding_direction := imm8[1:0]; get round control from imm8[1:0]FIM := imm8[7:4]; get the scaling factorcase (rounding_direction)M*SRC[63:0])00: TMP[63:0] := round_to_nearest_even_integer(2M01: TMP[63:0] := round_to_equal_or_smaller_integer(2*SRC[63:0])M10: TMP[63:0] := round_to_equal_or_larger_integer(2*SRC[63:0])M11: TMP[63:0] := round_to_nearest_smallest_magnitude_integer(2*SRC[63:0])ESAC-M-MDest[63:0] := 2* TMP[63:0] ; scale down back to 2if (imm8[3] = 0) Then; check SPEif (SRC[63:0] != Dest[63:0]) Then; check precision lostset_precision(); set #PEFI;FI;return(Dest[63:0])}VRNDSCALESD (EVEX encoded version)IF k1[0] or *no writemask*THENDEST[63:0] := RoundToIntegerDP(SRC2[63:0], Zero_upper_imm[7:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[63:0] := 0FI;FI;DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVRNDSCALESD __m128d _mm_roundscale_sd ( __m128d a, __m128d b, int imm);VRNDSCALESD __m128d _mm_roundscale_round_sd ( __m128d a, __m128d b, int imm, int sae);VRNDSCALESD __m128d _mm_mask_roundscale_sd (__m128d s, __mmask8 k, __m128d a, __m128d b, int imm);VRNDSCALESD __m128d _mm_mask_roundscale_round_sd (__m128d s, __mmask8 k, __m128d a, __m128d b, int imm, int sae);VRNDSCALESD __m128d _mm_maskz_roundscale_sd ( __mmask8 k, __m128d a, __m128d b, int imm);VRNDSCALESD __m128d _mm_maskz_roundscale_round_sd ( __mmask8 k, __m128d a, __m128d b, int imm, int sae);
```
