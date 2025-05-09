# VRNDSCALESH

Round Scalar FP16 Value to Include a Given Number of Fraction Bits

This instruction rounds the low FP16 value in the second source operand by the rounding mode specified in the immediate operand (see Table 5-22) and places the result in the destination operand.Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.The rounding process rounds the input to an integral value, plus number bits of fraction that are specified by imm8[7:4] (to be included in the result), and returns the result as a FP16 value.Note that no overflow is induced while executing this instruction (although the source is scaled by the imm8[7:4] value).The immediate operand also specifies control fields for the rounding operation.
Three bit fields are defined and shown in Table5-22, "Imm8 Controls for VRNDSCALEPH/VRNDSCALESH." Bit 3 of the immediate byte controls the processor behavior for a precision exception, bit 2 selects the source of rounding mode control, and bits 1:0 specify a non-sticky rounding-mode value.The Precision Floating-Point Exception is signaled according to the immediate operand.
If any source operand is an SNaN then it will be converted to a QNaN.The sign of the result of this instruction is preserved, including the sign of zero.
Special cases are described in Table 5-23.If this instruction encoding's SPE bit (bit 3) in the immediate operand is 1, VRNDSCALESH can set MXCSR.UE without MXCSR.PE.The formula of the operation on each data element for VRNDSCALESH is: íMM *Round_to_INT(x * 2, round_ctrl), round_ctrl = imm[3:0];ROUND(x) = 2M=imm[7:4];M is computed as if the exponent range is unlimited (i.e., no overflow ever occurs).The operation of x * 2

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Precision.
- Other Exceptions

## Operation

```C
VRNDSCALESH dest{k1}, src1, src2, imm8IF k1[0] or *no writemask*:DEST.fp16[0] := round_fp16_to_integer(src2.fp16[0], imm8) // see VRNDSCALEPHELSE IF *zeroing*:DEST.fp16[0] := 0//else DEST.fp16[0] remains unchangedIntel C/C++ Compiler Intrinsic EquivalentVRNDSCALESH __m128h _mm_mask_roundscale_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, int imm8, const int sae);VRNDSCALESH __m128h _mm_maskz_roundscale_round_sh (__mmask8 k, __m128h a, __m128h b, int imm8, const int sae);VRNDSCALESH __m128h _mm_roundscale_round_sh (__m128h a, __m128h b, int imm8, const int sae);VRNDSCALESH __m128h _mm_mask_roundscale_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, int imm8);VRNDSCALESH __m128h _mm_maskz_roundscale_sh (__mmask8 k, __m128h a, __m128h b, int imm8);VRNDSCALESH __m128h _mm_roundscale_sh (__m128h a, __m128h b, int imm8);
```
