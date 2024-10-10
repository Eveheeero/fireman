# PEXTRB/PEXTRD/PEXTRQ

Extract Byte/Dword/Qword

InstructionMode Feature SupportFlag66 0F 3A 14 /r ibAV/VSSE4_1Extract a byte integer value from xmm2 at the PEXTRB reg/m8, xmm2, imm8source byte offset specified by imm8 into reg or m8.
The upper bits of r32 or r64 are zeroed.66 0F 3A 16 /r ibAV/VSSE4_1Extract a dword integer value from xmm2 at the PEXTRD r/m32, xmm2, imm8source dword offset specified by imm8 into r/m32.66 REX.W 0F 3A 16 /r ibAV/N.E.SSE4_1Extract a qword integer value from xmm2 at the PEXTRQ r/m64, xmm2, imm8source qword offset specified by imm8 into r/m64.1/VAVXExtract a byte integer value from xmm2 at the VEX.128.66.0F3A.W0 14 /r ibAVsource byte offset specified by imm8 into reg or VPEXTRB reg/m8, xmm2, imm8m8.
The upper bits of r64/r32 is filled with zeros.VEX.128.66.0F3A.W0 16 /r ibAV/VAVXExtract a dword integer value from xmm2 at the VPEXTRD r32/m32, xmm2, imm8source dword offset specified by imm8 into r32/m32.2AVXExtract a qword integer value from xmm2 at the VEX.128.66.0F3A.W1 16 /r ibAV/Isource dword offset specified by imm8 into VPEXTRQ r64/m64, xmm2, imm8r64/m64.EVEX.128.66.0F3A.WIG 14 /r ibBV/VAVX512BWExtract a byte integer value from xmm2 at the VPEXTRB reg/m8, xmm2, imm8source byte offset specified by imm8 into reg or m8.
The upper bits of r64/r32 is filled with zeros.EVEX.128.66.0F3A.W0 16 /r ibBV/VAVX512DQExtract a dword integer value from xmm2 at the VPEXTRD r32/m32, xmm2, imm8source dword offset specified by imm8 into r32/m32.2AVX512DQExtract a qword integer value from xmm2 at the EVEX.128.66.0F3A.W1 16 /r ibBV/N.E.source dword offset specified by imm8 into VPEXTRQ r64/m64, xmm2, imm8r64/m64.NOTES:1.
In 64-bit mode, VEX.W1 is ignored for VPEXTRB (similar to legacy REX.W=1 prefix in PEXTRB).2.
VEX.W/EVEX.W in non-64 bit is ignored; the instructions behaves as if the W0 version is used.Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:r/m (w)ModRM:reg (r)imm8N/ABTuple1 ScalarModRM:r/m (w)ModRM:reg (r)imm8N/AExtract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0].
The destination can be a register or byte/dword/qword memory location.
If the destination is a register, the upper bits of the register are zero extended.In legacy non-VEX encoded version and if the destination operand is a register, the default operand size in 64-bit mode for PEXTRB/PEXTRD is 64 bits, the bits above the least significant byte/dword data are filled with zeros.
PEXTRQ is not encodable in non-64-bit modes and requires REX.W in 64-bit mode.Note: In VEX.128 encoded versions, VEX.vvvv is reserved and must be 1111b, VEX.L must be 0, otherwise the instruction will #UD.
In EVEX.128 encoded versions, EVEX.vvvv is reserved and must be 1111b, EVEX.L"L must be 0, otherwise the instruction will #UD.
If the destination operand is a register, the default operand size in 64-bit mode for VPEXTRB/VPEXTRD is 64 bits, the bits above th

## Flags affected

- None.

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-22, "Type 5 Class Exception Conditions."
  > EVEX-encoded instruction, see Table2-57, "Type E9NF Class Exception Conditions."
  > Additionally:
  - #UD - If VEX.L = 1 or EVEX.L'L > 0.
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
CASE ofPEXTRB: SEL := COUNT[3:0];TEMP := (Src >> SEL*8) AND FFH;IF (DEST = Mem8)THENMem8 := TEMP[7:0];ELSE IF (64-Bit Mode and 64-bit register selected)THENR64[7:0] := TEMP[7:0];r64[63:8] := ZERO_FILL; };ELSER32[7:0] := TEMP[7:0];r32[31:8] := ZERO_FILL; };FI;PEXTRD:SEL := COUNT[1:0];TEMP := (Src >> SEL*32) AND FFFF_FFFFH;DEST := TEMP;PEXTRQ:SEL := COUNT[0];TEMP := (Src >> SEL*64);DEST := TEMP;EASC:VPEXTRTD/VPEXTRQIF (64-Bit Mode and 64-bit dest operand)THENSrc_Offset := imm8[0]r64/m64 := (Src >> Src_Offset * 64)ELSESrc_Offset := imm8[1:0]r32/m32 := ((Src >> Src_Offset *32) AND 0FFFFFFFFh);FIVPEXTRB ( dest=m8)SRC_Offset := imm8[3:0]Mem8 := (Src >> Src_Offset*8)VPEXTRB ( dest=reg)IF (64-Bit Mode )THENSRC_Offset := imm8[3:0] DEST[7:0] := ((Src >> Src_Offset*8) AND 0FFh)DEST[63:8] := ZERO_FILL;ELSESRC_Offset := imm8[3:0];DEST[7:0] := ((Src >> Src_Offset*8) AND 0FFh);DEST[31:8] := ZERO_FILL;FIIntel C/C++ Compiler Intrinsic EquivalentPEXTRB int _mm_extract_epi8 (__m128i src, const int ndx);PEXTRD int _mm_extract_epi32 (__m128i src, const int ndx);
```
