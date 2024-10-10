# SHA1MSG2

Perform a Final Calculation for the Next Four SHA1 Message Dwords

InstructionMode Feature SupportFlagRMV/VSHAPerforms the final calculation for the next four SHA1 message NP 0F 38 CA /r dwords using intermediate results from xmm1 and the previous SHA1MSG2 xmm1, message dwords from xmm2/m128, storing the result in xmm1.xmm2/m128Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMModRM:reg (r, w)ModRM:r/m (r)N/AThe SHA1MSG2 instruction is one of two SHA1 message scheduling instructions.
The instruction performs the final calculation to derive the next four SHA1 message dwords.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
SHA1MSG2 W13 := SRC2[95:64] ; W14 := SRC2[63: 32] ; W15 := SRC2[31: 0] ; W16 := (SRC1[127:96] XOR W13 ) ROL 1; W17 := (SRC1[95:64] XOR W14) ROL 1; W18 := (SRC1[63: 32] XOR W15) ROL 1; W19 := (SRC1[31: 0] XOR W16) ROL 1; DEST[127:96] := W16; DEST[95:64] := W17; DEST[63:32] := W18; DEST[31:0] := W19; Intel C/C++ Compiler Intrinsic EquivalentSHA1MSG2 __m128i _mm_sha1msg2_epu32(__m128i, __m128i);
```
