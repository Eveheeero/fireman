# CLRSSBSY

Clear Busy Flag in a Supervisor Shadow Stack Token

Clear busy flag in supervisor shadow stack token reference by m64.
Subsequent to marking the shadow stack as not busy the SSP is loaded with value 0.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #UD - The CLRSSBSY instru
  > ction is not recognized in virtual-8086 mode.
- Compatibility Mode Exceptions
  - #UD - Same exceptions as in protected mode.
  - #GP(0) - Same exceptions as in protected mode.
  - #PF(fault-code) - If a page fault occurs.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  > IF IA32_S_CET.SH_STK_EN = 0.
  - #GP(0) - If memory operand linear address not aligned to 8 bytes.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If destination is located in a non-writeable segment.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  > If CPL is not 0.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
- Real-Address Mode Exceptions
  - #UD - The CLRSSBSY instruction is not recognized in real-address mode.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  > IF IA32_S_CET.SH_STK_EN = 0.
  - #GP(0) - If memory operand linear address not aligned to 8 bytes.
  > If CPL is not 0.
  > If the memory address is in a non-canonical form.
  > If token is invalid.

## Operation

```C
IF (CR4.CET = 0)THEN #UD; FI;IF (IA32_S_CET.SH_STK_EN = 0)THEN #UD; FI;IF CPL > 0THEN GP(0); FI;SSP_LA = Linear_Address(mem operand)IF SSP_LA not aligned to 8 bytesTHEN #GP(0); FI;expected_token_value = SSP_LA | BUSY_BIT(* busy bit - bit position 0 - must be set *)new_token_value = SSP_LA (* Clear the busy bit *)IF shadow_stack_lock_cmpxchg8b(SSP_LA, new_token_value, expected_token_value) != expected_token_valueinvalid_token := 1; FI(* Set the CF if invalid token was detected *)RFLAGS.CF = (invalid_token == 1) ? 1 : 0;RFLAGS.ZF,PF,AF,OF,SF := 0;SSP := 0
```
