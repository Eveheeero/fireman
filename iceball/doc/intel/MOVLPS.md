# MOVLPS

Move Low Packed Single Precision Floating-Point Values

InstructionMode Feature SupportFlagNP 0F 12 /rAV/VSSEMove two packed single precision floating-point values from MOVLPS xmm1, m64m64 to low quadword of xmm1.VEX.128.0F.WIG 12 /rBV/VAVXMerge two packed single precision floating-point values VMOVLPS xmm2, xmm1, m64from m64 and the high quadword of xmm1.EVEX.128.0F.W0 12 /rDV/VAVX512FMerge two packed single precision floating-point values VMOVLPS xmm2, xmm1, m64from m64 and the high quadword of xmm1.0F 13/rCV/VSSEMove two packed single precision floating-point values from MOVLPS m64, xmm1low quadword of xmm1 to m64.VEX.128.0F.WIG 13/rCV/VAVXMove two packed single precision floating-point values from VMOVLPS m64, xmm1low quadword of xmm1 to m64.EVEX.128.0F.W0 13/rEV/VAVX512FMove two packed single precision floating-point values from VMOVLPS m64, xmm1low quadword of xmm1 to m64.Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:reg (r, w)ModRM:r/m (r)N/AN/ABN/AModRM:reg (w)VEX.vvvv (r)ModRM:r/m (r)N/ACN/AModRM:r/m (w)ModRM:reg (r)N/AN/ADTuple2ModRM:reg (w)EVEX.vvvv (r)ModRM:r/m (r)N/AETuple2ModRM:r/m (w)ModRM:reg (r)N/AN/AThis instruction cannot be used for register to register or memory to memory moves.128-bit Legacy SSE load:Moves two packed single precision floating-point values from the source 64-bit memory operand and stores them in the low 64-bits of the destination XMM register.
The upper 64bits of the XMM register are preserved.
Bits (MAXVL-1:128) of the corresponding destination register are preserved.VEX.128 & EVEX encoded load:Loads two packed single precision floating-point values from the source 64-bit memory operand (the third operand), merges them with the upper 64-bits of the first source operand (the second operand), and stores them in the low 128-bits of the destination register (the first operand).
Bits (MAXVL-1:128) of the corresponding desti-nation register are zeroed.128-bit store:Loads two packed single precision floating-point values from the low 64-bits of the XMM register source (second operand) to the 64-bit memory location (first operand).Note: VMOVLPS (store) (VEX.128.0F 13 /r) is legal and has the same behavior as the existing 0F 13 store.
For VMOVLPS (store) VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instruction will #UD.If VMOVLPS is encoded with VEX.L or EVEX.L'L= 1, an a

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-22, "T
  > ype 5 Class Exception Conditions," additionally:

## Operation

```C
MOVLPS (128-bit Legacy SSE Load)DEST[63:0] := SRC[63:0]DEST[MAXVL-1:64] (Unmodified)VMOVLPS (VEX.128 & EVEX Encoded Load)DEST[63:0] := SRC2[63:0]DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0VMOVLPS (Store)DEST[63:0] := SRC[63:0]Intel C/C++ Compiler Intrinsic EquivalentMOVLPS __m128 _mm_loadl_pi ( __m128 a, __m64 *p)MOVLPS void _mm_storel_pi (__m64 *p, __m128 a)
```
