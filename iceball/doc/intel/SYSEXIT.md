# SYSEXIT

Fast Return from Fast System Call

Executes a fast return to privilege level 3 user code.
SYSEXIT is a companion instruction to the SYSENTER instruc-tion.
The instruction is optimized to provide the maximum performance for returns from system procedures executing at protections levels 0 to user procedures executing at protection level 3.
It must be executed from code executing at privilege level 0.
With a 64-bit operand size, SYSEXIT remains in 64-bit mode; otherwise, it either enters compatibility mode (if the logical processor is in IA-32e mode) or remains in protected mode (if it is not).Prior to executing SYSEXIT, software must specify the privilege level 3 code segment and code entry point, and the privilege level 3 stack segment and stack pointer by writing values into the following MSR and general-purpose registers: - IA32_SYSENTER_CS (MSR address 174H) - Contains a 32-bit value that is used to determine the segment selectors for the privilege level 3 code and stack segments (see the Operation section) - RDX - The canonical address in this register is loaded into RIP (thus, this value references the first instruction to be executed in the user code).
If the return is not to 64-bit mode, only bits31:0 are loaded.
- ECX - The canonical address in this register is loaded into RSP (thus, this value contains the stack pointer for the privilege level 3 stack).
If the return is not to 64-bit mode, only bits31:0 are loaded.The IA32_SYSENTER_CS MSR can be read from and written to using RDMSR and WRMSR.While SYSEXIT loads the CS and SS selectors with values derived from the IA32_SYSENTER_CS MSR, the CS and SS descriptor caches are not loaded from the descriptors (in GDT or LDT) referenced by those selectors.
Instead, the descriptor caches are loaded with fixed values.
See the Operation section for details.
It is the responsibility of OS software to ensure that the descriptors (in GDT or LDT) referenced by those selector values correspond to the fixed values loaded into the descriptor caches; the SYSEXIT instruction does not ensure this correspondence.The SYSEXIT instruction can be invoked from all operating modes except real-address mode and virtual-8086 mode.
The SYSENTER and SYSEXIT instructions were introduced into the IA-32 architecture in the Pentium II processor.
The availability of these instructions on a processor is indicated with the SYSENTER/SYSEXIT present (SEP) feature flag returned to the EDX register by the CPUID instruction.
An operating system that qualifies the SEP flag must also qualify the processor family and model to ensure that the SYSENTER/SYSEXIT instructions are actually present.
For example:IF CPUID SEP bit is set= 6) and (Model < 3) and (Stepping < 3) THEN IF (Family THENSYSENTER/SYSEXIT_Not_Supported; FI;ELSE SYSENTER/SYSEXIT_Supported; FI;FI; When the CPUID instruction is executed on the Pentium Pro processor (model1), the processor returns a the SEP When shadow stacks are enabled at privilege level 3 the instruction loads SSP with value from IA32_PL3_SSP MSR.
Refer to Chapter 6, "Interrupt and Exception Handling" and Chapter 17, "Control-flow Enforcement Technology ®(CET)" in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for additional CET details.Instruction ordering.
Instructions following a SYSEXIT may be fetched from memory before earlier instructions complete execution, but they will not execute (even speculatively) until all instructions prior to the SYSEXIT have completed execution (the later instructions may execute before data stored by the earlier instructions have become globally visible).

## Flags affected

- None.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - The SYSEXIT instruction is not recognized in virtual-8086 mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #GP(0) - If IA32_SYSENTER_CS = 0.
  > 
  >  
  > If CPL 
  > 0.
- Protected Mode Exceptions
  - #GP(0) - If IA32_SYSENTER_CS[15:2] = 0.
  > 
  >  
  > If CPL 
  > 0.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - The SYSEXIT instruction is not recognized in real-address mode.
  - #UD - If the LOCK prefix is used.

## Operation

```C
==    IF IA32_SYSENTER_CS[15:2]0 OR CR0.PE 0 OR CPL 0 THEN #GP(0); FI;IF operand size is 64-bitTHEN(* Return to 64-bit mode *)RSP := RCX;RIP := RDX;ELSE(* Return to protected mode or compatibility mode *)RSP := ECX;RIP := EDX;FI;IF operand size is 64-bit(* Operating system provides CS; RPL forced to 3 *)THEN CS.Selector := IA32_SYSENTER_CS[15:0] + 32;ELSE CS.Selector := IA32_SYSENTER_CS[15:0] + 16;FI;CS.Selector := CS.Selector OR 3;(* RPL forced to 3 *)(* Set rest of CS to a fixed value *)CS.Base := 0;(* Flat segment *)CS.Limit := FFFFFH;(* With 4-KByte granularity, implies a 4-GByte limit *)CS.Type := 11;(* Execute/read code, accessed *)CS.S := 1;CS.DPL := 3;CS.P := 1;IF operand size is 64-bitTHEN(* return to 64-bit mode *)CS.L := 1;(* 64-bit code segment *)CS.D := 0;(* Required if CS.L = 1 *)ELSE(* return to protected mode or compatibility mode *)CS.L := 0;CS.D := 1;(* 32-bit code segment*)FI;CS.G := 1;(* 4-KByte granularity *)CPL := 3;IF ShadowStackEnabled(CPL)THEN SSP := IA32_PL3_SSP;FI;SS.Selector := CS.Selector + 8;(* SS just above CS *)(* Set rest of SS to a fixed value *)SS.Base := 0;(* Flat segment *)SS.Limit := FFFFFH;(* With 4-KByte granularity, implies a 4-GByte limit *)SS.Type := 3;(* Read/write data, accessed *)SS.S := 1;SS.DPL := 3;SS.G := 1;(* 4-KByte granularity *)
```
