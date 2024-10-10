# VRNDSCALEPS

Round Packed Float32 Values to Include a Given Number of Fraction Bits

Round the single-precision floating-point values in the source operand by the rounding mode specified in the immediate operand (see Figure5-29) and places the result in the destination operand.The destination operand (the first operand) is a ZMM register conditionally updated according to the writemask.
The source operand (the second operand) can be a ZMM register, a 512-bit memory location, or a 512-bit vector broadcasted from a 32-bit memory location.The rounding process rounds the input to an integral value, plus number bits of fraction that are specified by imm8[7:4] (to be included in the result) and returns the result as a single-precision floating-point value.It should be noticed that no overflow is induced while executing this instruction (although the source is scaled by the imm8[7:4] value).The immediate operand also specifies control fields for the rounding operation, three bit fields are defined and shown in the "Immediate Control Description" figure below.
Bit 3 of the immediate byte controls the processor behavior for a precision exception, bit 2 selects the source of rounding mode control.
Bits 1:0 specify a non-sticky rounding-mode value (immediate control table below lists the encoded values for rounding-mode field).The Precision Floating-Point Exception is signaled according to the immediate operand.
If any source operand is an SNaN then it will be converted to a QNaN.
If DAZ is set to 1 then denormals will be converted to zero before rounding.The sign of the result of this instruction is preserved, including the sign of zero.The formula of the operation on each data element for VRNDSCALEPS is-MM*Round_to_INT(x*2, round_ctrl), ROUND(x) = 2round_ctrl = imm[3:0];M=imm[7:4];MThe operation of x*2 is computed as if the exponent range is unlimited (i.e., no overflow ever occurs).VRNDSCALEPS is a more general form of the VEX-encoded VROUNDPS instruction.
In VROUNDPS, the formula of the operation on each element isNote: EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.Handling of special case of input values are listed in Table 5-21.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
  > If SPE is enabled, precision ex
  > ception is not reported (regardless of MXCSR exception mask).

## Operation

```C
RoundToIntegerSP(SRC[31:0], imm8[7:0]) {if (imm8[2] = 1)rounding_direction := MXCSR:RC; get round control from MXCSRelserounding_direction := imm8[1:0]; get round control from imm8[1:0]FIM := imm8[7:4]; get the scaling factorcase (rounding_direction)M*SRC[31:0])00: TMP[31:0] := round_to_nearest_even_integer(2M01: TMP[31:0] := round_to_equal_or_smaller_integer(2*SRC[31:0])M10: TMP[31:0] := round_to_equal_or_larger_integer(2*SRC[31:0])M11: TMP[31:0] := round_to_nearest_smallest_magnitude_integer(2*SRC[31:0])ESAC;-M-MDest[31:0] := 2* TMP[31:0] ; scale down back to 2if (imm8[3] = 0) Then; check SPEif (SRC[31:0] != Dest[31:0]) Then; check precision lostset_precision(); set #PEFI;FI;return(Dest[31:0])}VRNDSCALEPS (EVEX encoded versions) (KL, VL) = (4, 128), (8, 256), (16, 512)IF *src is a memory operand*THEN TMP_SRC := BROADCAST32(SRC, VL, k1)ELSE TMP_SRC := SRCFI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := RoundToIntegerSP(TMP_SRC[i+31:i]), imm8[7:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FI;FI;Intel C/C++ Compiler Intrinsic EquivalentVRNDSCALEPS __m512 _mm512_roundscale_ps( __m512 a, int imm);VRNDSCALEPS __m512 _mm512_roundscale_round_ps( __m512 a, int imm, int sae);VRNDSCALEPS __m512 _mm512_mask_roundscale_ps(__m512 s, __mmask16 k, __m512 a, int imm);VRNDSCALEPS __m512 _mm512_mask_roundscale_round_ps(__m512 s, __mmask16 k, __m512 a, int imm, int sae);VRNDSCALEPS __m512 _mm512_maskz_roundscale_ps( __mmask16 k, __m512 a, int imm);VRNDSCALEPS __m512 _mm512_maskz_roundscale_round_ps( __mmask16 k, __m512 a, int imm, int sae);VRNDSCALEPS __m256 _mm256_roundscale_ps( __m256 a, int imm);VRNDSCALEPS __m256 _mm256_mask_roundscale_ps(__m256 s, __mmask8 k, __m256 a, int imm);VRNDSCALEPS __m256 _mm256_maskz_roundscale_ps( __mmask8 k, __m256 a, int imm);VRNDSCALEPS __m128 _mm_roundscale_ps( __m256 a, int imm);VRNDSCALEPS __m128 _mm_mask_roundscale_ps(__m128 s, __mmask8 k, __m128 a, int imm);VRNDSCALEPS __m128 _mm_maskz_roundscale_ps( __mmask8 k, __m128 a, int imm);
```
