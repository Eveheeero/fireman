# MOVHPD

Move High Packed Double Precision Floating-Point Value

InstructionMode Feature SupportFlag66 0F 16 /rAV/VSSE2Move double precision floating-point value from m64 to MOVHPD xmm1, m64high quadword of xmm1.VEX.128.66.0F.WIG 16 /rBV/VAVXMerge double precision floating-point value from m64 VMOVHPD xmm2, xmm1, m64and the low quadword of xmm1.EVEX.128.66.0F.W1 16 /rDV/VAVX512FMerge double precision floating-point value from m64 VMOVHPD xmm2, xmm1, m64and the low quadword of xmm1.66 0F 17 /rCV/VSSE2Move double precision floating-point value from high MOVHPD m64, xmm1quadword of xmm1 to m64.VEX.128.66.0F.WIG 17 /rCV/VAVXMove double precision floating-point value from high VMOVHPD m64, xmm1quadword of xmm1 to m64.EVEX.128.66.0F.W1 17 /rEV/VAVX512FMove double precision floating-point value from high VMOVHPD m64, xmm1quadword of xmm1 to m64.Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:reg (r, w)ModRM:r/m (r)N/AN/ABN/AModRM:reg (w)VEX.vvvv (r)ModRM:r/m (r)N/ACN/AModRM:r/m (w)ModRM:reg (r)N/AN/ADTuple1 ScalarModRM:reg (w)EVEX.vvvv (r)ModRM:r/m (r)N/AETuple1 ScalarModRM:r/m (w)ModRM:reg (r)N/AN/AThis instruction cannot be used for register to register or memory to memory moves.128-bit Legacy SSE load:Moves a double precision floating-point value from the source 64-bit memory operand and stores it in the high 64-bits of the destination XMM register.
The lower 64bits of the XMM register are preserved.
Bits (MAXVL-1:128) of the corresponding destination register are preserved.VEX.128 & EVEX encoded load:Loads a double precision floating-point value from the source 64-bit memory operand (the third operand) and stores it in the upper 64-bits of the destination XMM register (first operand).
The low 64-bits from the first source operand (second operand) are copied to the low 64-bits of the destination.
Bits (MAXVL-1:128) of the corre-sponding destination register are zeroed.128-bit store:Stores a double precision floating-point value from the high 64-bits of the XMM register source (second operand) to the 64-bit memory location (first operand).Note: VMOVHPD (store) (VEX.128.66.0F 17 /r) is legal and has the same behavior as the existing 66 0F 17 store.
For VMOVHPD (store) VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instruction will #UD.If VMOVHPD is encoded with VEX.L or EVEX.L'L= 1, an a

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-22, "T
  > ype 5 Class Exception Conditions," additionally:
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
MOVHPD (128-bit Legacy SSE Load)DEST[63:0] (Unmodified)DEST[127:64] := SRC[63:0]DEST[MAXVL-1:128] (Unmodified)VMOVHPD (VEX.128 & EVEX Encoded Load)DEST[63:0] := SRC1[63:0]DEST[127:64] := SRC2[63:0]DEST[MAXVL-1:128] := 0VMOVHPD (Store)DEST[63:0] := SRC[127:64]Intel C/C++ Compiler Intrinsic EquivalentMOVHPD __m128d _mm_loadh_pd ( __m128d a, double *p)MOVHPD void _mm_storeh_pd (double *p, __m128d a)
```
