# VCMPPH

Compare Packed FP16 Values

This instruction compares packed FP16 values from source operands and stores the result in the destination mask operand.
The comparison predicate operand (immediate byte bits 4:0) specifies the type of comparison performed on each of the pairs of packed values.
The destination elements are updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.

## Operation

```C
CASE (imm8 & 0x1F) OF0: CMP_OPERATOR := EQ_OQ;1: CMP_OPERATOR := LT_OS;2: CMP_OPERATOR := LE_OS;3: CMP_OPERATOR := UNORD_Q;4: CMP_OPERATOR := NEQ_UQ;5: CMP_OPERATOR := NLT_US;6: CMP_OPERATOR := NLE_US;7: CMP_OPERATOR := ORD_Q;8: CMP_OPERATOR := EQ_UQ;9: CMP_OPERATOR := NGE_US;10: CMP_OPERATOR := NGT_US;11: CMP_OPERATOR := FALSE_OQ;12: CMP_OPERATOR := NEQ_OQ;13: CMP_OPERATOR := GE_OS;14: CMP_OPERATOR := GT_OS;15: CMP_OPERATOR := TRUE_UQ;16: CMP_OPERATOR := EQ_OS;17: CMP_OPERATOR := LT_OQ;18: CMP_OPERATOR := LE_OQ;19: CMP_OPERATOR := UNORD_S;20: CMP_OPERATOR := NEQ_US;21: CMP_OPERATOR := NLT_UQ;24: CMP_OPERATOR := EQ_US;25: CMP_OPERATOR := NGE_UQ;26: CMP_OPERATOR := NGT_UQ;27: CMP_OPERATOR := FALSE_OS;28: CMP_OPERATOR := NEQ_OS;29: CMP_OPERATOR := GE_OQ;30: CMP_OPERATOR := GT_OQ;31: CMP_OPERATOR := TRUE_US;ESACVCMPPH (EVEX Encoded Versions)VL = 128, 256 or 512KL := VL/16FOR j := 0 TO KL-1:IF k2[j] OR *no writemask*:IF EVEX.b = 1:tsrc2 := SRC2.fp16[0]ELSE:tsrc2 := SRC2.fp16[j]DEST.bit[j] := SRC1.fp16[j] CMP_OPERATOR tsrc2ELSEDEST.bit[j] := 0DEST[MAXKL-1:KL] := 0Intel C/C++ Compiler Intrinsic EquivalentVCMPPH ___mmask8 _mm_cmp_ph_mask (__m128h a, __m128h b, const int imm8);VCMPPH ___mmask8 _mm_mask_cmp_ph_mask (__mmask8 k1, __m128h a, __m128h b, const int imm8);VCMPPH ___mmask16 _mm256_cmp_ph_mask (__m256h a, __m256h b, const int imm8);VCMPPH ___mmask16 _mm256_mask_cmp_ph_mask (__mmask16 k1, __m256h a, __m256h b, const int imm8);VCMPPH ___mmask32 _mm512_cmp_ph_mask (__m512h a, __m512h b, const int imm8);VCMPPH ___mmask32 _mm512_mask_cmp_ph_mask (__mmask32 k1, __m512h a, __m512h b, const int imm8);VCMPPH ___mmask32 _mm512_cmp_round_ph_mask (__m512h a, __m512h b, const int imm8, const int sae);VCMPPH ___mmask32 _mm512_mask_cmp_round_ph_mask (__mmask32 k1, __m512h a, __m512h b, const int imm8, const int sae);
```
