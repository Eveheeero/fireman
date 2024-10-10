# PCMPESTRM

Packed Compare Explicit Length Strings, Return Mask

The instruction compares data from two string fragments based on the encoded value in the imm8 contol byte (see Section 4.1, "Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM"), and gener-ates a mask stored to XMM0.Each string fragment is represented by two values.
The first value is an xmm (or possibly m128 for the second operand) which contains the data elements of the string (byte or word data).
The second value is stored in an input length register.
The input length register is EAX/RAX (for xmm1) or EDX/RDX (for xmm2/m128).
The length represents the number of bytes/words which are valid for the respective xmm/m128 data.
 The length of each input is interpreted as being the absolute-value of the value in the length register.
The absolute-value computation saturates to 16 (for bytes) and 8 (for words), based on the value of imm8[bit3] when the value in the length register is greater than 16 (8) or less than -16 (-8).The comparison and aggregation operations are performed according to the encoded value of imm8 bit fields (see Section 4.1).
As defined by imm8[6], IntRes2 is then either stored to the least significant bits of XMM0 (zero extended to 128 bits) or expanded into a byte/word-mask and then stored to XMM0.Note that the Arithmetic Flags are written in a non-standard manner in order to supply the most relevant informa-tion:CFlag - Reset if IntRes2 is equal to zero, set otherwiseZFlag - Set if absolute-value of EDX is < 16 (8), reset otherwiseSFlag - Set if absolute-value of EAX is < 16 (8), reset otherwiseOFlag -IntRes2[0]AFlag - ResetPFlag - ResetNote: In VEX.128 encoded versions, bits (MAXVL-1:128) ofEffective Operand SizeOperating mode/sizeOperand 1Operand 2Length 1Length 2Result16 bitxmmxmm/m128EAXEDXXMM032 bitxmmxmm/m128EAXEDXXMM064 bitxmmxmm/m128EAXEDXXMM064 bit + REX.Wxmmxmm/m128RAXRDXXMM0Intel C/C++ Compiler Intrinsic Equivalent For Returning Mask__m128i _mm_cmpestrm (__m128i a, int la, __m128i b, int lb, const int mode);Intel C/C++ Compiler Intrinsics For Reading EFlag Resultsint _mm_cmpestra (__m128i a, int la, __m128i b, int lb, const int mode);int _mm_cmpestrc (__m128i a, int la, __m128i b, int lb, const int mode);int _mm_cmpestro (__m128i a, int la, __m128i b, int lb, const int mode);int _mm_cmpestrs (__m128i a, int la, __m128i b, int lb, const int mode);int _mm_cmpestrz (__m128i a, int la, __m128i b, int lb, const int mode);

## Exceptions

- Other Exceptions
  > See Table2-21, "Type 4 Class Exception Conditions," additionally, this instruction does not cause #GP if the 
  > memory operand is not aligned to 16 Byte boundary, and:
  - #UD - If VEX.L = 1.
  > If VEX.vvvv 
- SIMD Floating-Point Exceptions
  > None.
