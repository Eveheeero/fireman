# SHA1MSG1

Perform an Intermediate Calculation for the Next Four SHA1 Message Dwords

InstructionMode Feature SupportFlagRMV/VSHAPerforms an intermediate calculation for the next four SHA1 mes-NP 0F 38 C9 /r sage dwords using previous message dwords from xmm1 and SHA1MSG1 xmm1, xmm2/m128, storing the result in xmm1.xmm2/m128Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMModRM:reg (r, w)ModRM:r/m (r)N/AThe SHA1MSG1 instruction is one of two SHA1 message scheduling instructions.
The instruction performs an inter-mediate calculation for the next four SHA1 message dwords.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
SHA1MSG1 W0 := SRC1[127:96] ; W1 := SRC1[95:64] ; W2 := SRC1[63: 32] ; W3 := SRC1[31: 0] ; W4 := SRC2[127:96] ; W5 := SRC2[95:64] ; DEST[127:96] := W2 XOR W0; DEST[95:64] := W3 XOR W1; DEST[63:32] := W4 XOR W2; DEST[31:0] := W5 XOR W3; Intel C/C++ Compiler Intrinsic EquivalentSHA1MSG1 __m128i _mm_sha1msg1_epu32(__m128i, __m128i);
```
