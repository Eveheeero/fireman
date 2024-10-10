# VDBPSADBW

Double Block Packed Sum-Absolute-Differences (SAD) on Unsigned Bytes

Compute packed SAD (sum of absolute differences) word results of unsigned bytes from two 32-bit dword elements.
Packed SAD word results are calculated in multiples of qword superblocks, producing 4 SAD word results in each 64-bit superblock of the destination register.
Within each super block of packed word results, the SAD results from two 32-bit dword elements are calculated as follows: - The lower two word results are calculated each from the SAD operation between a sliding dword element within a qword superblock from an intermediate vector with a stationary dword element in the corresponding qword superblock of the first source operand.
The intermediate vector, see "Tmp1" in Figure5-8, is constructed from the second source operand the imm8 byte as shuffle control to select dword elements within a 128-bit lane of the second source operand.
The two sliding dword elements in a qword superblock of Tmp1 are located at byte offset 0 and 1 within the superblock, respectively.
The stationary dword element in the qword superblock from the first source operand is located at byte offset 0.
- The next two word results are calculated each from the SAD operation between a sliding dword element within a qword superblock from the intermediate vector Tmp1 with a second stationary dword element in the corre-sponding qword superblock of the first source operand.
The two sliding dword elements in a qword superblock of Tmp1 are located at byte offset 2and 3 within the superblock, respectively.
The stationary dword element in the qword superblock from the first source operand is located at byte offset 4.
- The intermediate vector is constructed in 128-bits lanes.
Within each 128-bit lane, each dword element of the intermediate vector is selected by a two-bit field within the imm8 byte on the corresponding 128-bits of the second source operand.
The imm8 byte serves as dword shuffle control within each 128-bit lanes of the inter-mediate vector and the second source operand, similarly to PSHUFD.The first source operand is a ZMM/YMM/XMM register.
The second source operand is a ZMM/YMM/XMM register, or a 512/256/128-bit memory location.
The destination operan127+128*n95+128*n63+128*n31+128*n128*n128-bit Lane of Src2DW0DW3DW1DW200B: DW001B: DW110B: DW2imm8 shuffle control11B: DW371530127+128*n95+128*n63+128*n31+128*n128*n128-bit Lane of Tmp1Tmp1 qword superblock3915312385531473924Tmp1 sliding dword Tmp1 sliding dword 317231506339554732Src1 stationary dword 0Src1 stationary dword 1________absabsabsabsabsabsabsabs++472339311631231570Tmp1 sliding dwordTmp1 sliding dword 633955473231723150Src1 stationary dword 1Src1 stationary dword 0____absabsabsabs____absabsabsabs++634731150Destination qword superblockFigure 5-8.
 64-bit Super Block of SAD Operation in VDBPSADBW 

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
VDBPSADBW (EVEX Encoded Versions)(KL, VL) = (8, 128), (16, 256), (32, 512)Selection of quadruplets:FOR I = 0 to VL step 128TMP1[I+31:I] := select (SRC2[I+127: I], imm8[1:0])TMP1[I+63: I+32] := select (SRC2[I+127: I], imm8[3:2])TMP1[I+95: I+64] := select (SRC2[I+127: I], imm8[5:4])TMP1[I+127: I+96]  := select (SRC2[I+127: I], imm8[7:6])END FORSAD of quadruplets:FOR I =0 to VL step 64ABS(SRC1[I+23: I+16]- TMP1[I+23: I+16]) +ABS(SRC1[I+31: I+24]- TMP1[I+31: I+24]) TMP_DEST[I+31: I+16] := ABS(SRC1[I+7: I] - TMP1[I+15: I+8]) +ABS(SRC1[I+15: I+8]- TMP1[I+23: I+16]) +ABS(SRC1[I+23: I+16]- TMP1[I+31: I+24]) +ABS(SRC1[I+31: I+24]- TMP1[I+39: I+32])TMP_DEST[I+47: I+32] := ABS(SRC1[I+39: I+32] - TMP1[I+23: I+16]) +ABS(SRC1[I+47: I+40]- TMP1[I+31: I+24]) +ABS(SRC1[I+55: I+48]- TMP1[I+39: I+32]) +ABS(SRC1[I+63: I+56]- TMP1[I+47: I+40]) TMP_DEST[I+63: I+48] := ABS(SRC1[I+39: I+32] - TMP1[I+31: I+24]) +ABS(SRC1[I+47: I+40] - TMP1[I+39: I+32]) +ABS(SRC1[I+55: I+48] - TMP1[I+47: I+40]) +ABS(SRC1[I+63: I+56] - TMP1[I+55: I+48])ENDFORFOR j :=  0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN DEST[i+15:i] :=  TMP_DEST[i+15:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+15:i] :=  0FIFI;ENDFORDEST[MAXVL-1:VL] :=  0Intel C/C++ Compiler Intrinsic EquivalentVDBPSADBW __m512i _mm512_dbsad_epu8(__m512i a, __m512i b int imm8);VDBPSADBW __m512i _mm512_mask_dbsad_epu8(__m512i s, __mmask32 m, __m512i a, __m512i b int imm8);VDBPSADBW __m512i _mm512_maskz_dbsad_epu8(__mmask32 m, __m512i a, __m512i b int imm8);VDBPSADBW __m256i _mm256_dbsad_epu8(__m256i a, __m256i b int imm8);VDBPSADBW __m256i _mm256_mask_dbsad_epu8(__m256i s, __mmask16 m, __m256i a, __m256i b int imm8);VDBPSADBW __m256i _mm256_maskz_dbsad_epu8(__mmask16 m, __m256i a, __m256i b int imm8);VDBPSADBW __m128i _mm_dbsad_epu8(__m128i a, __m128i b int imm8);VDBPSADBW __m128i _mm_mask_dbsad_epu8(__m128i s, __mmask8 m, __m128i a, __m128i b int imm8);VDBPSADBW __m128i _mm_maskz_dbsad_epu8(__mmask8 m, __m128i a, __m128i b int imm8);
```
