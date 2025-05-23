# STR

Store Task Register

Stores the segment selector from the task register (TR) in the destination operand.
The destination operand can be a general-purpose register or a memory location.
The segment selector stored with this instruction points to the task state segment (TSS) for the currently running task.When the destination operand is a 32-bit register, the 16-bit segment selector is copied into the lower 16 bits of the register and the upper 16 bits of the register are cleared.
When the destination operand is a memory location, the segment selector is written to memory as a 16-bit quantity, regardless of operand size.In 64-bit mode, operation is the same.
The size of the memory operand is fixed at 16 bits.
In register stores, the 2-byte TR is zero extended if stored to a 64-bit register.The STR instruction is useful only in operating-system software.
It can only be executed in protected mode.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the destination is a memory operand that
  > is located in a non-writable segment or if the 
  > effective address is outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  > If CR4.UMIP= 1 and CPL> 0.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #GP(0) - If the memory address is in a non-canonical form.
  > If CR4.UMIP= 1 and CPL> 0.
  - #SS(0) - If the stack address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Virtual-8086 Mode Exceptions
  - #UD - The STR instruction is not recognized in virtual-8086 mode.
- Real-Address Mode Exceptions
  - #UD - The STR instruction is not recognized in real-address mode.

## Operation

```C
DEST := TR(SegmentSelector);
```
