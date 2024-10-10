# MOVHPS

Move High Packed Single Precision Floating-Point Values

InstructionMode Feature SupportFlagNP 0F 16 /rAV/VSSEMove two packed single precision floating-point values MOVHPS xmm1, m64from m64 to high quadword of xmm1.VEX.128.0F.WIG 16 /rBV/VAVXMerge two packed single precision floating-point values VMOVHPS xmm2, xmm1, m64from m64 and the low quadword of xmm1.EVEX.128.0F.W0 16 /rDV/VAVX512FMerge two packed single precision floating-point values VMOVHPS xmm2, xmm1, m64from m64 and the low quadword of xmm1.NP 0F 17 /rCV/VSSEMove two packed single precision floating-point values MOVHPS m64, xmm1from high quadword of xmm1 to m64.VEX.128.0F.WIG 17 /rCV/VAVXMove two packed single precision floating-point values VMOVHPS m64, xmm1from high quadword of xmm1 to m64.EVEX.128.0F.W0 17 /rEV/VAVX512FMove two packed single precision floating-point values VMOVHPS m64, xmm1from high quadword of xmm1 to m64.Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:reg (r, w)ModRM:r/m (r)N/AN/ABN/AModRM:reg (w)VEX.vvvv (r)ModRM:r/m (r)N/ACN/AModRM:r/m (w)ModRM:reg (r)N/AN/ADTuple2ModRM:reg (w)EVEX.vvvv (r)ModRM:r/m (r)N/AETuple2ModRM:r/m (w)ModRM:reg (r)N/AN/AThis instruction cannot be used for register to register or memory to memory moves.128-bit Legacy SSE load:Moves two packed single precision floating-point values from the source 64-bit memory operand and stores them in the high 64-bits of the destination XMM register.
The lower 64bits of the XMM register are preserved.
Bits (MAXVL-1:128) of the corresponding destination register are preserved.VEX.128 & EVEX encoded load:Loads two single precision floating-point values from the source 64-bit memory operand (the third operand) and stores it in the upper 64-bits of the destination XMM register (first operand).
The low 64-bits from the first source operand (the second operand) are copied to the lower 64-bits of the destination.
Bits (MAXVL-1:128) of the corre-sponding destination register are zeroed.128-bit store:Stores two packed single precision floating-point values from the high 64-bits of the XMM register source (second operand) to the 64-bit memory location (first operand).Note: VMOVHPS (store) (VEX.128.0F 17 /r) is legal and has the same behavior as the existing 0F 17 store.
For VMOVHPS (store) VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instruction will #UD.If VMOVHPS is encoded with VEX.L or EVEX.L'L= 1, an a

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-22, "T
  > ype 5 Class Exception Conditions," additionally:

## Operation

```C
MOVHPS (128-bit Legacy SSE Load)DEST[63:0] (Unmodified)DEST[127:64] := SRC[63:0]DEST[MAXVL-1:128] (Unmodified)VMOVHPS (VEX.128 and EVEX Encoded Load)DEST[63:0] := SRC1[63:0]DEST[127:64] := SRC2[63:0]DEST[MAXVL-1:128] := 0VMOVHPS (Store)DEST[63:0] := SRC[127:64]Intel C/C++ Compiler Intrinsic EquivalentMOVHPS __m128 _mm_loadh_pi ( __m128 a, __m64 *p)MOVHPS void _mm_storeh_pi (__m64 *p, __m128 a)
```
