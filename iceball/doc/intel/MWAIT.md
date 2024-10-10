# MWAIT

Monitor Wait

MWAIT instruction provides hints to allow the processor to enter an implementation-dependent optimized state.
There are two principal targeted usages: address-range monitor and advanced power management.
Both usages of MWAIT require the use of the MONITOR instruction.CPUID.01H:ECX.MONITOR[bit 3] indicates the availability of MONITOR and MWAIT in the processor.
When set, MWAIT may be executed only at privilege level 0 (use at any other privilege level results in an invalid-opcode exception).
The operating system or system BIOS may disable this instruction by using the IA32_MISC_ENABLE MSR; disabling MWAIT clears the CPUID feature flag and causes execution to generate an invalid-opcode excep-tion.
This instruction's operation is the same in non-64-bit modes and 64-bit mode.ECX specifies optional extensions for the MWAIT instruction.
EAX may contain hints such as the preferred optimized state the processor should enter.
The first processors to implement MWAIT supported only the zero value for EAX and ECX.
Later processors allowed setting ECX[0] to enable masked interrupts as break events for MWAIT (see below).
Software can use the CPUID instruction to determine the extensions and hints supported by the processor.MWAIT for Address Range MonitoringFor address-range monitoring, the MWAIT instruction operates with the MONITOR instruction.
The two instructions allow the definition of an address at which to wait (MONITOR) and a implementation-dependent-optimized opera-tion to commence at the wait address (MWAIT).
The execution of MWAIT is a hint to the processor that it can enter an implementation-dependent-optimized state while waiting for an event or a store operation to the address range armed by MONITOR.The following cause the processor to exit the implementation-dependent-optimized state: a store to the address range armed by the MONITOR instruction, an NMI or SMI, a debug exception, a machine check exception, the BINIT# signal, the INIT# signal, and the RESET# signal.
Other implementation-dependent events may also cause the processor to exit the implementation-dependent-optimized state.In addition, an external interrupt causes the processor to exit the implementation-dependent-optimized state either (1)if the interrupt would be delivered to software (e.g., as it would be if HLT had been executed instead of MWAIT); or (2)if ECX[0]= 1.
Software can execute MWAIT with ECX[0]= 1 only if CPUID.05H:ECX[bit 1] = 1.
(Implementation-specific conditions may result in an interrupt causing the processor to exit the implementation-dependent-optimized state even if interrupts are masked and ECX[0] = 0.)Following exit from the implementation-dependent-optimized state, control passes to the instruction following the MWAIT instruction.
A pending interrupt that is not masked (including an NMI or an SMI) may be delivered before execution of that instruction.
Unlike the HLT instruction, the MWAIT instruction does not support a restart at the MWAIT instruction following the handling of an SMI.
If the preceding MONITOR instruction did not successfully arm an address range or if the MONITOR instruction has not been executed prior to executing MWAIT, then the MWAIT for Power ManagementMWAIT accepts a hint and optional extension to the processor that it can enter a specified target C state while waiting for an event or a store operation to the address range armed by MONITOR.
Support for MWAIT extensions for power management is indicated by CPUID.05H:ECX[bit 0] reporting 1.
EAX and ECX are used to communicate the additional information to the MWAIT instruction, such as the kind of optimized state the processor should enter.
ECX specifies optional extensions for the MWAIT instruction.
EAX may contain hints such as the preferred optimized state the processor should enter.
Implementation-specific conditions may cause a processor to ignore the hint and enter a different optimized state.
Future processor implementations may implement several optimized "waiting" states and will select among those states based on the hint argument.Table4-10 describes the meaning of ECX and EAX registers for MWAIT extensions.Table 4-10.
 MWAIT Extension Register (ECX)BitsDescription0Treat interrupts as break events even if masked (e.g., even if EFLAGS.IF=0).
May be set only if CPUID.05H:ECX[bit 1] = 1.31: 1ReservedTable 4-11.
 MWAIT Hints Register (EAX)BitsDescription3 : 0Sub C-state within a C-state, indicated by bits [7:4]7 : 4Target C-state*Value of 0 means C1; 1 means C2 and so onValue of 01111B means C0Note: Target C states for MWAIT extensions are processor-specific C-states, not ACPI C-states31: 8ReservedNote that if MWAIT is used to enter any of the C-states that are numerically higher than C1, a store to the address range armed by the MONITOR instruction will cause the processor to exit MWAIT only if the store was originated by other processor agents.
A store from non-processor agent might not cause the processor to exit MWAIT in such cases.® 64 and For additional details of MWAIT extensions, see Chapter 15, "Power and Thermal Management," of IntelIA-32 Architectures Software Developer's Manual, Volume 3A.

## Exceptions

- 64-Bit Mode Exceptions
  > 
  - #GP(0) - If RCX[63:1]
  >  
  > 0.
  > If RCX[0] = 1 and CPUID.05H:ECX[bit 1] = 0.
- Virtual 8086 Mode Exceptions
  - #UD - The MWAIT instruction is not recognized in virtual-8086 mode (even if
  > CPUID.01H:ECX.MONITOR[bit 3] = 1).
- Protected Mode Exceptions
  > 
  - #GP(0) If - ECX[31:1]
  >  0.
  > If ECX[0] = 1 and CPUID.05H:ECX[bit 1] = 0.
  - #UD - If CPUID.01H:ECX.MONITOR[bit 3] = 0.
  > If current privilege level is not 0.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Numeric Exceptions
  > None.
- Real Address Mode Exceptions
  > 
  - #GP If - ECX[31:1]
  >  0.
  > If ECX[0] = 1 and CPUID.05H:ECX[bit 1] = 0.
  - #UD - If CPUID.01H:ECX.MONITOR[bit 3] = 0.

## Operation

```C
(* MWAIT takes the argument in EAX as a hint extension and is architected to take the argument in ECX as an instruction extension MWAIT EAX, ECX *){WHILE ( ("Monitor Hardware is in armed state")) {implementation_dependent_optimized_state(EAX, ECX); }Set the state of Monitor Hardware as triggered;}Intel C/C++ExampleMONITOR/MWAIT instruction pair must be coded in the same loop because execution of the MWAIT instruction will trigger the monitor hardware. It is not a proper usage to execute MONITOR once and then execute MWAIT in a loop. Setting up MONITOR without executing MWAIT has no adverse effects.Typically the MONITOR/MWAIT pair is used in a sequence, such as:EAX = Logical Address(Trigger)ECX = 0 (*Hints *)EDX = 0 (* Hints *)IF ( !trigger_store_happened) {MONITOR EAX, ECX, EDXIF ( !trigger_store_happened ) {MWAIT EAX, ECX}}The above code sequence makes sure that a triggering store does not happen between the first check of the trigger and the execution of the monitor instruction. Without the second check that triggering store would go un-noticed. Typical usage of MONITOR and MWAIT would have the above code sequence within a loop.
```
