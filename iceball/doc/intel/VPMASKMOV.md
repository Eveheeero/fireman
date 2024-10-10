# VPMASKMOV

Conditional SIMD Integer Packed Loads and Stores

Conditionally moves packed data elements from the second source operand into the corresponding data element of the destination operand, depending on the mask bits associated with each data element.
The mask bits are speci-fied in the first source operand.
The mask bit for each data element is the most significant bit of that element in the first source operand.
If a mask is 1, the corresponding data element is copied from the second source operand to the destination operand.
If the mask is 0, the corresponding data element is set to zero in the load form of these instructions, and unmodified in the store form.
The second source operand is a memory address for the load form of these instructions.
The destination operand is a memory address for the store form of these instructions.
The other operands are either XMM registers (for VEX.128 version) or YMM registers (for VEX.256 version).Faults occur only due to mask-bit required memory accesses that caused the faults.
Faults will not occur due to referencing any memory location if the corresponding mask bit for that memory location is 0.
For example, no faults will be detected if the mask bits are all zero.Unlike previous MASKMOV instructions (MASKMOVQ and MASKMOVDQU), a nontemporal hint is not applied to these instructions.Instruction behavior on alignment check reporting with mask VMASKMOV should not be used to access memory mapped I/O as the ordering of the individual loads or stores it does is implementation specific.
In cases where mask bits indicate data should not be loaded or stored paging A and D bits will be set in an imple-mentation dependent way.
However, A and D bits are always set for pages where data is actually loaded/stored.Note: for load forms, the first source (the mask) is encoded in VEX.vvvv; the second source is encoded in rm_field, and the destination register is encoded in reg_field.Note: for store forms, the first source (the mask) is encoded in VEX.vvvv; the second source register is encoded in reg_field, and the destination memory location is encoded in rm_field.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPMASKMOVD - 256-bit loadDEST[31:0] := IF (SRC1[31]) Load_32(mem) ELSE 0 DEST[63:32] := IF (SRC1[63]) Load_32(mem + 4) ELSE 0 DEST[95:64] := IF (SRC1[95]) Load_32(mem + 8) ELSE 0 DEST[127:96] := IF (SRC1[127]) Load_32(mem + 12) ELSE 0 DEST[159:128] := IF (SRC1[159]) Load_32(mem + 16) ELSE 0 DEST[191:160] := IF (SRC1[191]) Load_32(mem + 20) ELSE 0 DEST[223:192] := IF (SRC1[223]) Load_32(mem + 24) ELSE 0 DEST[255:224] := IF (SRC1[255]) Load_32(mem + 28) ELSE 0 VPMASKMOVD -128-bit load DEST[31:0] := IF (SRC1[31]) Load_32(mem) ELSE 0 DEST[63:32] := IF (SRC1[63]) Load_32(mem + 4) ELSE 0 DEST[95:64] := IF (SRC1[95]) Load_32(mem + 8) ELSE 0 DEST[127:97] := IF (SRC1[127]) Load_32(mem + 12) ELSE 0 DEST[MAXVL-1:128] := 0VPMASKMOVQ - 256-bit loadDEST[63:0] := IF (SRC1[63]) Load_64(mem) ELSE 0 DEST[127:64] := IF (SRC1[127]) Load_64(mem + 8) ELSE 0 DEST[195:128] := IF (SRC1[191]) Load_64(mem + 16) ELSE 0 DEST[255:196] := IF (SRC1[255]) Load_64(mem + 24) ELSE 0 VPMASKMOVQ - 128-bit load DEST[63:0] := IF (SRC1[63]) Load_64(mem) ELSE 0 DEST[127:64] := IF (SRC1[127]) Load_64(mem + 16) ELSE 0DEST[MAXVL-1:128] := 0VPMASKMOVD - 256-bit storeIF (SRC1[31]) DEST[31:0] := SRC2[31:0] IF (SRC1[63]) DEST[63:32] := SRC2[63:32] IF (SRC1[95]) DEST[95:64] := SRC2[95:64] IF (SRC1[127]) DEST[127:96] := SRC2[127:96] IF (SRC1[159]) DEST[159:128] :=SRC2[159:128] IF (SRC1[191]) DEST[191:160] := SRC2[191:160] VPMASKMOVD - 128-bit storeIF (SRC1[31]) DEST[31:0] := SRC2[31:0] IF (SRC1[63]) DEST[63:32] := SRC2[63:32] IF (SRC1[95]) DEST[95:64] := SRC2[95:64] IF (SRC1[127]) DEST[127:96] := SRC2[127:96] VPMASKMOVQ - 256-bit storeIF (SRC1[63]) DEST[63:0] := SRC2[63:0] IF (SRC1[127]) DEST[127:64] :=SRC2[127:64] IF (SRC1[191]) DEST[191:128] := SRC2[191:128] IF (SRC1[255]) DEST[255:192] := SRC2[255:192] VPMASKMOVQ - 128-bit storeIF (SRC1[63]) DEST[63:0] := SRC2[63:0] IF (SRC1[127]) DEST[127:64] :=SRC2[127:64] Intel C/C++ Compiler Intrinsic EquivalentVPMASKMOVD: __m256i _mm256_maskload_epi32(int const *a, __m256i mask)VPMASKMOVD: void    _mm256_maskstore_epi32(int *a, __m256i mask, __m256i b)VPMASKMOVQ: __m256i _mm256_maskload_epi64(__int64 const *a, __m256i mask);VPMASKMOVQ: void    _mm256_maskstore_epi64(__int64 *a, __m256i mask, __m256d b);VPMASKMOVD: __m128i _mm_maskload_epi32(int const *a, __m128i mask)VPMASKMOVD: void    _mm_maskstore_epi32(int *a, __m128i mask, __m128 b)VPMASKMOVQ: __m128i _mm_maskload_epi64(__int cont *a, __m128i mask);VPMASKMOVQ: void    _mm_maskstore_epi64(__int64 *a, __m128i mask, __m128i b);
```
