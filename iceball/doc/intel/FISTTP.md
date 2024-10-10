# FISTTP

Store Integer With Truncation

FISTTP converts the value in ST into a signed integer using truncation (chop) as rounding mode, transfers the result to the destination, and pop ST.
FISTTP accepts word, short integer, and long integer destinations.The following table shows the results obtained when storing various classes of numbers in integer format.Table 3-28.
 FISTTP ResultsST(0)DEST *or Value Too Large for DEST Format-   F-1- I<<F+ 10- 1-+ 1+ IF* or Value Too Large for DEST Format+ *NaNNOTES:FMeans finite floating-point value.Means integer.Indicates floating-point invalid-operation (#IA) exception.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Flags affected

- C1 is cleared; C0, C2, C3 undefined.

## Exceptions

- Real Address Mode Exceptions
  - GP(0) - If any part of the operand would lie outsid
  > e of the effective address space from 0 to 0FFFFH.
  - #NM - If CR0.EM[bit 2] = 1.
  > If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.SSE3[bit 0] = 0.
  > If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Virtual 8086 Mode Exceptions
  - GP(0) - If any part of the operand would lie outsid
  > e of the effective address space from 0 to 0FFFFH.
  - #NM - If CR0.EM[bit 2] = 1.
  > If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.SSE3[bit 0] = 0.
  > If the LOCK prefix is used.
  - #PF(fault-code) - For a page fault.
  - #AC(0) - For unaligned memory reference if the current privilege is 3.
- Protected Mode Exceptions
  - #GP(0) - If the destination is in a nonwritable segment.
  > For an illegal memory operand effective addr
  > ess in the CS, DS, ES, FS or GS segments.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF(fault-code) - For a page fault.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #NM - If CR0.EM[bit 2] = 1.
  > If CR0.TS[bit 3] = 1.
- Numeric Exceptions
  > Invalid, Stack Invalid (stack underflow), Precision.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
DEST := ST;pop ST;
```
