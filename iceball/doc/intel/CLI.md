# CLI

Clear Interrupt Flag

In most cases, CLI clears the IF flag in the EFLAGS register and no other flags are affected.
Clearing the IF flag causes the processor to ignore maskable external interrupts.
The IF flag and the CLI and STI instruction have no effect on the generation of exceptions and NMI interrupts.Operation is different in two modes defined as follows: - PVI mode (protected-mode virtual interrupts): CR0.PE= 1, EFLAGS.VM= 0, CPL= 3, and CR4.PVI= 1; - VME mode (virtual-8086 mode extensions): CR0.PE= 1, EFLAGS.VM= 1, and CR4.VME= 1.If IOPL< 3 and either VME mode or PVI mode is active, CLI clears the VIF flag in the EFLAGS register, leaving IF unaffected.Table 3-7 indicates the action of the CLI instruction depending on the processor operating mode, IOPL, and CPL.
Table 3-7.
 Decision Table for CLI ResultsModeIOPLCLI Result1  IF0Real-addressX=CPLIF 0 = 2Protected, not PVICPL#GP fault<  03IF =3Protected, PVI  00-2VIF=3IF  0=3Virtual-8086, not VME0-2#GP fault3IF  0=3Virtual-8086, VME  00-2VIF=NOTES:1.
X = This setting has no effect on instruction operation.2.
For this table, "protected mode" applies whenever CR0.PE= 1 and EFLAGS.VM= 0; it includes compatibility mode and 64-bit mode

## Flags affected

- Either the IF flag or the VIF flag is cleared to 0. Other flags are unaffected.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If CPL is greater than IOPL and PVI mode is not active.
  > If CPL is greater than IOPL and less than 3.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If IOPL is less than 3 and VME mode is not active.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF CR0.PE = 0THEN IF := 0; (* Reset Interrupt Flag *)ELSEIF IOPL  CPL(* CPL = 3 if EFLAGS.VM = 1 *)THEN IF := 0; (* Reset Interrupt Flag *)ELSEIF VME mode OR PVI modeTHEN VIF := 0; (* Reset Virtual Interrupt Flag *)ELSE #GP(0);FI;FI;FI;
```
