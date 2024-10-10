# LTR

Load Task Register

Loads the source operand into the segment selector field of the task register.
The source operand (a general-purpose register or a memory location) contains a segment selector that points to a task state segment (TSS).
After the segment selector is loaded in the task register, the processor uses the segment selector to locate the segment descriptor for the TSS in the global descriptor table (GDT).
It then loads the segment limit and base address for the TSS from the segment descriptor into the task register.
The task pointed to by the task register is marked busy, but a switch to the task does not occur.The LTR instruction is provided for use in operating-system software; it should not be used in application programs.
It can only be executed in protected mode when the CPL is 0.
It is commonly used in initialization code to establish the first task to be executed.The operand-size attribute has no effect on this instruction.
In 64-bit mode, the operand size is still fixed at 16 bits.
The instruction references a 16-byte descriptor to load the 64-bit base.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #UD - The LTR instruction is not re
  > cognized in virtual-8086 mode.
- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the source operand contains a NULL segment selector.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #GP(selector) - If the source selector points to a segment that is not a TSS or to one for a task that is already
  > busy.
  > If the selector points to LDT or is beyond the GDT limit.
  - #NP(selector) - If the TSS is marked not present.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #UD - The LTR instruction is not recognized in real-address mode.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the current privilege level is not 0.
  > If the memory address is in a non-canonical form.
  > If the source operand contains a NULL segment selector.
  - #GP(selector) - If the source selector points to a segment that is not a TSS or to one for a task that is already
  > busy.
  > If the selector points to LDT or is beyond the GDT limit.
  > If the descriptor type of the upper 8-byte
  >  of the 16-byte descriptor is non-zero.
  - #NP(selector) - If the TSS is marked not present.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF SRC is a NULL selectorTHEN #GP(0);IF SRC(Offset) > descriptor table limit OR IF SRC(type)  globalTHEN #GP(segment selector); FI;Read segment descriptor;IF segment descriptor is not for an available TSS THEN #GP(segment selector); FI;IF segment descriptor is not present THEN #NP(segment selector); FI;TSSsegmentDescriptor(busy) := 1; (* Locked read-modify-write operation on the entire descriptor when setting busy flag *)TaskRegister(SegmentSelector) := SRC;TaskRegister(SegmentDescriptor) := TSSSegmentDescriptor;
```
