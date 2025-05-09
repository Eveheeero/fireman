# VRCP14PD

Compute Approximate Reciprocals of Packed Float64 Values

This instruction performs a SIMD computation of the approximate reciprocals of eight/four/two packed double precision floating-point values in the source operand (the second operand) and stores the packed double precision -floating-point results in the destination operand.
The maximum relative error for this approximation is less than 214.
The source operand can be a ZMM register, a 512-bit memory location, or a 512-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM register conditionally updated according to the writemask.The VRCP14PD instruction is not affected by the rounding control bits in the MXCSR register.
When a source value is a 0.0, an  with the sign of the source value is returned.
A denormal source value will be treated as zero only in case of DAZ bit set in MXCSR.
Otherwise it is treated correctly (i.e., not as a 0.0).
Underflow results are flushed to zero only in case of FTZ bit set in MXCSR.
Otherwise it will be treated correctly (i.e., correct underflow result is written) with the sign of the operand.
When a source value is a SNaN or QNaN, the SNaN is converted to a QNaN or the source QNaN is returned.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.MXCSR exception flags are not affected by this instruction and floating-point exceptions are not reported.Table 5-16.
VRCP14PD/VRCP14SD Special CasesInput valueResult valueComments-10240  " X  " 2INFVery small denormal-1024 -2 " X  " -0-INFVery small denormal1022X > 2UnderflowUp to 18 bits of fractions are returned*1022-UnderflowUp to 18 bits of fractions are returned*X < -2-nnX = 22-nnX = -2-2* in this case the mantissa is shifted right by one or two bitsA numerically exact implementation of VRCP14xx can be found at https://software.intel.com/en-us/articles/refer-ence-implementations-for-IA-approximation-inst

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VRCP14PD ((EVEX encoded versions) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC *is memory*)THEN DEST[i+63:i] := APPROXIMATE(1.0/SRC[63:0]);ELSE DEST[i+63:i] := APPROXIMATE(1.0/SRC[i+63:i]);FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FI;FI;ENDFOR;DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVRCP14PD __m512d _mm512_rcp14_pd( __m512d a);VRCP14PD __m512d _mm512_mask_rcp14_pd(__m512d s, __mmask8 k, __m512d a);VRCP14PD __m512d _mm512_maskz_rcp14_pd( __mmask8 k, __m512d a);
```
