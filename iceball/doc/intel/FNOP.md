# FNOP

No Operation

Performs no FPU operation.
This instruction takes up space in the instruction stream but does not affect the FPU or machine context, except the EIP register and the FPU Instruction Pointer.This instruction's operation is the same in non-64-bit modes and 64-bit mode.FPU Flags AffectedC0, C1, C2, C3 undefined.

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Floating-Point Exceptions
  > None.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
