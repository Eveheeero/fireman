# WRMSR

Write to Model Specific Register

Writes the contents of registers EDX:EAX into the 64-bit model specific register (MSR) specified in the ECX register.
(On processors that support the Intel64 architecture, the high-order 32 bits of RCX are ignored.) The contents of the EDX register are copied to high-order 32 bits of the selected MSR and the contents of the EAX register are copied to low-order 32 bits of the MSR.
(On processors that support the Intel64 architecture, the high-order 32 bits of each of RAX and RDX are ignored.) Undefined or reserved bits in an MSR should be set to values previously read.This instruction must be executed at privilege level 0 or in real-address mode; otherwise, a general protection exception #GP(0) is generated.
Specifying a reserved or unimplemented MSR address in ECX will also cause a general protection exception.
The processor will also generate a general protection exception if software attempts to write to bits in a reserved MSR.When the WRMSR instruction is used to write to an MTRR, the TLBs are invalidated.
This includes global entries ® 64 and IA-32 Architectures Software Devel-(see "Translation Lookaside Buffers (TLBs)" in Chapter 3 of the Inteloper's Manual, Volume 3A).MSRs control functions for testability, execution tracing, performance-monitoring and machine check errors.
® 64 and IA-32 Architectures Software Developer's Chapter 2, "Model-Specific Registers (MSRs)," of the IntelManual, Volume 4, lists all MSRs that can be written with this instruction and their addresses.
Note that each processor family has its own set of MSRs.® The WRMSR instruction is a serializing instruction (see "Serializing Instructions" in Chapter 9 of the Intel64 and IA-32 Architectures Software Developer's Manual, Volume 3A).
Note that WRMSR to the IA32_TSC_DEADLINE MSR (MSR index 6E0H) and the X2APIC MSRs (MSR indices 802H to 83FH) are not serializing.The CPUID instruction should be used to determine whether MSRs are supported (CPUID.01H:EDX[5] = 1) before using this instruction.IA-32 Architecture CompatibilityThe MSRs and the ability to read them with the WRMSR instruction were introduced into the IA-32 architecture with the Pentium processor.
Execution of this instruction by an IA-32 processor earlier than the Pentium processor results in an invalid opcode exception #UD.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  > If the value in ECX specifies a reserved or unimplemented MSR address.
  > If the value in EDX:EAX sets bits that 
  > are reserved in the MSR specified by ECX.
  > If the source register contains a non-canonical address and ECX specifies one of the following 
  > MSRs: IA32_DS_AREA, IA32_FS_BASE, IA32_GS_BASE, IA32_KERNEL_GS_BASE, IA32_L-
  > STAR, IA32_SYSENTER_EIP, IA32_SYSENTER_ESP.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - The WRMSR instruction is not recognized in virtual-8086 mode.
- Real-Address Mode Exceptions
  - #GP - If the value in ECX specifies a reserved or unimplemented MSR address.
  > If the value in EDX:EAX sets bits that 
  > are reserved in the MSR specified by ECX.
  > If the source register contains a non-canonical address and ECX specifies one of the following 
  > MSRs: IA32_DS_AREA, IA32_FS_BASE, IA32_GS_BASE, IA32_KERNEL_GS_BASE, IA32_L-
  > STAR, IA32_SYSENTER_EIP, IA32_SYSENTER_ESP.
  - #UD - If the LOCK prefix is used.

## Operation

```C
MSR[ECX] := EDX:EAX;
```
