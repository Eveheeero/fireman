# MOVD/MOVQ

Move Doubleword/Move Quadword

InstructionModeFeature FlagNP 0F 6E /rAV/VMMXMove doubleword from r/m32 to mm.MOVD mm, r/m32NP REX.W + 0F 6E /rAV/N.E.MMXMove quadword from r/m64 to mm.MOVQ mm, r/m64NP 0F 7E /rBV/VMMXMove doubleword from mm to r/m32.MOVD r/m32, mmNP REX.W + 0F 7E /rBV/N.E.MMXMove quadword from mm to r/m64.MOVQ r/m64, mm66 0F 6E /rAV/VSSE2Move doubleword from r/m32 to xmm.MOVD xmm, r/m3266 REX.W 0F 6E /rAV/N.E.SSE2Move quadword from r/m64 to xmm.MOVQ xmm, r/m6466 0F 7E /rBV/VSSE2Move doubleword from xmm register to r/m32.MOVD r/m32, xmm 66 REX.W 0F 7E /rBV/N.E.SSE2Move quadword from xmm register to r/m64.MOVQ r/m64, xmmVEX.128.66.0F.W0 6E /AV/VAVXMove doubleword from r/m32 to xmm1.VMOVD xmm1, r32/m321.AVXMove quadword from r/m64 to xmm1.VEX.128.66.0F.W1 6E /rAV/N.EVMOVQ xmm1, r64/m64VEX.128.66.0F.W0 7E /rBV/VAVXMove doubleword from xmm1 register to r/m32.VMOVD r32/m32, xmm11.AVXMove quadword from xmm1 register to r/m64.VEX.128.66.0F.W1 7E /rBV/N.EVMOVQ r64/m64, xmm1EVEX.128.66.0F.W0 6E /rCV/VAVX512FMove doubleword from r/m32 to xmm1.VMOVD xmm1, r32/m321EVEX.128.66.0F.W1 6E /rCV/N.E.AVX512FMove quadword from r/m64 to xmm1.VMOVQ xmm1, r64/m64EVEX.128.66.0F.W0 7E /rDV/VAVX512FMove doubleword from xmm1 register to r/m32.VMOVD r32/m32, xmm11AVX512FMove quadword from xmm1 register to r/m64.EVEX.128.66.0F.W1 7E /rDV/N.E.VMOVQ r64/m64, xmm1NOTES:1.
For this specific instruction, VEX.W/EVEX.W in non-64 bit is ignored; the instruction behaves as if the W0 version is used.Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:reg (w)ModRM:r/m (r)N/AN/ABN/AModRM:r/m (w)ModRM:reg (r)N/AN/ACopies a doubleword from the source operand (second operand) to the destination operand (first operand).
The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations.
This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location.
The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.When the destination operand is an MMX technology register, the source operand is written to the low doubleword of the register, and the register is zero-extended to 64 bits.
When the destination operand is an XMM register, the source operand is written to the low doubleword of the register, and the register is zero-extended to 128 bits.In 64-bit mode, the instruction's default operation size is 32 bits.
Use of the REX.R prefix permits access to addi-tional registers (R8-R15).
Use of the REX.W prefix promotes operation to 64 bits.
See the summary chart at the beginning of this section for encoding data and limits.MOVD/Q with XMM destination:Moves a dword/qword integer from the source operand and stores it in the low 32/64-bits of the destination XMM register.
The upper bits of the destination are zeroed.
The source operand can be a 32/64-bit register or 32/64-bit memory location.
128-bit Legacy SSE version: Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.
Qword operation requires the use of REX.W=1.VEX.128 encoded version: Bits (MAXVL-1:128) of the destination register are zeroed.
Qword operation requires the use of VEX.W=1.EVEX.128 encoded version: Bits (MAXVL-1:128) of the destination register are zeroed.
Qword operation requires the use of EVEX.W=1.MOVD/Q with 32/64 reg/mem destination:Stores the low dword/qword of the source XMM register to 32/64-bit memory location or general-purpose register.
Qword operation requires the use of REX.W=1, VEX.W=1, or EVEX.W=1.Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.If VMOVD or VMOVQ is encoded with VEX.L= 1, an attempt to execute the instruction encoded with VEX.L= 1 will cause an #UD exception.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-22, "Type 5 Class Exception Conditions."
  > EVEX-encoded instruction, see Table2-57, "Type E9NF Class Exception Conditions."
  > Additionally:
  - #UD - If VEX.L = 1.

## Operation

```C
MOVD (When Destination Operand is an MMX Technology Register)DEST[31:0] := SRC;DEST[63:32] := 00000000H;MOVD (When Destination Operand is an XMM Register)DEST[31:0] := SRC;DEST[127:32] := 000000000000000000000000H;DEST[MAXVL-1:128] (Unmodified)MOVD (When Source Operand is an MMX Technology or XMM Register)DEST := SRC[31:0];VMOVD (VEX-Encoded Version when Destination is an XMM Register)DEST[31:0] := SRC[31:0]DEST[MAXVL-1:32] := 0MOVQ (When Destination Operand is an XMM Register)DEST[63:0] := SRC[63:0];MOVQ (When Destination Operand is r/m64)DEST[63:0] := SRC[63:0];MOVQ (When Source Operand is an XMM Register or r/m64)DEST := SRC[63:0];VMOVQ (VEX-Encoded Version When Destination is an XMM Register)DEST[63:0] := SRC[63:0]DEST[MAXVL-1:64] := 0VMOVD (EVEX-Encoded Version When Destination is an XMM Register)DEST[31:0] := SRC[31:0]DEST[MAXVL-1:32] := 0VMOVQ (EVEX-Encoded Version When Destination is an XMM Register)DEST[63:0] := SRC[63:0]DEST[MAXVL-1:64] := 0Intel C/C++ Compiler Intrinsic EquivalentMOVD __m64 _mm_cvtsi32_si64 (int i )MOVD int _mm_cvtsi64_si32 ( __m64m ) MOVD __m128i _mm_cvtsi32_si128 (int a) MOVD int _mm_cvtsi128_si32 ( __m128i a)MOVQ __int64 _mm_cvtsi128_si64(__m128i); MOVQ __m128i _mm_cvtsi64_si128(__int64);VMOVD __m128i _mm_cvtsi32_si128( int);VMOVD int _mm_cvtsi128_si32( __m128i );VMOVQ __m128i _mm_cvtsi64_si128 (__int64);VMOVQ __int64 _mm_cvtsi128_si64(__m128i );VMOVQ __m128i _mm_loadl_epi64( __m128i * s);VMOVQ void _mm_storel_epi64( __m128i * d, __m128i s);
```
