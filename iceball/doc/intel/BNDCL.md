# BNDCL

Check Lower Bound

InstructionMode Feature SupportFlagF3 0F 1A /rRMNE/VMPXGenerate a #BR if the address in r/m32 is lower than the lower BNDCL bnd, r/m32bound in bnd.LB.F3 0F 1A /rRMV/NEMPXGenerate a #BR if the address in r/m64 is lower than the lower BNDCL bnd, r/m64bound in bnd.LB.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMModRM:reg (w)ModRM:r/m (r)N/ACompare the address in the second operand with the lower bound in bnd.
The second operand can be either a register or memory operand.
If the address is lower than the lower bound in bnd.LB, it will set BNDSTATUS to 01H and signal a #BR exception.This instruction does not cause any memory access, and does not read or write any flags.


## Flags affected

- None

## Exceptions

- Protected Mode Exceptions
  - #BR - If lower bound check fails.
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
- Virtual-8086 Mode Exceptions
  - #BR - If lower bound check fails.
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 16-bit addressing is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
- Real-Address Mode Exceptions
  - #BR - If lower bound check fails.
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 16-bit addressing is used.

## Operation

```C
BNDCL BND, regIF reg < BND.LB ThenBNDSTATUS := 01H; #BR; FI;BNDCL BND, memTEMP := LEA(mem); IF TEMP < BND.LB ThenBNDSTATUS := 01H; #BR; FI;Intel C/C++ Compiler Intrinsic EquivalentBNDCL void   _bnd_chk_ptr_lbounds(const void *q)
```
