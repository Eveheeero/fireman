# PINSRB/PINSRD/PINSRQ

Insert Byte/Dword/Qword

Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand).
(The other elements in the desti-nation register are left untouched.) The source operand can be a general-purpose register or a memory location.
(When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destina-tion operand is an XMM register.
The count operand is an 8-bit immediate.
When specifying a qword[dword, byte] In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15, R8-15).
Use of REX.W permits the use of 64 bit general purpose regis-ters.128-bit Legacy SSE version: Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: Bits (MAXVL-1:128) of the destination register are zeroed.
VEX.L must be 0, otherwise the instruction will #UD.
Attempt to execute VPINSRQ in non-64-bit mode will cause #UD.EVEX.128 encoded version: Bits (MAXVL-1:128) of the destination register are zeroed.
EVEX.L'L must be 0, other-wise the instruction will #UD.


## Flags affected

- None.

## Exceptions

- Other Exceptions
  > EVEX-encoded instruction, see Table2-22, "Type 5 Class Exception Conditions."
  > EVEX-encoded instruction, see Table2-57, "Type E9NF Class Exception Conditions."

## Operation

```C
CASE OFPINSRB:SEL := COUNT[3:0];MASK := (0FFH << (SEL * 8)); TEMP := (((SRC[7:0] << (SEL *8)) AND MASK);PINSRD:SEL := COUNT[1:0];MASK := (0FFFFFFFFH << (SEL * 32)); TEMP := (((SRC << (SEL *32)) AND MASK);PINSRQ:SEL := COUNT[0]MASK := (0FFFFFFFFFFFFFFFFH << (SEL * 64)); TEMP := (((SRC << (SEL *64)) AND MASK);ESAC;DEST := ((DEST AND NOT MASK) OR TEMP); VPINSRB (VEX/EVEX Encoded Version)SEL := imm8[3:0]DEST[127:0] := write_b_element(SEL, SRC2, SRC1)DEST[MAXVL-1:128] := 0VPINSRD (VEX/EVEX Encoded Version)SEL := imm8[1:0]DEST[127:0] := write_d_element(SEL, SRC2, SRC1)DEST[MAXVL-1:128] := 0VPINSRQ (VEX/EVEX Encoded Version)SEL := imm8[0]DEST[127:0] := write_q_element(SEL, SRC2, SRC1)DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentPINSRB __m128i _mm_insert_epi8 (__m128i s1, int s2, const int ndx);PINSRD __m128i _mm_insert_epi32 (__m128i s2, int s, const int ndx);PINSRQ __m128i _mm_insert_epi64(__m128i s2, __int64 s, const int ndx);
```
