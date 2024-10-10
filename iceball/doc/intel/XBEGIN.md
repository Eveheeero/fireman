# XBEGIN

Transactional Begin

The XBEGIN instruction specifies the start of an RTM code region.
If the logical processor was not already in trans-actional execution, then the XBEGIN instruction causes the logical processor to transition into transactional execu-tion.
The XBEGIN instruction that transitions the logical processor into transactional execution is referred to as the outermost XBEGIN instruction.
The instruction also specifies a relative offset to compute the address of the fallback code path following a transactional abort.
(Use of the 16-bit operand size does not cause this address to be trun-cated to 16 bits, unlike a near jump to a relative offset.)On an RTM abort, the logical processor discards all architectural register and memory updates performed during the RTM execution and restores architectural state to that corresponding to the outermost XBEGIN instruction.
The fallback address following an abort is computed from the outermost XBEGIN instruction.Execution of XBEGIN while in a suspend read address tracking region causes a transactional abort.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXBEGIN unsigned int _xbegin( void );

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - If the fallback address is ou
  > tside the address space 0000H and FFFFH.
  - #UD - CPUID.(EAX=7, ECX=0):EBX.RTM[bit 11]=0.
  > If LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP(0) - If the fallback address is ou
  > tside the address space 0000H and FFFFH.
  - #UD - CPUID.(EAX=7, ECX=0):EBX.RTM[bit 11]=0.
  > If LOCK prefix is used.
- SIMD Floating-Point Exceptions
  > None.
- 64-bit Mode Exceptions
  - #UD - CPUID.(EAX=7, ECX=0):EBX.RTM[bit 11] = 0.
- Protected Mode Exceptions
  - #UD - CPUID.(EAX=7, ECX=0):EBX.RTM[bit 11]=0.
  > If LOCK prefix is used.
  - #GP(0) - If the fallback address is outside the CS segment.

## Operation

```C
XBEGINIF RTM_NEST_COUNT < MAX_RTM_NEST_COUNT AND SUSLDTRK_ACTIVE = 0THENRTM_NEST_COUNT++IF RTM_NEST_COUNT = 1 THENIF 64-bit ModeTHENIF OperandSize = 16THEN fallbackRIP := RIP + SignExtend64(rel16);ELSE fallbackRIP := RIP + SignExtend64(rel32);FI;IF fallbackRIP is not canonicalTHEN #GP(0);FI;ELSEIF OperandSize = 16THEN fallbackEIP := EIP + SignExtend32(rel16);ELSE fallbackEIP := EIP + rel32;FI;IF fallbackEIP outside code segment limitTHEN #GP(0);RTM_ACTIVE := 1Enter RTM Execution (* record register state, start tracking memory state*)FI; (* RTM_NEST_COUNT = 1 *)ELSE (* RTM_NEST_COUNT = MAX_RTM_NEST_COUNT OR SUSLDTRK_ACTIVE = 1 *)GOTO RTM_ABORT_PROCESSINGFI;(* For any RTM abort condition encountered during RTM execution *)RTM_ABORT_PROCESSING:Restore architectural register stateDiscard memory updates performed in transactionUpdate EAX with statusRTM_NEST_COUNT := 0RTM_ACTIVE := 0SUSLDTRK_ACTIVE := 0IF 64-bit modeTHENRIP := fallbackRIPELSEEIP := fallbackEIPFI;END
```
