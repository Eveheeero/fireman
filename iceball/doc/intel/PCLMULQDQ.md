# PCLMULQDQ

Carry-Less Multiplication Quadword

Performs a carry-less multiplication of two quadwords, selected from the first source and second source operand according to the value of the immediate byte.
Bits 4 and 0 are used to select which 64-bit half of each operand to use according to Table4-13, other bits of the immediate byte are ignored.
Table 4-13.
 PCLMULQDQ Quadword Selection of Immediate ByteImm[4]Imm[0]PCLMULQDQ Operation100CL_MUL( SRC2[63:0], SRC1[63:0] )01CL_MUL( SRC2[63:0], SRC1[127:64] )10CL_MUL( SRC2[127:64], SRC1[63:0] )11CL_MUL( SRC2[127:64], SRC1[127:64] )NOTES:1.
SRC2 denotes the second source operand, which can be a register or memory; SRC1 denotes the first source and destination oper-and.The first source operand and the destination operand are the same and must be a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register or a 512/256/128-bit memory location.
Bits (VL_MAX-1:128) of the corresponding YMM destination register remain unchanged.Compilers and assemblers may implement the following pseudo-op syntax to simplify programming and emit the required encoding for imm8.Table 4-14.
 Pseudo-Op and PCLMULQDQ ImplementationPseudo-OpImm8 EncodingPCLMULLQLQDQ xmm1, xmm20000_0000BPCLMULHQLQDQ xmm1, xmm20000_0001BPCLMULLQHQDQ xmm1, xmm20001_0000BPCLMULHQHQDQ xmm1, xmm20001_0001B

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > See Table2-21, "Type 4 Class Exception Conditions," additionally:
  - #UD - If VEX.L = 1.

## Operation

```C
define PCLMUL128(X,Y): // helper functionFOR i := 0 to 63:TMP [ i ] := X[ 0 ] and Y[ i ]FOR j := 1 to i:TMP [ i ] := TMP [ i ] xor (X[ j ] and Y[ i - j ])DEST[ i ] := TMP[ i ]FOR i := 64 to 126:TMP [ i ] := 0FOR j := i - 63 to 63:TMP [ i ] := TMP [ i ] xor (X[ j ] and Y[ i - j ])DEST[ i ] := TMP[ i ]PCLMULQDQ (SSE Version)IF imm8[0] = 0:TEMP1 := SRC1.qword[0]ELSE:TEMP1 := SRC1.qword[1]IF imm8[4] = 0:TEMP2 := SRC2.qword[0]ELSE:TEMP2 := SRC2.qword[1]DEST[127:0] := PCLMUL128(TEMP1, TEMP2)DEST[MAXVL-1:128] (Unmodified)VPCLMULQDQ (128b and 256b VEX Encoded Versions)(KL,VL) = (1,128), (2,256)FOR i= 0 to KL-1:IF imm8[0] = 0:TEMP1 := SRC1.xmm[i].qword[0]ELSE:TEMP1 := SRC1.xmm[i].qword[1]IF imm8[4] = 0:TEMP2 := SRC2.xmm[i].qword[0]ELSE:TEMP2 := SRC2.xmm[i].qword[1]DEST.xmm[i] := PCLMUL128(TEMP1, TEMP2)DEST[MAXVL-1:VL] := 0VPCLMULQDQ (EVEX Encoded Version)(KL,VL) = (1,128), (2,256), (4,512)FOR i = 0 to KL-1:IF imm8[0] = 0:TEMP1 := SRC1.xmm[i].qword[0]ELSE:TEMP1 := SRC1.xmm[i].qword[1]IF imm8[4] = 0:TEMP2 := SRC2.xmm[i].qword[0]ELSE:TEMP2 := SRC2.xmm[i].qword[1]DEST.xmm[i] := PCLMUL128(TEMP1, TEMP2)DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic Equivalent(V)PCLMULQDQ __m128i  _mm_clmulepi64_si128 (__m128i, __m128i, const int)VPCLMULQDQ __m256i _mm256_clmulepi64_epi128(__m256i, __m256i, const int);VPCLMULQDQ __m512i _mm512_clmulepi64_epi128(__m512i, __m512i, const int);
```
