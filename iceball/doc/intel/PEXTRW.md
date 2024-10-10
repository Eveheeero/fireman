# PEXTRW

Extract Word

Copies the word in the source operand (second operand) specified by the count operand (third operand) to the destination operand (first operand).
The source operand can be an MMX technology register or an XMM register.
The destination operand can be the low word of a general-purpose register or a 16-bit memory address.
The count operand is an 8-bit immediate.
When specifying a word location in an MMX technology register, the 2 least-signifi-cant bits of the count operand specify the location; for an XMM register, the 3 least-significant bits specify the loca-tion.
The content of the destination register above bit 16 is cleared (set to all 0s).In 64-bit mode, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15, R8-15).
If the destination operand is a geneNote: In VEX.128 encoded versions, VEX.vvvv is reserved and must be 1111b, VEX.L must be 0, otherwise the instruction will #UD.
In EVEX.128 encoded versions, EVEX.vvvv is reserved and must be 1111b, EVEX.L must be 0, otherwise the instruction will #UD.
If the destination operand is a register, the default operand size in 64-bit mode for VPEXTRW is 64 bits, the bits above the least significant byte/word/dword data are filled with zeros.

## Flags affected

- None.

## Exceptions

- Numeric Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-22, "Type 5 Class Exception Conditions."
  > EVEX-encoded instruction, see Table2-57, "Type E9NF Class Exception Conditions."
  > Additionally:

## Operation

```C
IF (DEST = Mem16)THENSEL := COUNT[2:0];TEMP := (Src >> SEL*16) AND FFFFH;Mem16 := TEMP[15:0];ELSE IF (64-Bit Mode and destination is a general-purpose register)THENFOR (PEXTRW instruction with 64-bit source operand){SEL := COUNT[1:0]; 16)) AND FFFFH;TEMP := (SRC >> (SEL r64[15:0] := TEMP[15:0];r64[63:16] := ZERO_FILL; };FOR (PEXTRW instruction with 128-bit source operand) {SEL := COUNT[2:0]; 16)) AND FFFFH;TEMP := (SRC >> (SEL r64[15:0] := TEMP[15:0];r64[63:16] := ZERO_FILL; }ELSEFOR (PEXTRW instruction with 64-bit source operand){SEL := COUNT[1:0]; 16)) AND FFFFH;TEMP := (SRC >> (SEL r32[15:0] := TEMP[15:0];r32[31:16] := ZERO_FILL; };FOR (PEXTRW instruction with 128-bit source operand){SEL := COUNT[2:0]; 16)) AND FFFFH;TEMP := (SRC >> (SEL r32[15:0] := TEMP[15:0];r32[31:16] := ZERO_FILL; };FI;FI;VPEXTRW ( dest=m16)SRC_Offset := imm8[2:0]Mem16 := (Src >> Src_Offset*16)VPEXTRW ( dest=reg)IF (64-Bit Mode )THENSRC_Offset := imm8[2:0]DEST[15:0] := ((Src >> Src_Offset*16) AND 0FFFFh)DEST[63:16] := ZERO_FILL;ELSESRC_Offset := imm8[2:0]DEST[15:0] := ((Src >> Src_Offset*16) AND 0FFFFh)Intel C/C++ Compiler Intrinsic EquivalentPEXTRW int _mm_extract_pi16 (__m64 a, int n)PEXTRW int _mm_extract_epi16 ( __m128i a, int imm) 
```
