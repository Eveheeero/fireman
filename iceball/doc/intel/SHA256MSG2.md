# SHA256MSG2

Perform a Final Calculation for the Next Four SHA256 Message Dwords

InstructionMode Feature SupportFlagRMV/VSHAPerforms the final calculation for the next four SHA256 message NP 0F 38 CD /r dwords using previous message dwords from xmm1 and SHA256MSG2 xmm1, xmm2/m128, storing the result in xmm1.xmm2/m128Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMModRM:reg (r, w)ModRM:r/m (r)N/AThe SHA256MSG2 instruction is one of two SHA2 message scheduling instructions.
The instruction performs the final calculation for the next four SHA256 message dwords.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
SHA256MSG2 W14 := SRC2[95:64] ; W15 := SRC2[127:96] ; ( W14) ; W16 := SRC1[31: 0] + 1W17 := SRC1[63: 32] + ( W15) ; 1W18 := SRC1[95: 64] + ( W16) ; 1W19 := SRC1[127: 96] + ( W17) ; 1DEST[127:96] := W19 ; DEST[95:64] := W18 ; DEST[63:32] := W17 ; DEST[31:0] := W16; Intel C/C++ Compiler Intrinsic EquivalentSHA256MSG2 __m128i _mm_sha256msg2_epu32(__m128i, __m128i);
```
