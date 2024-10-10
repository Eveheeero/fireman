# XEND

Transactional End

The instruction marks the end of an RTM code region.
If this corresponds to the outermost scope (that is, including this XEND instruction, the number of XBEGIN instructions is the same as number of XEND instructions), the logical processor will attempt to commit the logical processor state atomically.
If the commit fails, the logical processor will rollback all architectural register and memory updates performed during the RTM execution.
The logical processor will resume execution at the fallback address computed from the outermost XBEGIN instruction.
The EAX register is updated to reflect RTM abort information.Execution of XEND outside a transactional region causes a general-protection exception (#GP).
Execution of XEND while in a suspend read address tracking region causes a transactional abort.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXEND void _xend( void );

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  - #UD - CPUID.(EAX=7, ECX=0):EBX.RTM[bit 11] = 0.

## Operation

```C
XENDIF (RTM_ACTIVE = 0) THENSIGNAL #GPELSEIF SUSLDTRK_ACTIVE = 1THEN GOTO RTM_ABORT_PROCESSING;FI;RTM_NEST_COUNT--IF (RTM_NEST_COUNT = 0) THENTry to commit transactionIF fail to commit transactional executionTHENGOTO RTM_ABORT_PROCESSING;ELSE (* commit success *)RTM_ACTIVE := 0FI;FI;FI;(* For any RTM abort condition encountered during RTM execution *)RTM_ABORT_PROCESSING:Restore architectural register stateDiscard memory updates performed in transactionUpdate EAX with statusRTM_NEST_COUNT := 0RTM_ACTIVE := 0SUSLDTRK_ACTIVE := 0RIP := fallbackRIPELSEEIP := fallbackEIPFI;END
```
