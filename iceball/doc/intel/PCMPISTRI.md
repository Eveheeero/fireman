# PCMPISTRI

Packed Compare Implicit Length Strings, Return Index

The instruction compares data from two strings based on the encoded value in the imm8 control byte (see Section 4.1, "Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM"), and generates an index stored to ECX.Each string is represented by a single value.
The value is an xmm (or possibly m128 for the second operand) which contains the data elements of the string (byte or word data).
Each input byte/word is augmented with a valid/invalid tag.
A byte/word is considered valid only if it has a lower index than the least significant null byte/word.
(The least significant null byte/word is also considered invalid.) The comparison and aggregation operations are performed according to the encoded value of imm8 bit fields (see Section 4.1).
The index of the first (or last, according to imm8[6]) set bit of IntRes2 is returned in ECX.
If no bits are set in IntRes2, ECX is set to 16 (8).Note that the Arithmetic Flags are written in a non-standard manner in order to supply the most relevant informa-tion:CFlag - Reset if IntRes2 is equal to zero, set otherwiseZFlag - Set if any byte/word of xmm2/mem128 is null, reset otherwiseSFlag - Set if any byte/word of xmm1 is null, reset otherwiseOFlag -IntRes2[0]AFlag - ResetPFlag - ResetNote: In VEX.128 encoded version, VEX.vvvv is reserved and must be 1111b, VEX.L must be 0, otherwise the instruction will #UD.Effective Operand SizeOperating mode/sizeOperand 1Operand 2Result16 bitxmmxmm/m128ECX32 bitxmmxmm/m128ECX64 bitxmmxmm/m128ECXIntel C/C++ Compiler Intrinsic Intel C/C++ Compiler Intrinsics For Reading EFlag Resultsint _mm_cmpistra (__m128i a, __m128i b, const int mode);int _mm_cmpistrc (__m128i a, __m128i b, const int mode);int _mm_cmpistro (__m128i a, __m128i b, const int mode);int _mm_cmpistrs (__m128i a, __m128i b, const int mode);int _mm_cmpistrz (__m128i a, __m128i b, const int mode);

## Exceptions

- Other Exceptions
  > See Table2-21, "Type 4 Class Exception Conditions," additionally, this instruction does not cause #GP if the 
  > memory operand is not aligned to 16 Byte boundary, and:
  - #UD - If VEX.L = 1.
  > If VEX.vvvv 
- SIMD Floating-Point Exceptions
  > None.
