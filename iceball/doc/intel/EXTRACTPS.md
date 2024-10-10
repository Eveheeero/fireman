# EXTRACTPS

Extract Packed Floating-Point Values

Extracts a single precision floating-point value from the source operand (second operand) at the 32-bit offset spec-ified from imm8.
Immediate bits higher than the most significant offset for the vector length are ignored.The extracted single precision floating-point value is stored in the low 32-bits of the destination operandIn 64-bit mode, destination register operand has default operand size of 64 bits.
The upper 32-bits of the register are filled with zero.
REX.W is ignored.VEX.128 and EVEX encoded version: When VEX.W1 or EVEX.W1 form is used in 64-bit mode with a general purpose register (GPR) as a destination operand, the packed single quantity is zero extended to 64 bits.
VEX.vvvv/EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.128-bit Legacy SSE version: When a REX.W prefix is used in 64-bit mode with a general purpose register (GPR) as a destination operand, the packed single quantity is zero extended to 64 bits.The source register is an XMM register.
Imm8[1:0] determine the starting DWORD offset from which to extract the 32-bit floating-point value.If VEXTRACTPS is encoded with VEX.L= 1, an attempt to execute the instruction encoded with VEX.L= 1 will cause an #UD exception.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > VEX-encoded instructions, see Table2-22, "Type 5 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-57, "Type E9NF Class Exception Conditions."
  > Additionally:
  - #UD - IF VEX.L = 0.

## Operation

```C
VEXTRACTPS (EVEX and VEX.128 Encoded Version)SRC_OFFSET := IMM8[1:0]IF (64-Bit Mode and DEST is register)DEST[31:0] := (SRC[127:0] >> (SRC_OFFSET*32)) AND 0FFFFFFFFhDEST[63:32] := 0ELSEEXTRACTPS (128-bit Legacy SSE Version)SRC_OFFSET := IMM8[1:0]IF (64-Bit Mode and DEST is register)DEST[31:0] := (SRC[127:0] >> (SRC_OFFSET*32)) AND 0FFFFFFFFhDEST[63:32] := 0ELSEDEST[31:0] := (SRC[127:0] >> (SRC_OFFSET*32)) AND 0FFFFFFFFhFIIntel C/C++ Compiler Intrinsic EquivalentEXTRACTPS int _mm_extract_ps (__m128 a, const int nidx);
```
