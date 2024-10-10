# VPBROADCASTM

Broadcast Mask to Vector Register

Broadcasts the zero-extended 64/32 bit value of the low byte/word of the source operand (the second operand) to each 64/32 bit element of the destination operand (the first operand).
The source operand is an opmask register.
The destination operand is a ZMM register (EVEX.512), YMM register (EVEX.256), or XMM register (EVEX.128).EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None

## Operation

```C
VPBROADCASTMB2Q(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j*64DEST[i+63:i] := ZeroExtend(SRC[7:0])ENDFORDEST[MAXVL-1:VL] := 0VPBROADCASTMW2D(KL, VL) = (4, 128), (8, 256),(16, 512)FOR j := 0 TO KL-1i := j*32DEST[i+31:i] := ZeroExtend(SRC[15:0])Intel C/C++ Compiler Intrinsic EquivalentVPBROADCASTMB2Q __m512i _mm512_broadcastmb_epi64( __mmask8);VPBROADCASTMW2D __m512i _mm512_broadcastmw_epi32( __mmask16);VPBROADCASTMB2Q __m256i _mm256_broadcastmb_epi64( __mmask8);VPBROADCASTMW2D __m256i _mm256_broadcastmw_epi32( __mmask8);VPBROADCASTMB2Q __m128i _mm_broadcastmb_epi64( __mmask8);VPBROADCASTMW2D __m128i _mm_broadcastmw_epi32( __mmask8);
```
