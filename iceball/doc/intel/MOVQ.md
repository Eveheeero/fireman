# MOVQ

Move Quadword

InstructionModeFeature FlagNP 0F 6F /rAV/VMMXMove quadword from mm/m64 to mm.MOVQ mm, mm/m64NP 0F 7F /rBV/VMMXMove quadword from mm to mm/m64.MOVQ mm/m64, mmF3 0F 7E /rAV/VSSE2Move quadword from xmm2/mem64 to xmm1.MOVQ xmm1, xmm2/m64VEX.128.F3.0F.WIG 7E /rAV/VAVXMove quadword from xmm2 to xmm1.VMOVQ xmm1, xmm2/m64EVEX.128.F3.0F.W1 7E /rCV/VAVX512FMove quadword from xmm2/m64 to xmm1.VMOVQ xmm1, xmm2/m6466 0F D6 /rBV/VSSE2Move quadword from xmm1 to xmm2/mem64.MOVQ xmm2/m64, xmm1VEX.128.66.0F.WIG D6 /rBV/VAVXMove quadword from xmm2 register to xmm1/m64.VMOVQ xmm1/m64, xmm2EVEX.128.66.0F.W1 D6 /rDV/VAVX512FMove quadword from xmm2 register to xmm1/m64.VMOVQ xmm1/m64, xmm2Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:reg (w)ModRM:r/m (r)N/AN/ABN/AModRM:r/m (w)ModRM:reg (r)N/AN/ACTuple1 ScalarModRM:reg (w)ModRM:r/m (r)N/AN/ADTuple1 ScalarModRM:r/m (w)ModRM:reg (r)N/AN/ACopies a quadword from the source operand (second operand) to the destination operand (first operand).
The source and destination operands can be MMX technology registers, XMM registers, or 64-bit memory locations.
This instruction can be used to move a quadword between two MMX technology registers or between an MMX tech-nology register and a 64-bit memory location, or to move data between two XMM registers or between an XMM register and a 64-bit memory location.
The instruction cannot be used to transfer data between memory locations.
When the source operand is an XMM register, the low quadword is moved; when the destination operand is an XMM register, the quadword is stored to the low quadword of the register, and the high quadword is cleared to all 0s.In 64-bit mode and if not encoded using VEX/EVEX, use of the REX prefix in the form of REX.R permits this instruc-tion to access additional registers (XMM8-XMM15).Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b, otherwise instructions will #UD.If VMOVQ is encoded with VEX.L= 1, an attempt to execute the instruction encoded with VEX.L= 1 will cause an #UD exception.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentVMOVQ __m128i _mm_loadu_si64( void * s);VMOVQ void _mm_storeu_si64( void * d, __m128i s);MOVQ m128i _mm_move_epi64(__m128i a)

## Exceptions

- Other Exceptions
  > ®
  > See Table23-8, "Exception Conditions for Legacy SIMD/M
  > MX Instructions without FP Exception," in the Intel

## Operation

```C
MOVQ Instruction When Operating on MMX TeMOVQ Instruction When Source and Destination Operands are XMM RegistersDEST[63:0] := SRC[63:0];DEST[127:64] := 0000000000000000H;MOVQ Instruction When Source Operand is XMM Register and Destinationoperand is memory location:DEST := SRC[63:0];MOVQ Instruction When Source Operand is Memory Location and Destinationoperand is XMM register:DEST[63:0] := SRC;DEST[127:64] := 0000000000000000H;VMOVQ (VEX.128.F3.0F 7E) With XMM Register Source and DestinationDEST[63:0] := SRC[63:0]DEST[MAXVL-1:64] := 0VMOVQ (VEX.128.66.0F D6) With XMM Register Source and DestinationDEST[63:0] := SRC[63:0]DEST[MAXVL-1:64] := 0VMOVQ (7E - EVEX Encoded Version) With XMM Register Source and DestinationDEST[63:0] := SRC[63:0]DEST[MAXVL-1:64] := 0VMOVQ (D6 - EVEX Encoded Version) With XMM Register Source and DestinationDEST[63:0] := SRC[63:0]DEST[MAXVL-1:64] := 0VMOVQ (7E) With Memory SourceDEST[63:0] := SRC[63:0]DEST[MAXVL-1:64] := 0VMOVQ (7E - EVEX Encoded Version) With Memory SourceDEST[63:0] := SRC[63:0]DEST[:MAXVL-1:64] := 0VMOVQ (D6) With Memory DESTDEST[63:0] := SRC2[63:0]
```
