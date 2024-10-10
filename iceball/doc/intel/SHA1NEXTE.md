# SHA1NEXTE

Calculate SHA1 State Variable E After Four Rounds

InstructionMode Feature SupportFlagRMV/VSHACalculates SHA1 state variable E after four rounds of operation NP 0F 38 C8 /r from the current SHA1 state variable A in xmm1.
The calculated SHA1NEXTE xmm1, value of the SHA1 state variable E is added to the scheduled xmm2/m128dwords in xmm2/m128, and stored with some of the scheduled dwords in xmm1.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMModRM:reg (r, w)ModRM:r/m (r)N/AThe SHA1NEXTE calculates the SHA1 state variable E after four rounds of operation from the current SHA1 state variable A in the destination operand.
The calculated value of the SHA1 state variable E is added to the source operand, which contains the scheduled dwords.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
SHA1NEXTE TMP := (SRC1[127:96] ROL 30); DEST[127:96] := SRC2[127:96] + TMP; DEST[95:64] := SRC2[95:64]; DEST[63:32] := SRC2[63:32]; DEST[31:0] := SRC2[31:0]; Intel C/C++ Compiler Intrinsic EquivalentSHA1NEXTE __m128i _mm_sha1nexte_epu32(__m128i, __m128i);
```
