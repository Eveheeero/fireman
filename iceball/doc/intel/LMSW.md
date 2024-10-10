# LMSW

Load Machine Status Word

Loads the source operand into the machine status word, bits 0 through 15 of register CR0.
The source operand can be a 16-bit general-purpose register or a memory location.
Only the low-order 4 bits of the source operand (which contains the PE, MP, EM, and TS flags) are loaded into CR0.
The PG, CD, NW, AM, WP, NE, and ET flags of CR0 are not affected.
The operand-size attribute has no effect on this instruction.If the PE flag of the source operand (bit 0) is set to 1, the instruction causes the processor to switch to protected mode.
While in protected mode, the LMSW instruction cannot be used to clear the PE flag and force a switch back to real-address mode.The LMSW instruction is provided for use in operating-system software; it should not be used in application programs.
In protected or virtual-8086 mode, it can only be executed at CPL 0.This instruction is provided for compatibility with the Intel 286 processor; programs and procedures intended to run on IA-32 and Intel 64 processors beginning with Intel386 processors should use the MOV (control registers) instruction to load the whole CR0 register.
The MOV CR0 instruction can be used to set and clear the PE flag in CR0, allowing a procedure or program to switch between protected and real-address modes.This instruction is a serializing instruction.This instruction's operation is the same in non-64-bit modes and 64-bit mode.
Note that the operand size is fixed at 16 bits.® 64 and IA-32 Archi-See "Changes to Instruction Behavior in VMX Non-Root Operation" in Chapter 26 of the Inteltectures Software Developer's Manual, Volume 3C, for more information about the behavior of this instruction in VMX non-root operation.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - The LMSW instruction is not recognized in virtual-8086 mode.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the current privilege level is not 0.
  > If the memory address is in a non-canonical form.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
CR0[0:3] := SRC[0:3];
```
