# VPSHUFBITQMB

Shuffle Bits From Quadword Elements Using Byte Indexes Into Mask

The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data.
Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand).
A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).Control data for each output bit is stored in 8 bit elements of SRC2, but only the 6 least significant bits of each element are used.This instruction uses write masking (zeroing only).
This instruction supports memory fault suppression.The first source operand is a ZMM register.
The second source operand is a ZMM register or a memory location.
The destination operand is a mask register.

## Operation

```C
VPSHUFBITQMB DEST, SRC1, SRC2(KL, VL) = (16,128), (32,256), (64, 512)FOR i := 0 TO KL/8-1: //QwordFOR j := 0 to 7: // ByteIF k2[i*8+j] or *no writemask*:m := SRC2.qword[i].byte[j] & 0x3Fk1[i*8+j] := SRC1.qword[i].bit[m]ELSE:k1[i*8+j] := 0k1[MAX_KL-1:KL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPSHUFBITQMB __mmask16 _mm_bitshuffle_epi64_mask(__m128i, __m128i);VPSHUFBITQMB __mmask16 _mm_mask_bitshuffle_epi64_mask(__mmask16, __m128i, __m128i);VPSHUFBITQMB __mmask32 _mm256_bitshuffle_epi64_mask(__m256i, __m256i);VPSHUFBITQMB __mmask32 _mm256_mask_bitshuffle_epi64_mask(__mmask32, __m256i, __m256i);VPSHUFBITQMB __mmask64 _mm512_bitshuffle_epi64_mask(__m512i, __m512i);
```
