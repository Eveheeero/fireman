# VCMPSH

Compare Scalar FP16 Values

This instruction compares the FP16 values from the lowest element of the source operands and stores the result in the destination mask operand.
The comparison predicate operand (immediate byte bits 4:0) specifies the type of comparison performed on the pair of packed FP16 values.
The low destination bit is updated according to the writemask.
Bits MAXKL-1:1 of the destination operand are zeroed.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.

## Operation

```C
CASE (imm8 & 0x1F) OF0: CMP_OPERATOR := EQ_OQ;1: CMP_OPERATOR := LT_OS;2: CMP_OPERATOR := LE_OS;3: CMP_OPERATOR := UNORD_Q;4: CMP_OPERATOR := NEQ_UQ;5: CMP_OPERATOR := NLT_US;6: CMP_OPERATOR := NLE_US;7: CMP_OPERATOR := ORD_Q;8: CMP_OPERATOR := EQ_UQ;9: CMP_OPERATOR := NGE_US;10: CMP_OPERATOR := NGT_US;11: CMP_OPERATOR := FALSE_OQ;12: CMP_OPERATOR := NEQ_OQ;13: CMP_OPERATOR := GE_OS;14: CMP_OPERATOR := GT_OS;15: CMP_OPERATOR := TRUE_UQ;16: CMP_OPERATOR := EQ_OS;17: CMP_OPERATOR := LT_OQ;18: CMP_OPERATOR := LE_OQ;19: CMP_OPERATOR := UNORD_S;20: CMP_OPERATOR := NEQ_US;21: CMP_OPERATOR := NLT_UQ;22: CMP_OPERATOR := NLE_UQ;23: CMP_OPERATOR := ORD_S;24: CMP_OPERATOR := EQ_US;25: CMP_OPERATOR := NGE_UQ;26: CMP_OPERATOR := NGT_UQ;27: CMP_OPERATOR := FALSE_OS;28: CMP_OPERATOR := NEQ_OS;31: CMP_OPERATOR := TRUE_US;ESAC VCMPSH (EVEX Encoded Versions)IF k2[0] OR *no writemask*:DEST.bit[0] := SRC1.fp16[0] CMP_OPERATOR SRC2.fp16[0]ELSEDEST.bit[0] := 0DEST[MAXKL-1:1] := 0Intel C/C++ Compiler Intrinsic EquivalentVCMPSH __mmask8 _mm_cmp_round_sh_mask (__m128h a, __m128h b, const int imm8, const int sae);VCMPSH __mmask8 _mm_mask_cmp_round_sh_mask (__mmask8 k1, __m128h a, __m128h b, const int imm8, const int sae);VCMPSH __mmask8 _mm_cmp_sh_mask (__m128h a, __m128h b, const int imm8);VCMPSH __mmask8 _mm_mask_cmp_sh_mask (__mmask8 k1, __m128h a, __m128h b, const int imm8);
```
