# SMSW

Store Machine Status Word

Stores the machine status word (bits 0 through 15 of control register CR0) into the destination operand.
The desti-nation operand can be a general-purpose register or a memory location.In non-64-bit modes, when the destination operand is a 32-bit register, the low-order 16 bits of register CR0 are copied into the low-order 16 bits of the register and the high-order 16 bits are undefined.
When the destination operand is a memory location, the low-order 16 bits of register CR0 are written to memory as a 16-bit quantity, regardless of the operand size.In 64-bit mode, the behavior of the SMSW instruction is defined by the following examples: - SMSW r16 operand size 16, store CR0[15:0] in r16 - SMSW r32 operand size 32, zero-extend CR0[31:0], and store in r32 - SMSW r64 operand size 64, zero-extend CR0[63:0], and store in r64 - SMSW m16 operand size 16, store CR0[15:0] in m16 - SMSW m16 operand size 32, store CR0[15:0] in m16 (not m32) - SMSW m16 operands size 64, store CR0[15:0] in m16 (not m64)SMSW is only useful in operating-system software.
However, it is not a privileged instruction and can be used in application programs if CR4.UMIP= 0.
It is provided for compatibility with the Intel 286 processor.
Programs and procedures intended to run on IA-32 and Intel 64 processors beginning with the Intel386 processors should use the MOV CR instruction to load the machine status word.®See "Changes to Instruction Behavior in VMX Non-Root Operation" in Chapter 26 of the Intel 64 and IA-32 Archi-tectures Software Developer's Manual, Volume 3C, for more information about the behavior of this instruction in VMX non-root operation.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the destination is located in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  > If CR4.UMIP= 1 and CPL> 0.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an una
  > ligned memory reference is made while CPL= 3.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  > If CR4.UMIP= 1 and CPL> 0.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an una
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If CR4.UMIP= 1.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
DEST := CR0[15:0]; (* Machine status word *)
```
