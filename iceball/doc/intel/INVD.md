# INVD

Invalidate Internal Caches

EnModeLeg Mode0F 08INVDZOValidValidFlush internal caches; initiate flushing of external caches.NOTES:1.
See the IA-32 Architecture Compatibility section below.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3Operand 4ZON/AN/AN/AN/AInvalidates (flushes) the processor's internal caches and issues a special-function bus cycle that directs external caches to also flush themselves.
Data held in internal caches is not written back to main memory.
After executing this instruction, the processor does not wait for the external caches to complete their flushing oper-ation before proceeding with instruction execution.
It is the responsibility of hardware to respond to the cache flush signal.The INVD instruction is a privileged instruction.
When the processor is running in protected mode, the CPL of a program or procedure must be 0 to execute this instruction.The INVD instruction may be used when the cache is used as temporary memory and the cache contents need to be invalidated rather than written back to memory.
When the cache is used as temporary memory, no external device should be actively writing data to main memory.
Use this instruction with care.
Data cached internally and not written back to main memory will be lost.
Note that any data from an external device to main memory (for example, via a PCIWrite) can be temporarily stored in the caches; these data can be lost when an INVD instruction is executed.
Unless there is a specific requirement or benefit to flushing caches without writing back modified cache lines (for example, temporary memory, testing, or fault recovery where cache coherency with main memory is not a concern), software should instead use the WBINVD instruction.This instruction's operation is the same in non-64-bit modes and 64-bit mode.IA-32 Architecture CompatibilityThe INVD instruction is implementation dependent; it may be implemented differently on different families of Intel 64 or IA-32 processors.
This instruction is not supported on IA-32 processors earlier than the Intel486 processor.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #GP(0) - The INVD instruction cannot be executed in virtual-8086 mode.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.

## Operation

```C
Flush(InternalCaches);SignalFlush(ExternalCaches);Continue (* Continue execution *)
```
