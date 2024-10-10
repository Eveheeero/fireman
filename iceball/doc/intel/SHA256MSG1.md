# SHA256MSG1

Perform an Intermediate Calculation for the Next Four SHA256 Message Dwords

The SHA256MSG1 instruction is one of two SHA256 message scheduling instructions.
The instruction performs an intermediate calculation for the next four SHA256 message dwords.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
SHA256MSG1 W4 := SRC2[31: 0] ; W3 := SRC1[127:96] ; W2 := SRC1[95:64] ; W1 := SRC1[63: 32] ; W0 := SRC1[31: 0] ; ( W4); DEST[127:96] := W3 + 0DEST[95:64] := W2 + ( W3); 0DEST[63:32] := W1 + ( W2); 0DEST[31:0] := W0 + ( W1); 0Intel C/C++ Compiler Intrinsic EquivalentSHA256MSG1 __m128i _mm_sha256msg1_epu32(__m128i, __m128i);
```
