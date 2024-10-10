# VP2INTERSECTD/VP2INTERSECTQ

Compute Intersection Between DWORDS/QUADWORDS to a Pair of Mask Registers

This instruction writes an even/odd pair of mask registers.
The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair.
The low bit of that field is masked off (set to zero) 

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VP2INTERSECTD destmask, src1, src2(KL, VL) = (4, 128), (8, 256), (16, 512)// dest_mask_reg_id is the register id specified in the instruction for destmask:=dest_base  dest_mask_reg_id & ~1// maskregs[ ] is an array representing the mask registers:=maskregs[dest_base+0][MAX_KL-1:0]  0:=maskregs[dest_base+1][MAX_KL-1:0]  0:= 0 to KL-1:FOR i :=FOR j  0 to KL-1::=match  (src1.dword[i] == src2.dword[j])maskregs[dest_base+0].bit[i] |= matchmaskregs[dest_base+1].bit[j] |= matchVP2INTERSECTQ destmask, src1, src2(KL, VL) = (2, 128), (4, 256), (8, 512)// dest_mask_reg_id is the register id specified in the instruction for destmask:= dest_mask_reg_id & ~1dest_base // maskregs[ ] is an array representing the mask registers:=maskregs[dest_base+0][MAX_KL-1:0]  0:=maskregs[dest_base+1][MAX_KL-1:0]  0FOR i = 0 to KL-1:FOR j = 0 to KL-1::=match  (src1.qword[i] == src2.qword[j])maskregs[dest_base+0].bit[i] |=  matchmaskregs[dest_base+1].bit[j] |=  matchIntel C/C++ Compiler Intrinsic EquivalentVP2INTERSECTD void _mm_2intersect_epi32(__m128i, __m128i, __mmask8 *, __mmask8 *);VP2INTERSECTD void _mm256_2intersect_epi32(__m256i, __m256i, __mmask8 *, __mmask8 *);VP2INTERSECTD void _mm512_2intersect_epi32(__m512i, __m512i, __mmask16 *, __mmask16 *);VP2INTERSECTQ void _mm_2intersect_epi64(__m128i, __m128i, __mmask8 *, __mmask8 *);VP2INTERSECTQ void _mm256_2intersect_epi64(__m256i, __m256i, __mmask8 *, __mmask8 *);VP2INTERSECTQ void _mm512_2intersect_epi64(__m512i, __m512i, __mmask8 *, __mmask8 *);
```
