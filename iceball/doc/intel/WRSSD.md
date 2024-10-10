# WRSSD/WRSSQ

Write to Shadow Stack

Writes bytes in register source to the shadow stack.

## Flags affected

- None.C/C++ Compiler Intrinsic Equivalent

## Exceptions

- Real-Address Mode Exceptions
  - #UD - The WRSS instruction is not recognized in real-address mode.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  > If CPL = 3 and IA32_U_CET.SH_STK_EN = 0.
  > If CPL < 3 and IA32_S_CET.SH_STK_EN = 0.
  > If CPL = 3 and IA32_U_CET.WR_SHSTK_EN = 0.
  > If CPL < 3 and IA32_S_CET.WR_SHSTK_EN = 0.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If destination is located in a non-writeable segment.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  > If linear address of destinat
  > ion is not 4 byte aligned.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs if destination is
  > not a user shadow stack when CPL3 and not a supervisor 
  > shadow stack when CPL < 3.
  > Other terminal and non-terminal faults.
- Virtual-8086 Mode Exceptions
  - #UD - The WRSS instruction is not recognized in virtual-8086 mode.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  > If CPL = 3 and IA32_U_CET.SH_STK_EN = 0.
  > If CPL < 3 and IA32_S_CET.SH_STK_EN = 0.
  > If CPL = 3 and IA32_U_CET.WR_SHSTK_EN = 0.
  > If CPL < 3 and IA32_S_CET.WR_SHSTK_EN = 0.
  - #GP(0) - If a memory address is in a non-canonical form.
  > If linear address of destinat
  > ion is not 4 byte aligned.
  - #PF(fault-code) - If a page fault occurs if destination is
  > not a user shadow stack when CPL3 and not a supervisor 
- Compatibility Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  > If CPL = 3 and IA32_U_CET.SH_STK_EN = 0.
  > If CPL < 3 and IA32_S_CET.SH_STK_EN = 0.
  > If CPL = 3 and IA32_U_CET.WR_SHSTK_EN = 0.
  > If CPL < 3 and IA32_S_CET.WR_SHSTK_EN = 0.
  - #PF(fault-code) - If a page fault occurs if destination is
  > not a user shadow stack when CPL3 and not a supervisor 
  > shadow stack when CPL < 3.
  > Other terminal and non-terminal faults.

## Operation

```C
IF CPL = 3IF (CR4.CET & IA32_U_CET.SH_STK_EN) = 0THEN #UD; FI;IF (IA32_U_CET.WR_SHSTK_EN) = 0THEN #UD; FI;ELSEIF (CR4.CET & IA32_S_CET.SH_STK_EN) = 0THEN #UD; FI;IF (IA32_S_CET.WR_SHSTK_EN) = 0THEN #UD; FI;FI;DEST_LA = Linear_Address(mem operand)IF (operand size is 64 bit)THEN(* Destination not 8B aligned *)IF DEST_LA[2:0]THEN GP(0); FI;Shadow_stack_store 8 bytes of SRC to DEST_LA;ELSE(* Destination not 4B aligned *)IF DEST_LA[1:0]THEN GP(0); FI;Shadow_stack_store 4 bytes of SRC[31:0] to DEST_LA;FI;
```
