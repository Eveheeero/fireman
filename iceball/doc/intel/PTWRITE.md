# PTWRITE

Write Data to a Processor Trace Packet

This instruction reads data in the source operand and sends it to the Intel Processor Trace hardware to be encoded in a PTW packet if TriggerEn, ContextEn, FilterEn, and PTWEn are all set to 1.
For more details on these values, see ®Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 3C, Section 33.2.2, "Software Trace Instrumentation with PTWRITE." The size of data is 64-bit if using REX.W in 64-bit mode, otherwise 32-bits of data are copied from the source operand.Note: The instruction will #UD if prefix 66H is used.

## Flags affected

- None.

## Exceptions

- Real-Address Mode Exceptions
  - #GP(0) - If any part of the operand lies outside
  > of the effective address space from 0 to 0FFFFH.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #UD - If CPUID.(EAX=14H, ECX=0
  > ):EBX.PTWRITE [Bit 4] = 0.
  > If LOCK prefix is used.
  > If 66H prefix is used.
- Virtual 8086 Mode Exceptions
  - #GP(0) - If any part of the operand lies outside
  > of the effective address space from 0 to 0FFFFH.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF (fault-code) - For a page fault.
  - #AC(0) - If an unaligned memory reference is
  > made while alignment checking is enabled.
  - #UD - If CPUID.(EAX=14H, ECX=0
  > ):EBX.PTWRITE [Bit 4] = 0.
  > If LOCK prefix is used.
  > If 66H prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in Protected Mode.
- 64-Bit Mode Exceptions
  - #GP(0) - If the memory address is in a non-canonical form.
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #PF (fault-code) - For a page fault.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If CPUID.(EAX=14H, ECX=0
  > ):EBX.PTWRITE [Bit 4] = 0.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  >  outside the CS, DS, ES, FS or GS segments.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF (fault-code) - For a page fault.
  - #AC(0) - If an unaligned memory reference is made wh
  > ile the current privilege level is 3 and alignment 
  > checking is enabled.
  - #UD - If CPUID.(EAX=14H, ECX=0
  > ):EBX.PTWRITE [Bit 4] = 0.

## Operation

```C
IF (IA32_RTIT_STATUS.TriggerEn & IA32_RTIT_STATUS.ContextEn & IA32_RTIT_STATUS.FilterEn & IA32_RTIT_CTL.PTWEn) = 1PTW.PayloadBytes := Encoded payload size;PTW.IP := IA32_RTIT_CTL.FUPonPTW IF IA32_RTIT_CTL.FUPonPTW = 1Insert FUP packet with IP of PTWRITE;FI;FI;
```
