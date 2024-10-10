# BNDCU/BNDCN

Check Upper Bound

InstructionMode Feature SupportFlagF2 0F 1A /rRMNE/VMPXGenerate a #BR if the address in r/m32 is higher than the upper BNDCU bnd, r/m32bound in bnd.UB (bnb.UB in 1's complement form).F2 0F 1A /rRMV/NEMPXGenerate a #BR if the address in r/m64 is higher than the upper BNDCU bnd, r/m64bound in bnd.UB (bnb.UB in 1's complement form).F2 0F 1B /rRMNE/VMPXGenerate a #BR if the address in r/m32 is higher than the upper BNDCN bnd, r/m32bound in bnd.UB (bnb.UB not in 1's complement form).F2 0F 1B /rRMV/NEMPXGenerate a #BR if the address in r/m64 is higher than the upper BNDCN bnd, r/m64bound in bnd.UB (bnb.UB not in 1's complement form).Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMModRM:reg (w)ModRM:r/m (r)N/ACompare the address in the second operand with the upper bound in bnd.
The second operand can be either a register or a memory operand.
If the address is higher than the upper bound in bnd.UB, it will set BNDSTATUS to 01H and signal a #BR exception.BNDCU perform 1's complement operation on the upper bound of bnd first before proceeding with address compar-ison.
BNDCN perform address comparison directly using the upper bound in bnd that is already reverted out of 1's complement form.
This instruction does not cause any memory access, and does not read or write any flags.
Effective address computation of m32/64 has identical behavior to LEA

## Flags affected

- None

## Exceptions

- 64-Bit Mode Exceptions
- Protected Mode Exceptions
  - #BR - If upper bound check fails.
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 67H prefix is not used and CS.D=0.
  > If 67H prefix is used and CS.D=1.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #BR - If upper bound check fails.
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 16-bit addressing is used.
- Virtual-8086 Mode Exceptions
  - #BR - If upper bound check fails.
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 16-bit addressing is used.

## Operation

```C
BNDCU BND, regIF reg > NOT(BND.UB) ThenBNDSTATUS := 01H; #BR; FI;BNDCU BND, memTEMP := LEA(mem); IF TEMP > NOT(BND.UB) ThenBNDSTATUS := 01H; #BR; FI;BNDCN BND, regIF reg > BND.UB ThenBNDSTATUS := 01H; BNDCN BND, memTEMP := LEA(mem); IF TEMP > BND.UB ThenBNDSTATUS := 01H; #BR; FI;Intel C/C++ Compiler Intrinsic EquivalentBNDCU .void   _bnd_chk_ptr_ubounds(const void *q)
```
