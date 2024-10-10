# MOVLPD

Move Low Packed Double Precision Floating-Point Value

InstructionMode Feature SupportFlag66 0F 12 /rAV/VSSE2Move double precision floating-point value from m64 to MOVLPD xmm1, m64low quadword of xmm1.VEX.128.66.0F.WIG 12 /rBV/VAVXMerge double precision floating-point value from m64 and VMOVLPD xmm2, xmm1, m64the high quadword of xmm1.EVEX.128.66.0F.W1 12 /rDV/VAVX512FMerge double precision floating-point value from m64 and VMOVLPD xmm2, xmm1, m64the high quadword of xmm1.66 0F 13/rCV/VSSE2Move double precision floating-point value from low MOVLPD m64, xmm1quadword of xmm1 to m64.VEX.128.66.0F.WIG 13/rCV/VAVXMove double precision floating-point value from low VMOVLPD m64, xmm1quadword of xmm1 to m64.EVEX.128.66.0F.W1 13/rEV/VAVX512FMove double precision floating-point value from low VMOVLPD m64, xmm1quadword of xmm1 to m64.Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:reg (r, w)ModRM:r/m (r)N/AN/ABN/AModRM:r/m (r)VEX.vvvv (r)ModRM:r/m (r)N/ACN/AModRM:r/m (w)ModRM:reg (r)N/AN/ADTuple1 ScalarModRM:reg (w)EVEX.vvvv (r)ModRM:r/m (r)N/AETuple1 ScalarModRM:r/m (w)ModRM:reg (r)N/AN/AThis instruction cannot be used for register to register or memory to memory moves.128-bit Legacy SSE load:Moves a double precision floating-point value from the source 64-bit memory operand and stores it in the low 64-bits of the destination XMM register.
The upper 64bits of the XMM register are preserved.
Bits (MAXVL-1:128) of the corresponding destination register are preserved.VEX.128 & EVEX encoded load:Loads a double precision floating-point value from the source 64-bit memory operand (third operand), merges it with the upper 64-bits of the first source XMM register (second operand), and stores it in the low 128-bits of the destination XMM register (first operand).
Bits (MAXVL-1:128) of the corresponding destination register are zeroed.128-bit store:Stores a double precision floating-point value from the low 64-bits of the XMM register source (second operand) to the 64-bit memory location (first operand).Note: VMOVLPD (store) (VEX.128.66.0F 13 /r) is legal and has the same behavior as the existing 66 0F 13 store.
For VMOVLPD (store) VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instruction will #UD.If VMOVLPD is encoded with VEX.L or EVEX.L'L= 1, an attempt to execute the instruction encoded with VEX.L or EVEX.L'L= 1 will cause an #UD exception.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-22, "T
  > ype 5 Class Exception Conditions," additionally:

## Operation

```C
MOVLPD (128-bit Legacy SSE Load)VMOVLPD (VEX.128 & EVEX Encoded Load)DEST[63:0] := SRC2[63:0]DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0VMOVLPD (Store)DEST[63:0] := SRC[63:0]Intel C/C++ Compiler Intrinsic EquivalentMOVLPD __m128d _mm_loadl_pd ( __m128d a, double *p)MOVLPD void _mm_storel_pd (double *p, __m128d a)
```
