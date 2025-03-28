# VRCP14PS

Compute Approximate Reciprocals of Packed Float32 Values

This instruction performs a SIMD computation of the approximate reciprocals of the packed single-precision floating-point values in the source operand (the second operand) and stores the packed single-precision floating-point results in the destination operand (the first operand).
The maximum relative error for this approximation is -14.
less than 2The source operand can be a ZMM register, a 512-bit memory location or a 512-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM register conditionally updated according to the writemask.The VRCP14PS instruction is not affected by the rounding control bits in the MXCSR register.
When a source value is a 0.0, an  with the sign of the source value is returned.
A denormal source value will be treated as zero only in case of DAZ bit set in MXCSR.
Otherwise it is treated correctly (i.e., not as a 0.0).
Underflow results are flushed to zero only in case of FTZ bit set in MXCSR.
Otherwise it will be treated correctly (i.e., correct underflow result is written) with the sign of the operand.
When a source value is a SNaN or QNaN, the SNaN is converted to a QNaN or the source QNaN is returned.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.MXCSR exception flags are not affected by this instruction and floating-point exceptions are not reported.Table 5-17.
VRCP14PS/VRCP14SS Special CasesInput valueResult valueComments-1280  " X  " 2INFVery small denormal-128 -2 " X  " -0-INFVery small denormal1261X > 2UnderflowUp to 18 bits of fractions are returned1261X < -2-UnderflowUp to 18 bits of fractions are returned-nnX = 22-nnX = -2-2NOTES:1.
In this case, the mantissa is shifted right by one or two bits.A numerically exact implementation of VRCP14xx can be found at:https://software.intel.com/en-us/articles/reference-implementations-for-IA-approximation-instructions-vrcp14-

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VRCP14PS (EVEX encoded versions) (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC *is memory*)THEN DEST[i+31:i] := APPROXIMATE(1.0/SRC[31:0]);ELSE DEST[i+31:i] := APPROXIMATE(1.0/SRC[i+31:i]);FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FI;FI;ENDFOR;DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVRCP14PS __m512 _mm512_rcp14_ps( __m512 a);VRCP14PS __m512 _mm512_mask_rcp14_ps(__m512 s, __mmask16 k, __m512 a);VRCP14PS __m512 _mm512_maskz_rcp14_ps( __mmask16 k, __m512 a);VRCP14PS __m256 _mm256_rcp14_ps( __m256 a);VRCP14PS __m256 _mm512_mask_rcp14_ps(__m256 s, __mmask8 k, __m256 a);VRCP14PS __m256 _mm512_maskz_rcp14_ps( __mmask8 k, __m256 a);VRCP14PS __m128 _mm_rcp14_ps( __m128 a);VRCP14PS __m128 _mm_mask_rcp14_ps(__m128 s, __mmask8 k, __m128 a);VRCP14PS __m128 _mm_maskz_rcp14_ps( __mmask8 k, __m128 a);
```
