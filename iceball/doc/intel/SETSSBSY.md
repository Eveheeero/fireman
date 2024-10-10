# SETSSBSY

Mark Shadow Stack Busy

The SETSSBSY instruction verifies the presence of a non-busy supervisor shadow stack token at the address in the IA32_PL0_SSP MSR and marks it busy.
Following successful execution of the instruction, the SSP is set to the value of the IA32_PL0_SSP MSR.

## Flags affected

- None.C/C++ Compiler Intrinsic EquivalentSETSSBSYvoid _setssbsy(void);

## Exceptions

- Virtual-8086 Mode Exceptions
  - #UD - The SETSSBSY instruction is not recognized in virtual-8086 mode.
- Real-Address Mode Exceptions
  - #UD - The SETSSBSY instruction is not recognized in real-address mode.
- Compatibility Mode Exceptions
  > Same as protected mode exceptions.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  > IF IA32_S_CET.SH_STK_EN = 0.
  - #GP(0) - If IA32_PL0_SSP not aligned to 8 bytes.
  > If CPL is not 0.
  - #PF(fault-code) - If a page fault occurs.

## Operation

```C
IF (CR4.CET = 0)THEN #UD; FI;IF (IA32_S_CET.SH_STK_EN = 0)THEN #UD; FI;IF CPL > 0THEN GP(0); FI;SSP_LA = IA32_PL0_SSPIf SSP_LA not aligned to 8 bytesTHEN #GP(0); FI;expected_token_value = SSP_LA (* busy bit must not be set *)new_token_value          = SSP_LA | BUSY_BIT(* set busy bit; bit position 0 *)IF shadow_stack_lock_cmpxchg8B(SSP_LA, new_token_value, expected_token_value) != expected_token_valueTHEN #CP(SETSSBSY); FI;SSP = SSP_LA
```
