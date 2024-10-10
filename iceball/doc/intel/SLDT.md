# SLDT

Store Local Descriptor Table Register

Stores the segment selector from the local descriptor table register (LDTR) in the destination operand.
The desti-nation operand can be a general-purpose register or a memory location.
The segment selector stored with this instruction points to the segment descriptor (located in the GDT) for the current LDT.
This instruction can only be executed in protected mode.Outside IA-32e mode, when the destination operand is a 32-bit register, the 16-bit segment selector is copied into the low-order 16 bits of the register.
The high-order 16 bits of the register are cleared for the Pentium 4, Intel Xeon, and P6 family processors.
They are undefined for Pentium, Intel486, and Intel386 processors.
When the destina-tion operand is a memory location, the segment selector is written to memory as a 16-bit quantity, regardless of the operand size.In compatibility mode, when the destination operand is a 32-bit register, the 16-bit segment selector is copied into the low-order 16 bits of the register.
The high-order 16 bits of the register are cleared.
When the destination operand is a memory location, the segment selector is written to memory as a 16-bit quantity, regardless of the operand size.In 64-bit mode, using a REX prefix in the form of REX.R permits access to additional registers (R8-R15).
The behavior of SLDT with a 64-bit register is to zero-extend the 16-bit selector and store it in the register.
If the desti-nation is memory and operand size is 64, SLDT will write the 16-bit selector to memory as a 16-bit quantity, regardless of the operand size.

## Flags affected

- None.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  > If CR4.UMIP= 1 and CPL> 0.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an una
- Protected Mode Exceptions
  - #GP(0) - If the destination is located in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  > If CR4.UMIP= 1 and CPL> 0.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an un
  > aligned memory reference is made while CPL= 3.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #UD - The SLDT instruction is not recognized in virtual-8086 mode.

## Operation

```C
DEST := LDTR(SegmentSelector);
```
