# PSIGNB/PSIGNW/PSIGND

Packed SIGN

(V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero.
If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged.
If a data element in the source operand is zero, the corre-sponding data element in the destination operand is set to zero.(V)PSIGNB operates on signed bytes.
(V)PSIGNW operates on 16-bit signed words.
(V)PSIGND operates on signed 32-bit integers.Legacy SSE instructions: Both operands can be MMX registers.
In 64-bit mode, use the REX prefix to access addi-tional registers.
128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding YMM destina-tion register remain unchanged.VEX.128 encoded version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.L must be 0, otherwise instructions will #UD.VEX.256 encoded version: The first source and destination operands are YMM registers.
The second source operand is an YMM register or a 256-bit memory location.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
def byte_sign(control, input_val):   if control<0:      return negate(input_val)   elif control==0:      return 0   return input_val   def word_sign(control, input_val):   if control<0:      return negate(input_val)   elif control==0:      return 0   return input_val   def dword_sign(control, input_val):   if control<0:      return negate(input_val)   elif control==0:      return 0   return input_valPSIGNB srcdest, src// MMX 64-bit OperandsVL=64KL := VL/8for i in 0...KL-1:   srcdest.byte[i] := byte_sign(src.byte[i], srcdest.byte[i])PSIGNW srcdest, src   // MMX 64-bit OperandsVL=64KL := VL/16FOR i in 0...KL-1:   srcdest.word[i] PSIGND srcdest, src   // MMX 64-bit OperandsVL=64KL := VL/32FOR i in 0...KL-1:   srcdest.dword[i] := dword_sign(src.dword[i], srcdest.dword[i])PSIGNB srcdest, src   // SSE 128-bit OperandsVL=128KL := VL/8FOR i in 0...KL-1:   srcdest.byte[i] := byte_sign(src.byte[i], srcdest.byte[i])PSIGNW srcdest, src   // SSE 128-bit OperandsVL=128KL := VL/16FOR i in 0...KL-1:   srcdest.word[i] := word_sign(src.word[i], srcdest.word[i])PSIGND srcdest, src   // SSE 128-bit OperandsVL=128KL := VL/32FOR i in 0...KL-1:   srcdest.dword[i] := dword_sign(src.dword[i], srcdest.dword[i])VPSIGNB dest, src1, src2   // AVX 128-bit or 256-bit OperandsVL=(128,256)KL := VL/8FOR i in 0...KL-1:   dest.byte[i] := byte_sign(src2.byte[i], src1.byte[i])DEST[MAXVL-1:VL] := 0VPSIGNW dest, src1, src2   // AVX 128-bit or 256-bit OperandsVL=(128,256)KL := VL/16FOR i in 0...KL-1:   dest.word[i] := word_sign(src2.word[i], src1.word[i])DEST[MAXVL-1:VL] := 0VPSIGND dest, src1, src2    // AVX 128-bit or 256-bit OperandsVL=(128,256)KL := VL/32FOR i in 0...KL-1:Intel C/C++ Compiler Intrinsic EquivalentPSIGNB __m64 _mm_sign_pi8 (__m64 a, __m64 b)(V)PSIGNB __m128i _mm_sign_epi8 (__m128i a, __m128i b)VPSIGNB __m256i _mm256_sign_epi8 (__m256i a, __m256i b)PSIGNW __m64 _mm_sign_pi16 (__m64 a, __m64 b)(V)PSIGNW __m128i _mm_sign_epi16 (__m128i a, __m128i b)VPSIGNW __m256i _mm256_sign_epi16 (__m256i a, __m256i b)PSIGND __m64 _mm_sign_pi32 (__m64 a, __m64 b)(V)PSIGND __m128i _mm_sign_epi32 (__m128i a, __m128i b)VPSIGND __m256i _mm256_sign_epi32 (__m256i a, __m256i b)
```
